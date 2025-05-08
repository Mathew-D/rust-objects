/*
Made by: Mathew Dusome
May 2 2025
Adds a text input object

In your mod.rs file located in the modules folder add the following to the end of the file
        pub mod text_input;


Add with the other use statements
    use crate::modules::text_input::TextBox;

Then to use this you would put the following above the loop: 
    let mut textbox = TextBox::new(100.0, 100.0, 300.0, 40.0, 50.0);
Where the parameters are x, y, width, height, font size

You can customize the text box using various methods:

APPEARANCE CUSTOMIZATION:
    // Set colors (text, border, background, cursor)
    textbox.with_colors(WHITE, BLUE, DARKGRAY, RED);
    
    // Set individual colors
    textbox.set_text_color(WHITE)
          .set_border_color(BLUE)
          .set_background_color(DARKGRAY)
          .set_cursor_color(RED);
    
    // Set custom font
    textbox.with_font(my_font.clone());
    
    // Change position and dimensions
    textbox.set_position(150.0, 150.0);
    textbox.set_dimensions(250.0, 50.0);
    
TEXT MANIPULATION:
    // Get current text
    let current_text = textbox.get_text();
    
    // Set text content
    textbox.set_text("Hello World");
    
    // Check active state
    if textbox.is_active() {
        // Do something when textbox is active
    }
    
    // Set cursor position
    textbox.set_cursor_index(5);

Then in the main loop you would use:
    // Update and draw the textbox in one step
    textbox.draw();
*/
use macroquad::prelude::*;

pub struct TextBox {
    // Make all fields private for complete encapsulation
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: String,
    active: bool,
    cursor_index: usize,
    cursor_timer: f32,
    cursor_visible: bool,
    font_size: f32,
    text_color: Color,
    border_color: Color,
    background_color: Color,
    cursor_color: Color,
    font: Option<Font>,
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
            font: None, // Default to None (use system font)
        }
    }
    
    // Position and dimension getters/setters
    #[allow(unused)]
    pub fn get_x(&self) -> f32 {
        self.x
    }
    
    #[allow(unused)]
    pub fn set_x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }
    
    #[allow(unused)]
    pub fn get_y(&self) -> f32 {
        self.y
    }
    
    #[allow(unused)]
    pub fn set_y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }
    
    #[allow(unused)]
    pub fn get_width(&self) -> f32 {
        self.width
    }
    
    #[allow(unused)]
    pub fn set_width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        self
    }
    
    #[allow(unused)]
    pub fn get_height(&self) -> f32 {
        self.height
    }
    
    #[allow(unused)]
    pub fn set_height(&mut self, height: f32) -> &mut Self {
        self.height = height;
        self
    }
    
    // Position convenience methods
    #[allow(unused)]
    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    
    #[allow(unused)]
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
    
    // Dimension convenience methods
    #[allow(unused)]
    pub fn get_dimensions(&self) -> (f32, f32) {
        (self.width, self.height)
    }
    
    #[allow(unused)]
    pub fn set_dimensions(&mut self, width: f32, height: f32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }
    
    // Add a method to change colors
    #[allow(unused)]
    pub fn with_colors(&mut self, text_color: Color, border_color: Color, background_color: Color, cursor_color: Color) -> &mut Self {
        self.text_color = text_color;
        self.border_color = border_color;
        self.background_color = background_color;
        self.cursor_color = cursor_color;
        self
    }

    // Method to set custom font
    #[allow(unused)]
    pub fn with_font(&mut self, font: Font) -> &mut Self {
        self.font = Some(font);
        self
    }

    // Get the current text content
    #[allow(unused)]
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
    
    // Set the text content - now accepts both String and &str
    #[allow(unused)]
    pub fn set_text<T: Into<String>>(&mut self, text: T) -> &mut Self {
        self.text = text.into();
        if self.cursor_index > self.text.len() {
            self.cursor_index = self.text.len();
        }
        self
    }
    
    // Active state getters/setters
    #[allow(unused)]
    pub fn is_active(&self) -> bool {
        self.active
    }

    #[allow(unused)]
    pub fn set_active(&mut self, active: bool) -> &mut Self {
        self.active = active;
        self
    }

    // Cursor index getters/setters
    #[allow(unused)]
    pub fn get_cursor_index(&self) -> usize {
        self.cursor_index
    }

    #[allow(unused)]
    pub fn set_cursor_index(&mut self, index: usize) -> &mut Self {
        if index <= self.text.len() {
            self.cursor_index = index;
        }
        self
    }

    // Font size getters/setters
    #[allow(unused)]
    pub fn get_font_size(&self) -> f32 {
        self.font_size
    }

    #[allow(unused)]
    pub fn set_font_size(&mut self, size: f32) -> &mut Self {
        self.font_size = size;
        self
    }

    // Color getters/setters
    #[allow(unused)]
    pub fn get_text_color(&self) -> Color {
        self.text_color
    }

    #[allow(unused)]
    pub fn set_text_color(&mut self, color: Color) -> &mut Self {
        self.text_color = color;
        self
    }

    #[allow(unused)]
    pub fn get_border_color(&self) -> Color {
        self.border_color
    }

    #[allow(unused)]
    pub fn set_border_color(&mut self, color: Color) -> &mut Self {
        self.border_color = color;
        self
    }

    #[allow(unused)]
    pub fn get_background_color(&self) -> Color {
        self.background_color
    }

    #[allow(unused)]
    pub fn set_background_color(&mut self, color: Color) -> &mut Self {
        self.background_color = color;
        self
    }

    #[allow(unused)]
    pub fn get_cursor_color(&self) -> Color {
        self.cursor_color
    }

    #[allow(unused)]
    pub fn set_cursor_color(&mut self, color: Color) -> &mut Self {
        self.cursor_color = color;
        self
    }

    // Font getter/setter
    #[allow(unused)]
    pub fn get_font(&self) -> Option<&Font> {
        self.font.as_ref()
    }

    // Primary method - both updates and draws the textbox
    #[allow(unused)]
    pub fn draw(&mut self) {
        self.update_internal();
        self.draw_internal();
    }
    
    // For cases when only drawing is needed without updating
    #[allow(unused)]
    pub fn draw_only(&self) {
        self.draw_internal();
    }
    
    // For cases when only updating is needed without drawing
    #[allow(unused)]
    pub fn update_only(&mut self) {
        self.update_internal();
    }

    // Now private - internal implementation only
    fn update_internal(&mut self) {
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
                    let char_width = match &self.font {
                        Some(font) => measure_text(
                            &self.text[self.cursor_index..self.cursor_index + 1], 
                            Some(font), 
                            self.font_size as u16, 
                            1.0
                        ).width,
                        None => measure_text(
                            &self.text[self.cursor_index..self.cursor_index + 1], 
                            None, 
                            self.font_size as u16, 
                            1.0
                        ).width,
                    };
                    
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
    
    // Now private - internal implementation only
    fn draw_internal(&self) {
        let padding = 5.0;
        let text_x = self.x + padding;
        let text_y = self.y + self.height / 2.0 + self.font_size / 2.5;
    
        // Draw the background and border with customizable colors
        draw_rectangle(self.x, self.y, self.width, self.height, self.background_color);
        
        // Draw text with the appropriate font
        match &self.font {
            Some(font) => {
                draw_text_ex(
                    &self.text,
                    text_x,
                    text_y,
                    TextParams {
                        font: Some(font),
                        font_size: self.font_size as u16,
                        color: self.text_color,
                        ..Default::default()
                    },
                );
            },
            None => {
                draw_text(&self.text, text_x, text_y, self.font_size, self.text_color);
            }
        }
    
        if self.active && self.cursor_visible {
            let mut cursor_offset = 0.0;
            if self.cursor_index > 0 {
                let cursor_text = &self.text[..self.cursor_index];
                
                // Calculate cursor position based on font
                if let Some(font) = &self.font {
                    // Use custom font for measurement
                    for c in cursor_text.chars() {
                        cursor_offset += measure_text(&c.to_string(), Some(font), self.font_size as u16, 1.0).width;
                    }
                } else {
                    // Use default font for measurement
                    for c in cursor_text.chars() {
                        cursor_offset += measure_text(&c.to_string(), None, self.font_size as u16, 1.0).width;
                    }
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


