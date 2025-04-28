/*
Made by: Mathew Dusome
Feb 6 2025
To import you need:
Adds a button object
In the mod objects section add:
    pub mod txt_buttons;

Then with the other use commands add:
use objects::txt_buttons::TextButton;

Then above the loop section to use you would go:
   
    let text_button = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Click Me",
        BLUE,
        GREEN,
        WHITE,
        30
    );

You can also specify a custom font with:
    text_button.with_font(my_font.clone());
Otherwise the default system font will be used.

Then in the loop you would use:
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
    pub enabled: bool,
    pub normal_color: Color,
    pub hover_color: Color,
    off_color: Color,
    pub text_color: Color,
    pub font_size: u16,
    pub font: Option<Font>, // Store the font directly since Font is Clone
}

impl TextButton {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: impl Into<String>, normal_color: Color, hover_color: Color, text_color: Color,font_size:u16) -> Self {
        let enabled = true;
        let off_color = lerp_color(normal_color, GRAY, 0.5);
        Self {
            x,
            y,
            width,
            height,
            text: text.into(),
            enabled,
            normal_color,
            hover_color,
            off_color,
            text_color,
            font_size,
            font: None, // Default to None (use system font)
        }
    }

    // Method to set custom font - taking Font by value since it implements Clone
    #[allow(unused)]
    pub fn with_font(&mut self, font: Font) -> &mut Self {
        self.font = Some(font);
        self
    }

    pub fn click(&self) -> bool {
        // Get mouse position
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = Vec2::new(mouse_x, mouse_y);

        // Check if mouse is over the button
        let rect = Rect::new(self.x, self.y, self.width, self.height);
        let is_hovered = rect.contains(mouse_pos);

        // Draw the text button (change color on hover)
        let button_color = if self.enabled {
            if is_hovered {
                self.hover_color
            } else {
                self.normal_color
            }
        } else {
            self.off_color
        };

        draw_rectangle(self.x, self.y, self.width, self.height, button_color);

        // Calculate text dimensions based on the chosen font
        let text_width = match &self.font {
            Some(font) => measure_text(&self.text, Some(font), self.font_size, 1.0).width,
            None => measure_text(&self.text, None, self.font_size, 1.0).width,
        };

        // Draw the text with the appropriate font
        match &self.font {
            Some(font) => {
                draw_text_ex(
                    &self.text,
                    self.x + (self.width / 2.0) - (text_width / 2.0),
                    self.y + (self.height / 2.0),
                    TextParams {
                        font: Some(font),
                        font_size: self.font_size,
                        color: self.text_color,
                        ..Default::default()
                    },
                );
            },
            None => {
                // Use the default draw_text function
                draw_text(
                    &self.text,
                    self.x + (self.width / 2.0) - (text_width / 2.0),
                    self.y + (self.height / 2.0),
                    self.font_size.into(),
                    self.text_color,
                );
            }
        }

        // After drawing, check if the button was clicked
        is_hovered && self.enabled && is_mouse_button_pressed(MouseButton::Left)
    }
}
fn lerp_color(c1: Color, c2: Color, factor: f32) -> Color {
    Color::new(c1.r * (1.0 - factor) + c2.r * factor, c1.g * (1.0 - factor) + c2.g * factor, c1.b * (1.0 - factor) + c2.b * factor, 1.0)
}
