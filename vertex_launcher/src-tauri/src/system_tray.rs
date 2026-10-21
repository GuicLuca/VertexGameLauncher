use crate::quit_app;
use log::trace;
use tauri::menu::{Menu, MenuEvent, MenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Manager};

pub fn setup_system_tray(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu_on_left_click(false)
        .on_menu_event(|app, event| match on_tray_menu_event(app, event) {
            Ok(_) => (),
            Err(e) => eprintln!("Error handling tray menu event: {:?}", e),
        })
        .on_tray_icon_event(|tray, event| match on_tray_icon_event(tray, &event) {
            Ok(_) => (),
            Err(e) => eprintln!("Error handling tray icon event: {:?}", e),
        })
        .build(app)?;

    update_tray_menu(app.handle())?;

    Ok(())
}

pub fn update_tray_menu(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Create menu elements
    let hide_item = MenuItem::with_id(app, "hide", "Hide launcher", true, None::<&str>)?;
    if let Some(window) = app.get_webview_window("main") {
        if !window.is_visible().unwrap() {
            hide_item.set_text("Show launcher")?;
        } else {
            hide_item.set_text("Hide launcher")?;
        }
    }

    // Quit
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    // Combine menu elements into a menu
    let menu = Menu::with_items(app, &[&hide_item, &quit_item])?;

    // Add the menu to the tray
    Ok(app.tray_by_id("tray").unwrap().set_menu(Some(menu))?)
}

pub fn on_tray_menu_event(
    app: &AppHandle,
    event: MenuEvent,
) -> Result<(), Box<dyn std::error::Error>> {
    trace!("Event tray menu received : {}", event.id.as_ref());
    match event.id.as_ref() {
        "quit" => {
            quit_app(app);
            Ok(())
        }
        "hide" => {
            trace!("Hide menu item clicked");
            // update window visibility and refresh tray menu
            if let Some(window) = app.get_webview_window("main") {
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                }
            }
            update_tray_menu(app)?;
            Ok(())
        }
        _ => {
            trace!("Unhandled tray menu event: {:?}", event);
            Ok(())
        }
    }
}

fn on_tray_icon_event(
    tray: &TrayIcon,
    event: &TrayIconEvent,
) -> Result<(), Box<dyn std::error::Error>> {
    match event {
        TrayIconEvent::DoubleClick { .. } => {
            trace!("system tray received a double click");
            let app = tray.app_handle();

            if let Some(window) = app.get_webview_window("main") {
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                }
            }

            update_tray_menu(app)?;
            Ok(())
        }
        _ => {
            // other events : Click, Enter, Leave, Move
            Ok(())
        }
    }
}
