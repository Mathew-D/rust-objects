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

In the mod objects section add:
    pub mod collision;
Then in with the other use command add:

use objects::collision::check_collision;
 
Then in the loop you would use the follow to check if two images hit: 
 let collision = check_collision(&img1, &img2, 1); //Where 1 is the number of pixels to skip
    if collision {
        println!("Collision detected!");
    } else {
        println!("No collision.");
    }
*/

use crate::objects::images_obj::ImageObject;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

pub fn check_collision(
    img1: &ImageObject,
    img2: &ImageObject,
    skip_pixels: usize
) -> bool {
    let pos1 = img1.pos();
    let size1 = img1.size();
    let mask1 = img1.get_mask();
    let texture1_size = img1.texture_size();

    let pos2 = img2.pos();
    let size2 = img2.size();
    let mask2 = img2.get_mask();
    let texture2_size = img2.texture_size();

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
