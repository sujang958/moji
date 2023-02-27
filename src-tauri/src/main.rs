#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process;

use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayEvent, Manager};
use tauri_plugin_autostart::MacosLauncher;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec![])))
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                _app.get_window("main").unwrap().show().expect("Error");     
            }
            SystemTrayEvent::MenuItemClick { id, ..} => {
                if id == "quit" {
                    process::exit(1);
                }
            },
            _ => println!("Helo"), })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
        

    // the tauri_plugin_window_state is not working when decorations: false`
    // also the tauri_plugin_autostart is not working when there's a system tray
    // fuck fuck fuck fuckl  fuck fuck fuck fuck fuckl fuck fuck

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
