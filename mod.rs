/*
--------------------------------------------
modules/mod.rs
This file lists which modules (Rust files) are part of the "modules" folder.

This file just tells Rust what's available. It's like a directory of all the tools.

Example:
pub mod grid;

Once listed here, you can import from main.rs:
use crate::modules::grid::draw_grid;
--------------------------------------------
*/
// Add modules below
// Include the scale module when the scale feature is enabled
#[cfg(feature = "scale")]
pub mod scale;

// Include the grid module
pub mod grid;--------------------------------
modules/mod.rs
This file lists which modules (Rust files) are part of the "modules" folder.

This file just tells Rust what’s available. It’s like a directory of all the tools.

Example:
pub mod grid;

Once listed here, you can import from main.rs:
use crate::modules::grid::draw_grid;
--------------------------------------------
*/
// Add modules below