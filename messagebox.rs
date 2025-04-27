/*
Made by: Mathew Dusome
April 26 2025
To import you need:
Adds a message box (dialog) component for displaying messages and options to users
In the mod objects section add:
    pub mod messagebox;
    
Then add the following with the use commands:
use objects::messagebox::{MessageBox, MessageBoxResult};

Then to use this you would put the following above the loop: 
    // Create a simple message box with OK button
    let mut info_box = MessageBox::new(
        "Information",                   // Title
        "Operation completed successfully!", // Message
        vec!["OK"],                      // Buttons (just "OK" button)
        None,                            // Default selected button (first one)
        400.0, 200.0                     // Width, height
    );
    
    // Show the message box when program starts (IMPORTANT!)
    info_box.show();
    
    // Center the message box on screen
    info_box.centered();
    
    // Or create a message box with multiple options
    let mut confirm_box = MessageBox::new(
        "Confirm Action",                // Title
        "Do you want to save your progress?", // Message
        vec!["Yes", "No", "Cancel"],     // Buttons
        Some(0),                         // Default button (Yes)
        400.0, 200.0                     // Width, height
    );
    
    // Customize appearance
    confirm_box.with_colors(
        DARKBLUE,       // Title background
        SKYBLUE,        // Dialog background
        WHITE,          // Title text color
        BLACK,          // Message text color
        Color::new(0.0, 0.0, 0.0, 0.5)  // Modal overlay color
    );
    
    // Set position
    confirm_box.centered(); // Center in screen
    
    // Or manually position
    confirm_box.set_position(100.0, 100.0);
    
    // Show with modal overlay (click blocker)
    confirm_box.with_modal(true);
    
    // Make sure to show the message box when you want it to appear
    confirm_box.show();

Then inside the loop, MOST IMPORTANTLY, make sure to draw the message box AFTER 
clearing the background and drawing other elements:

    // Clear background and draw other elements first
    clear_background(RED);
    draw_your_other_elements();

    
    // LAST: Update and draw the message box if visible
    if let Some(result) = confirm_box.update_and_draw() {
        match result {
            MessageBoxResult::ButtonPressed(0) => {
                // "Yes" button was pressed
                // Save game...
                confirm_box.hide(); // Hide the dialog after handling result
            },
            MessageBoxResult::ButtonPressed(1) => {
                // "No" button was pressed
                // Continue without saving...
                confirm_box.hide();
            },
            MessageBoxResult::ButtonPressed(2) => {
                // "Cancel" button was pressed
                // Abort operation...
                confirm_box.hide();
            },
            MessageBoxResult::ButtonPressed(_) => {
                // Handle any other button presses
                confirm_box.hide();
            },
            MessageBoxResult::Closed => {
                // Dialog was closed with X or Escape key
                // Handle as cancel...
                confirm_box.hide();
            }
        }
    }
    
    // Show message box again when a key is pressed
    if is_key_pressed(KeyCode::S) {
        confirm_box.show();
        confirm_box.centered(); // Re-center in case window was resized
    }
    
TROUBLESHOOTING:
- If the message box doesn't appear, make sure you called .show() on it
- Make sure update_and_draw() is called AFTER drawing other elements
- Don't call .show() inside the main loop unless in response to an event
- Make sure to handle all possible MessageBoxResult variants in your match statement
*/

use macroquad::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum MessageBoxResult {
    ButtonPressed(usize),  // Index of the button pressed
    Closed,                // Dialog was closed via X button or escape key
}

pub struct MessageBox {
    visible: bool,
    title: String,
    message: String,
    buttons: Vec<String>,
    default_button: Option<usize>,
    selected_button: Option<usize>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    title_height: f32,
    button_height: f32,
    padding: f32,
    title_bg_color: Color,
    bg_color: Color,
    title_text_color: Color,
    message_text_color: Color,
    button_bg_color: Color,
    button_hover_color: Color,
    button_text_color: Color,
    close_button_size: f32,
    show_close_button: bool,
    modal: bool,
    modal_color: Color,
    result: Option<MessageBoxResult>,
    dragging: bool,
    drag_offset_x: f32,
    drag_offset_y: f32,
}

impl MessageBox {
    // Create a new message box
    pub fn new(
        title: impl Into<String>,
        message: impl Into<String>,
        buttons: Vec<impl Into<String>>,
        default_button: Option<usize>,
        width: f32,
        height: f32,
    ) -> Self {
        let title = title.into();
        let message = message.into();
        let buttons: Vec<String> = buttons.into_iter().map(|b| b.into()).collect();
        let default_button = if let Some(idx) = default_button {
            if idx < buttons.len() { Some(idx) } else { None }
        } else {
            None
        };
        
        Self {
            visible: false,
            title,
            message,
            buttons,
            default_button,
            selected_button: default_button,
            x: 0.0,
            y: 0.0,
            width,
            height,
            title_height: 30.0,
            button_height: 40.0,
            padding: 15.0,
            title_bg_color: DARKBLUE,
            bg_color: Color::new(0.9, 0.9, 0.9, 1.0), // Light gray
            title_text_color: WHITE,
            message_text_color: BLACK,
            button_bg_color: LIGHTGRAY,
            button_hover_color: GRAY,
            button_text_color: BLACK,
            close_button_size: 20.0,
            show_close_button: true,
            modal: true,
            modal_color: Color::new(0.0, 0.0, 0.0, 0.5),
            result: None,
            dragging: false,
            drag_offset_x: 0.0,
            drag_offset_y: 0.0,
        }
    }
    
    // Center the dialog in the screen
    pub fn centered(&mut self) -> &mut Self {
        self.x = (screen_width() - self.width) / 2.0;
        self.y = (screen_height() - self.height) / 2.0;
        self
    }
    
    // Set dialog position
    #[allow(dead_code)]
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
    
    // Set dialog size
    #[allow(dead_code)]
    pub fn set_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }
    
    // Customize colors
    #[allow(dead_code)]
    pub fn with_colors(
        &mut self,
        title_bg: Color,
        dialog_bg: Color,
        title_text: Color,
        message_text: Color,
        modal_overlay: Color,
    ) -> &mut Self {
        self.title_bg_color = title_bg;
        self.bg_color = dialog_bg;
        self.title_text_color = title_text;
        self.message_text_color = message_text;
        self.modal_color = modal_overlay;
        self
    }
    
    // Customize button colors
    #[allow(dead_code)]
    pub fn with_button_colors(
        &mut self,
        button_bg: Color,
        button_hover: Color,
        button_text: Color,
    ) -> &mut Self {
        self.button_bg_color = button_bg;
        self.button_hover_color = button_hover;
        self.button_text_color = button_text;
        self
    }
    
    // Configure modal overlay
    #[allow(dead_code)]
    pub fn with_modal(&mut self, modal: bool) -> &mut Self {
        self.modal = modal;
        self
    }
    
    // Configure close button
    #[allow(dead_code)]
    pub fn with_close_button(&mut self, show: bool) -> &mut Self {
        self.show_close_button = show;
        self
    }
    
    // Show the dialog
    pub fn show(&mut self) -> &mut Self {
        self.visible = true;
        self.result = None;
        self.selected_button = self.default_button;
        self
    }
    
    // Hide the dialog
    pub fn hide(&mut self) -> &mut Self {
        self.visible = false;
        self.result = None;
        self
    }
    
    // Check if dialog is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    // Get the last result
    #[allow(dead_code)]
    pub fn get_result(&self) -> Option<&MessageBoxResult> {
        self.result.as_ref()
    }
    
    // Clear the result
    #[allow(dead_code)]
    pub fn clear_result(&mut self) -> &mut Self {
        self.result = None;
        self
    }
    
    // Update and draw the message box, returning a result if a button was clicked
    pub fn update_and_draw(&mut self) -> Option<MessageBoxResult> {
        if !self.visible {
            return None;
        }
        
        // Draw modal background if enabled
        if self.modal {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), self.modal_color);
        }
        
        // Handle dragging
        self.handle_dragging();
        
        // Draw dialog background
        draw_rectangle(self.x, self.y, self.width, self.height, self.bg_color);
        
        // Draw title bar
        draw_rectangle(
            self.x,
            self.y,
            self.width,
            self.title_height,
            self.title_bg_color
        );
        
        // Draw title text
        let title_font_size = 18.0;
        let title_size = measure_text(&self.title, None, title_font_size as u16, 1.0);
        draw_text(
            &self.title,
            self.x + self.padding,
            self.y + (self.title_height + title_size.height) / 2.0,
            title_font_size,
            self.title_text_color
        );
        
        // Draw close button if enabled
        if self.show_close_button {
            let close_x = self.x + self.width - self.close_button_size - self.padding;
            let close_y = self.y + (self.title_height - self.close_button_size) / 2.0;
            
            // Check if mouse is over close button
            let (mouse_x, mouse_y) = mouse_position();
            let is_over_close = mouse_x >= close_x && mouse_x <= close_x + self.close_button_size &&
                                mouse_y >= close_y && mouse_y <= close_y + self.close_button_size;
            
            // Draw close button (X)
            let close_color = if is_over_close {
                Color::new(1.0, 0.3, 0.3, 1.0) // Highlight red on hover
            } else {
                self.title_text_color
            };
            
            // Draw X symbol
            let thickness = 2.0;
            draw_line(
                close_x,
                close_y,
                close_x + self.close_button_size,
                close_y + self.close_button_size,
                thickness,
                close_color
            );
            draw_line(
                close_x,
                close_y + self.close_button_size,
                close_x + self.close_button_size,
                close_y,
                thickness,
                close_color
            );
            
            // Handle close button click
            if is_over_close && is_mouse_button_pressed(MouseButton::Left) {
                self.result = Some(MessageBoxResult::Closed);
                return self.result.clone();
            }
        }
        
        // Draw message
        let message_font_size = 16.0;
        let message_y = self.y + self.title_height + self.padding;
        
        // Handle multiline messages
        let max_line_width = self.width - 2.0 * self.padding;
        let lines = self.wrap_text(&self.message, max_line_width, message_font_size);
        
        let line_height = message_font_size * 1.2;
        for (i, line) in lines.iter().enumerate() {
            draw_text(
                line,
                self.x + self.padding,
                message_y + i as f32 * line_height + message_font_size,
                message_font_size,
                self.message_text_color
            );
        }
        
        // Draw buttons
        let button_font_size = 16.0;
        let button_spacing = 10.0;
        let num_buttons = self.buttons.len();
        
        if num_buttons > 0 {
            let total_button_width: f32 = if num_buttons == 1 {
                self.width * 0.33 // Single button takes 1/3 of dialog width
            } else {
                num_buttons as f32 * 100.0 + (num_buttons - 1) as f32 * button_spacing
            };
            
            let first_button_x = self.x + (self.width - total_button_width) / 2.0;
            let button_y = self.y + self.height - self.button_height - self.padding;
            
            for (i, button_text) in self.buttons.iter().enumerate() {
                let button_width = if num_buttons == 1 {
                    self.width * 0.33 // Single button takes 1/3 of dialog width
                } else {
                    100.0 // Multiple buttons have fixed width
                };
                
                let button_x = first_button_x + i as f32 * (button_width + button_spacing);
                
                // Check if mouse is over this button
                let (mouse_x, mouse_y) = mouse_position();
                let is_over_button = mouse_x >= button_x && mouse_x <= button_x + button_width &&
                                    mouse_y >= button_y && mouse_y <= button_y + self.button_height;
                
                // Highlight selected button
                let button_color = if self.selected_button == Some(i) || is_over_button {
                    self.button_hover_color
                } else {
                    self.button_bg_color
                };
                
                // Draw button background
                draw_rectangle(
                    button_x,
                    button_y,
                    button_width,
                    self.button_height,
                    button_color
                );
                
                // Draw button border
                draw_rectangle_lines(
                    button_x,
                    button_y,
                    button_width,
                    self.button_height,
                    1.0,
                    DARKGRAY
                );
                
                // Draw button text
                let text_size = measure_text(button_text, None, button_font_size as u16, 1.0);
                draw_text(
                    button_text,
                    button_x + (button_width - text_size.width) / 2.0,
                    button_y + (self.button_height + text_size.height) / 2.0,
                    button_font_size,
                    self.button_text_color
                );
                
                // Handle button click
                if is_over_button {
                    if is_mouse_button_pressed(MouseButton::Left) {
                        self.result = Some(MessageBoxResult::ButtonPressed(i));
                        return self.result.clone();
                    }
                    
                    // Update selected button on hover for keyboard navigation
                    self.selected_button = Some(i);
                }
            }
        }
        
        // Handle keyboard navigation
        if is_key_pressed(KeyCode::Tab) {
            if let Some(selected) = self.selected_button {
                // Shift selection to next button (with wrap-around)
                let next = (selected + 1) % num_buttons;
                self.selected_button = Some(next);
            } else if num_buttons > 0 {
                // Select first button if none selected
                self.selected_button = Some(0);
            }
        }
        
        // Handle Enter key to activate selected button
        if is_key_pressed(KeyCode::Enter) {
            if let Some(selected) = self.selected_button {
                self.result = Some(MessageBoxResult::ButtonPressed(selected));
                return self.result.clone();
            }
        }
        
        // Handle Escape key to close dialog
        if is_key_pressed(KeyCode::Escape) {
            self.result = Some(MessageBoxResult::Closed);
            return self.result.clone();
        }
        
        self.result.clone()
    }
    
    // Handle dialog dragging
    fn handle_dragging(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        
        // Check if mouse is in title bar area
        let in_title_bar = mouse_x >= self.x && mouse_x <= self.x + self.width &&
                           mouse_y >= self.y && mouse_y <= self.y + self.title_height;
        
        // Start dragging when clicking on title bar
        if in_title_bar && is_mouse_button_pressed(MouseButton::Left) {
            self.dragging = true;
            self.drag_offset_x = mouse_x - self.x;
            self.drag_offset_y = mouse_y - self.y;
        }
        
        // Continue dragging while button is held
        if self.dragging {
            if is_mouse_button_down(MouseButton::Left) {
                self.x = mouse_x - self.drag_offset_x;
                self.y = mouse_y - self.drag_offset_y;
                
                // Keep dialog within screen bounds
                self.x = self.x.max(0.0).min(screen_width() - self.width);
                self.y = self.y.max(0.0).min(screen_height() - self.height);
            } else {
                // Stop dragging when button is released
                self.dragging = false;
            }
        }
    }
    
    // Wrap text to fit within width
    fn wrap_text(&self, text: &str, max_width: f32, font_size: f32) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        
        for word in text.split_whitespace() {
            let word_with_space = if current_line.is_empty() {
                word.to_string()
            } else {
                format!(" {}", word)
            };
            
            let test_line = format!("{}{}", current_line, word_with_space);
            let test_width = measure_text(&test_line, None, font_size as u16, 1.0).width;
            
            if test_width <= max_width {
                current_line = test_line;
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line.clone());
                }
                current_line = word.to_string();
            }
        }
        
        if !current_line.is_empty() {
            lines.push(current_line);
        }
        
        // Handle case where no lines were created (empty message)
        if lines.is_empty() {
            lines.push(String::new());
        }
        
        lines
    }
}