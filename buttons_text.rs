/*
Made by: Mathew Dusome
Feb 6 2025
To import you need:
Adds a button object 
mod objects {
    pub mod txt_buttons;
}
use objects::txt_buttons::TextButton;

Then to use you would go: 
    let text_button = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Click Me".to_string(),
        BLUE,
        GREEN,
    );
Then:
if text_button.click() {

}
*/
use macroquad::prelude::*;

// Custom struct for TextButton
pub struct TextButton {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub text: String,
    pub normal_color: Color,
    pub hover_color: Color,
}

impl TextButton {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: String, normal_color: Color, hover_color: Color) -> Self {
        Self { x, y, width, height, text, normal_color, hover_color }
    }

    pub fn click(&self) -> bool {
        // Get mouse position
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Vec2::new(mouse_x, mouse_y);

        // Check if mouse is over the button
        let rect = Rect::new(self.x, self.y, self.width, self.height);
        let is_hovered = rect.contains(mouse_pos);

        // Draw the text button (change color on hover)
        let button_color = if is_hovered {
            self.hover_color
        } else {
            self.normal_color
        };

        draw_rectangle(self.x, self.y, self.width, self.height, button_color);

        // Draw the text
        let text_width = measure_text(&self.text, None, 30, 1.0).width;
        draw_text(
            &self.text,
            self.x + (self.width / 2.0) - (text_width / 2.0),
            self.y + (self.height / 2.0),
            30.0,
            WHITE,
        );

        // After drawing, check if the button was clicked
        is_hovered && is_mouse_button_pressed(MouseButton::Left)
    }
}
