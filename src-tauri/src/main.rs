// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_app(path: &str) -> Result<(), String> {
    let path = std::path::Path::new(path);
    std::process::Command::new("open")
        .arg("-a")
        .arg(path.as_os_str())
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn main() {
    let system_tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("Version", "Version 0.0.0".to_string()).disabled())
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new(
            "Launch on Login",
            "Launch on Login".to_string(),
        ))
        .add_item(CustomMenuItem::new(
            "Show in Dock",
            "Show in Dock".to_string(),
        ))
        .add_item(CustomMenuItem::new(
            "Edit Shortcuts",
            "Edit Shortcuts".to_string(),
        ))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("Github", "Github".to_string()))
        .add_item(CustomMenuItem::new("Quit", "Quit".to_string()));
    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(|app, event| match event {
            tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "Edit Shortcuts" => {
                    app.show().expect("failed to show app");
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                "Quit" => app.exit(0),
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet, open_app])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                event.window().hide().unwrap();
            }
            _ => {
                println!("Window event: {:?}", event.event())
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
