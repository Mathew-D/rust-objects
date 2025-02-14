/*
Made by: Mathew Dusome
Feb 6 2025
To import you need:
Adds a image object 
mod objects {
    pub mod images_obj;
}
use objects::img_buttons::ImageButton;

Then to use you would go: 
    let img = image_obj::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "assets/button.png",
        "assets/button_hover.png",
    ).await;


use macroquad::prelude::*;
use macroquad::texture::Texture2D;
pub struct ImageObject {
    texture: Texture2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl ImageObject {
    // Constructor for ImageObject with asset path and x, y location
    pub async fn new(asset_path: &str, width: f32, height: f32, x: f32, y: f32) -> Self {
        let texture = load_texture(asset_path).await.unwrap(); // Load the texture from the asset path
        texture.set_filter(FilterMode::Nearest); // Set the filter to Nearest directly here

        Self {
            texture,
            x,
            y,
            width,
            height,
        }
    }

    // Method to draw the image with current settings
    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.height)),
                ..Default::default()
            },
        );
    }
}
