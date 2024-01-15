// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::core::DatabaseManager;
use core::{start_recording, CaptureHandles};
use std::{
    fs,
    sync::{Arc, Mutex},
};
use tauri::{
    AppHandle, CustomMenuItem, LogicalPosition, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem, SystemTrayMenuItemHandle,
};
use tokio::sync::oneshot;

mod core;
mod server;

fn start_server(local_data_dir: String, db: Arc<Mutex<Option<DatabaseManager>>>) {
    println!("starting server...");
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        server::start_frame_server(tx, local_data_dir.to_string(), db.clone()).await;
    });
    // Wait for the server to start
    // let _ = rx.await;
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

fn ensure_local_data_dir(app_handle: AppHandle) -> Result<String, ()> {
    let local_data_dir = app_handle.path_resolver().app_local_data_dir();
    if let Some(dir) = local_data_dir.clone() {
        let path = dir.to_string_lossy().to_string();
        if let Ok(()) = fs::create_dir_all(path.clone()) {
            return Ok(path);
        }
    }
    Err(())
}

fn setup_db(local_data_dir: String, db: Arc<Mutex<Option<DatabaseManager>>>) {
    let mut db = db.lock().unwrap();
    let db_ = DatabaseManager::new(&format!("{}/db.sqlite", local_data_dir)).unwrap();
    *db = Some(db_);
}

#[tokio::main]
async fn main() {
    println!("starting app...");
    let is_capturing = Arc::new(Mutex::new(false));
    let handles: Arc<Mutex<Option<CaptureHandles>>> = Arc::new(Mutex::new(None));
    let db: Arc<Mutex<Option<DatabaseManager>>> = Arc::new(Mutex::new(None));

    let db_setup_ref = db.clone();
    let db_system_tray_ref = db.clone();

    tauri::Builder::default()
        .setup(move |app| {
            let path = ensure_local_data_dir(app.app_handle()).unwrap_or_else(|_| {
                panic!("Failed to create local data dir");
            });
            setup_db(path.clone(), db_setup_ref.clone());
            start_server(path.clone(), db_setup_ref.clone());
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
            let db = db_system_tray_ref.clone();
            move |app, event| match event {
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    let item_handle = app.tray_handle().get_item(&id);
                    match id.as_str() {
                        "quit" => std::process::exit(0),
                        "toggle_recording" => {
                            toggle_recording(
                                app,
                                db.clone(),
                                is_capturing.clone(),
                                handles.clone(),
                                &item_handle,
                            );
                        }
                        "toggle_search" => {
                            toggle_search(app, &item_handle);
                        }
                        "toggle_timeline" => {
                            toggle_timeline(app, item_handle);
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

fn toggle_timeline(app: &AppHandle, item_handle: SystemTrayMenuItemHandle) -> tauri::Result<()> {
    let timeline = app.get_window("timeline").unwrap();
    if !hide_timeline(app).unwrap_or(false) {
        if let Some(monitor) = timeline.current_monitor().unwrap() {
            hide_search(app)?;
            let size = monitor.size();
            let scale_factor = monitor.scale_factor();
            timeline.set_size(size.to_logical::<u32>(scale_factor))?;
            timeline.set_position(LogicalPosition::new(0.0, 0.0))?;
            timeline.show()?;
            item_handle.set_title("Close Timeline").unwrap();
        }
    }
    Ok(())
}

fn toggle_search(app: &AppHandle, item_handle: &SystemTrayMenuItemHandle) -> tauri::Result<()> {
    // If search is visible, hide it, otherwise, show it and hide everything else
    if !hide_search(app).unwrap_or(false) {
        let search = app.get_window("search").unwrap();
        if let Some(monitor) = search.current_monitor().unwrap() {
            hide_timeline(app)?;
            let size = monitor.size();
            let scale_factor = monitor.scale_factor();
            search.set_size(size.to_logical::<u32>(scale_factor))?;
            search.set_position(LogicalPosition::new(0.0, 0.0))?;
            search.show()?;
            item_handle.set_title("Close Search").unwrap();
        }
    }
    Ok(())
}

fn hide_search(app: &AppHandle) -> tauri::Result<bool> {
    let search = app.get_window("search").unwrap();
    if search.is_visible().unwrap() {
        search.hide()?;
        app.tray_handle()
            .get_item("toggle_search")
            .set_title("Open Search")
            .unwrap();
        return Ok(true);
    }
    Ok(false)
}

fn hide_timeline(app: &AppHandle) -> tauri::Result<bool> {
    let timeline = app.get_window("timeline").unwrap();
    if timeline.is_visible().unwrap() {
        timeline.hide()?;
        app.tray_handle()
            .get_item("toggle_timeline")
            .set_title("Open Timeline")
            .unwrap();
        return Ok(true);
    }
    Ok(false)
}

fn stop_recording(
    app: &AppHandle,
    is_capturing: Arc<Mutex<bool>>,
    handles: Arc<Mutex<Option<CaptureHandles>>>,
) -> tauri::Result<bool> {
    let mut is_capturing = is_capturing.lock().unwrap();
    let mut handles = handles.lock().unwrap();
    if *is_capturing {
        if let Some(ref mut handles) = *handles {
            handles.stop_recording()
        }
        *is_capturing = false;
        app.tray_handle()
            .get_item("toggle_recording")
            .set_title("Start Recording")
            .unwrap();
        return Ok(true);
    }
    Ok(false)
}

fn toggle_recording(
    app: &AppHandle,
    db: Arc<Mutex<Option<DatabaseManager>>>,
    is_capturing: Arc<Mutex<bool>>,
    handles: Arc<Mutex<Option<CaptureHandles>>>,
    item_handle: &SystemTrayMenuItemHandle,
) -> tauri::Result<()> {
    if !stop_recording(app, is_capturing.clone(), handles.clone()).unwrap_or(false) {
        hide_timeline(app)?;
        hide_search(app)?;

        let local_data_dir = app.path_resolver().app_local_data_dir();
        if let Some(dir) = local_data_dir.clone() {
            let path = dir.to_string_lossy().to_string();

            let mut is_capturing = is_capturing.lock().unwrap();
            let mut handles = handles.lock().unwrap();
            *handles = Some(start_recording(path, db));
            *is_capturing = true;
            item_handle.set_title("Stop Recording").unwrap();
        }
    }

    Ok(())
}
