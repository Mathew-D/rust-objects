/*
Made by: Mathew Dusome
April 26 2025

Adds a list view widget
In the mod objects section add:
        pub mod listview;


Add with the other use statements
    use objects::listview::ListView;

Then to use this you would put the following above the loop: 
    let list_items = vec!["Item 1", "Item 2", "Item 3", "Item 4"];
    let list_view = ListView::new(&list_items, 50.0, 100.0, 20);
Where the numbers are x, y, font size

You can also set the colors and item spacing by using:
    .with_colors(BLACK, Some(LIGHTGRAY), Some(BLUE))
Where the colors are text color, background color, and selection color respectively. 
    
    .with_spacing(1.5)
    .with_padding(10.0)
Where the spacing is the line spacing and padding is the padding around the text.

You can set visible items and enable scrolling with:
    .with_max_visible_items(5)

List management functions:
    .add_item("New Item")    - Add a single item to the list
    .add_items(&vec!["Item A", "Item B"])    - Add multiple items at once
    .clear()    - Remove all items from the list
    .remove_item(index)    - Remove item at specific index

Full Example:
   
    let list_items = vec!["Item 1", "Item 2", "Item 3", "Item 4"];
    let mut list_view = ListView::new(&list_items, 50.0, 100.0, 20)
        .with_colors(BLACK, Some(LIGHTGRAY), Some(BLUE))
        .with_spacing(1.5)
        .with_padding(10.0)
        .with_max_visible_items(5); // Enable scrolling by limiting visible items

    // list_items can still be used here since ListView::new() takes a reference
    
    // Adding more items (note the & reference)
    let more_items = vec!["Item 5", "Item 6"];
    list_view.add_items(&more_items);
    // more_items can still be used here

Then in the loop you would do:
    list_view.draw(); 
*/

use macroquad::prelude::*;

pub struct ListView {
    items: Vec<String>,
    x: f32,
    y: f32,
    font_size: u16,
    foreground: Color,
    background: Option<Color>,
    selection_color: Option<Color>,
    selected_index: Option<usize>,
    item_spacing: f32,
    item_padding: f32,
    scroll_offset: usize,
    max_visible_items: Option<usize>,
    show_scrollbar: bool,
    scrollbar_width: f32,
    scrollbar_color: Color,
    scrollbar_handle_color: Color,
}

impl ListView {
    // Constructor with a vector of strings (takes a reference to avoid taking ownership)
    pub fn new<T: ToString + Clone>(items: &Vec<T>, x: f32, y: f32, font_size: u16) -> Self {
        Self {
            items: items.iter().map(|item| item.to_string()).collect(),
            x,
            y,
            font_size,
            foreground: BLACK, // Default text color
            background: None,  // No background by default
            selection_color: Some(SKYBLUE), // Default selection color
            selected_index: None,
            item_spacing: 1.2, // Default line spacing
            item_padding: 5.0, // Default padding
            scroll_offset: 0,
            max_visible_items: None, // By default, show all items
            show_scrollbar: true,
            scrollbar_width: 10.0,
            scrollbar_color: Color::new(0.7, 0.7, 0.7, 0.7), // Light gray, semi-transparent
            scrollbar_handle_color: Color::new(0.5, 0.5, 0.5, 0.8), // Darker gray
        }
    }

    // Method to set foreground, background, and selection colors
    pub fn with_colors(mut self, foreground: Color, background: Option<Color>, selection_color: Option<Color>) -> Self {
        self.foreground = foreground;
        self.background = background;
        self.selection_color = selection_color;
        self
    }

    // Method to set item spacing
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.item_spacing = spacing;
        self
    }

    // Method to set item padding
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.item_padding = padding;
        self
    }

    // Method to set max visible items (enables scrolling)
    pub fn with_max_visible_items(mut self, count: usize) -> Self {
        self.max_visible_items = Some(count);
        self
    }

    // Method to customize scrollbar
    #[allow(unused)]
    pub fn with_scrollbar_settings(mut self, show: bool, width: f32, color: Color, handle_color: Color) -> Self {
        self.show_scrollbar = show;
        self.scrollbar_width = width;
        self.scrollbar_color = color;
        self.scrollbar_handle_color = handle_color;
        self
    }

    // Method to add an item
    #[allow(unused)]
    pub fn add_item<T: ToString>(&mut self, item: T) {
        self.items.push(item.to_string());
    }

    // Method to add multiple items
    #[allow(unused)]
    pub fn add_items<T: ToString + Clone>(&mut self, items: &Vec<T>) {
        for item in items {
            self.items.push(item.to_string());
        }
    }

    // Method to clear all items
    #[allow(unused)]
    pub fn clear(&mut self) {
        self.items.clear();
        self.selected_index = None;
        self.scroll_offset = 0;
    }

    // Method to remove an item at specific index
    #[allow(unused)]
    pub fn remove_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
            // If we removed the selected item, clear the selection
            if let Some(selected) = self.selected_index {
                if selected == index || selected >= self.items.len() {
                    self.selected_index = None;
                } else if selected > index {
                    // Adjust selection if we removed an item before it
                    self.selected_index = Some(selected - 1);
                }
            }
            
            // Adjust scroll offset if needed
            if self.scroll_offset > 0 && self.scroll_offset >= self.items.len() {
                self.scroll_offset = self.items.len().saturating_sub(1);
            }
        }
    }

    // Method to get the current selected item
    pub fn selected_item(&self) -> Option<&String> {
        self.selected_index.and_then(|index| self.items.get(index))
    }

    // Method to select an item
    #[allow(unused)]
    pub fn select_item(&mut self, index: Option<usize>) {
        if index.is_none() || index.unwrap() < self.items.len() {
            self.selected_index = index;
            
            // Auto-scroll to show selected item if needed
            if let Some(idx) = index {
                if let Some(max_items) = self.max_visible_items {
                    if idx < self.scroll_offset {
                        // Selected item is above visible area
                        self.scroll_offset = idx;
                    } else if idx >= self.scroll_offset + max_items {
                        // Selected item is below visible area
                        self.scroll_offset = idx.saturating_sub(max_items) + 1;
                    }
                }
            }
        }
    }

    // Calculate dimensions based on content
    fn calculate_dimensions(&self) -> (f32, f32) {
        let item_height = self.font_size as f32 * self.item_spacing;
        
        // Find the maximum width of any item
        let content_width = self.items.iter()
            .map(|item| measure_text(item, None, self.font_size, 1.0).width)
            .fold(0.0, f32::max);
            
        let width = content_width + 2.0 * self.item_padding;
        
        // Calculate height based on number of visible items
        let visible_count = match self.max_visible_items {
            Some(count) => count.min(self.items.len()),
            None => self.items.len(),
        };
        
        let height = visible_count as f32 * item_height + 2.0 * self.item_padding;
        
        (width, height)
    }

    // Handle mouse wheel scrolling
    fn handle_scroll(&mut self) {
        if let Some(max_visible) = self.max_visible_items {
            if self.items.len() <= max_visible {
                // No need to scroll if all items fit
                return;
            }
            
            let wheel_movement = mouse_wheel().1;
            if wheel_movement != 0.0 {
                let mouse_pos = mouse_position();
                let (width, height) = self.calculate_dimensions();
                
                // Check if mouse is over the list area
                let list_rect = Rect::new(
                    self.x - self.item_padding, 
                    self.y - self.font_size as f32 + self.item_padding,
                    width + if self.show_scrollbar { self.scrollbar_width } else { 0.0 },
                    height
                );
                
                if list_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
                    // Scroll up or down based on wheel movement
                    if wheel_movement > 0.0 {
                        // Scroll up
                        self.scroll_offset = self.scroll_offset.saturating_sub(1);
                    } else {
                        // Scroll down, ensuring we don't scroll past the last item
                        let max_offset = self.items.len().saturating_sub(max_visible);
                        self.scroll_offset = (self.scroll_offset + 1).min(max_offset);
                    }
                }
            }
        }
    }

    // Handle scrollbar interaction
    fn handle_scrollbar_interaction(&mut self) {
        if !self.show_scrollbar || self.max_visible_items.is_none() {
            return;
        }
        
        let max_visible = self.max_visible_items.unwrap();
        if self.items.len() <= max_visible {
            return; // No need for scrollbar
        }
        
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let (width, height) = self.calculate_dimensions();
            
            // Scrollbar area
            let scrollbar_rect = Rect::new(
                self.x + width,
                self.y - self.font_size as f32 + self.item_padding,
                self.scrollbar_width,
                height
            );
            
            if scrollbar_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
                // Calculate the relative position on the scrollbar (0.0 to 1.0)
                let relative_y = (mouse_pos.1 - scrollbar_rect.y) / scrollbar_rect.h;
                let max_offset = self.items.len().saturating_sub(max_visible);
                
                // Set scroll offset based on relative position
                self.scroll_offset = (relative_y * max_offset as f32).round() as usize;
                self.scroll_offset = self.scroll_offset.min(max_offset);
            }
        }
    }

    // Method to handle click and update selection
    fn update(&mut self) {
        // Handle scrolling with mouse wheel
        self.handle_scroll();
        
        // Handle scrollbar dragging
        self.handle_scrollbar_interaction();
        
        // Handle item selection
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let item_height = self.font_size as f32 * self.item_spacing;
            let (width, _) = self.calculate_dimensions();
            
            // Check if click is within the items area (not on scrollbar)
            let list_rect = Rect::new(
                self.x - self.item_padding,
                self.y - self.font_size as f32 + self.item_padding,
                width, // Don't include scrollbar in click detection for items
                match self.max_visible_items {
                    Some(count) => count as f32 * item_height,
                    None => self.items.len() as f32 * item_height,
                }
            );
            
            if list_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
                // Calculate which item was clicked based on y position
                let relative_y = mouse_pos.1 - (self.y - self.font_size as f32 + self.item_padding);
                let item_index = (relative_y / item_height).floor() as usize + self.scroll_offset;
                
                if item_index < self.items.len() {
                    self.selected_index = Some(item_index);
                }
            }
        }
    }

    // Method to draw the list view
    pub fn draw(&mut self) {
        // Handle all updates first (previously in the update method)
        self.update();
        
        let item_height = self.font_size as f32 * self.item_spacing;
        let (width, height) = self.calculate_dimensions();
        
        // Calculate total width including scrollbar if needed
        let total_width = width + if self.show_scrollbar && self.max_visible_items.is_some() && 
                               self.items.len() > self.max_visible_items.unwrap() {
            self.scrollbar_width
        } else {
            0.0
        };
        
        // Draw the overall background if specified
        if let Some(bg) = self.background {
            draw_rectangle(
                self.x - self.item_padding,
                self.y - self.font_size as f32 + self.item_padding,
                total_width,
                height,
                bg,
            );
        }
        
        // Determine visible range of items
        let visible_count = match self.max_visible_items {
            Some(count) => count.min(self.items.len()),
            None => self.items.len(),
        };
        
        let end_idx = (self.scroll_offset + visible_count).min(self.items.len());
        let visible_items = &self.items[self.scroll_offset..end_idx];
        
        // Draw visible items
        for (i, item) in visible_items.iter().enumerate() {
            let actual_index = i + self.scroll_offset;
            let y_pos = self.y + i as f32 * item_height;
            
            // Draw selection background if this is the selected item
            if let Some(sel_index) = self.selected_index {
                if actual_index == sel_index && self.selection_color.is_some() {
                    draw_rectangle(
                        self.x - self.item_padding,
                        y_pos - self.font_size as f32 + self.item_padding,
                        width,
                        item_height,
                        self.selection_color.unwrap(),
                    );
                }
            }
            
            // Calculate vertical centering for the text
            let text_dims = measure_text(item, None, self.font_size, 1.0);
            let text_y_offset = (item_height - text_dims.height) / 2.0;
            
            // Draw the item text (vertically centered)
            draw_text(
                item, 
                self.x, 
                y_pos + text_y_offset, 
                self.font_size as f32, 
                self.foreground
            );
        }
        
        // Draw scrollbar if needed
        if self.show_scrollbar && self.max_visible_items.is_some() {
            let max_visible = self.max_visible_items.unwrap();
            if self.items.len() > max_visible {
                // Draw scrollbar background
                draw_rectangle(
                    self.x + width,
                    self.y - self.font_size as f32 + self.item_padding,
                    self.scrollbar_width,
                    height,
                    self.scrollbar_color,
                );
                
                // Calculate and draw scrollbar handle
                let handle_ratio = max_visible as f32 / self.items.len() as f32;
                let handle_height = height * handle_ratio;
                let max_scrollable = self.items.len() - max_visible;
                let handle_position = if max_scrollable > 0 {
                    (self.scroll_offset as f32 / max_scrollable as f32) * (height - handle_height)
                } else {
                    0.0
                };
                
                draw_rectangle(
                    self.x + width,
                    self.y - self.font_size as f32 + self.item_padding + handle_position,
                    self.scrollbar_width,
                    handle_height,
                    self.scrollbar_handle_color,
                );
            }
        }
    }
}
