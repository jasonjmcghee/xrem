// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs,
    sync::{Arc, Mutex},
};
use tauri::{
    CustomMenuItem, LogicalPosition, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
use tokio::sync::oneshot;
use core::{start_recording, CaptureHandles};

mod core;
mod server;


fn start_server(app_handle: tauri::AppHandle) {
    println!("starting server...");
    let local_data_dir = app_handle.path_resolver().app_local_data_dir();
    let (tx, rx) = oneshot::channel();

    if let Some(dir) = local_data_dir.clone() {
        let path = dir.to_string_lossy().to_string();
        if let Ok(()) = fs::create_dir_all(path.clone()) {
            let path_clone = path.clone();
            tokio::spawn(async move {
                server::start_frame_server(tx, path_clone).await;
            });
        }
    }
    println!("started server...");
}

fn make_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let toggle_timeline = CustomMenuItem::new("toggle_timeline".to_string(), "Open Timeline");
    let toggle_search = CustomMenuItem::new("toggle_search".to_string(), "Open Search");
    let record = CustomMenuItem::new("toggle_recording".to_string(), "Start Recording");
    let tray_menu = SystemTrayMenu::new()
        .add_item(record)
        .add_item(toggle_timeline)
        .add_item(toggle_search)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);
    return tray;
}

#[tokio::main]
async fn main() {
    println!("starting app...");
    // Wait for the server to start
    // let _ = rx.await;

    let is_capturing = Arc::new(Mutex::new(false));
    let handles: Arc<Mutex<Option<CaptureHandles>>> = Arc::new(Mutex::new(None));

    tauri::Builder::default()
        .setup(|app| {
            start_server(app.app_handle());
            Ok(())
        })
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
                            let local_data_dir = app.path_resolver().app_local_data_dir();

                            if let Some(dir) = local_data_dir.clone() {
                                if *is_capturing {
                                    if let Some(ref mut handles) = *handles {
                                        handles.stop_recording()
                                    }
                                    *is_capturing = false;
                                    item_handle.set_title("Start Recording").unwrap();
                                } else {
                                    let path = dir.to_string_lossy().to_string();
                                    *handles = Some(start_recording(path));
                                    *is_capturing = true;
                                    item_handle.set_title("Stop Recording").unwrap();
                                }
                            }
                        }
                        "toggle_search" => {
                            let search = app.get_window("search").unwrap();
                            if search.is_visible().unwrap() {
                                search.hide();
                                item_handle.set_title("Open Search").unwrap();
                            } else if let Some(monitor) = search.current_monitor().unwrap() {
                                let timeline = app.get_window("timeline").unwrap();
                                if timeline.is_visible().unwrap() {
                                    timeline.hide();
                                    item_handle.set_title("Open Timeline").unwrap();
                                }
                                let size = monitor.size();
                                let scale_factor = monitor.scale_factor();
                                search.set_size(size.to_logical::<u32>(scale_factor));
                                search.set_position(LogicalPosition::new(0.0, 0.0));
                                search.show();
                                item_handle.set_title("Close Search").unwrap();
                            }
                        }
                        "toggle_timeline" => {
                            let timeline = app.get_window("timeline").unwrap();
                            if timeline.is_visible().unwrap() {
                                timeline.hide();
                                item_handle.set_title("Open Timeline").unwrap();
                            } else if let Some(monitor) = timeline.current_monitor().unwrap() {
                                let search = app.get_window("search").unwrap();
                                if search.is_visible().unwrap() {
                                    search.hide();
                                    item_handle.set_title("Open Timeline").unwrap();
                                }
                                let size = monitor.size();
                                let scale_factor = monitor.scale_factor();
                                timeline.set_size(size.to_logical::<u32>(scale_factor));
                                timeline.set_position(LogicalPosition::new(0.0, 0.0));
                                timeline.show();
                                item_handle.set_title("Close Timeline").unwrap();
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
