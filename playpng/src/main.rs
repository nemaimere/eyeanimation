use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use image::GenericImageView;
use std::time::{Duration, Instant};

const WIDTH: u32 = 480;
const HEIGHT: u32 = 800;
const FRAME_DURATION_MS: u64 = 100;

fn main() -> Result<(), String> {
    // Load all frames into memory
    let frame_paths = [
        "assets/blink_one_eye/blink01.png",
        "assets/blink_one_eye/blink02.png",
        "assets/blink_one_eye/blink03.png",
        "assets/blink_one_eye/blink04.png",
        "assets/blink_one_eye/blink05.png",
        "assets/blink_one_eye/blink06.png",
    ];

    println!("Loading frames...");
    let mut frames: Vec<Vec<u8>> = Vec::new();

    for path in &frame_paths {
        let img = image::open(path)
            .map_err(|e| format!("Failed to load {}: {}", path, e))?;
        let (width, height) = img.dimensions();

        if width != WIDTH || height != HEIGHT {
            return Err(format!(
                "Image {} has wrong dimensions: {}x{}, expected {}x{}",
                path, width, height, WIDTH, HEIGHT
            ));
        }

        // Convert image to RGB buffer
        let rgb_img = img.to_rgb8();
        frames.push(rgb_img.into_raw());
        println!("Loaded {}", path);
    }

    println!("All frames loaded! Starting animation...");

    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Blink Animation", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Animation state
    let mut current_frame_index = 0;
    let mut direction = 1i32; // 1 for forward, -1 for backward
    let mut last_frame_time = Instant::now();
    let frame_duration = Duration::from_millis(FRAME_DURATION_MS);

    let mut event_pump = sdl_context.event_pump()?;

    // Main loop
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let now = Instant::now();

        // Check if it's time to advance to the next frame
        if now.duration_since(last_frame_time) >= frame_duration {
            last_frame_time = now;

            // Advance frame index
            current_frame_index = (current_frame_index as i32 + direction) as usize;

            // Check for ping-pong boundaries
            if current_frame_index >= frames.len() {
                // Hit the end, reverse direction
                direction = -1;
                current_frame_index = frames.len() - 2;
            } else if current_frame_index == 0 && direction == -1 {
                // Back at the start after reversing, go forward again
                direction = 1;
            }
        }

        // Display the current frame
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
            .map_err(|e| e.to_string())?;

        texture
            .update(None, &frames[current_frame_index], (WIDTH * 3) as usize)
            .map_err(|e| e.to_string())?;

        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();

        // Small sleep to avoid burning CPU
        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}
