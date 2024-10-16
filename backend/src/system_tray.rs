use tauri::{App, AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

pub fn setup_system_tray() -> SystemTray {
    
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide launcher");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu);
    
    tray
}

pub fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        // SystemTrayEvent::LeftClick {
        //     position: _,
        //     size: _,
        //     ..
        // } => {
        //     println!("system tray received a left click");
        // }
        // SystemTrayEvent::RightClick {
        //     position: _,
        //     size: _,
        //     ..
        // } => {
        //     println!("system tray received a right click");
        // }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
            if let Some(window) = app.get_window("main") {
                let tray_menu = app.tray_handle().get_item("hide");
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                    tray_menu.set_title("Show launcher").unwrap()
                } else {
                    window.show().unwrap();
                    tray_menu.set_title("Hide launcher").unwrap()
                }
            }
        }
        
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    if let Some(window) = app.get_window("main") {
                        let tray_menu = app.tray_handle().get_item("hide");
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                            tray_menu.set_title("Show launcher").unwrap()
                        } else {
                            window.show().unwrap();
                            tray_menu.set_title("Hide launcher").unwrap()
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

/// Used to update the system tray
/// based on the current state of the application
pub fn update_system_tray(app: &AppHandle) {
    
    // Show/hide the system tray menu item
    let tray_menu = app.tray_handle().get_item("hide");
    if let Some(window) = app.get_window("main") {
        if window.is_visible().unwrap() {
            tray_menu.set_title("Hide launcher").unwrap()
        } else {
            tray_menu.set_title("Show launcher").unwrap()
        }
    }else { 
        tray_menu.set_title("Show launcher").unwrap()
    }
}