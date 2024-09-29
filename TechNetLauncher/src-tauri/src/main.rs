// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use tauri::{Manager, PhysicalPosition, Window};
use tauri::utils::config::Position;
// Close splash screen on loading complete
// #[tauri::command]
// async fn close_splashscreen(window: Window) {
//   // Close the splash screen
//   window.get_window("splashscreen").expect("No window labeled `splashscreen` found.").close().unwrap();
//   // Show the main window
//   window.get_window("main").expect("No window labeled `main` found.").show().unwrap();
// }


// Register all commands and event listeners
fn main() {
    tauri::Builder::default()
        .setup(|app| {
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
            Ok(())
        })
        //.invoke_handler(tauri::generate_handler![close_splashscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



