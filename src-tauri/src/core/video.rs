use ffmpeg_next::{
    format, format::Pixel, media, software::scaling, util::frame::video::Video,
};
use image::{DynamicImage, ImageBuffer, Rgb};

pub fn extract_frames_from_video(
    video_path: &str,
    frame_numbers: &[i64],
) -> Result<Vec<DynamicImage>, ffmpeg_next::Error> {
    ffmpeg_next::init()?;

    let mut images: Vec<DynamicImage> = vec![];

    let mut ictx = format::input(&video_path)?;
    let input_stream = ictx
        .streams()
        .best(media::Type::Video)
        .ok_or(ffmpeg_next::Error::StreamNotFound)?;
    let video_stream_index = input_stream.index();

    let context_decoder =
        ffmpeg_next::codec::context::Context::from_parameters(input_stream.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;
    println!("Trying to scale...");
    let mut scaler = scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        scaling::Flags::BILINEAR,
    )?;

    let mut receive_and_process_frames =
        |decoder: &mut ffmpeg_next::decoder::Video| -> Result<Video, ffmpeg_next::Error> {
            let mut fni: usize = 0;

            let mut decoded = Video::empty();
            let mut vi: i64 = 0;
            if let Some(&last_frame) = frame_numbers.last() {
                let mut rgb_frame = Video::empty();
                while vi <= last_frame && decoder.receive_frame(&mut decoded).is_ok() {
                    if vi == frame_numbers[fni] {
                        scaler.run(&decoded, &mut rgb_frame)?;
                        // next frame in list...
                        fni += 1;
                    }
                    vi += 1;
                }
                return Ok(rgb_frame);
            };
            Err(ffmpeg_next::Error::Unknown)
        };

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            let frames = receive_and_process_frames(&mut decoder)?;
            for (i, _) in frame_numbers.iter().enumerate() {
                let frame = frames.data(i);
                let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(
                    decoder.width() as u32,
                    decoder.height() as u32,
                    frame.to_vec(),
                )
                .ok_or("Failed to create image from buffer")
                .unwrap();
                images.push(DynamicImage::ImageRgb8(img));
            }
        }
    }

    Ok(images)
}
