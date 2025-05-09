/*
Made by: Mathew Dusome
Lets us check for collisions with pixels.  One version for web one for native 
linux and windows
Must add the following to Cargo.toml

# Conditionally include Rayon only for native platforms (not Wasm)
rayon = { version = "1.7", optional = true }
[features]
default = ["native"]  # Default feature includes "native"
native = ["rayon"]    # The "native" feature enables Rayon
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
rayon = "1.7"  # Rayon is only included for native builds

In your mod.rs file located in the modules folder add the following to the end of the file:
    pub mod collision;
Then in with the other use command add:

use crate::modules::collision::check_collision;
 
Then in the loop you would use the follow to check if two images hit: 
// This works with StillImage:
let collision = check_collision(&img1, &img2, 1); //Where 1 is the number of pixels to skip

// If you're using AnimatedImage:
let collision = check_collision(&anim1, &anim2, 1);

// You can even mix them:
let collision = check_collision(&img1, &anim1, 1);
*/

use macroquad::prelude::{Vec2};

// Import StillImage which is our base image type
use crate::modules::still_image::StillImage;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

// Define a trait for objects that can collide
pub trait Collidable {
    fn pos(&self) -> Vec2;
    fn size(&self) -> Vec2;
    fn texture_size(&self) -> Vec2;
    fn get_mask(&self) -> Option<Vec<u8>>;
}

// Implement for StillImage
impl Collidable for StillImage {
    fn pos(&self) -> Vec2 {
        self.pos()
    }
    
    fn size(&self) -> Vec2 {
        self.size()
    }
    
    fn texture_size(&self) -> Vec2 {
        self.texture_size()
    }
    
    fn get_mask(&self) -> Option<Vec<u8>> {
        self.get_mask()
    }
}

// The AnimatedImage implementation is in a separate module that will only
// be included when the animated_image module is available
pub mod animated {
    use super::Collidable;
    use macroquad::prelude::Vec2;
    
    // Try to import AnimatedImage - this will only compile if the module exists
    #[cfg(not(any(test, doc)))]  // This is a trick to make it work in docs and tests
    use crate::modules::animated_image::AnimatedImage;
    
    // AnimatedImage will only be available in actual code if the module exists
    #[cfg(not(any(test, doc)))]
    impl Collidable for AnimatedImage {
        fn pos(&self) -> Vec2 {
            self.pos()
        }
        
        fn size(&self) -> Vec2 {
            self.size()
        }
        
        fn texture_size(&self) -> Vec2 {
            self.texture_size()
        }
        
        fn get_mask(&self) -> Option<Vec<u8>> {
            self.get_mask()
        }
    }
}

// Generic collision detection function that works with anything implementing Collidable
pub fn check_collision<T, U>(obj1: &T, obj2: &U, skip_pixels: usize) -> bool
where
    T: Collidable,
    U: Collidable,
{
    let pos1 = obj1.pos();
    let size1 = obj1.size();
    let mask1_opt = obj1.get_mask();
    let texture1_size = obj1.texture_size();

    let pos2 = obj2.pos();
    let size2 = obj2.size();
    let mask2_opt = obj2.get_mask();
    let texture2_size = obj2.texture_size();
    
    // If either mask is None, we can use a simplified bounding box collision
    if mask1_opt.is_none() || mask2_opt.is_none() {
        // Simple bounding box check
        let overlap_x = pos1.x.max(pos2.x);
        let overlap_y = pos1.y.max(pos2.y);
        let overlap_w = (pos1.x + size1.x).min(pos2.x + size2.x) - overlap_x;
        let overlap_h = (pos1.y + size1.y).min(pos2.y + size2.y) - overlap_y;
        
        return overlap_w > 0.0 && overlap_h > 0.0;
    }
    
    // Unwrap the masks - safe because we checked above
    let mask1 = mask1_opt.unwrap();
    let mask2 = mask2_opt.unwrap();

    let overlap_x = pos1.x.max(pos2.x);
    let overlap_y = pos1.y.max(pos2.y);
    let overlap_w = (pos1.x + size1.x).min(pos2.x + size2.x) - overlap_x;
    let overlap_h = (pos1.y + size1.y).min(pos2.y + size2.y) - overlap_y;
    
    if overlap_w <= 0.0 || overlap_h <= 0.0 {
        return false; // No overlap
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Parallel processing (Rayon) on Linux/Windows
        return (0..overlap_h as usize).into_par_iter().step_by(skip_pixels).any(|y| {
            (0..overlap_w as usize).into_par_iter().step_by(skip_pixels).any(|x| {
                let tx1 = ((overlap_x + x as f32 - pos1.x) / size1.x * texture1_size.x) as usize;
                let ty1 = ((overlap_y + y as f32 - pos1.y) / size1.y * texture1_size.y) as usize;
                let tx2 = ((overlap_x + x as f32 - pos2.x) / size2.x * texture2_size.x) as usize;
                let ty2 = ((overlap_y + y as f32 - pos2.y) / size2.y * texture2_size.y) as usize;

                let idx1 = ty1 * texture1_size.x as usize + tx1;
                let idx2 = ty2 * texture2_size.x as usize + tx2;

                // Check the corresponding bit for both masks
                let mask1_byte = mask1[idx1 / 8];
                let mask2_byte = mask2[idx2 / 8];
                let mask1_bit = (mask1_byte >> (7 - (idx1 % 8))) & 1;
                let mask2_bit = (mask2_byte >> (7 - (idx2 % 8))) & 1;
                
                // If both bits are set, we have a collision
                if mask1_bit == 1 && mask2_bit == 1 {
                    return true; // Collision detected, exit early
                }
                false // No collision at this pixel
            })
        });
    }

    #[cfg(target_arch = "wasm32")]
    {
        // Sequential for Web (WASM)
        for y in (0..overlap_h as usize).step_by(skip_pixels) {
            for x in (0..overlap_w as usize).step_by(skip_pixels) {
                let tx1 = ((overlap_x + x as f32 - pos1.x) / size1.x * texture1_size.x) as usize;
                let ty1 = ((overlap_y + y as f32 - pos1.y) / size1.y * texture1_size.y) as usize;
                let tx2 = ((overlap_x + x as f32 - pos2.x) / size2.x * texture2_size.x) as usize;
                let ty2 = ((overlap_y + y as f32 - pos2.y) / size2.y * texture2_size.y) as usize;

                let idx1 = ty1 * texture1_size.x as usize + tx1;
                let idx2 = ty2 * texture2_size.x as usize + tx2;

                let mask1_byte = mask1[idx1 / 8];
                let mask2_byte = mask2[idx2 / 8];
                let mask1_bit = (mask1_byte >> (7 - (idx1 % 8))) & 1;
                let mask2_bit = (mask2_byte >> (7 - (idx2 % 8))) & 1;

                if mask1_bit == 1 && mask2_bit == 1 {
                    return true; // Collision detected
                }
            }
        }
        false
    }
}
