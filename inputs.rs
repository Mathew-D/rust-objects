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
    cursor_index: usize,
    cursor_timer: f32,
    cursor_visible: bool,
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
            cursor_index: 0,
            cursor_timer: 0.0,
            cursor_visible: true,
        }
    }
    
    pub fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            self.active = mx >= self.x && mx <= self.x + self.width && my >= self.y && my <= self.y + self.height;
    
            if self.active {
                let font_size = 20.0;
                let mut current_x = self.x + 5.0;
                self.cursor_index = 0;
    
                for (i, c) in self.text.char_indices() {
                    let w = measure_text(&c.to_string(), None, font_size as u16, 1.0).width;
                    if mx < current_x + w / 2.0 {
                        break;
                    }
                    current_x += w;
                    self.cursor_index = i + c.len_utf8();
                }
    
                // If mouse is past end, snap to end
                if mx > current_x {
                    self.cursor_index = self.text.len();
                }
            }
        }
    
        if self.active {
            while let Some(character) = get_char_pressed() {
                if character == '\u{8}' {
                    // Backspace
                    if self.cursor_index > 0 {
                        let prev_index = self.text[..self.cursor_index]
                            .char_indices()
                            .rev()
                            .next()
                            .map(|(i, _)| i)
                            .unwrap_or(0);
                        self.text.replace_range(prev_index..self.cursor_index, "");
                        self.cursor_index = prev_index;
                    }
                } else {
                    self.text.insert(self.cursor_index, character);
                    self.cursor_index += character.len_utf8();
                }
            }
    
            if is_key_pressed(KeyCode::Left) && self.cursor_index > 0 {
                self.cursor_index = self.text[..self.cursor_index]
                    .char_indices()
                    .rev()
                    .next()
                    .map(|(i, _)| i)
                    .unwrap_or(0);
            }
    
            if is_key_pressed(KeyCode::Right) && self.cursor_index < self.text.len() {
                self.cursor_index += self.text[self.cursor_index..]
                    .char_indices()
                    .next()
                    .map(|(_i, c)| c.len_utf8())
                    .unwrap_or(0);
            }
    
          
    
            if is_key_pressed(KeyCode::Home) {
                self.cursor_index = 0;
            }
    
            if is_key_pressed(KeyCode::End) {
                self.cursor_index = self.text.len();
            }
    
            self.cursor_timer += get_frame_time();
            if self.cursor_timer >= 0.5 {
                self.cursor_visible = !self.cursor_visible;
                self.cursor_timer = 0.0;
            }
        } else {
            self.cursor_visible = false;
        }
    }
        
    

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, LIGHTGRAY);
    
        let font_size = 20.0;
        let text_x = self.x + 5.0;
        let text_y = self.y + self.height / 2.0 + 5.0;
    
        draw_text(&self.text, text_x, text_y, font_size, BLACK);
    
        if self.active && self.cursor_visible {
            let cursor_text = &self.text[..self.cursor_index];
            let cursor_offset = measure_text(cursor_text, None, font_size as u16, 1.0).width;
            draw_line(
                text_x + cursor_offset,
                text_y - font_size,
                text_x + cursor_offset,
                text_y + 5.0,
                2.0,
                BLACK,
            );
        }
    
        let border_color = if self.active { BLUE } else { DARKGRAY };
        draw_rectangle_lines(self.x, self.y, self.width, self.height, 2.0, border_color);
    }
    
    
}
