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
let collision = check_collision(&img1, &img2, 1); //Where 1 is the number of pixels to skip
    if collision {
        println!("Collision detected!");
    } else {
        println!("No collision.");
    }
*/

use macroquad::prelude::{Vec2};
use crate::modules::still_image::StillImage;
use crate::modules::animated_image::AnimatedImage;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

// Define the Collidable trait
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
/* 
use crate::modules::animated_image::AnimatedImage;
// Implement for AnimatedImage
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
*/
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
    
    // Calculate bounding box overlap
    let overlap_x = pos1.x.max(pos2.x);
    let overlap_y = pos1.y.max(pos2.y);
    let overlap_w = (pos1.x + size1.x).min(pos2.x + size2.x) - overlap_x;
    let overlap_h = (pos1.y + size1.y).min(pos2.y + size2.y) - overlap_y;
    
    // Quick early exit if no bounding box overlap
    if overlap_w <= 0.0 || overlap_h <= 0.0 {
        return false; // No overlap
    }
    
    // If both masks are None, use simple bounding box collision
    if mask1_opt.is_none() && mask2_opt.is_none() {
        return true; // Bounding boxes overlap
    }
    
    // Handle case where only one mask is available
    if mask1_opt.is_some() && mask2_opt.is_none() {
        // Only obj1 has a mask
        return check_one_masked_collision(
            &pos1, &size1, &texture1_size, &mask1_opt.unwrap(),
            &pos2, &size2,
            &overlap_x, &overlap_y, &overlap_w, &overlap_h,
            skip_pixels
        );
    }
    
    if mask1_opt.is_none() && mask2_opt.is_some() {
        // Only obj2 has a mask
        return check_one_masked_collision(
            &pos2, &size2, &texture2_size, &mask2_opt.unwrap(),
            &pos1, &size1,
            &overlap_x, &overlap_y, &overlap_w, &overlap_h,
            skip_pixels
        );
    }
    
    // If we get here, both objects have masks
    let mask1 = mask1_opt.unwrap();
    let mask2 = mask2_opt.unwrap();

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
                mask1_bit == 1 && mask2_bit == 1
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

// Helper function for collision detection when only one object has a mask
#[inline]
fn check_one_masked_collision(
    masked_pos: &Vec2,
    masked_size: &Vec2,
    masked_tex_size: &Vec2,
    mask: &Vec<u8>,
    other_pos: &Vec2,
    other_size: &Vec2,
    overlap_x: &f32,
    overlap_y: &f32,
    overlap_w: &f32,
    overlap_h: &f32,
    skip_pixels: usize
) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Parallel processing for Linux/Windows
        return (0..*overlap_h as usize).into_par_iter().step_by(skip_pixels).any(|y| {
            (0..*overlap_w as usize).into_par_iter().step_by(skip_pixels).any(|x| {
                // Calculate texture coordinate in the masked object
                let tx = ((*overlap_x + x as f32 - masked_pos.x) / masked_size.x * masked_tex_size.x) as usize;
                let ty = ((*overlap_y + y as f32 - masked_pos.y) / masked_size.y * masked_tex_size.y) as usize;
                
                // Calculate bit index and check if pixel is opaque
                let idx = ty * masked_tex_size.x as usize + tx;
                let mask_byte = mask[idx / 8];
                let mask_bit = (mask_byte >> (7 - (idx % 8))) & 1;
                
                // The point overlaps if it's within the other object's bounds and the mask bit is set
                let point_x = *overlap_x + x as f32;
                let point_y = *overlap_y + y as f32;
                let in_other_bounds = point_x >= other_pos.x && point_x < other_pos.x + other_size.x &&
                                     point_y >= other_pos.y && point_y < other_pos.y + other_size.y;
                
                mask_bit == 1 && in_other_bounds
            })
        });
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        // Sequential for Web (WASM)
        for y in (0..*overlap_h as usize).step_by(skip_pixels) {
            for x in (0..*overlap_w as usize).step_by(skip_pixels) {
                // Calculate texture coordinate in the masked object
                let tx = ((*overlap_x + x as f32 - masked_pos.x) / masked_size.x * masked_tex_size.x) as usize;
                let ty = ((*overlap_y + y as f32 - masked_pos.y) / masked_size.y * masked_tex_size.y) as usize;
                
                // Calculate bit index and check if pixel is opaque
                let idx = ty * masked_tex_size.x as usize + tx;
                let mask_byte = mask[idx / 8];
                let mask_bit = (mask_byte >> (7 - (idx % 8))) & 1;
                
                // The point overlaps if it's within the other object's bounds and the mask bit is set
                let point_x = *overlap_x + x as f32;
                let point_y = *overlap_y + y as f32;
                let in_other_bounds = point_x >= other_pos.x && point_x < other_pos.x + other_size.x &&
                                     point_y >= other_pos.y && point_y < other_pos.y + other_size.y;
                
                if mask_bit == 1 && in_other_bounds {
                    return true;
                }
            }
        }
        false
    }
}
