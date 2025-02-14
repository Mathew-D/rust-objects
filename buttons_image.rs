/*
Made by: Mathew Dusome
Feb 6 2025
To import you need:
Adds a button object 
mod objects {
    pub mod img_buttons;
}
use objects::img_buttons::ImageButton;

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
// Custom struct for ImageButton
pub struct ImageButton {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    texture: Texture2D,
    hover_texture: Texture2D, // Texture for mouseover
}

impl ImageButton {
    // New function loads textures asynchronously
    pub async fn new(x: f32, y: f32, width: f32, height: f32, texture_path: &str, hover_texture_path: &str) -> Self {
        let texture = load_texture(texture_path).await.unwrap();
        let hover_texture = load_texture(hover_texture_path).await.unwrap();
        texture.set_filter(FilterMode::Linear);
        hover_texture.set_filter(FilterMode::Linear);

        Self { x, y, width, height, texture, hover_texture }
    }

    pub fn click(&self) -> bool {
        // Get mouse position
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Vec2::new(mouse_x, mouse_y);

        // Check if mouse is over the button
        let rect = Rect::new(self.x, self.y, self.width, self.height);
        let is_hovered = rect.contains(mouse_pos);

        // Draw the correct texture depending on mouseover state
        let texture_to_draw = if is_hovered {
            &self.hover_texture
        } else {
            &self.texture
        };

        // Draw the image button
          draw_texture_ex(
            texture,
            x, // x position
            y, // y position
            WHITE, // Tint color (use WHITE to keep original colors)
            DrawTextureParams {
                dest_size: Some(vec2(width, height)), // Scale to fit
                ..Default::default()
            },
        );

        // After drawing, check if the button was clicked
        is_hovered && is_mouse_button_pressed(MouseButton::Left)
    }
}

