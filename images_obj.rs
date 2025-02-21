/*
Made by: Mathew Dusome
Feb 6 2025
To import you need:
Adds a image object 
mod objects {
    pub mod images_obj;
}
use objects::image_obj::ImageObject;

Then to use you would go: 
    let img = image_obj::new(
        "assets/button.png",
        100.0,
        200.0,
        200.0,
        60.0,
    ).await;
*/
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub struct ImageObject {
    texture: Texture2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    transparency_mask: Vec<u8>, // Now storing raw transparency data (bitmask)
}

impl ImageObject {
    // Constructor for ImageObject with asset path and x, y location
    pub async fn new(asset_path: &str, width: f32, height: f32, x: f32, y: f32) -> Self {
        let texture = load_texture(asset_path).await.unwrap(); // Load the texture from the asset path
        texture.set_filter(FilterMode::Nearest); // Set the filter to Nearest directly here
        let tex_width = texture.width() as usize;
        let tex_height = texture.height() as usize;

        // Generate the mask once and store it in the object
        let transparency_mask = generate_mask(asset_path, tex_width, tex_height).await;

        Self { x, y, width, height, texture, transparency_mask }
    }

    // Method to draw the image with current settings
    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.height)),
                ..Default::default()
            },
        );
    }

    // Accessors for image properties
    pub fn pos(&self) -> Vec2 {
        vec2(self.x, self.y)
    }

    pub fn size(&self) -> Vec2 {
        vec2(self.width, self.height)
    }

    pub fn texture_size(&self) -> Vec2 {
        vec2(self.texture.width(), self.texture.height())
    }

    // Get the transparency mask (bitmask)
    pub fn get_mask(&self) -> Vec<u8> {
        return self.transparency_mask.clone();
    }
}

// ✅ Works for Web and Native by loading the image as raw bytes
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
