#![allow(unused_doc_comments)]

use std::collections::HashMap;
use std::sync::Arc;
use log::{error, info};
use serde_json::{json};
use tauri::{App, AppHandle, Builder, Emitter, Manager, RunEvent, Window, WindowEvent, Wry};
use tauri_plugin_fs::FsExt;
use tauri_plugin_http::reqwest;
use tauri_plugin_log::{Target};
use tauri_plugin_store::{JsonValue, Store, StoreExt};
use crate::env::LOCAL_GAME_LIST;
use crate::games::Game;

mod commands;
mod env;
mod errors;
mod system_tray;
mod games;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let tauri_builder: Builder<Wry> = tauri::Builder::default();

    ///### Plugins configuration
    /// For special configurations options, prefer using the env module
    let tauri_builder = tauri_builder.plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_log::Builder::default()
            .targets(
                env::LOG_TARGETS
                    .into_iter()
                    .map(|target| Target::new(target))
                    .collect::<Vec<Target>>()
            )
            .max_file_size(env::LOG_MAX_SIZE)
            .rotation_strategy(env::LOG_ROTATION_STRATEGY)
            .level(env::LOG_LEVEL)
            .level_for("reqwest", log::LevelFilter::Warn)
            .with_colors(*env::LOG_COLORS)
            .timezone_strategy(env::LOG_TIMEZONE)
            .build()
        );
    
    ///### Application setup
    /// The setup function performe initialization tasks on startup such as:
    /// - Setting up the system tray
    /// - Fetching the remote games list from `ONLINE_CONFIGURATION_FILE` URL.
    /// - Saving the games list to the store
    let tauri_builder = tauri_builder.setup(|app| {
        match application_setup(app) {
            Ok(_) => {
                Ok(())
            },
            Err(error) => {
                Err(error.into())
            },
        }
    });
    
    ///### Fronted events bindings
    /// Handle frontend events such as window close requests.<br>
    /// See the **`Application Runing`** section for backend events.
    let tauri_builder = tauri_builder.on_window_event(
        |window, event| match frontend_event_handler(window, event) {
            Ok(_) => (),
            Err(e) => eprintln!("Error handling window event: {:?}", e),
        },
    );
    
    ///### Tauri commands registration
    /// Register the commands that can be called from the frontend.
    /// - **Warning**: always specify the crate of the command to prevent conflicts. (The frontend will ignore the prefix and will only use the function name)<br>
    /// - **See** the commands module to see the list of available commands.<br>
    let tauri_builder = tauri_builder.invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::get_launcher_version,
            commands::get_game_list,
            commands::download,
        ]);
    
    ///### Application building
    /// see https://v2.tauri.app documentation for basics app behaviour.
    let tauri_builder = tauri_builder.build(tauri::generate_context!())
        .expect("error while running tauri application");
    
    ///### Application running
    /// Add bindings on backend events.
    let _tauri_builder = tauri_builder.run(|app, event| {
        backend_event_handler(app, event);
    });
}

/// Handle backend events
fn backend_event_handler(_app: &AppHandle, event: RunEvent) {
    match event {
        RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {
            // println!("Unhandled event: {:?}", event);
        }
    }
}

/// Handle frontend events
fn frontend_event_handler(
    window: &Window,
    event: &WindowEvent,
) -> errors::Result<()> {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            window.hide()?;
            system_tray::update_tray_menu(window.app_handle())?;
            api.prevent_close();

            Ok(())
        }
        _ => Ok(()),
    }
}

/// Perform the application setup
/// run on startup
fn application_setup(app: &mut App) -> errors::Result<()>
{
    // Allow the app to access the app directory only
    app.fs_scope().allow_directory(app.handle().path().app_data_dir()?, true);

    // Set up the system tray
    match system_tray::setup_system_tray(app) {
        Ok(_) => {
            println!("System tray initialized !");
        }
        Err(error) => {
            println!("Error setting up system tray: {}", error.to_string());
        }
    };


    // Get the splashscreen and main window to show/hide them at the end of the initialization
    let splashscreen_window = app
        .get_webview_window("splashscreen")
        .expect("No window labeled `splashscreen` found.");

    
    let main_window = app
        .get_webview_window("main")
        .expect("No window labeled `main` found.");
    
    let app_handle = app.handle().clone();
    
    /// ## Initialization async task
    /// Perform the initialization code on a new task so the app doesn't freeze. <br>
    /// Add every initialization task that doesn't require access to App here. (Only the app_handle can be moved to the task) <br>
    /// Encapsulate each task in a local scope to avoid keeping references once the task is done.
    tauri::async_runtime::spawn(async move {
        info!("[+] Initializing vertex launcher...");
        // keep references with all the initialisation function scope here for
        // frequently used variables.
        let store: Arc<Store<Wry>>;
        
        /// ### Initialize the store
        /// In this step, ensure that all entries in the store are initialized 
        /// to avoid dealing with null values in the future.
        /// - If the store fails to open, log the error and close the app.
        {
            info!("- Initializing store...");
            match app_handle.store(env::STORE_FILE_NAME) {
                Ok(fetched_store) => {
                    store = fetched_store;
                    
                    if store.get(env::STORE_REMOTE_GAME_LIST_KEY).is_some() == false {
                        store.set(env::STORE_REMOTE_GAME_LIST_KEY, json!([]));
                    }
                    if store.get(env::STORE_LOCAL_GAME_LIST_KEY).is_some() == false {
                        store.set(env::STORE_LOCAL_GAME_LIST_KEY, json!([]));
                    }
                    info!("Store has been initialized.");
                }
                Err(e) => {
                    eprintln!("The store failed to open. {}", e);
                    quit_app(&app_handle);
                    return;
                }
            };
            
        }
        // At this point the store variable is initialized and can be used.

        /// ### Fetch the remote games list
        /// Fetch the remote games list from the ONLINE_CONFIGURATION_FILE URL
        /// and save it to the store. 
        /// - Failing should close the app and log the error if there is no local games list.
        {
            info!("- Fetching remote games list...");
            let has_local_game_list = {
                if let Some(local_game_list) = store.get(env::STORE_LOCAL_GAME_LIST_KEY) {
                    local_game_list.is_array() && local_game_list.as_array().unwrap().len() > 0
                }else {
                    // there is no local game list
                    false
                }
            };

            let result = reqwest::get(env::ONLINE_CONFIGURATION_FILE).await;
            match result {
                Ok(response) => {
                    if response.status().is_success() == false {
                        error!("Error fetching remote games list: {:?}", response.status());
                        if has_local_game_list == false {
                            eprintln!("No local games list found and failed to fetch remote games list. Closing app.");
                            quit_app(&app_handle);
                            return;
                        }
                    }

                    info!("Remote games list fetched successfully.");
                    let response_text = response.text().await.unwrap();
                    
                    info!("Checking remote games list validity...");
                    //TODO: check the validity of the remote games list

                    info!("- Saving games list to store...");
                    store.set(
                        env::STORE_REMOTE_GAME_LIST_KEY,
                        serde_json::from_str::<JsonValue>(&response_text).unwrap_or_else(|e| {
                            error!("Error parsing remote games list: {:?}", e);
                            if has_local_game_list == false {
                                eprintln!("No local games list found and failed to fetch remote games list. Closing app.");
                                quit_app(&app_handle);
                            }
                            // this line will be reached if games list is not found but the local games list is valide
                            json!([])
                        }));

                    info!("Games list saved to store.");
                }
                Err(e) => {
                    error!("Error fetching remote games list: {:?}", e);
                    if has_local_game_list == false {
                        eprintln!("No local games list found and failed to fetch remote games list. Closing app.");
                        quit_app(&app_handle);
                    }
                }
            }
        }
        
        /// ### Load local games list
        /// Load the local games list from the store and save it to the LOCAL_GAME_LIST global variable.
        {
            info!("- Loading local games list...");
            let store_local_game_list = {
                match store.get(env::STORE_LOCAL_GAME_LIST_KEY) {
                    Some(local_game_list) => {
                        local_game_list
                    }
                    None => {
                        json!([])
                    }
                }
            };
            
            let mut global_local_game_list = LOCAL_GAME_LIST.write().await;
            *global_local_game_list = serde_json::from_value::<HashMap<u8, Game>>(store_local_game_list).unwrap_or_else(|e| {
                error!("Error loading local games list: {:?}", e);
                HashMap::new()
            });
            
            info!("Local games list loaded successfully.");
        }
        
        
        /// ### Download games resources
        /// For each game in the remote games list :
        /// - Add them in the local games list if they are not already there.
        /// - Check if local file are at the latest revision and download them if not.
        /// - Save the game to the store.
        {
            info!("- Downloading games resources...");
            let distant_game_list = {
                match store.get(env::STORE_REMOTE_GAME_LIST_KEY) {
                    Some(remote_game_list) => {
                        remote_game_list["games"].as_array().unwrap().to_owned()
                    }
                    None => {
                        eprintln!("No remote games list found. Closing app.");
                        quit_app(&app_handle);
                        return;
                    }
                }
            };
            
            let mut global_local_game_list = LOCAL_GAME_LIST.write().await;

            for raw_remote_game in distant_game_list {
                let remote_game = match Game::initialize_game_from_json(&raw_remote_game) {
                    Ok(game) => {game}
                    Err(e) => {
                        error!("Error creating a game struct with {:?}\nError:\n{:?}", raw_remote_game, e);
                        // Skip the game if it can't be created
                        continue;
                    }
                };

                if global_local_game_list.contains_key(&remote_game.id) == true {
                    // The game is already in the local list then only update it
                    let local_game = global_local_game_list.get_mut(&remote_game.id).unwrap();
                    match Game::update_game(&app_handle, local_game, &remote_game).await {
                        Err(e) => {
                            error!("Error updating game resources of {}: {:?}", remote_game.title, e);
                        }
                        _ => {}
                    };
                }else {
                    // The game is not in the local list then download resources and add
                    // the game to the local list
                    let mut new_game = remote_game.clone();
                    match Game::update_game(&app_handle, &mut new_game, &remote_game).await {
                        Err(e) => {
                            error!("Error updating game resources of {}: {:?}", new_game.title, e);
                        }
                        _ => {}
                    }

                    global_local_game_list.insert(new_game.id, new_game);
                }

                info!("Game resources of {} has been downloaded successfully.", remote_game.title);
            }
            
            // Save the local games list to the store
            store.set(env::STORE_LOCAL_GAME_LIST_KEY, serde_json::to_value(&*global_local_game_list).unwrap());
        }
        
        /// ### End of initialization
        /// After all the initialization tasks are done, emit the app_initialized event.
        /// Then close the splashscreen and show the main window.
        info!("[+] Done initializing.");
        // Emit the app_initialized event
        match app_handle.emit("app_initialized", ()) {
            Ok(_) => {}
            Err(e) => {
                error!("Error emitting app_initialized event: {:?}", e);
            }
        };
        

        // After it's done, close the splashscreen and display the main window
        let _ = splashscreen_window.close();
        let _ = main_window.show();
        let _ = system_tray::update_tray_menu(main_window.app_handle());
    });
    
    Ok(())
}

/// Common function to quit the app this function is here
/// to execute some code before quitting the app.
pub fn quit_app(app: &AppHandle) {
    match app.store(env::STORE_FILE_NAME) {
        Ok(store) => {
            store.close_resource();
        }
        Err(e) => {
            eprintln!("Error clearing store: {:?}", e);
        }
    };

    println!("Quitting app...");
    app.cleanup_before_exit();
    std::process::exit(0);
}
