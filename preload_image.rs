/*
Made by:Mathew Dusome
Date: 2025-05-03
Program Details: Central texture manager for preloading and sharing textures

To use this:
1. In your mod.rs file located in the modules folder add the following to the end of the file:
    pub mod preload_image;
    
2. Add the following use command:
    use crate::modules::preload_image::TextureManager;

3. Create and initialize a TextureManager:
    let mut texture_manager = TextureManager::new();
    
4. Preload your textures at startup:
    // Preload a list of textures
    texture_manager.preload_all(&["assets/image1.png", "assets/image2.png"]).await;
    
    // Or preload individual textures
    texture_manager.preload("assets/image3.png").await;
    
5. Get preloaded textures for use with ImageObject - two approaches:

   // Approach 1: Using unwrap() - Simple but will panic if image doesn't exist
   // Only use this when you're certain the texture was preloaded
   image_obj.set_preload(texture_manager.get_preload("assets/image1.png").unwrap());
   
   // Approach 2: Using if let Some() - Safer, handles missing textures gracefully
   if let Some(preloaded) = texture_manager.get_preload("assets/image2.png") {
       img.set_preload(preloaded);
   } else {
       println!("Warning: Image not found in texture manager");
       // Handle the error case (e.g., try to load it or use a placeholder)
   }
    
6. Access textures by index:
    // Using unwrap() approach:
    img.set_preload(texture_manager.get_preload_by_index(0).unwrap());
    
    // Using if let Some() approach:
    if let Some(preloaded) = texture_manager.get_preload_by_index(1) {
        img.set_preload(preloaded);
    }
    
7. Getting the number of preloaded textures:
    let count = texture_manager.texture_count();
    
8. For implementing features like image slideshows, you can increment an index
   and wrap around to cycle through all images:
    current_index = (current_index + 1) % texture_manager.texture_count();
    
    // Using unwrap() (assumes there are textures available):
    img.set_preload(texture_manager.get_preload_by_index(current_index).unwrap());
    
    // Or more safely with error handling:
    if texture_manager.texture_count() > 0 {
        if let Some(preloaded) = texture_manager.get_preload_by_index(current_index) {
            img.set_preload(preloaded);
        }
    }

Note: For clearing images, use the clear() method directly on the ImageObject:
    img.clear();
*/
use macroquad::texture::Texture2D;
use std::collections::HashMap;
use crate::modules::still_image::set_texture_main;

/// A central texture manager to preload and share textures
/// This reduces memory usage and prevents flickering when switching images
pub struct TextureManager {
    textures: HashMap<String, (Texture2D, Vec<u8>)>,
    load_order: Vec<String>, // Store just the order textures were loaded in
}

impl TextureManager {
    /// Create a new texture manager
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            load_order: Vec::new(),
        }
    }
    
    /// Preload a texture by its file path
    pub async fn preload(&mut self, path: &str) {
        if !self.textures.contains_key(path) {
            let (texture, mask) = set_texture_main(path).await;
            self.textures.insert(path.to_string(), (texture, mask));
            self.load_order.push(path.to_string()); // Store just the load order
        }
    }
    
    /// Preload multiple textures at once
    #[allow(unused)]
    pub async fn preload_all(&mut self, paths: &[&str]) {
        for path in paths {
            self.preload(path).await;
        }
    }
    
    /// Get a preloaded texture for use in an ImageObject
    #[allow(unused)]
    pub fn get_preload(&self, path: &str) -> Option<(Texture2D, Vec<u8>, String)> {
        self.textures.get(path).map(|(texture, mask)| 
            (texture.clone(), mask.clone(), path.to_string())
        )
    }
    
    /// Get a preloaded texture by its index in the preload order
    #[allow(unused)]
    pub fn get_preload_by_index(&self, index: usize) -> Option<(Texture2D, Vec<u8>, String)> {
        if index < self.load_order.len() {
            let path = &self.load_order[index];
            self.get_preload(path)
        } else {
            None
        }
    }
    
    /// Get the number of preloaded textures
    #[allow(unused)]
    pub fn texture_count(&self) -> usize {
        self.load_order.len()
    }
    
    /// Get a list of all preloaded texture paths in load order
    #[allow(unused)]
    pub fn get_texture_paths(&self) -> &[String] {
        &self.load_order
    }
}
