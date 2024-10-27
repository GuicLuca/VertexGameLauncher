
use std::thread;
use tauri::{App, AppHandle, Manager, PhysicalPosition, RunEvent, Window, WindowEvent};

mod system_tray;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Application setup
        .setup(|app| {
            application_setup(app);
            Ok(())
        })
        // Frontend event handling
        .on_window_event(|window, event| {
            match frontend_event_handler(window, event) {
                Ok(_) => (),
                Err(e) => eprintln!("Error handling window event: {:?}", e),
            }
        })
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
        },
        _ => {
            // println!("Unhandled event: {:?}", event);
        }
    }
}


/// Handle frontend events
fn frontend_event_handler(window: &Window, event: &WindowEvent) -> Result<(), Box<dyn std::error::Error>>{
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            window.hide().unwrap();
            system_tray::update_tray_menu(window.app_handle())?;
            api.prevent_close();

            Ok(())
        },
        _ => {
            Ok(())
        }
    }
}


/// Perform the application setup
/// run on startup
fn application_setup(app: &mut App) {

    let splashscreen_window = app.get_webview_window("splashscreen").expect("No window labeled `splashscreen` found.");
    let monitor = splashscreen_window.primary_monitor().expect("No primary monitor found.").expect("No primary monitor found.");

    // Center the splashscreen window
    let splashscreen_size = splashscreen_window.outer_size().expect("No outer size found.");
    let splashscreen_position: PhysicalPosition<u32> = PhysicalPosition {
        x: (monitor.size().width - splashscreen_size.width) / 2,
        y: (monitor.size().height - splashscreen_size.height) / 2
    };

    match splashscreen_window.set_position::<PhysicalPosition<u32>>(splashscreen_position) {
        Ok(_) => {},
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

    let main_window = app.get_webview_window("main").expect("No window labeled `main` found.");
    // Perform the initialization code on a new task so the app doesn't freeze
    tauri::async_runtime::spawn(async move {
        // initialize the app here
        println!("Initializing...");
        thread::sleep(std::time::Duration::from_millis(500));
        println!("Done initializing.");

        // After it's done, close the splashscreen and display the main window
        splashscreen_window.close().unwrap();
        main_window.show().unwrap();
        system_tray::update_tray_menu(main_window.app_handle()).unwrap();
    });
}