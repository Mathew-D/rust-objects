/*
Made by: Mathew Dusome
Feb 6 2025
Add a grid object for placement
To import you need:

In the mod modules section add:
    pub mod grid;
    
Then add the following with the use commands:
use crate::modules::grid::draw_grid;

Then in side the loop I would use:
    draw_grid(50.0, BROWN);
*/
use macroquad::prelude::*;
pub fn draw_grid(grid_size: f32, color: Color) {
    let screen_width = screen_width();
    let screen_height = screen_height();
    
    // Draw vertical lines and labels
    for x in (0..screen_width as i32).step_by(grid_size as usize) {
        draw_line(x as f32, 0.0, x as f32, screen_height, 1.0, color);
        draw_text(&format!("{}", x), x as f32 + 2.0, 12.0, 16.0, color);
    }
    
    // Draw horizontal lines and labels
    for y in (0..screen_height as i32).step_by(grid_size as usize) {
        draw_line(0.0, y as f32, screen_width, y as f32, 1.0, color);
        draw_text(&format!("{}", y), 2.0, y as f32 + 12.0, 16.0, color);
    }
}
