# A Collection of UI and Graphics Components for Macroquad

A Custom group of objects to use in macroquad rust program. The goal is they can be added to any program to add extra functions quickly. 
This is used with my VS Code extension: 
**https://github.com/Mathew-D/vs_code_rust**

## Components

### Buttons
- **ImageButton** (`buttons_image.rs`): Creates customizable image-based buttons with hover effects. Supports transparency detection for pixel-perfect clicking.
- **TextButton** (`buttons_text.rs`): Creates text-based buttons with customizable colors, hover effects, and enabled/disabled states.

### UI Elements
- **Label** (`label.rs`): A text display component with support for multiline text, customizable colors, and optional background.
- **TextBox** (`text_input.rs`): Interactive text input field with cursor control, handling keyboard input and text editing.
- **ListView** (`listview.rs`): A scrollable list of items with selection, customizable appearances, and mouse wheel support.
- **Slider** (`slider.rs`): Adjustable slider control for numeric input with both horizontal and vertical orientations. Ideal for volume controls, settings adjustment, and other value inputs.
- **MessageBox** (`messagebox.rs`): Dialog component for displaying messages and getting user feedback with customizable buttons. Features include:
  - Modal overlay to block background interaction
  - Multiple button options with keyboard navigation
  - Customizable colors and appearance
  - Draggable dialog window
  - Automatic text wrapping for long messages
  - Close button and Escape key support
- **ProgressBar** (`progressbar.rs`): Visual indicator for displaying progress or loading status. Features include:
  - Customizable appearance with configurable colors and borders
  - Support for horizontal and vertical orientations
  - Percentage display option
  - Min/max value range customization
  - Smooth animations for value changes

### Graphics and Layout
- **ImageObject** (`images_obj.rs`): Basic image display with support for scaling and positioning. Creates transparency masks for collision detection.
- **AnimatedImage** (`animated_image.rs`): Versatile animation component with three creation methods:
  - Spritesheet-based animations (grid of frames in one image)
  - Frame-by-frame animations (sequence of individual image files)
  - Direct GIF loading (supports animated GIFs with variable frame timing)
  
  Features include play/pause/stop controls, frame navigation, looping options, and full collision detection support. Works on both web and native platforms.
- **Grid** (`grid.rs`): Utility for drawing coordinate grids across the screen, useful for positioning elements during development.

### Collision
- **Collision** (`collision.rs`): Advanced pixel-perfect collision detection between image objects. Optimized versions for both web (WASM) and native platforms.

## Usage

Each component has detailed usage instructions in its corresponding file. To use these components:

1. Add the desired module to your project
2. Import the component in your code
3. Create and configure the component object
4. Use the component's methods in your game/application loop

See individual file headers for specific usage examples.

## Feature Dependencies

Some components require additional crates to enable full functionality:

- **AnimatedImage GIF Support**: Add the following to your Cargo.toml in the dependencies section:
  ```toml
  gif = "0.13"
  ```

- **Collision Detection for Native Platforms**: Add the following to your Cargo.toml:
  ```toml
  # Conditionally include Rayon only for native platforms (not Wasm)
  Into the dependencies section
  rayon = { version = "1.7", optional = true }
  
  Add the following as well.
  [features]
  default = ["native"]  # Default feature includes "native"
  native = ["rayon"]    # The "native" feature enables Rayon
  
  [target.'cfg(not(target_arch = "wasm32"))'.dependencies]
  rayon = "1.7"  # Rayon is only included for native builds
  ```
