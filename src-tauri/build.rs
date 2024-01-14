use std::env;
use std::path::PathBuf;

fn main() {
    // // Get the manifest directory (location of Cargo.toml)
    // let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    //
    // // Construct the path to the ffmpeg binary
    // let ffmpeg_binary_path = PathBuf::from(manifest_dir)
    //     .join("binaries")
    //     .join("ffmpeg");
    //
    // // Convert the path to a string and add it to the linker search path
    // let ffmpeg_binary_path = ffmpeg_binary_path.to_str().unwrap();
    // println!("cargo:rustc-link-search=native={}", ffmpeg_binary_path);
    tauri_build::build()
}
