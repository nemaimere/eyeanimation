use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use image::GenericImageView;
use std::time::{Duration, Instant};
use std::fs;
use std::path::PathBuf;

const WIDTH: u32 = 480;
const HEIGHT: u32 = 800;
const FRAME_DURATION_MS: u64 = 30;
const FRAMES_DIR: &str = "assets/blink_one_eye";

fn main() -> Result<(), String> {
    // Read all PNG files from the target folder
    let mut frame_paths: Vec<PathBuf> = fs::read_dir(FRAMES_DIR)
        .map_err(|e| format!("Failed to read directory {}: {}", FRAMES_DIR, e))?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.extension().and_then(|s| s.to_str()) == Some("png") {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();

    // Sort frames alphabetically to ensure correct order
    frame_paths.sort();

    if frame_paths.is_empty() {
        return Err(format!("No PNG files found in {}", FRAMES_DIR));
    }

    println!("Found {} frames in {}", frame_paths.len(), FRAMES_DIR);
    println!("Loading frames...");
    let mut frames: Vec<Vec<u8>> = Vec::new();

    for path in &frame_paths {
        let img = image::open(path)
            .map_err(|e| format!("Failed to load {}: {}", path.display(), e))?;
        let (width, height) = img.dimensions();

        if width != WIDTH || height != HEIGHT {
            return Err(format!(
                "Image {} has wrong dimensions: {}x{}, expected {}x{}",
                path.display(), width, height, WIDTH, HEIGHT
            ));
        }

        // Convert image to RGB buffer
        let rgb_img = img.to_rgb8();
        frames.push(rgb_img.into_raw());
        println!("Loaded {}", path.display());
    }

    println!("All frames loaded! Starting animation...");

    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Blink Animation", WIDTH, HEIGHT)
        .position_centered()
        .fullscreen_desktop()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut is_fullscreen = true;
    let texture_creator = canvas.texture_creator();

    // Animation state
    let mut current_frame_index = 0;
    let mut direction = 1i32; // 1 for forward, -1 for backward
    let mut last_frame_time = Instant::now();
    let frame_duration = Duration::from_millis(FRAME_DURATION_MS);
    let mut paused = true;
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
                Event::KeyDown {
                    keycode: Some(Keycode::PageDown),
                    ..
                } => {
                    paused = !paused;
                    println!("{}", if paused { "Paused" } else { "Resumed" });
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    // Toggle fullscreen mode
                    is_fullscreen = !is_fullscreen;
                    let fullscreen_type = if is_fullscreen {
                        sdl2::video::FullscreenType::Desktop
                    } else {
                        sdl2::video::FullscreenType::Off
                    };
                    canvas.window_mut().set_fullscreen(fullscreen_type)
                        .map_err(|e| e.to_string())?;
                    println!("{}", if is_fullscreen { "Fullscreen enabled" } else { "Fullscreen disabled" });
                }
                _ => {}
            }
        }

        let now = Instant::now();


        // Check if it's time to advance to the next frame (only if not paused and not in loop pause)
        if !paused && now.duration_since(last_frame_time) >= frame_duration {
            last_frame_time = now;

            // Advance frame index
            current_frame_index = (current_frame_index as i32 + direction) as usize;

            // Check for ping-pong boundaries
            if current_frame_index >= frames.len() {
                // Hit the end, reverse direction
                direction = -1;
                current_frame_index = frames.len() - 2;
            } else if current_frame_index == 0 && direction == -1 {
                // Back at the start after reversing, start loop pause before going forward again
                direction = 1;
                paused = true;
            }
        }

        // Display the current frame
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
            .map_err(|e| e.to_string())?;

        texture
            .update(None, &frames[current_frame_index], (WIDTH * 3) as usize)
            .map_err(|e| e.to_string())?;

        // Get the window size (which may differ from WIDTH/HEIGHT in fullscreen)
        let (window_width, window_height) = canvas.output_size()
            .map_err(|e| e.to_string())?;

        // Calculate the destination rectangle to maintain aspect ratio
        let image_aspect = WIDTH as f32 / HEIGHT as f32;
        let window_aspect = window_width as f32 / window_height as f32;

        let (dst_width, dst_height) = if window_aspect > image_aspect {
            // Window is wider than image - pillarbox (black bars on sides)
            let scaled_height = window_height;
            let scaled_width = (scaled_height as f32 * image_aspect) as u32;
            (scaled_width, scaled_height)
        } else {
            // Window is taller than image - letterbox (black bars on top/bottom)
            let scaled_width = window_width;
            let scaled_height = (scaled_width as f32 / image_aspect) as u32;
            (scaled_width, scaled_height)
        };

        // Center the image in the window
        let dst_x = (window_width as i32 - dst_width as i32) / 2;
        let dst_y = (window_height as i32 - dst_height as i32) / 2;
        let dst_rect = Rect::new(dst_x, dst_y, dst_width, dst_height);

        canvas.clear();
        canvas.copy(&texture, None, Some(dst_rect))?;
        canvas.present();

        // Small sleep to avoid burning CPU
        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}
