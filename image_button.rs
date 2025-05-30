/*
Made by: Mathew Dusome
Feb 6 2025
Program Details: Adds a button object with image support

To import you need:
In your mod.rs file located in the modules folder add the following to the end of the file:
    pub mod image_button;

Then add the following with the use commands:
    use crate::modules::image_button::ImageButton;

Usage examples:
1. Create an image button:
    let btn_image = ImageButton::new(
        100.0,  // x position
        200.0,  // y position
        200.0,  // width
        60.0,   // height
        "assets/button.png",        // normal state image
        "assets/button_hover.png",  // hover state image
    ).await;

2. Check for clicks in your game loop:
    if btn_image.click() {
        // Handle button click
    }

3. Change button images:
    // Change both normal and hover images
    btn_image.set_image(
        "assets/new_button.png",
        "assets/new_button_hover.png"
    ).await;

4. Use with preloaded textures from TextureManager:
    if let (Some(normal_preloaded), Some(hover_preloaded)) = (
        texture_manager.get_preload("assets/new_button.png"),
        texture_manager.get_preload("assets/new_button_hover.png")
    ) {
        btn_image.set_preload(normal_preloaded, hover_preloaded);
    }
*/

use macroquad::prelude::*;
use macroquad::texture::Texture2D;
#[cfg(feature = "scale")]
use crate::modules::scale::mouse_position_world as mouse_position;

pub struct ImageButton {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub enabled: bool,
    texture: Texture2D,
    hover_texture: Texture2D,
    transparency_mask: Vec<u8>, // Stores transparency data
    tex_width: usize,
    tex_height: usize,
    pub visible: bool,
    filename: String, // Adding filename field to track the current texture path
}

impl ImageButton {
    pub async fn new(x: f32, y: f32, width: f32, height: f32, texture_path: &str, hover_texture_path: &str) -> Self {
       
        let (texture, transparency_mask, tex_width, tex_height) = set_texture(texture_path).await;
        
        let hover_texture = load_texture(hover_texture_path).await.unwrap();
        let enabled = true;
        hover_texture.set_filter(FilterMode::Linear);
        Self { x, y, width, height, enabled,texture, hover_texture, transparency_mask, tex_width, tex_height, visible: true, filename: texture_path.to_string() }
    }
   
    /// Method to set new images for the button
    pub async fn set_image(&mut self, texture_path: &str, hover_texture_path: &str) {
        // Update normal texture
        let (texture, transparency_mask, tex_width, tex_height) = set_texture(texture_path).await;
        self.texture = texture;
        self.transparency_mask = transparency_mask;
        self.tex_width = tex_width;
        self.tex_height = tex_height;
        
        // Update hover texture
        let hover_texture = load_texture(hover_texture_path).await.unwrap();
        hover_texture.set_filter(FilterMode::Linear);
        self.hover_texture = hover_texture;

        // Update the filename
        self.filename = texture_path.to_string();
    }

    /// Set button textures from preloaded images for both normal and hover states
    #[allow(unused)]
    pub fn set_preload(
        &mut self, 
        normal_preloaded: (Texture2D, Option<Vec<u8>>, String),
        hover_preloaded: (Texture2D, Option<Vec<u8>>, String)
    ) {
        // Extract normal texture components
        let (texture, mask_option, filename) = normal_preloaded;
        let (hover_texture, _, _) = hover_preloaded;
        
        // Update normal texture and filename
        self.texture = texture;
        self.filename = filename;
        
        // Update hover texture
        self.hover_texture = hover_texture;
        
        // Update transparency mask and dimensions
        if let Some(mask) = mask_option {
            self.transparency_mask = mask;
            self.tex_width = self.texture.width() as usize;
            self.tex_height = self.texture.height() as usize;
        } else {
            // If no mask is provided, create a default fully opaque mask
            let tex_width = self.texture.width() as usize;
            let tex_height = self.texture.height() as usize;
            let mask_size = (tex_width * tex_height + 7) / 8;
            self.transparency_mask = vec![0xFF; mask_size]; // 0xFF means all bits are 1 (fully opaque)
            self.tex_width = tex_width;
            self.tex_height = tex_height;
        }
    }
   
    pub fn click(&self) -> bool {
        if !self.visible {
            return false; // If the button is not visible, don't process clicks
        }
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Vec2::new(mouse_x, mouse_y);

        let rect = Rect::new(self.x, self.y, self.width, self.height);
        let is_hovered = rect.contains(mouse_pos);

        let texture_to_draw = if self.enabled && is_hovered && self.is_hovered(mouse_x, mouse_y) {
            &self.hover_texture
        } else {
            &self.texture
        };
        //let gray_overlay = Color::new(0.6, 0.6, 0.6, 1.0); // A grayish blend
        
        //draw_texture_ex(texture, x, y, gray_overlay, DrawTextureParams::default());
        let color = if !self.enabled {
            Color::new(0.6, 0.6, 0.6, 1.0) // Grayscale effect (gray overlay)
        } else {
            WHITE // Normal color (no effect)
        };

        draw_texture_ex(
            texture_to_draw,
            self.x,
            self.y,
            color,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.height)),
                ..Default::default()
            },
        );

        is_hovered && self.enabled && is_mouse_button_pressed(MouseButton::Left)
    }

    fn is_hovered(&self, mouse_x: f32, mouse_y: f32) -> bool {
        let tex_x = ((mouse_x - self.x) * self.tex_width as f32 / self.width) as usize;
        let tex_y = ((mouse_y - self.y) * self.tex_height as f32 / self.height) as usize;

        if tex_x < self.tex_width && tex_y < self.tex_height {
            let byte_idx = (tex_y * self.tex_width + tex_x) / 8; // Find byte index
            let bit_idx = (tex_y * self.tex_width + tex_x) % 8; // Find bit index within byte

            // Check the bit value (is it 0 or 1?)
            return (self.transparency_mask[byte_idx] >> (7 - bit_idx)) & 1 == 1; // Non-transparent
        }

        false
    }
}

// ✅ Works for Web and Native by loading the image as raw bytes
async fn generate_mask(texture_path: &str, width: usize, height: usize) -> Option<Vec<u8>> {
    let image = load_image(texture_path).await.unwrap();
    let pixels = image.bytes; // Image pixels in RGBA8 format
    
    // Check if the image format has an alpha channel at all (RGBA)
    // If pixels length isn't divisible by 4, it's not RGBA format
    if pixels.len() != width * height * 4 {
        // No alpha channel, return None immediately
        return None;
    }

    // First, check if the image has any transparency at all
    let mut has_transparency = false;
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

    // If there's no transparency, return None (no mask needed)
    if !has_transparency {
        return None;
    }

    // Image has transparency, create the transparency mask
    let mut mask = vec![0; (width * height + 7) / 8]; // One byte per 8 pixels

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4; // Each pixel is 4 bytes (RGBA)
            let alpha = pixels[idx + 3]; // Get alpha channel
            let byte_idx = (y * width + x) / 8;
            let bit_idx = (y * width + x) % 8;

            // Set bit to 1 if pixel is non-transparent
            if alpha > 0 {
                mask[byte_idx] |= 1 << (7 - bit_idx); // Set the bit to 1 (non-transparent)
            }
        }
    }

    Some(mask)
}

pub async fn set_texture(texture_path: &str) -> (Texture2D, Vec<u8>, usize, usize) {
    let texture = load_texture(texture_path).await.unwrap();
    texture.set_filter(FilterMode::Linear);
    let tex_width = texture.width() as usize;
    let tex_height = texture.height() as usize;
    
    // Generate transparency mask or create a default fully opaque mask if none
    let transparency_mask = generate_mask(texture_path, tex_width, tex_height).await
        .unwrap_or_else(|| {
            // If no transparency is detected, create a fully opaque mask
            // (all bits set to 1, meaning every pixel is clickable)
            let mask_size = (tex_width * tex_height + 7) / 8;
            vec![0xFF; mask_size] // 0xFF means all bits are 1 (fully opaque)
        });
        
    return (texture, transparency_mask, tex_width, tex_height);
}
