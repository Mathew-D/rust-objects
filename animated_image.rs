/*
Made by: Mathew Dusome
Jun 13, 2026
To import you need:
Adds an animated image object for sprite animations with GIF support

In the ui.rs file add the following to the end :
    pub mod animated_image;
    
Then add the following with the use commands:
use crate::ui::animated_image::AnimatedImage;

GIF ANIMATION SUPPORT:
This module now supports animated GIFs on both desktop and web platforms.
For GIF support, ensure this dependency is enabled in Cargo.toml:
[dependencies]
image = { version = "0.25.10", default-features = false, features = ["gif"] }

Then to use this you would put the following above the loop: 
    // Create with a single spritesheet
    let mut animated_sprite = AnimatedImage::new(
        "assets/character_spritesheet.png", 
        100.0, 100.0,                      
        64.0, 64.0,                        
        4, 1,                              
        0.1,                               
        true                               
    ).await;

    Where the options are:
    "assets/character_spritesheet.png" = spritesheet path
    100.0, 100.0 = position
    64.0, 64.0 = size
    4, 1 = grid (cols, rows) in the spritesheet
    0.1 = frame duration in seconds
    true = (loop animation)


    // Or create with individual frames
    let mut animated_sprite2 = AnimatedImage::from_frames(
        vec![
            "assets/frame1.png",
            "assets/frame2.png",
            "assets/frame3.png",
        ],
        200.0, 100.0,   
        64.0, 64.0,     
        0.15,           
        true            
    ).await;
    Where the options are:
    vec! = vector of frame paths
    200.0, 100.0 = position
    64.0, 64.0 = size
    0.15 = frame duration in seconds
    true = (loop animation)


    // Or load directly from a GIF file (works on both web and native platforms)
    let mut gif_sprite = AnimatedImage::from_gif(
        "assets/animation.gif", 
        300.0, 100.0,          
        128.0, 128.0,          
        true                   
    ).await;
    Where the options are:
    "assets/animation.gif" = GIF file path
    300.0, 100.0 = position
    128.0, 128.0 = size
    true = (loop animation)


    // Or build directly from preloaded GIF data (no async):
    let preloaded = texture_manager
        .get_preloaded_animated_gif("assets/animation.gif")
        .unwrap();

    let mut gif_sprite_preloaded = AnimatedImage::from_preloaded_gif(
        preloaded.texture,
        preloaded.transparency_mask,
        preloaded.frame_masks,
        preloaded.frame_delays,
        300.0,
        100.0,
        128.0,
        128.0,
        true,
    );

    Where the options are:
    preloaded.texture = preloaded spritesheet texture
    preloaded.transparency_mask = full texture collision mask
    preloaded.frame_masks = per-frame collision masks
    preloaded.frame_delays = GIF frame timing in seconds
    300.0, 100.0 = position
    128.0, 128.0 = size
    true = (loop animation)

Then inside the loop you would use:
    // Draw the current frame (animation updates automatically!)
    animated_sprite.draw();
    
    // You can also control animation:
    if is_key_pressed(KeyCode::Space) {
        animated_sprite.play();  // Start or resume animation
    }
    
    if is_key_pressed(KeyCode::S) {
        animated_sprite.stop();  // Stop animation
    }
    
    if is_key_pressed(KeyCode::P) {
        animated_sprite.pause(); // Pause animation
    }
    
    if is_key_pressed(KeyCode::R) {
        animated_sprite.reset(); // Reset to first frame
    }

    // For collision detection:
    let collision = check_collision(&animated_sprite, &other_object, 1);
*/

use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use image::ImageDecoder;
use std::io::Cursor;

#[derive(PartialEq)]
#[allow(unused)]
pub enum AnimationState {
    Playing,
    Paused,
    Stopped,
}

pub struct AnimatedImage {
    texture: Texture2D,
    frame_textures: Option<Vec<Texture2D>>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    transparency_mask: Option<Vec<u8>>, // Main transparency mask for the entire spritesheet
    frame_masks: Option<Vec<Vec<u8>>>,  // Individual transparency masks for each frame
    cols: usize,
    #[allow(unused)]
    rows: usize,
    current_frame: usize,
    total_frames: usize,
    frame_width: f32,
    frame_height: f32,
    frame_duration: f32,
    frame_durations: Option<Vec<f32>>, // For variable frame durations (GIF)
    time_accumulated: f32,
    state: AnimationState,
    loop_animation: bool,
    last_update: f32, // Store the last update time
    angle: f32, // Rotation angle
}

impl AnimatedImage {
    // Create from a spritesheet (grid of frames)
    #[allow(unused)]
    pub async fn new(
        spritesheet_path: &str,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        cols: usize,
        rows: usize,
        frame_duration: f32,
        loop_animation: bool,
    ) -> Self {
        let (texture, transparency_mask) = set_texture(spritesheet_path).await;
        
        let frame_width = texture.width() / cols as f32;
        let frame_height = texture.height() / rows as f32;
        let total_frames = cols * rows;
        
        Self {
            texture,
            frame_textures: None,
            x,
            y,
            width,
            height,
            transparency_mask,
            frame_masks: None,
            cols,
            rows,
            current_frame: 0,
            total_frames,
            frame_width,
            frame_height,
            frame_duration,
            frame_durations: None,
            time_accumulated: 0.0,
            state: AnimationState::Playing,
            loop_animation,
            last_update: get_time() as f32,
            angle: 0.0,
        }
    }
    
    // Create from individual frames
    #[allow(unused)]
    pub async fn from_frames(
        frame_paths: Vec<&str>,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        frame_duration: f32,
        loop_animation: bool,
    ) -> Self {
        // For simplicity, we'll create a horizontal spritesheet from all frames
        let frames = frame_paths.len();
        
        // Load the first frame to determine the dimensions
        let first_image = load_image(frame_paths[0]).await.unwrap();
        let frame_width = first_image.width() as f32;
        let frame_height = first_image.height() as f32;
        
        // Create a new image that will contain all frames side by side
        let spritesheet_width = frame_width * frames as f32;
        let spritesheet_height = frame_height;
        
        // Create a new image for our spritesheet
        let mut combined_image = Image::gen_image_color(
            spritesheet_width as u16, 
            spritesheet_height as u16, 
            Color::new(0.0, 0.0, 0.0, 0.0)
        );
        
        // Place each frame in the spritesheet
        for (i, path) in frame_paths.iter().enumerate() {
            let frame_image = load_image(path).await.unwrap();
            let x_offset = i as f32 * frame_width;
            
            // Copy pixels from this frame to our combined image
            for y in 0..frame_image.height() as u32 {
                for x in 0..frame_image.width() as u32 {
                    let pixel_idx = (y * frame_image.width() as u32 + x) as usize * 4;
                    let r = frame_image.bytes[pixel_idx];
                    let g = frame_image.bytes[pixel_idx + 1];
                    let b = frame_image.bytes[pixel_idx + 2];
                    let a = frame_image.bytes[pixel_idx + 3];
                    
                    let dest_x = x_offset as u32 + x;
                    combined_image.set_pixel(
                        dest_x, 
                        y, 
                        Color::new(
                            r as f32 / 255.0, 
                            g as f32 / 255.0, 
                            b as f32 / 255.0, 
                            a as f32 / 255.0
                        )
                    );
                }
            }
        }
        
        // Create transparency mask manually
        let texture_width = combined_image.width() as usize;
        let texture_height = combined_image.height() as usize;
        let mut transparency_mask = vec![0; (texture_width * texture_height + 7) / 8];
        
        for y in 0..texture_height {
            for x in 0..texture_width {
                let idx = (y * texture_width + x) * 4;
                let alpha = combined_image.bytes[idx + 3];
                let mask_byte_idx = (y * texture_width + x) / 8;
                let bit_offset = (y * texture_width + x) % 8;
                
                if alpha > 0 {
                    transparency_mask[mask_byte_idx] |= 1 << (7 - bit_offset);
                }
            }
        }
        
        // Convert the combined image to a texture
        let texture = Texture2D::from_image(&combined_image);
        texture.set_filter(FilterMode::Nearest);
        
        Self {
            texture,
            frame_textures: None,
            x,
            y,
            width,
            height,
            transparency_mask: Some(transparency_mask),
            frame_masks: None,
            cols: frames,
            rows: 1,
            current_frame: 0,
            total_frames: frames,
            frame_width,
            frame_height,
            frame_duration,
            frame_durations: None,
            time_accumulated: 0.0,
            state: AnimationState::Playing,
            loop_animation,
            last_update: get_time()as f32,
            angle: 0.0,
        }
    }
    
    // Create from a GIF file using the image crate (works on both web and native platforms)
    #[allow(unused)]
    pub async fn from_gif(
        gif_path: &str,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        loop_animation: bool,
    ) -> Self {
        // Load the GIF file data
        match load_file(gif_path).await {
            Ok(file_data) => {
                // Try to process the GIF frames directly
                if let Some((frames, delays, width_px, height_px)) = Self::process_gif_data(&file_data) {
                    let frame_count = frames.len();
                    
                    // Create a texture for each decoded frame.
                    let mut frame_textures = Vec::with_capacity(frame_count);

                    // Create masks for each individual frame for pixel-perfect collision
                    let mut frame_masks = Vec::with_capacity(frame_count);
                    
                    for frame in frames.iter() {
                        let frame_data = frame.clone();

                        // Create a frame-specific mask
                        let mut frame_mask = vec![0; (width_px * height_px + 7) / 8];
                        
                        for y in 0..height_px {
                            for x in 0..width_px {
                                let src_idx = (y * width_px + x) * 4;
                                if src_idx + 3 >= frame_data.len() {
                                    continue;
                                }

                                let src_a = frame_data[src_idx + 3];
                                
                                // For the frame mask
                                if src_a > 0 {
                                    let mask_byte_idx = (y * width_px + x) / 8;
                                    let bit_offset = (y * width_px + x) % 8;
                                    frame_mask[mask_byte_idx] |= 1 << (7 - bit_offset);
                                }
                            }
                        }
                        
                        let frame_image = Image {
                            width: width_px as u16,
                            height: height_px as u16,
                                                    bytes: frame_data.clone(),
                        };
                        let frame_texture = Texture2D::from_image(&frame_image);
                        frame_texture.set_filter(FilterMode::Nearest);
                        frame_textures.push(frame_texture);

                        // Add this frame's mask to our collection
                        frame_masks.push(frame_mask);
                    }

                    let texture = frame_textures[0].clone();

                    // Create a global transparency mask for the first frame.
                    let mut transparency_mask = vec![0; (width_px * height_px + 7) / 8];
                    for y in 0..height_px {
                        for x in 0..width_px {
                            let idx = (y * width_px + x) * 4;
                            if frame_textures
                                .first()
                                .and_then(|_| frames[0].get(idx + 3).copied())
                                .unwrap_or(0)
                                > 0
                            {
                                let mask_byte_idx = (y * width_px + x) / 8;
                                let bit_offset = (y * width_px + x) % 8;
                                transparency_mask[mask_byte_idx] |= 1 << (7 - bit_offset);
                            }
                        }
                    }
                    
                    // Calculate default frame duration
                    let default_frame_duration = if !delays.is_empty() {
                        delays.iter().sum::<f32>() / delays.len() as f32
                    } else {
                        0.1 // Default to 10 FPS if no delay info
                    };
                    
                    // Create the AnimatedImage
                    return Self {
                        texture,
                        frame_textures: Some(frame_textures),
                        x,
                        y,
                        width,
                        height,
                        transparency_mask: Some(transparency_mask),
                        frame_masks: Some(frame_masks),
                        cols: frame_count,
                        rows: 1,
                        current_frame: 0,
                        total_frames: frame_count,
                        frame_width: width_px as f32,
                        frame_height: height_px as f32,
                        frame_duration: default_frame_duration,
                        frame_durations: Some(delays),
                        time_accumulated: 0.0,
                        state: AnimationState::Playing,
                        loop_animation,
                        last_update: get_time() as f32,
                        angle: 0.0,
                    };
                } else {
                    // Fall back to loading as a regular texture if GIF processing fails
                    println!("Failed to process GIF frames, falling back to regular texture");
                    return Self::new(gif_path, x, y, width, height, 1, 1, 0.1, loop_animation).await;
                }
            },
            Err(e) => {
                println!("Failed to load GIF file: {}", e);
            }
        }
        
        // Return empty animation if anything fails
        println!("Could not load GIF '{}', returning empty animation", gif_path);
        Self::create_empty(x, y, width, height)
    }

    // Build directly from preloaded raw data — no async or file I/O needed.
    // Pass the fields from a PreloadedAnimatedGif (or any source) directly.
    #[allow(unused)]
    pub fn from_preloaded_gif(
        texture: Texture2D,
        transparency_mask: Option<Vec<u8>>,
        frame_masks: Vec<Vec<u8>>,
        frame_delays: Vec<f32>,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        loop_animation: bool,
    ) -> Self {
        let frame_count = frame_delays.len().max(1);
        let frame_width = texture.width() / frame_count as f32;
        let frame_height = texture.height();
        let default_frame_duration = frame_delays.first().copied().unwrap_or(0.1);

        Self {
            texture,
            frame_textures: None,
            x,
            y,
            width,
            height,
            transparency_mask,
            frame_masks: Some(frame_masks),
            cols: frame_count,
            rows: 1,
            current_frame: 0,
            total_frames: frame_count,
            frame_width,
            frame_height,
            frame_duration: default_frame_duration,
            frame_durations: Some(frame_delays),
            time_accumulated: 0.0,
            state: AnimationState::Playing,
            loop_animation,
            last_update: get_time() as f32,
            angle: 0.0,
        }
    }

    // Process GIF data in a way that works on all platforms including WebAssembly
    #[allow(unused)]
    fn process_gif_data(data: &[u8]) -> Option<(Vec<Vec<u8>>, Vec<f32>, usize, usize)> {
        let decoder = GifDecoder::new(Cursor::new(data)).ok()?;
        let (width, height) = decoder.dimensions();

        let frames = decoder.into_frames().collect_frames().ok()?;
        if frames.is_empty() {
            return None;
        }

        let mut frame_bytes = Vec::with_capacity(frames.len());
        let mut delays = Vec::with_capacity(frames.len());

        for frame in frames {
            let delay: std::time::Duration = frame.delay().into();
            let delay_sec = delay.as_secs_f32().max(0.001);
            delays.push(delay_sec);
            frame_bytes.push(frame.into_buffer().into_raw());
        }

        Some((frame_bytes, delays, width as usize, height as usize))
    }
    
    // Create an empty animation (used as fallback)
    #[allow(unused)]
    fn create_empty(x: f32, y: f32, width: f32, height: f32) -> Self {
        // Create a 1x1 transparent image
        let empty_image = Image::gen_image_color(1, 1, Color::new(0.0, 0.0, 0.0, 0.0));
        let texture = Texture2D::from_image(&empty_image);
        
        Self {
            texture,
            frame_textures: None,
            x,
            y,
            width,
            height,
            transparency_mask: None,
            frame_masks: None,
            cols: 1,
            rows: 1,
            current_frame: 0,
            total_frames: 1,
            frame_width: 1.0,
            frame_height: 1.0,
            frame_duration: 0.1,
            frame_durations: None,
            time_accumulated: 0.0,
            state: AnimationState::Stopped,
            loop_animation: false,
            last_update: 0.0,
            angle: 0.0,
        }
    }
    
    // Calculate source rectangle for the current frame
    fn get_current_frame_rect(&self) -> Rect {
        if self.frame_textures.is_some() {
            return Rect {
                x: 0.0,
                y: 0.0,
                w: self.frame_width,
                h: self.frame_height,
            };
        }

        let frame_row = self.current_frame / self.cols;
        let frame_col = self.current_frame % self.cols;
        
        Rect {
            x: frame_col as f32 * self.frame_width,
            y: frame_row as f32 * self.frame_height,
            w: self.frame_width,
            h: self.frame_height,
        }
    }
    
    // Draw the current animation frame
    pub fn draw(&mut self) {
        if self.total_frames == 0 {
            return;
        }
        
        // Auto-update animation based on elapsed time
        if self.state == AnimationState::Playing && self.total_frames > 1 {
            let current_time = get_time();
            let delta_time = (current_time - self.last_update as f64) as f32;
            self.last_update = current_time as f32;
            
            self.time_accumulated += delta_time;
            
            // Get current frame duration
            let current_duration = if let Some(durations) = &self.frame_durations {
                durations[self.current_frame % durations.len()]
            } else {
                self.frame_duration
            };
            
            // Advance to next frame if needed
            if self.time_accumulated >= current_duration {
                self.time_accumulated -= current_duration;
                self.current_frame += 1;
                
                // Handle end of animation
                if self.current_frame >= self.total_frames {
                    if self.loop_animation {
                        self.current_frame = 0; // Loop back to start
                    } else {
                        self.current_frame = self.total_frames - 1; // Stay on last frame
                        self.state = AnimationState::Stopped;
                    }
                }
            }
        }
        
        let using_frame_textures = self.frame_textures.is_some();
        let texture = if let Some(frame_textures) = &self.frame_textures {
            &frame_textures[self.current_frame]
        } else {
            &self.texture
        };
        let source_rect = if using_frame_textures {
            None
        } else {
            Some(self.get_current_frame_rect())
        };
        
        draw_texture_ex(
            texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                rotation: self.angle,
                dest_size: Some(vec2(self.width, self.height)),
                source: source_rect,
                ..Default::default()
            },
        );
    }
    
    // Animation control methods
    
    // Start or resume animation
    #[allow(unused)]
    pub fn play(&mut self) {
        self.state = AnimationState::Playing;
    }
    
    // Pause animation (maintains current frame)
    #[allow(unused)]
    pub fn pause(&mut self) {
        self.state = AnimationState::Paused;
    }
    
    // Stop animation (resets to first frame)
    #[allow(unused)]
    pub fn stop(&mut self) {
        self.state = AnimationState::Stopped;
        self.current_frame = 0;
        self.time_accumulated = 0.0;
    }
    
    // Reset to first frame without changing state
    #[allow(unused)]
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.time_accumulated = 0.0;
    }
    
    // Set specific frame
    #[allow(unused)]
    pub fn set_frame(&mut self, frame_index: usize) {
        if frame_index < self.total_frames {
            self.current_frame = frame_index;
            self.time_accumulated = 0.0;
        }
    }
    
    // Set animation speed (frame duration in seconds)
    #[allow(unused)]
    pub fn set_speed(&mut self, frame_duration: f32) {
        self.frame_duration = frame_duration.max(0.001); // Prevent division by zero
    }
    
    // Set animation looping
    #[allow(unused)]
    pub fn set_looping(&mut self, loop_animation: bool) {
        self.loop_animation = loop_animation;
    }
    
    // Get current state
    #[allow(unused)]
    pub fn state(&self) -> &AnimationState {
        &self.state
    }
    
    // Get current frame index
    #[allow(unused)]
    pub fn current_frame_index(&self) -> usize {
        self.current_frame
    }
    
    // Get total number of frames
    #[allow(unused)]
    pub fn frame_count(&self) -> usize {
        self.total_frames
    }
    
    // Set position
    #[allow(unused)]
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    #[allow(unused)]
    pub fn set_angle(&mut self, x: f32) {
        self.angle = x;
    }
    #[allow(unused)]
    pub fn get_angle(&self) -> f32 {
        self.angle
    }
    // Get and set x position
    #[allow(unused)]
    pub fn get_x(&self) -> f32 {
        self.x
    }

    #[allow(unused)]
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    // Get and set y position
    #[allow(unused)]
    pub fn get_y(&self) -> f32 {
        self.y
    }

    #[allow(unused)]
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    
    // Set size
    #[allow(unused)]
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
    
    // Is animation finished?
    #[allow(unused)]
    pub fn is_finished(&self) -> bool {
        self.state == AnimationState::Stopped && self.current_frame == self.total_frames - 1
    }
    
    // Required methods for collision detection compatibility
    
    // Get current position
    #[allow(unused)]
    pub fn pos(&self) -> Vec2 {
        vec2(self.x, self.y)
    }
    
    // Get current size
    #[allow(unused)]
    pub fn size(&self) -> Vec2 {
        vec2(self.width, self.height)
    }
    
    // Get texture size
    #[allow(unused)]
    pub fn texture_size(&self) -> Vec2 {
        // For multi-frame animations, we should return the size of a single frame
        // rather than the entire texture (which contains all frames side by side)
        if self.total_frames > 1 {
            vec2(self.frame_width, self.frame_height)
        } else {
            vec2(self.texture.width(), self.texture.height())
        }
    }
    
    // Get transparency mask for collision detection
    #[allow(unused)]
    pub fn get_mask(&self) -> Option<Vec<u8>> {
        // If we have frame-specific masks and there's more than one frame, use the current frame's mask
        if let Some(frame_masks) = &self.frame_masks {
            if self.total_frames > 1 && self.current_frame < frame_masks.len() {
                return Some(frame_masks[self.current_frame].clone());
            }
        }
        
        // Fall back to the global mask if frame-specific masks aren't available
        self.transparency_mask.clone()
    }
}

#[allow(unused)]
async fn generate_mask(texture_path: &str, width: usize, height: usize) -> Option<Vec<u8>> {
    let image = load_image(texture_path).await.unwrap();
    let pixels = image.bytes; // Image pixels in RGBA8 format
    
    // Check if the image format has an alpha channel at all (RGBA)
    // If pixels length isn't divisible by 4, it's not RGBA format
    if pixels.len() != width * height * 4 {
        // No alpha channel, return None immediately
        return None;
    }


    let mut has_transparency = false;

    // First, scan to see if the image has any transparency at all
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4; // Each pixel is 4 bytes (RGBA)
            let alpha = pixels[idx + 3]; // Get alpha channel
            
            if alpha < 255 {
                has_transparency = true;
                break;
            }
        }
        if has_transparency {
            break;
        }
    }

    // If there's no transparency, return None
    if !has_transparency {
        return None;
    }
 // Only create the mask if we know the image has transparency
 let mut mask = vec![0; (width * height + 7) / 8]; // Create a bitmask with enough bytes
    // Otherwise, create the transparency mask
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4; // Each pixel is 4 bytes (RGBA)
            let alpha = pixels[idx + 3]; // Get alpha channel
            let mask_byte_idx = (y * width + x) / 8; // Find which byte this pixel belongs to
            let bit_offset = (y * width + x) % 8; // Find the bit offset inside the byte

            if alpha > 0 {
                // Set the bit to 1 (opaque pixel)
                mask[mask_byte_idx] |= 1 << (7 - bit_offset);
            } else {
                // Set the bit to 0 (transparent pixel)
                mask[mask_byte_idx] &= !(1 << (7 - bit_offset));
            }
        }
    }

    Some(mask)
}

#[allow(unused)]
async fn set_texture(texture_path: &str) -> (Texture2D, Option<Vec<u8>>) {
    let texture = load_texture(texture_path).await.unwrap();
    texture.set_filter(FilterMode::Nearest); // Better for pixel art
    let tex_width = texture.width() as usize;
    let tex_height = texture.height() as usize;
    let transparency_mask = generate_mask(texture_path, tex_width, tex_height).await;
    return (texture, transparency_mask);
}