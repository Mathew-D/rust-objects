/*
Made by: Mathew Dusome
April 26 2025
To import you need:
Adds a progress bar component for displaying completion status

In your mod.rs file located in the modules folder add the following to the end of the file
    pub mod progressbar;
    
Then add the following with the use commands:
use crate::modules::progressbar::ProgressBar;

Then to use this you would put the following above the loop: 
    // Create a basic horizontal progress bar
    let mut progress = ProgressBar::new(
        100.0, 100.0,      // Position (x, y)
        200.0, 30.0,       // Size (width, height)
        0.0, 100.0,        // Range (min, max)
        0.0                // Initial value
    );
    
    // Customize appearance
    progress.with_colors(DARKBLUE, SKYBLUE, WHITE)
           .with_label("Loading...")
           .with_border(true, DARKGRAY, 2.0)
           .with_percentage(true);
           
    // Or create a vertical progress bar
    let mut vertical_progress = ProgressBar::new_vertical(
        400.0, 100.0,      // Position (x, y)
        30.0, 200.0,       // Size (width, height)
        0.0, 100.0,        // Range (min, max)
        25.0               // Initial value
    );
    
    // Options for animation
    progress.with_animation(true, 2.0);  // Enable animation with speed factor

Then inside the loop you would use:
    // Update progress (manually or based on some calculation)
    progress.set_value(50.0);  // Update to 50%
    
    // Or increment progress
    progress.increment(0.5);   // Add 0.5 to the current value
    
    // Draw the progress bar
    progress.draw();
    
    // Check if progress is complete
    if progress.is_complete() {
        // Do something when progress reaches 100%
    }
*/

use macroquad::prelude::*;

#[derive(PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum ProgressBarOrientation {
    Horizontal,
    Vertical,
}

#[allow(dead_code)]
pub struct ProgressBar {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    min_value: f32,
    max_value: f32,
    current_value: f32,
    target_value: f32,  // For animation
    orientation: ProgressBarOrientation,
    background_color: Color,
    fill_color: Color,
    text_color: Color,
    show_percentage: bool,
    show_value: bool,
    label: Option<String>,
    border: bool,
    border_color: Color,
    border_thickness: f32,
    animate: bool,
    animation_speed: f32,
    completed_callback: Option<Box<dyn Fn()>>,
}

impl ProgressBar {
    // Create a new horizontal progress bar
    pub fn new(x: f32, y: f32, width: f32, height: f32, min_value: f32, max_value: f32, initial_value: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            min_value,
            max_value: max_value.max(min_value + 0.001),  // Prevent min == max
            current_value: initial_value.clamp(min_value, max_value),
            target_value: initial_value.clamp(min_value, max_value),
            orientation: ProgressBarOrientation::Horizontal,
            background_color: GRAY,
            fill_color: GREEN,
            text_color: WHITE,
            show_percentage: false,
            show_value: false,
            label: None,
            border: false,
            border_color: DARKGRAY,
            border_thickness: 2.0,
            animate: false,
            animation_speed: 5.0,
            completed_callback: None,
        }
    }
    
    // Create a new vertical progress bar
    #[allow(dead_code)]
    pub fn new_vertical(x: f32, y: f32, width: f32, height: f32, min_value: f32, max_value: f32, initial_value: f32) -> Self {
        let mut progress_bar = Self::new(x, y, width, height, min_value, max_value, initial_value);
        progress_bar.orientation = ProgressBarOrientation::Vertical;
        progress_bar
    }
    
    // Set colors for the progress bar
    pub fn with_colors(&mut self, background: Color, fill: Color, text: Color) -> &mut Self {
        self.background_color = background;
        self.fill_color = fill;
        self.text_color = text;
        self
    }
    
    // Set a label for the progress bar
    #[allow(dead_code)]
    pub fn with_label(&mut self, label: impl Into<String>) -> &mut Self {
        self.label = Some(label.into());
        self
    }
    
    // Configure value display options
    #[allow(dead_code)]
    pub fn with_percentage(&mut self, show: bool) -> &mut Self {
        self.show_percentage = show;
        self
    }
    
    // Show actual value
    #[allow(dead_code)]
    pub fn with_value(&mut self, show: bool) -> &mut Self {
        self.show_value = show;
        self
    }
    
    // Configure border
    #[allow(dead_code)]
    pub fn with_border(&mut self, show: bool, color: Color, thickness: f32) -> &mut Self {
        self.border = show;
        self.border_color = color;
        self.border_thickness = thickness;
        self
    }
    
    // Configure animation
    #[allow(dead_code)]
    pub fn with_animation(&mut self, animate: bool, speed: f32) -> &mut Self {
        self.animate = animate;
        self.animation_speed = speed.max(0.1);  // Prevent too slow animation
        self
    }
    
    // Set a callback for when progress completes
    #[allow(dead_code)]
    pub fn on_complete<F: Fn() + 'static>(&mut self, callback: F) -> &mut Self {
        self.completed_callback = Some(Box::new(callback));
        self
    }
    
    // Get current progress value
    #[allow(dead_code)]
    pub fn value(&self) -> f32 {
        self.current_value
    }
    
    // Get progress as percentage (0-100)
    pub fn percentage(&self) -> f32 {
        let range = self.max_value - self.min_value;
        if range <= 0.0 {
            return 100.0;
        }
        ((self.current_value - self.min_value) / range) * 100.0
    }
    
    // Check if progress is complete
    pub fn is_complete(&self) -> bool {
        self.current_value >= self.max_value
    }
    
    // Set progress value
    pub fn set_value(&mut self, value: f32) {
        let was_complete = self.is_complete();
        let new_value = value.clamp(self.min_value, self.max_value);
        
        if self.animate {
            self.target_value = new_value;
        } else {
            self.current_value = new_value;
        }
        
        // Call completion callback if we just completed
        if !was_complete && self.is_complete() {
            if let Some(callback) = &self.completed_callback {
                callback();
            }
        }
    }
    
    // Increment progress by specified amount
    pub fn increment(&mut self, amount: f32) {
        self.set_value(self.current_value + amount);
    }
    
    // Update animation
    pub fn update(&mut self) {
        if self.animate && self.current_value != self.target_value {
            let was_complete = self.is_complete();
            
            // Calculate animation step based on delta time
            let diff = self.target_value - self.current_value;
            let step = diff * self.animation_speed * get_frame_time();
            
            // Apply smoother animation with minimum step
            if diff.abs() > 0.001 {
                self.current_value += step;
                
                // Ensure we don't overshoot
                if (step > 0.0 && self.current_value > self.target_value) ||
                   (step < 0.0 && self.current_value < self.target_value) {
                    self.current_value = self.target_value;
                }
                
                // Clamp to valid range
                self.current_value = self.current_value.clamp(self.min_value, self.max_value);
            } else {
                self.current_value = self.target_value;
            }
            
            // Call completion callback if we just completed
            if !was_complete && self.is_complete() {
                if let Some(callback) = &self.completed_callback {
                    callback();
                }
            }
        }
    }
    
    // Draw the progress bar
    pub fn draw(&mut self) {
        // Update animation state
        self.update();
        
        // Draw background
        if self.border {
            // Draw border
            draw_rectangle_lines(
                self.x, 
                self.y, 
                self.width, 
                self.height, 
                self.border_thickness, 
                self.border_color
            );
            
            // Draw background with inset to account for border
            draw_rectangle(
                self.x + self.border_thickness, 
                self.y + self.border_thickness, 
                self.width - 2.0 * self.border_thickness, 
                self.height - 2.0 * self.border_thickness, 
                self.background_color
            );
        } else {
            // Draw background without border
            draw_rectangle(self.x, self.y, self.width, self.height, self.background_color);
        }
        
        // Calculate fill size based on progress
        let progress_ratio = (self.current_value - self.min_value) / (self.max_value - self.min_value);
        let progress_ratio = progress_ratio.clamp(0.0, 1.0);
        
        let inset = if self.border { self.border_thickness } else { 0.0 };
        let available_width = self.width - 2.0 * inset;
        let available_height = self.height - 2.0 * inset;
        
        match self.orientation {
            ProgressBarOrientation::Horizontal => {
                // Draw fill bar
                let fill_width = progress_ratio * available_width;
                draw_rectangle(
                    self.x + inset,
                    self.y + inset,
                    fill_width,
                    available_height,
                    self.fill_color
                );
            },
            ProgressBarOrientation::Vertical => {
                // For vertical progress bar, fill from bottom to top
                let fill_height = progress_ratio * available_height;
                draw_rectangle(
                    self.x + inset,
                    self.y + available_height - fill_height + inset,
                    available_width,
                    fill_height,
                    self.fill_color
                );
            }
        }
        
        // Draw text content centered on the progress bar
        let mut display_text = String::new();
        
        // Add label if present
        if let Some(label) = &self.label {
            display_text.push_str(label);
            if self.show_percentage || self.show_value {
                display_text.push_str(": ");
            }
        }
        
        // Add percentage if enabled
        if self.show_percentage {
            display_text.push_str(&format!("{:.1}%", self.percentage()));
            if self.show_value {
                display_text.push_str(" - ");
            }
        }
        
        // Add actual value if enabled
        if self.show_value {
            display_text.push_str(&format!("{:.1}/{:.1}", self.current_value, self.max_value));
        }
        
        // Draw the text if we have anything to display
        if !display_text.is_empty() {
            let font_size = (self.height * 0.6).min(20.0) as u16;
            let text_size = measure_text(&display_text, None, font_size, 1.0);
            
            // Center text horizontally and vertically
            let text_x = self.x + (self.width - text_size.width) / 2.0;
            let text_y = self.y + (self.height + text_size.height) / 2.0;
            
            draw_text(
                &display_text,
                text_x,
                text_y,
                font_size as f32,
                self.text_color
            );
        }
    }
}