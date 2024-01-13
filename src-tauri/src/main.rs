// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    process::exit,
    sync::{Arc, Mutex},
};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

mod core;

use core::{start_recording, CaptureHandles};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn make_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("toggle_recording".to_string(), "Start Recording");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);
    return tray;
}

fn main() {
    let is_capturing = Arc::new(Mutex::new(false));
    let handles: Arc<Mutex<Option<CaptureHandles>>> = Arc::new(Mutex::new(None));

    tauri::Builder::default()
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .system_tray(make_tray())
        .on_system_tray_event({
            let is_capturing = Arc::clone(&is_capturing);
            let handles = Arc::clone(&handles);

            move |app, event| match event {
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    // get a handle to the clicked menu item
                    // note that `tray_handle` can be called anywhere,
                    // just get an `AppHandle` instance with `app.handle()` on the setup hook
                    // and move it to another function or thread

                    let mut is_capturing = is_capturing.lock().unwrap();
                    let mut handles = handles.lock().unwrap();

                    let item_handle = app.tray_handle().get_item(&id);
                    match id.as_str() {
                        "quit" => {
                            std::process::exit(0);
                        }
                        "toggle_recording" => {
                            if *is_capturing {
                                item_handle.set_title("Start Recording").unwrap();
                                if let Some(ref mut handles) = *handles {
                                    handles.stop_recording()
                                }
                                *is_capturing = false;
                            } else {
                                item_handle.set_title("Stop Recording").unwrap();
                                *handles = Some(start_recording());
                                *is_capturing = true;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
