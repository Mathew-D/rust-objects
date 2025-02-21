/*
Made by: Mathew Dusome
Feb 6 2025
To import you need:
Adds a button object 
mod objects {
    pub mod img_button;
}
use objects::img_button::ImageButton;

Then to use you would go: 
    let img_button = ImageButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "assets/button.png",
        "assets/button_hover.png",
    ).await;

Then:
if img_button.click() {

}
*/

use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub struct ImageButton {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    texture: Texture2D,
    hover_texture: Texture2D,
    transparency_mask: Vec<bool>, // Stores transparency data
    tex_width: usize,
    tex_height: usize,
}

impl ImageButton {
    pub async fn new(x: f32, y: f32, width: f32, height: f32, texture_path: &str, hover_texture_path: &str) -> Self {
        let texture = load_texture(texture_path).await.unwrap();
        let hover_texture = load_texture(hover_texture_path).await.unwrap();

        texture.set_filter(FilterMode::Linear);
        hover_texture.set_filter(FilterMode::Linear);

        let tex_width = texture.width() as usize;
        let tex_height = texture.height() as usize;

        // Create transparency mask manually
        let transparency_mask = generate_mask(texture_path, tex_width, tex_height).await;

        Self { x, y, width, height, texture, hover_texture, transparency_mask, tex_width, tex_height }
    }

    pub fn click(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Vec2::new(mouse_x, mouse_y);

        let rect = Rect::new(self.x, self.y, self.width, self.height);
        let is_hovered = rect.contains(mouse_pos);

        let texture_to_draw = if is_hovered && self.is_hovered(mouse_x, mouse_y) {
            &self.hover_texture
        } else {
            &self.texture
        };

        draw_texture_ex(
            texture_to_draw,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.height)),
                ..Default::default()
            },
        );

        is_hovered && is_mouse_button_pressed(MouseButton::Left)
    }

    fn is_hovered(&self, mouse_x: f32, mouse_y: f32) -> bool {
        let tex_x = ((mouse_x - self.x) * self.tex_width as f32 / self.width) as usize;
        let tex_y = ((mouse_y - self.y) * self.tex_height as f32 / self.height) as usize;

        if tex_x < self.tex_width && tex_y < self.tex_height {
            let pixel_idx = tex_y * self.tex_width + tex_x;
            return !self.transparency_mask[pixel_idx];
        }

        false
    }
}

// ✅ Works for Web and Native by loading the image as raw bytes
async fn generate_mask(texture_path: &str, width: usize, height: usize) -> Vec<bool> {
    let image = load_image(texture_path).await.unwrap();
    let pixels = image.bytes; // Image pixels in RGBA8 format

    let mut mask = vec![false; width * height];

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4; // Each pixel is 4 bytes (RGBA)
            let alpha = pixels[idx + 3]; // Get alpha channel
            mask[y * width + x] = alpha == 0; // True if transparent
        }
    }

    mask
}
