use std::{io::Cursor, sync::Arc};

use axum::{
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};

use image::ImageOutputFormat;
use tokio::sync::oneshot;
use tower_http::cors::CorsLayer;

use crate::core::extract_frames_from_video;

#[derive(Clone)]
struct AppState {
    local_data_dir: String,
}

async fn get_frame_handler(
    Path(frame_number): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Bytes) {
    let video_path = format!(
        "{}/2024-01-14 03:52:34.529456 UTC.mp4",
        state.local_data_dir
    );
    match extract_frames_from_video(&video_path, &[frame_number]) {
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
    (StatusCode::NOT_FOUND, Bytes::new())
}

pub async fn start_frame_server(tx: oneshot::Sender<()>, local_data_dir: String) {
    let state = Arc::new(AppState { local_data_dir });

    let app = Router::new()
        .route("/get_frame/:frame_number", get(get_frame_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // Send signal that the server has started
    let _ = tx.send(());
}