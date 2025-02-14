/*
Made by: Mathew Dusome
Feb 6 2025
To import you need:
Adds a button object 
mod objects {
    pub mod buttons;
}
use objects::img_buttons::ImageButton;

Then to use you would go: 
    let text_button = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        img
        hover_img,
    );
Then:
if text_button.click() {

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
    pub fn new(x: f32, y: f32, width: f32, height: f32, texture: Texture2D, hover_texture: Texture2D) -> Self {
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
        draw_texture(texture_to_draw, self.x, self.y, WHITE);

        // After drawing, check if the button was clicked
        is_hovered && is_mouse_button_pressed(MouseButton::Left)
    }
}
