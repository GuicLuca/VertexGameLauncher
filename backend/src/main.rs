// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod system_tray;

use std::thread;
use tauri::{App, AppHandle, GlobalWindowEvent, Manager, PhysicalPosition, RunEvent, WindowEvent};


// Register all commands and event listeners
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            application_setup(app);
            Ok(())
        })
        .system_tray(system_tray::setup_system_tray())
        .on_system_tray_event(|app, event| {
            system_tray::on_system_tray_event(app, event);
        })
        .on_window_event(|event| {
            frontend_event_handler(&event);
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            backend_event_handler(app, event);
        });
}



/// Perform the application setup
/// run on startup
fn application_setup(app: &mut App) {
    let splashscreen_window = app.get_window("splashscreen").expect("No window labeled `splashscreen` found.");
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

    let main_window = app.get_window("main").expect("No window labeled `main` found.");

    // Perform the initialization code on a new task so the app doesn't freeze
    tauri::async_runtime::spawn(async move {
        // initialize the app here
        println!("Initializing...");
        thread::sleep(std::time::Duration::from_secs(4));
        println!("Done initializing.");

        // After it's done, close the splashscreen and display the main window
        splashscreen_window.close().unwrap();
        main_window.show().unwrap();
    });
}

