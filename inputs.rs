/*
Made by: Mathew Dusome
Feb 6 2025
To import you need:
Adds a text input object 
mod objects {
    pub mod text_input;
}
use objects::text_input::TextBox;

Then to use you would go: 
let mut textbox = TextBox::new(100.0, 100.0, 300.0, 40.0);
    
    textbox.update();
    textbox.draw();
*/
use macroquad::prelude::*;

pub struct TextBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub text: String,
    pub active: bool,
}
impl TextBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            text: String::new(),
            active: false,
        }
    }
    pub fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            self.active = mx >= self.x && mx <= self.x + self.width && my >= self.y && my <= self.y + self.height;
        }

        if self.active {
            while let Some(character) = get_char_pressed() {
                if character == '\u{8}' {
                    self.text.pop();
                } else {
                    self.text.push(character);
                }
            }
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, LIGHTGRAY);
        draw_text(&self.text, self.x + 5.0, self.y + self.height / 2.0 + 5.0, 20.0, BLACK);
        if self.active {
            draw_rectangle_lines(self.x, self.y, self.width, self.height, 2.0, BLUE);
        } else {
            draw_rectangle_lines(self.x, self.y, self.width, self.height, 2.0, DARKGRAY);
        }
    }
}
