/*
Made by: Mathew Dusome
Feb 6 2025
To import you need:
Adds a button object 

In your mod.rs file located in the modules folder add the following to the end of the file:
    pub mod image_button;

Then add the following with the use commands:

use crate::modules::image_button::ImageButton;

Then to use this you would put the following above the loop: 
    let btn_image = ImageButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "assets/button.png",
        "assets/button_hover.png",
    ).await;

Then in side the loop you would use:
if btn_image.click() {

}
*/

use macroquad::prelude::*;
use macroquad::texture::Texture2D;

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
}

impl ImageButton {
    pub async fn new(x: f32, y: f32, width: f32, height: f32, texture_path: &str, hover_texture_path: &str) -> Self {
       
        let (texture, transparency_mask, tex_width, tex_height) = set_texture(texture_path).await;
        
        let hover_texture = load_texture(hover_texture_path).await.unwrap();
        let enabled = true;
        hover_texture.set_filter(FilterMode::Linear);
        Self { x, y, width, height, enabled,texture, hover_texture, transparency_mask, tex_width, tex_height }
    }
   
    pub fn click(&self) -> bool {
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
async fn generate_mask(texture_path: &str, width: usize, height: usize) -> Vec<u8> {
    let image = load_image(texture_path).await.unwrap();
    let pixels = image.bytes; // Image pixels in RGBA8 format

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

    mask
}
pub async fn set_texture(texture_path: &str) -> (Texture2D, Vec<u8>, usize, usize) {
    let texture = load_texture(texture_path).await.unwrap();
    texture.set_filter(FilterMode::Linear);
    let tex_width = texture.width() as usize;
    let tex_height = texture.height() as usize;
    let transparency_mask = generate_mask(texture_path, tex_width, tex_height).await;
    return (texture, transparency_mask, tex_width, tex_height);
}
