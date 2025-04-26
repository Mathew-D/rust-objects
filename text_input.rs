/*
Made by: Mathew Dusome
April 26 2025
To import you need:
Adds a text input object
At the top of your main.rs file you need to add:
    mod objects {
        pub mod text_input;
    }

Add with the other use statements
    use objects::text_input::TextBox;

Then to use you would go:
    let mut textbox = TextBox::new(100.0, 100.0, 300.0, 40.0,50.0);
Where the numbers are x, y, width, height, font size
You can also set the colors of the text box by using:
    .with_colors(WHITE, BLUE, DARKGRAY, RED);
Where the colors are text color, border color, background color, and cursor color respectively.

Then in the loop you would do:

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
    pub cursor_index: usize,
    pub cursor_timer: f32,
    pub cursor_visible: bool,
    pub font_size: f32, // Font size field
    pub text_color: Color, // Text color field
    pub border_color: Color, // Border color field
    pub background_color: Color, // Background color field
    pub cursor_color: Color, // Cursor color field
}

impl TextBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32, font_size: f32) -> Self {
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
            font_size,
            text_color: BLACK, // Default color for text
            border_color: DARKGRAY, // Default color for border
            background_color: LIGHTGRAY, // Default color for background
            cursor_color: BLACK, // Default color for cursor
        }
    }
    
    // Add a method to change colors
    #[allow(unused)]
    pub fn with_colors(mut self, text_color: Color, border_color: Color, background_color: Color, cursor_color: Color) -> Self {
        self.text_color = text_color;
        self.border_color = border_color;
        self.background_color = background_color;
        self.cursor_color = cursor_color;
        self
    }

    pub fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            self.active = mx >= self.x && mx <= self.x + self.width && my >= self.y && my <= self.y + self.height;
    
            if self.active {
                // Clicking to place the cursor
                let text_x = self.x + 5.0;
                let mouse_pos = mx - text_x;
                self.cursor_index = 0;
    
                let mut cursor_offset = 0.0;
                while self.cursor_index < self.text.len() {
                    let char_width = measure_text(&self.text[self.cursor_index..self.cursor_index + 1], None, self.font_size as u16, 1.0).width;
                    cursor_offset += char_width;
                    if cursor_offset > mouse_pos {
                        break;
                    }
                    self.cursor_index += self.text[self.cursor_index..].chars().next().unwrap().len_utf8();
                }
            }
        }
    
        if self.active {
            // Handle typing
            while let Some(c) = get_char_pressed() {
                if !c.is_control() {
                    self.text.insert(self.cursor_index, c);
                    self.cursor_index += c.len_utf8();
                }
            }
    
            // Handle Delete
            if is_key_pressed(KeyCode::Delete) && self.cursor_index < self.text.len() {
                if let Some((_, c)) = self.text[self.cursor_index..].char_indices().next() {
                    let char_len = c.len_utf8();
                    self.text.replace_range(self.cursor_index..self.cursor_index + char_len, "");
                }
            }
    
            // Handle Backspace
            if is_key_pressed(KeyCode::Backspace) && self.cursor_index > 0 {
                if let Some((prev_offset, _c)) = self.text[..self.cursor_index].char_indices().rev().next() {
                    self.text.replace_range(prev_offset..self.cursor_index, "");
                    self.cursor_index = prev_offset;
                }
            }
    
            // Handle Arrow Keys (Left and Right)
            if is_key_pressed(KeyCode::Left) && self.cursor_index > 0 {
                let prev_char = self.text[..self.cursor_index].chars().last().unwrap();
                let char_len = prev_char.len_utf8();
                self.cursor_index -= char_len;
            }
    
            if is_key_pressed(KeyCode::Right) && self.cursor_index < self.text.len() {
                let next_char = self.text[self.cursor_index..].chars().next().unwrap();
                let char_len = next_char.len_utf8();
                self.cursor_index += char_len;
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
        let padding = 5.0;
        let text_x = self.x + padding;
        let text_y = self.y + self.height / 2.0 + self.font_size / 2.5;
    
        // Draw the background and border with customizable colors
        draw_rectangle(self.x, self.y, self.width, self.height, self.background_color);
        draw_text(&self.text, text_x, text_y, self.font_size, self.text_color);
    
        if self.active && self.cursor_visible {
            let mut cursor_offset = 0.0;
            if self.cursor_index > 0 {
                let cursor_text = &self.text[..self.cursor_index];
                for c in cursor_text.chars() {
                    cursor_offset += measure_text(&c.to_string(), None, self.font_size as u16, 1.0).width;
                }
            }
    
            // Add a small spacing between the text and cursor (2.0 pixels)
            let cursor_spacing = 2.0;
            
            // Draw the cursor with customizable color and added spacing
            draw_line(
                text_x + cursor_offset + cursor_spacing,
                text_y - self.font_size,
                text_x + cursor_offset + cursor_spacing,
                text_y + 5.0,
                2.0,
                self.cursor_color,
            );
        }
    
        // Draw the border with customizable color
        draw_rectangle_lines(self.x, self.y, self.width, self.height, 2.0, self.border_color);
    }
}

