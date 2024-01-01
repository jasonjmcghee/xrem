use chrono::Utc;
use image::DynamicImage;
use rusty_tesseract::{image_to_string, Args, Image};
use screenshots::Screen;
use std::io::Cursor;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

mod db;
mod embed;
mod video;

const FRAME_BUFFER_SIZE: usize = 30;
const SCREENSHOT_INTERVAL: Duration = Duration::from_secs(2);
const OCR_THREAD_POOL_SIZE: usize = 4;
const IMAGE_ENCODE_THREADS: usize = 4;

fn start_recording() -> thread::JoinHandle<()> {
    let config_path = "models/gte-small/config.json";
    let tokenizer_path = "models/gte-small/tokenizer.json";
    let weights_path = "models/gte-small/model.safetensors";

    // Initialize the model first
    embed::init_model(config_path, tokenizer_path, weights_path, false);

    let frame_buffer = Arc::new((Mutex::new(Vec::new()), Condvar::new()));
    let ocr_pool = ThreadPool::new(OCR_THREAD_POOL_SIZE);

    // Capture thread
    let buffer_clone = frame_buffer.clone();
    thread::spawn(move || {
        capture_screenshots(buffer_clone, &ocr_pool).expect("Error capturing screenshots");
    });

    return thread::spawn(move || {
        // Main thread for processing frames
        let (buffer, cvar) = &*frame_buffer;
        loop {
            let mut frames = buffer.lock().unwrap();
            while frames.len() < FRAME_BUFFER_SIZE {
                frames = cvar.wait(frames).unwrap();
            }

            // Drain frames and process with FFmpeg
            let frames_to_process = frames.drain(..).collect::<Vec<_>>();
            stream_to_ffmpeg(frames_to_process);
        }
    });
}

fn capture_screenshots(
    frame_buffer: Arc<(Mutex<Vec<DynamicImage>>, Condvar)>,
    ocr_pool: &ThreadPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let screens = Screen::all()?;
    let screen = screens.first().unwrap();

    loop {
        let buffer = screen.capture()?;
        let image = DynamicImage::ImageRgba8(buffer.clone());

        // Send image to OCR thread pool
        let image_clone = image.clone();
        ocr_pool.execute(move || {
            let _ocr_result = match perform_ocr(&image_clone) {
                Ok(result) => {
                    // Embed the recognized text!
                    let embeddings = embed::generate_embeddings(&result);
                    println!("Embeddings length: {}", embeddings.len);
                    result
                }
                Err(e) => {
                    println!("OCR Failed! {:?}", e);
                    return;
                }
            };

            // Here's where we'll write to the DB
        });

        let (lock, cvar) = &*frame_buffer;
        let mut frames = lock.lock().unwrap();
        frames.push(image);

        if frames.len() >= FRAME_BUFFER_SIZE {
            println!("buffer size met!! {:?}", frames.len());
            cvar.notify_one();
        }

        thread::sleep(SCREENSHOT_INTERVAL);
    }
}

fn perform_ocr(dynamic_image: &DynamicImage) -> Result<String, Box<dyn std::error::Error>> {
    let args = Args::default();
    let image = Image::from_dynamic_image(dynamic_image).unwrap();

    // OCR
    let text = image_to_string(&image, &args)?;
    println!("OCR: {}", text);

    Ok(text)
}

fn stream_to_ffmpeg(frames: Vec<DynamicImage>) {
    let encode_pool = ThreadPool::new(IMAGE_ENCODE_THREADS); // Define NUM_ENCODE_THREADS based on your CPU
    print!("getting ready to stream..");
    let time = Utc::now();
    let output_name = format!("{}.mp4", time);
    let mut child = Command::new("ffmpeg")
        .args([
            "-f",
            "image2pipe",
            "-vcodec",
            "png",
            "-i",
            "-",
            //"-vcodec",
            //"h264_videotoolbox",
            "-vcodec",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            "-crf",
            "25",
            &output_name,
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start FFmpeg");

    print!("opened stdin...");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let (sender, receiver) = channel();

    print!("ready to write...");

    for frame in frames {
        let sender = sender.clone();

        encode_pool.execute(move || {
            let mut cursor = Cursor::new(Vec::new());

            frame
                .write_to(&mut cursor, image::ImageOutputFormat::Png)
                .expect("Failed to write frame to buffer");

            sender
                .send(cursor.into_inner())
                .expect("Failed to send png buffer.");
        });
    }

    drop(sender);

    for png_buffer in receiver {
        stdin
            .write_all(&png_buffer)
            .expect("Failed to write to stdin");
    }

    println!("finished writing to stdin");

    stdin.flush().expect("Failed to flush stdin");

    println!("flushed");
    drop(stdin);

    println!("dropped");
    let _ = child.wait().expect("FFmpeg process wasn't running");
    println!("waited?");
}

fn main() {
    start_recording().join();
}
