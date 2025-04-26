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

### Graphics and Layout
- **ImageObject** (`images_obj.rs`): Basic image display with support for scaling and positioning. Creates transparency masks for collision detection.
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
