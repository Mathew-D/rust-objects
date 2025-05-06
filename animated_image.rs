/*
Made by: Mathew Dusome
April 26 2025
To import you need:
Adds an animated image object for sprite animations with GIF support
In the mod objects section add:
    pub mod animated_image;
    
Then add the following with the use commands:
use crate::objects::animated_image::AnimatedImage;

GIF ANIMATION SUPPORT:
This module now supports animated GIFs on both desktop and web platforms.
For GIF support, add this dependency to Cargo.toml:
[dependencies]
gif = "0.13"

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

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum AnimationState {
    Playing,
    Paused,
    Stopped,
}

pub struct AnimatedImage {
    texture: Texture2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    transparency_mask: Vec<u8>,
    cols: usize,
    #[allow(dead_code)]
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
}

impl AnimatedImage {
    // Create from a spritesheet (grid of frames)
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
            x,
            y,
            width,
            height,
            transparency_mask,
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
        }
    }
    
    // Create from individual frames
    #[allow(dead_code)]
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
            x,
            y,
            width,
            height,
            transparency_mask,
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
        }
    }
    
    // Create from a GIF file using the image crate (works on both web and native platforms)
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
                    
                    // If it's a static image (not animated), create a regular image
                    if frame_count <= 1 {
                        // Just load it as a regular texture
                        return Self::new(gif_path, x, y, width, height, 1, 1, 0.1, loop_animation).await;
                    }
                    
                    // Create combined image as a horizontal strip
                    let spritesheet_width = width_px * frame_count;
                    let spritesheet_height = height_px;
                    
                    // Create the spritesheet image
                    let mut combined_image = Image::gen_image_color(
                        spritesheet_width as u16,
                        spritesheet_height as u16,
                        Color::new(0.0, 0.0, 0.0, 0.0)
                    );
                    
                    // Copy each frame into the spritesheet
                    for (i, frame) in frames.iter().enumerate() {
                        let x_offset = i * width_px;
                        
                        // Copy the frame into the combined image
                        for y in 0..height_px {
                            for x in 0..width_px {
                                let src_idx = (y * width_px + x) * 4;
                                let src_r = frame[src_idx];
                                let src_g = frame[src_idx + 1];
                                let src_b = frame[src_idx + 2];
                                let src_a = frame[src_idx + 3];
                                
                                let dest_x = x_offset + x;
                                combined_image.set_pixel(
                                    dest_x as u32,
                                    y as u32,
                                    Color::new(
                                        src_r as f32 / 255.0,
                                        src_g as f32 / 255.0,
                                        src_b as f32 / 255.0,
                                        src_a as f32 / 255.0
                                    )
                                );
                            }
                        }
                    }
                    
                    // Create transparency mask
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
                    
                    // Calculate default frame duration
                    let default_frame_duration = if !delays.is_empty() {
                        delays.iter().sum::<f32>() / delays.len() as f32
                    } else {
                        0.1 // Default to 10 FPS if no delay info
                    };
                    
                    // Create the AnimatedImage
                    return Self {
                        texture,
                        x,
                        y,
                        width,
                        height,
                        transparency_mask,
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
    
    // Process GIF data in a way that works on all platforms including WebAssembly
    fn process_gif_data(data: &[u8]) -> Option<(Vec<Vec<u8>>, Vec<f32>, usize, usize)> {
        // Try to decode the GIF using the gif crate
        match gif::Decoder::new(data) {
            Ok(mut decoder) => {
                let mut frames = Vec::new();
                let mut delays = Vec::new();
                
                // Get dimensions from global header
                let width = decoder.width() as usize;
                let height = decoder.height() as usize;
                
                // Process each frame
                while let Ok(Some(frame)) = decoder.read_next_frame() {
                    // Calculate delay in seconds (GIF delay is in 1/100 seconds)
                    let delay_sec = frame.delay as f32 / 100.0;
                    delays.push(delay_sec);
                    
                    // Create an RGBA frame from the GIF frame
                    let mut frame_data = vec![0; width * height * 4];
                    
                    // GIF frames have a local rectangle, not necessarily the full image
                    let frame_width = frame.width as usize;
                    let frame_height = frame.height as usize;
                    let frame_left = frame.left as usize;
                    let frame_top = frame.top as usize;
                    
                    // Fill with transparency first
                    for y in 0..height {
                        for x in 0..width {
                            let idx = (y * width + x) * 4;
                            frame_data[idx + 3] = 0; // Alpha = 0 (transparent)
                        }
                    }
                    
                    // Fill in the frame data
                    for y in 0..frame_height {
                        for x in 0..frame_width {
                            let src_idx = y * frame_width + x;
                            let pixel_idx = frame.buffer[src_idx];
                            
                            // Convert to global image coordinates
                            let global_x = frame_left + x;
                            let global_y = frame_top + y;
                            
                            // Skip pixels outside the global image bounds
                            if global_x >= width || global_y >= height {
                                continue;
                            }
                            
                            let dest_idx = (global_y * width + global_x) * 4;
                            
                            // Get color from palette
                            if let Some(palette) = &frame.palette {
                                if pixel_idx > 0 { // Non-transparent pixel
                                    // The palette in the gif crate has RGB bytes for each color
                                    // Each color is stored as [r, g, b] for the palette entry
                                    let color_index = pixel_idx as usize * 3; // Each color is 3 bytes (RGB)
                                    
                                    // Make sure we don't go out of bounds
                                    if color_index + 2 < palette.len() {
                                        frame_data[dest_idx] = palette[color_index];     // R
                                        frame_data[dest_idx + 1] = palette[color_index + 1]; // G
                                        frame_data[dest_idx + 2] = palette[color_index + 2]; // B
                                        frame_data[dest_idx + 3] = 255;      // A (fully opaque)
                                    } else {
                                        // Fallback if palette indexing is wrong
                                        frame_data[dest_idx] = 255;     // R
                                        frame_data[dest_idx + 1] = 255; // G
                                        frame_data[dest_idx + 2] = 255; // B
                                        frame_data[dest_idx + 3] = 255; // A
                                    }
                                }
                            }
                        }
                    }
                    
                    frames.push(frame_data);
                }
                
                if frames.is_empty() {
                    return None;
                }
                
                // If we don't have any valid delays, use default
                if delays.is_empty() || delays.iter().all(|&d| d <= 0.0) {
                    delays = vec![0.1; frames.len()]; // 10 FPS default
                }
                
                return Some((frames, delays, width, height));
            },
            Err(e) => {
                println!("Failed to decode GIF: {}", e);
                
                // Pure macroquad fallback without using the image crate
                // Try to load it as a regular texture and use that as a single frame
                println!("Fallback: trying to load as a regular image");
                return None;
            }
        }
    }
    
    // Create an empty animation (used as fallback)
    fn create_empty(x: f32, y: f32, width: f32, height: f32) -> Self {
        // Create a 1x1 transparent image
        let empty_image = Image::gen_image_color(1, 1, Color::new(0.0, 0.0, 0.0, 0.0));
        let texture = Texture2D::from_image(&empty_image);
        
        Self {
            texture,
            x,
            y,
            width,
            height,
            transparency_mask: vec![0],
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
        }
    }
    
    // Calculate source rectangle for the current frame
    fn get_current_frame_rect(&self) -> Rect {
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
        
        let source_rect = self.get_current_frame_rect();
        
        draw_texture_ex(
            &self.texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.height)),
                source: Some(source_rect),
                ..Default::default()
            },
        );
    }
    
    // Animation control methods
    
    // Start or resume animation
    #[allow(dead_code)]
    pub fn play(&mut self) {
        self.state = AnimationState::Playing;
    }
    
    // Pause animation (maintains current frame)
    #[allow(dead_code)]
    pub fn pause(&mut self) {
        self.state = AnimationState::Paused;
    }
    
    // Stop animation (resets to first frame)
    #[allow(dead_code)]
    pub fn stop(&mut self) {
        self.state = AnimationState::Stopped;
        self.current_frame = 0;
        self.time_accumulated = 0.0;
    }
    
    // Reset to first frame without changing state
    #[allow(dead_code)]
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
        vec2(self.texture.width(), self.texture.height())
    }
    
    // Get transparency mask for collision detection
    #[allow(unused)]
    pub fn get_mask(&self) -> Vec<u8> {
        self.transparency_mask.clone()
    }
}

// Generate transparency mask for collision detection
async fn generate_mask(texture_path: &str, width: usize, height: usize) -> Vec<u8> {
    let image = load_image(texture_path).await.unwrap();
    let pixels = image.bytes; // Image pixels in RGBA8 format

    let mut mask = vec![0; (width * height + 7) / 8]; // Create a bitmask with enough bytes

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

    mask
}

async fn set_texture(texture_path: &str) -> (Texture2D, Vec<u8>) {
    let texture = load_texture(texture_path).await.unwrap();
    texture.set_filter(FilterMode::Nearest); // Better for pixel art
    let tex_width = texture.width() as usize;
    let tex_height = texture.height() as usize;
    let transparency_mask = generate_mask(texture_path, tex_width, tex_height).await;
    return (texture, transparency_mask);
}