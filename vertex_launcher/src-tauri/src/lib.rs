use serde_json::json;
use tauri::{App, AppHandle, Manager, PhysicalPosition, RunEvent, Window, WindowEvent};
use tauri_plugin_http::reqwest;
use tauri_plugin_store::StoreExt;

mod system_tray;
mod env;

static APP_STORE: &'static str = "vertex_store.json";


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        // Application setup
        .setup(|app| {
            application_setup(app);
            Ok(())
        })
        // Frontend event handling
        .on_window_event(
            |window, event| match frontend_event_handler(window, event) {
                Ok(_) => (),
                Err(e) => eprintln!("Error handling window event: {:?}", e),
            },
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        // Backend event handling
        .run(|app, event| {
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
    let monitor = splashscreen_window
        .primary_monitor()
        .expect("No primary monitor found.")
        .expect("No primary monitor found.");

    // Center the splashscreen window
    let splashscreen_size = splashscreen_window
        .outer_size()
        .expect("No outer size found.");
    let splashscreen_position: PhysicalPosition<u32> = PhysicalPosition {
        x: (monitor.size().width - splashscreen_size.width) / 2,
        y: (monitor.size().height - splashscreen_size.height) / 2,
    };

    match splashscreen_window.set_position::<PhysicalPosition<u32>>(splashscreen_position) {
        Ok(_) => {}
        Err(e) => {
            println!("Error setting splashscreen position: {}", e);
        }
    }

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
        println!("Initializing vertex launcher...");
        //thread::sleep(std::time::Duration::from_millis(1500));
        println!("Fetching remote games list...");
        
        let mut has_setup_errors = false;
        
        let result = reqwest::get(env::GAME_SERVER_ADDR).await;
        match result {
            Ok(response) => {
                if !response.status().is_success() {
                    eprintln!("Error fetching remote games list: {:?}", response.status());
                    has_setup_errors = true;
                    quit_app(app_handle);
                    return;
                }
                
                println!("Remote games list fetched successfully.");
                let response_text = response.text().await.unwrap();
                println!("{:?}", response_text);
                println!("Saving games list to store...");
                match app_handle.store(APP_STORE){
                    Ok(store) => {
                        store.set("games", json!(response_text));
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
        
        if has_setup_errors {
            quit_app(app_handle);
            return;
        }
        
        println!("Done initializing.");
        println!("{:?}", app_handle.store(APP_STORE).unwrap().get("games").unwrap());

        // After it's done, close the splashscreen and display the main window
        splashscreen_window.close().unwrap();
        main_window.show().unwrap();
        system_tray::update_tray_menu(main_window.app_handle()).unwrap();
    });
}

/// Common function to quit the app this function is here 
/// to execute some code before quitting the app.
fn quit_app(app: AppHandle) {
    match app.store(APP_STORE) {
        Ok(store) => {
            store.close_resource();
        }
        Err(e) => {
            eprintln!("Error clearing store: {:?}", e);
        }
    }
    
    app.exit(0);
}
