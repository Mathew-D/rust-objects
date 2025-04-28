/*
Made by: Mathew Dusome
April 26 2025
To import you need:
Adds a label object
In the mod objects section add:
        pub mod label;
    

Add with the other use statements
    use objects::label::Label;

Then to use this you would put the following above the loop: 
    let lbl_out = Label::new("Hello\nWorld", 50.0, 100.0, 30);
Where the numbers are x, y, font size
You can also set the colors of the text box by using:
     lbl_out.with_colors(WHITE, Some(DARKGRAY));
Where the colors are text color and background color respectively.

You can also specify a custom font with:
     lbl_out.with_font(font);
Example:
     // Load font once at the beginning of your program
     let font = load_ttf_font("assets/love.ttf").await.unwrap();
     
     // Create label and apply custom font
     let mut lbl_out = Label::new("Hello\nWorld", 50.0, 100.0, 30);
     lbl_out.with_colors(WHITE, Some(DARKGRAY))
            .with_font(font.clone());
Otherwise the default system font will be used.

Then in the loop you would use:
    lbl_out.draw();
*/

use macroquad::prelude::*;

pub struct Label {
    text: String,
    x: f32,
    y: f32,
    font_size: u16,
    foreground: Color,
    background: Option<Color>,
    line_spacing: f32,
    font: Option<Font>, // Store the font directly since Font is Clone
}

impl Label {
    // Constructor using x and y separately
    pub fn new(text: &str, x: f32, y: f32, font_size: u16) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
            font_size,
            foreground: BLACK, // Default to black
            background: None,  // No background by default
            line_spacing: 1.2,
            font: None,        // Default to None (use system font)
        }
    }

    // Method to set foreground and background colors
    #[allow(unused)]
    pub fn with_colors(&mut self, foreground: Color, background: Option<Color>) -> &mut Self {
        self.foreground = foreground;
        self.background = background;
        self
    }

    // Method to set custom font - taking Font by value since it implements Clone
    #[allow(unused)]
    pub fn with_font(&mut self, font: Font) -> &mut Self {
        self.font = Some(font);
        self
    }

    // Method to set text
    #[allow(unused)]
    pub fn set_text(&mut self, new_text: &str) {
        self.text = new_text.to_string();
    }

    // Method to draw the label
    pub fn draw(&self) {
        let lines: Vec<&str> = self.text.split('\n').collect();
        let line_height = self.font_size as f32 * self.line_spacing;

        for (i, line) in lines.iter().enumerate() {
            let x = self.x;
            let y = self.y + i as f32 * line_height;

            // Calculate text dimensions based on the chosen font
            let dimensions = match &self.font {
                Some(font) => measure_text(line, Some(font), self.font_size, 1.0),
                None => measure_text(line, None, self.font_size, 1.0),
            };

            // Draw background only if it's Some
            if let Some(bg) = self.background {
                draw_rectangle(
                    x - 5.0,
                    y - self.font_size as f32,
                    dimensions.width + 10.0,
                    line_height,
                    bg,
                );
            }

            // Draw the text - use draw_text_ex if we have a custom font
            match &self.font {
                Some(font) => {
                    draw_text_ex(
                        line,
                        x,
                        y,
                        TextParams {
                            font: Some(font),
                            font_size: self.font_size,
                            color: self.foreground,
                            ..Default::default()
                        },
                    );
                },
                None => {
                    // Use the default draw_text function
                    draw_text(line, x, y, self.font_size as f32, self.foreground);
                }
            }
        }
    }
}
