use std::sync::Mutex;
use std::{io::Cursor, sync::Arc};

use axum::{
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use image::ImageOutputFormat;
use serde::Serialize;
use tokio::sync::oneshot;
use tower_http::cors::CorsLayer;

use crate::core::{extract_frames_from_video, DatabaseManager};

#[derive(Clone)]
struct AppState {
    local_data_dir: String,
    db: Arc<Mutex<Option<DatabaseManager>>>,
}

#[derive(Serialize)]
struct FrameInfo {
    max_frame: i64,
}

// TODO: Optimize this to do chunk loading, instead of starting from scratch with the
// frame every single time
// TODO: Also, cache the frames in memory using an LRU cache
async fn get_frame_handler(
    Path(frame_number): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Bytes) {
    let db_video_ref = state.db.clone();
    let maybe_video_path = {
        let mut db_clone = db_video_ref.lock().unwrap();
        db_clone
            .as_mut()
            .unwrap()
            .get_frame(frame_number)
            .expect("Failed to get frame")
    };
    if let Some((offset_index, video_path)) = maybe_video_path {
        match extract_frames_from_video(&video_path, &[offset_index]) {
            Ok(frames) => {
                if let Some(frame) = frames.into_iter().next() {
                    let mut cursor = Cursor::new(Vec::new());
                    if frame.write_to(&mut cursor, ImageOutputFormat::Png).is_ok() {
                        return (StatusCode::OK, Bytes::from(cursor.into_inner()));
                    }
                }
            }
            _ => {}
        }
    }
    (StatusCode::NOT_FOUND, Bytes::new())
}

async fn get_max_frame_handler(State(state): State<Arc<AppState>>) -> Json<FrameInfo> {
    let db_video_ref = state.db.clone();
    let max_frame = {
        let mut db_clone = db_video_ref.lock().unwrap();
        db_clone
            .as_mut()
            .unwrap()
            .get_max_frame()
            .expect("Failed to get max frame")
    };
    Json(FrameInfo { max_frame })
}

pub async fn start_frame_server(
    tx: oneshot::Sender<()>,
    local_data_dir: String,
    db: Arc<Mutex<Option<DatabaseManager>>>,
) {
    let state = Arc::new(AppState { local_data_dir, db });

    let app = Router::new()
        .route("/frames/max", get(get_max_frame_handler))
        .route("/frames/:frame_number", get(get_frame_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // Send signal that the server has started
    let _ = tx.send(());
}
