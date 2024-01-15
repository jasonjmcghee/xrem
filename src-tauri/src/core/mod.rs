mod core;
mod db;
mod embed;
mod video;

pub use core::start_recording;
pub use core::CaptureHandles;
pub use db::DatabaseManager;
pub use video::extract_frames_from_video;
