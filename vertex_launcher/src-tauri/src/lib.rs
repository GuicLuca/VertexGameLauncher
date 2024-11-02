#![allow(unused_doc_comments)]

use serde_json::json;
use tauri::{App, AppHandle, Builder, Emitter, Manager, RunEvent, Window, WindowEvent, Wry};
use tauri_plugin_http::reqwest;
use tauri_plugin_log::{Target};
use tauri_plugin_store::StoreExt;

mod commands;
mod env;
mod errors;
mod system_tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let tauri_builder: Builder<Wry> = tauri::Builder::default();

    ///### Plugins configuration
    /// For special configurations options, prefer using the env module
    let tauri_builder = tauri_builder.plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_http::init())
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
        application_setup(app);
        Ok(())
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
            commands::get_game_list
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
) -> Result<(), Box<dyn std::error::Error>> {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            window.hide().unwrap();
            system_tray::update_tray_menu(window.app_handle())?;
            api.prevent_close();

            Ok(())
        }
        _ => Ok(()),
    }
}

/// Perform the application setup
/// run on startup
fn application_setup(app: &mut App) {
    let splashscreen_window = app
        .get_webview_window("splashscreen")
        .expect("No window labeled `splashscreen` found.");
    // let monitor = splashscreen_window
    //     .primary_monitor()
    //     .expect("No primary monitor found.")
    //     .expect("No primary monitor found.");
    // 
    // // Center the splashscreen window
    // let splashscreen_size = splashscreen_window
    //     .outer_size()
    //     .expect("No outer size found.");
    // let splashscreen_position: PhysicalPosition<u32> = PhysicalPosition {
    //     x: (monitor.size().width - splashscreen_size.width) / 2,
    //     y: (monitor.size().height - splashscreen_size.height) / 2,
    // };
    // 
    // match splashscreen_window.set_position::<PhysicalPosition<u32>>(splashscreen_position) {
    //     Ok(_) => {}
    //     Err(e) => {
    //         println!("Error setting splashscreen position: {}", e);
    //     }
    // }

    // Set up the system tray
    match system_tray::setup_system_tray(app) {
        Ok(_) => {
            println!("System tray initialized !");
        }
        Err(error) => {
            println!("Error setting up system tray: {}", error.to_string());
        }
    };

    let main_window = app
        .get_webview_window("main")
        .expect("No window labeled `main` found.");

    let app_handle = app.handle().clone();
    // Perform the initialization code on a new task so the app doesn't freeze
    tauri::async_runtime::spawn(async move {
        // initialize the app here
        println!("[+] Initializing vertex launcher...");
        //thread::sleep(std::time::Duration::from_millis(1500));
        println!("- Fetching remote games list...");

        let mut has_setup_errors = false;

        let result = reqwest::get(env::ONLINE_CONFIGURATION_FILE).await;
        match result {
            Ok(response) => {
                if !response.status().is_success() {
                    eprintln!("Error fetching remote games list: {:?}", response.status());
                    // Following line is commented out because it's not used due to the direct app closing.
                    //has_setup_errors = true;
                    quit_app(app_handle);
                    return;
                }

                println!("Remote games list fetched successfully.");
                let response_text = response.text().await.unwrap();

                println!("- Saving games list to store...");
                match app_handle.store(env::STORE_FILE_NAME) {
                    Ok(store) => {
                        store.set(env::STORE_GAME_LIST_KEY, json!(response_text));
                        println!("Games list saved to store.");
                    }
                    Err(e) => {
                        eprintln!("Error saving games list to store: {:?}", e);
                        has_setup_errors = true;
                    }
                };
            }
            Err(e) => {
                eprintln!("Error fetching remote games list: {:?}", e);
                has_setup_errors = true;
            }
        }

        if has_setup_errors == true {
            quit_app(app_handle);
            return;
        }

        match app_handle.emit("app_initialized", ()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error emitting app_initialized event: {:?}", e);
            }
        };
        
        println!("- Done initializing.");
        

        // After it's done, close the splashscreen and display the main window
        splashscreen_window.close().unwrap();
        main_window.show().unwrap();
        system_tray::update_tray_menu(main_window.app_handle()).unwrap();
    });
}

/// Common function to quit the app this function is here
/// to execute some code before quitting the app.
pub fn quit_app(app: AppHandle) {
    match app.store(env::STORE_FILE_NAME) {
        Ok(store) => {
            store.close_resource();
        }
        Err(e) => {
            eprintln!("Error clearing store: {:?}", e);
        }
    };

    println!("Quitting app...");
    std::process::exit(0);
}
