/*
Made by: Mathew Dusome
April 26 2025
To import you need:
Adds a slider UI component for numeric input
In the mod objects section add:
    pub mod slider;
    
Then add the following with the use commands:
use crate::objects::slider::Slider;

Then to use this you would put the following above the loop: 
    // Create a basic slider (horizontal by default)
    let mut volume_slider = Slider::new(
        100.0, 200.0,         
        200.0, 30.0,          
        0.0, 100.0,           
        50.0                  
    );
    Where the the numbers are:
    // Position (x, y)
    // Size (width, height)
    // Range (min, max)
    // Initial value

    
    // Customize the appearance
    volume_slider.with_colors(GRAY, DARKBLUE, WHITE, SKYBLUE);
    
    // Or create a vertical slider
    let mut vertical_slider = Slider::new_vertical(
        400.0, 100.0,         // Position (x, y)
        30.0, 200.0,          // Size (width, height)
        0.0, 1.0,             // Range (min, max)
        0.5                   // Initial value
    );

Then inside the loop you would use:
    // Update slider state
    volume_slider.update();
    
    // Draw the slider
    volume_slider.draw();
    
    // Get the current value
    let volume = volume_slider.value();
    
    // You can use the value in your application
    play_sound(sound, PlaySoundParams { volume: volume / 100.0, looped: true });
*/

use macroquad::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

pub struct Slider {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    min_value: f32,
    max_value: f32,
    current_value: f32,
    orientation: SliderOrientation,
    track_color: Color,
    handle_color: Color,
    handle_hover_color: Color,
    label_color: Color,
    handle_radius: f32,
    dragging: bool,
    show_value: bool,
    value_precision: usize,
    label: Option<String>,
}

impl Slider {
    // Create a new horizontal slider
    pub fn new(x: f32, y: f32, width: f32, height: f32, min_value: f32, max_value: f32, initial_value: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            min_value,
            max_value,
            current_value: initial_value.clamp(min_value, max_value),
            orientation: SliderOrientation::Horizontal,
            track_color: GRAY,
            handle_color: DARKGRAY,
            handle_hover_color: BLUE,
            label_color: WHITE,
            handle_radius: height / 2.0,
            dragging: false,
            show_value: true,
            value_precision: 1,
            label: None,
        }
    }
    
    // Create a new vertical slider
    pub fn new_vertical(x: f32, y: f32, width: f32, height: f32, min_value: f32, max_value: f32, initial_value: f32) -> Self {
        let mut slider = Self::new(x, y, width, height, min_value, max_value, initial_value);
        slider.orientation = SliderOrientation::Vertical;
        slider.handle_radius = width / 2.0;
        slider
    }
    
    // Set colors for the slider
    #[allow(unused)]
    pub fn with_colors(&mut self, track: Color, handle: Color, label: Color, hover: Color) -> &mut Self {
        self.track_color = track;
        self.handle_color = handle;
        self.label_color = label;
        self.handle_hover_color = hover;
        self
    }
    
    // Set a label for the slider
    #[allow(unused)]
    pub fn with_label(&mut self, label: impl Into<String>) -> &mut Self {
        self.label = Some(label.into());
        self
    }
    
    // Configure value display options
    #[allow(unused)]
    pub fn with_value_display(&mut self, show: bool, precision: usize) -> &mut Self {
        self.show_value = show;
        self.value_precision = precision;
        self
    }
    
    // Set handle size
    #[allow(unused)]
    pub fn with_handle_radius(&mut self, radius: f32) -> &mut Self {
        self.handle_radius = radius;
        self
    }
    
    // Get current slider value
    pub fn value(&self) -> f32 {
        self.current_value
    }
    
    // Set slider value programmatically
    #[allow(unused)]
    pub fn set_value(&mut self, value: f32) {
        self.current_value = value.clamp(self.min_value, self.max_value);
    }
    
    // Calculate handle position based on current value
    fn handle_position(&self) -> Vec2 {
        match self.orientation {
            SliderOrientation::Horizontal => {
                let normalized_value = (self.current_value - self.min_value) / (self.max_value - self.min_value);
                let handle_x = self.x + normalized_value * (self.width - self.handle_radius * 2.0) + self.handle_radius;
                Vec2::new(handle_x, self.y + self.height / 2.0)
            },
            SliderOrientation::Vertical => {
                let normalized_value = 1.0 - (self.current_value - self.min_value) / (self.max_value - self.min_value);
                let handle_y = self.y + normalized_value * (self.height - self.handle_radius * 2.0) + self.handle_radius;
                Vec2::new(self.x + self.width / 2.0, handle_y)
            }
        }
    }
    
    // Calculate value from handle position
    fn value_from_position(&self, position: Vec2) -> f32 {
        match self.orientation {
            SliderOrientation::Horizontal => {
                let track_width = self.width - self.handle_radius * 2.0;
                let x_pos = (position.x - self.x - self.handle_radius).clamp(0.0, track_width);
                let normalized_value = x_pos / track_width;
                self.min_value + normalized_value * (self.max_value - self.min_value)
            },
            SliderOrientation::Vertical => {
                let track_height = self.height - self.handle_radius * 2.0;
                let y_pos = (position.y - self.y - self.handle_radius).clamp(0.0, track_height);
                let normalized_value = 1.0 - y_pos / track_height;
                self.min_value + normalized_value * (self.max_value - self.min_value)
            }
        }
    }
    
    // Check if mouse is over handle
    fn is_mouse_over_handle(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        let handle_pos = self.handle_position();
        let distance = ((mouse_x - handle_pos.x).powi(2) + (mouse_y - handle_pos.y).powi(2)).sqrt();
        distance <= self.handle_radius
    }
    
    // Update slider state
    pub fn update(&mut self) {
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        
        // Check for initial click
        if is_mouse_button_pressed(MouseButton::Left) && self.is_mouse_over_handle() {
            self.dragging = true;
        }
        
        // Handle dragging
        if self.dragging {
            self.current_value = self.value_from_position(mouse_pos);
            
            // Stop dragging when mouse is released
            if !is_mouse_button_down(MouseButton::Left) {
                self.dragging = false;
            }
        }
        
        // Handle track click (direct value selection)
        if is_mouse_button_pressed(MouseButton::Left) && !self.is_mouse_over_handle() {
            // Check if click is within track bounds
            let in_track = match self.orientation {
                SliderOrientation::Horizontal => {
                    mouse_pos.x >= self.x && 
                    mouse_pos.x <= self.x + self.width &&
                    mouse_pos.y >= self.y &&
                    mouse_pos.y <= self.y + self.height
                },
                SliderOrientation::Vertical => {
                    mouse_pos.x >= self.x && 
                    mouse_pos.x <= self.x + self.width &&
                    mouse_pos.y >= self.y &&
                    mouse_pos.y <= self.y + self.height
                }
            };
            
            if in_track {
                self.current_value = self.value_from_position(mouse_pos);
                self.dragging = true;
            }
        }
    }
    
    // Draw slider
    pub fn draw(&self) {
        // Draw track
        match self.orientation {
            SliderOrientation::Horizontal => {
                // Draw track (horizontal)
                draw_rectangle(
                    self.x + self.handle_radius, 
                    self.y + (self.height - self.handle_radius) / 2.0,
                    self.width - self.handle_radius * 2.0, 
                    self.handle_radius,
                    self.track_color
                );
                
                // Draw filled part of track
                let fill_width = (self.current_value - self.min_value) / (self.max_value - self.min_value) * (self.width - self.handle_radius * 2.0);
                draw_rectangle(
                    self.x + self.handle_radius, 
                    self.y + (self.height - self.handle_radius) / 2.0,
                    fill_width,
                    self.handle_radius,
                    self.handle_color
                );
            },
            SliderOrientation::Vertical => {
                // Draw track (vertical)
                draw_rectangle(
                    self.x + (self.width - self.handle_radius) / 2.0, 
                    self.y + self.handle_radius,
                    self.handle_radius, 
                    self.height - self.handle_radius * 2.0,
                    self.track_color
                );
                
                // Draw filled part of track
                let fill_height = (self.current_value - self.min_value) / (self.max_value - self.min_value) * (self.height - self.handle_radius * 2.0);
                draw_rectangle(
                    self.x + (self.width - self.handle_radius) / 2.0, 
                    self.y + self.height - self.handle_radius - fill_height,
                    self.handle_radius, 
                    fill_height,
                    self.handle_color
                );
            }
        }
        
        // Draw handle
        let handle_pos = self.handle_position();
        let handle_color = if self.is_mouse_over_handle() || self.dragging { 
            self.handle_hover_color 
        } else { 
            self.handle_color 
        };
        
        draw_circle(handle_pos.x, handle_pos.y, self.handle_radius, handle_color);
        
        // Draw label if present
        if let Some(label) = &self.label {
            match self.orientation {
                SliderOrientation::Horizontal => {
                    let text_width = measure_text(label, None, 20, 1.0).width;
                    draw_text(
                        label,
                        self.x + (self.width - text_width) / 2.0,
                        self.y - 10.0,
                        20.0,
                        self.label_color
                    );
                },
                SliderOrientation::Vertical => {
                    let text_width = measure_text(label, None, 20, 1.0).width;
                    draw_text(
                        label,
                        self.x + (self.width - text_width) / 2.0,
                        self.y - 10.0,
                        20.0,
                        self.label_color
                    );
                }
            }
        }
        
        // Draw value if enabled
        if self.show_value {
            let value_text = format!("{:.1$}", self.current_value, self.value_precision);
            let text_width = measure_text(&value_text, None, 16, 1.0).width;
            
            match self.orientation {
                SliderOrientation::Horizontal => {
                    draw_text(
                        &value_text,
                        self.x + (self.width - text_width) / 2.0,
                        self.y + self.height + 20.0,
                        16.0,
                        self.label_color
                    );
                },
                SliderOrientation::Vertical => {
                    draw_text(
                        &value_text,
                        self.x + self.width + 10.0,
                        self.y + self.height / 2.0 + 8.0,
                        16.0,
                        self.label_color
                    );
                }
            }
        }
    }
}