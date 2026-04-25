# A Collection of UI and Graphics Components for Macroquad

A Custom group of modules to use in macroquad rust program. The goal is they can be added to any program to add extra functions quickly. 
This is used with my VS Code extension: 
**https://github.com/Mathew-D/vs_code_rust**

## Components

### Buttons
- **ImageButton** (`image_button.rs`): Creates customizable image-based buttons with hover effects. Supports transparency detection for pixel-perfect clicking.
- **TextButton** (`text_button.rs`): Creates text-based buttons with customizable colors, hover effects, and enabled/disabled states.

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
- **StillImage** (`still_image.rs`): Basic image display with support for scaling and positioning. Creates transparency masks for collision detection.
- **AnimatedImage** (`animated_image.rs`): Versatile animation component with three creation methods:
  - Spritesheet-based animations (grid of frames in one image)
  - Frame-by-frame animations (sequence of individual image files)
  - Direct GIF loading (supports animated GIFs with variable frame timing)
  
  Features include play/pause/stop controls, frame navigation, looping options, and full collision detection support. Works on both web and native platforms.
- **Grid** (`grid.rs`): Utility for drawing coordinate grids across the screen, useful for positioning elements during development.

### Utilities
- **Scale** (`scale.rs`): Provides functions and helpers for scaling UI elements and graphics to fit different screen sizes and resolutions. Useful for responsive layouts and adapting to various device displays.
- **TextureManager** (`image_preload.rs`): Central texture manager for preloading and sharing textures. Reduces memory usage and prevents flickering when switching images. Provides methods for loading images individually or in batches and accessing them by path or index.

### Collision
- **Collision** (`collision.rs`): Advanced pixel-perfect collision detection between image objects. Optimized versions for both web (WASM) and native platforms.

### Data Management
**TextFile** (`textfiles.rs`):
  - Cross-platform file I/O utility for saving and loading text files, numbers, and strings.
  - Unified async API for both native and web (WASM) platforms.
  - On desktop: saves files with the exact filename you provide (e.g., `player_names.txt`).
  - On web: uses browser's localStorage for persistence; asset loading requires files in the `assets` directory.
  - Example usage and error handling are provided in the file header.

**Database** (`database.rs`):
  - Turso (libSQL) database connectivity only (other providers removed).
  - Works on both native and web (WASM) platforms with the same API—no extra dependencies or imports required for WASM.
  - Table creation, management, and full CRUD operations (Create, Read, Update, Delete).
  - Flexible querying, including fetch by ID and custom SQL queries.
  - Usage examples and schema customization instructions are provided in the file header.

## Usage

Each component has detailed usage instructions in its corresponding file. To use these components:

1. Add the desired module to your project
2. Import the component in your code
3. Create and configure the component object
4. Use the component's methods in your game/application loop

See individual file headers for specific usage examples.

## Feature Dependencies


Some components require additional crates to enable full functionality. Make sure to add the following to your `Cargo.toml` as needed:

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


- **TextFile**: No extra dependencies are required for any platform. All file and localStorage support is built-in.

- **Database Connectivity (Turso only)**: Add the following to your `Cargo.toml`:
  ```toml
  [dependencies]
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  
  
  [target.'cfg(not(target_arch = "wasm32"))'.dependencies]
  ureq = { version = "2.9", features = ["json"] }
  ```
