# Rust Advanced Guide
*Created by: Mathew Dusome*  
*Date: April 27, 2025*

This guide contains advanced Rust programming concepts for use with Macroquad:

1. Structs and Enums
2. Option and Result Types
3. Ownership and Borrowing
4. Multi-Screen Management
5. Functional Programming Features
6. Implementing Traits

## 1. Structs and Enums

### Defining and Using Structs

```rust
// Basic struct definition
struct Player {
    name: String,
    health: i32,
    position: (f32, f32),
    is_active: bool,
}

// Creating a struct instance
let player1 = Player {
    name: String::from("Player 1"),
    health: 100,
    position: (0.0, 0.0),
    is_active: true,
};

// Accessing struct fields
println!("Player name: {}", player1.name);
println!("Current health: {}", player1.health);

// Using a mutable struct
let mut player2 = Player {
    name: String::from("Player 2"),
    health: 100,
    position: (10.0, 10.0),
    is_active: true,
};

// Modifying fields
player2.health -= 20;
player2.position = (15.0, 20.0);
```

### Struct Methods

```rust
// Define a struct with methods
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    // Constructor method
    fn new(width: f32, height: f32) -> Self {
        Rectangle { width, height }
    }
    
    // Method that calculates area
    fn area(&self) -> f32 {
        self.width * self.height
    }
    
    // Method that modifies the rectangle
    fn scale(&mut self, factor: f32) {
        self.width *= factor;
        self.height *= factor;
    }
}

// Using the struct with methods
let mut rect = Rectangle::new(30.0, 50.0);
println!("Area: {}", rect.area());  // 1500.0

rect.scale(2.0);
println!("Scaled area: {}", rect.area());  // 6000.0
```

### Enums

```rust
// Basic enum
enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

// Using the enum
let current_state = GameState::MainMenu;

// Making decisions based on enum values
match current_state {
    GameState::MainMenu => println!("In the main menu"),
    GameState::Playing => println!("Game is running"),
    GameState::Paused => println!("Game is paused"),
    GameState::GameOver => println!("Game over!"),
}

// Enum with associated values
enum Item {
    Weapon(String, i32),  // name, damage
    Potion(String, i32),  // type, healing amount
    Armor(i32),          // defense value
}

// Using enum with values
let inventory = vec![
    Item::Weapon(String::from("Sword"), 25),
    Item::Potion(String::from("Health"), 50),
    Item::Armor(10),
];

// Processing items
for item in &inventory {
    match item {
        Item::Weapon(name, damage) => println!("{} deals {} damage", name, damage),
        Item::Potion(kind, amount) => println!("{} potion heals {} health", kind, amount),
        Item::Armor(defense) => println!("Armor provides {} defense", defense),
    }
}
```

## 2. Option and Result Types

### Option Type

```rust
// Option represents a value that might be present or absent
// Option<T> can be either Some(T) or None

// Function that may return a value
fn find_player_by_id(id: i32) -> Option<String> {
    if id == 1 {
        Some(String::from("Player 1"))
    } else {
        None
    }
}

// Using Option with pattern matching
let player_name = find_player_by_id(1);
match player_name {
    Some(name) => println!("Found player: {}", name),
    None => println!("Player not found"),
}

// Using if let for simpler Option handling
if let Some(name) = find_player_by_id(2) {
    println!("Found player: {}", name);
} else {
    println!("Player not found");
}

// Unwrapping Option (only use when you're sure it's Some)
let name = find_player_by_id(1).unwrap();  // Returns value or panics if None
println!("Player: {}", name);

// Providing a default with unwrap_or
let name = find_player_by_id(3).unwrap_or(String::from("Unknown Player"));
println!("Player: {}", name);  // "Unknown Player"
```

### Result Type

```rust
// Result represents success (Ok) or failure (Err)
// Result<T, E> can be either Ok(T) or Err(E)

// Function that might fail
fn divide(a: f32, b: f32) -> Result<f32, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Using Result with pattern matching
let result = divide(10.0, 2.0);
match result {
    Ok(value) => println!("Result: {}", value),
    Err(error) => println!("Error: {}", error),
}

// Using if let for simpler Result handling
if let Ok(value) = divide(10.0, 0.0) {
    println!("Result: {}", value);
} else {
    println!("Division failed");
}

// Error propagation with the ? operator
fn calculate_average(values: Vec<f32>) -> Result<f32, String> {
    let sum: f32 = values.iter().sum();
    let count = values.len() as f32;
    
    // The ? operator returns the error if divide returns Err
    // or unwraps the Ok value if successful
    let average = divide(sum, count)?;
    
    Ok(average)
}
```

## 3. Ownership and Borrowing

### Ownership Rules

```rust
// Each value has one owner
// When the owner goes out of scope, the value is dropped

// String ownership example
let s1 = String::from("hello");
let s2 = s1;  // s1 is moved to s2; s1 is no longer valid

// This would cause an error: 
// println!("s1: {}", s1);  // Use of moved value

// Cloning to avoid moves
let s1 = String::from("hello");
let s2 = s1.clone();  // Deep copy, s1 and s2 are distinct
println!("s1: {}, s2: {}", s1, s2);  // Both are valid

// Primitive types are copied by default
let x = 5;
let y = x;  // x is copied to y, both are valid
println!("x: {}, y: {}", x, y);  // Both print successfully
```

### Borrowing

```rust
// Borrowing using references
let s1 = String::from("hello");

// Immutable borrowing (multiple allowed)
let len = calculate_length(&s1);
println!("The length of '{}' is {}.", s1, len);

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Mutable borrowing (only one allowed at a time)
let mut s = String::from("hello");
change(&mut s);
println!("Changed string: {}", s);  // "hello, world"

fn change(s: &mut String) {
    s.push_str(", world");
}

// Mutable borrow restrictions
let mut s = String::from("hello");

// This code would fail:
// let r1 = &mut s;
// let r2 = &mut s;  // Cannot have multiple mutable borrows
// println!("{}, {}", r1, r2);

// Multiple immutable borrows are fine
let r1 = &s;
let r2 = &s;
println!("{}, {}", r1, r2);  // Both work fine

// Cannot mix mutable and immutable borrows in same scope
// let r1 = &s;
// let r2 = &mut s;  // Error: cannot borrow as mutable if already borrowed as immutable
// println!("{}, {}", r1, r2);
```

### Slices

```rust
// String slices
let s = String::from("hello world");
let hello = &s[0..5];  // slice of s from index 0 to 4
let world = &s[6..11];  // slice of s from index 6 to 10
println!("{} {}", hello, world);  // "hello world"

// Slice function example
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]  // Return the whole string if no space is found
}

let s = String::from("hello world");
let word = first_word(&s);
println!("First word: {}", word);  // "hello"

// Array slices
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
println!("Slice: {:?}", slice);  // [2, 3]
```

## 4. Multi-Screen Management

### Creating a Screen Manager

Macroquad applications with multiple screens can be organized using a modular approach where each screen is in its own file. This keeps code organized and maintainable.

#### Main Screen Manager (main.rs)

```rust
mod modules;

mod screen1;  // Import screen modules
mod screen2;
use macroquad::prelude::*;

#[macroquad::main("Screen Manager")]
async fn main() {
    let mut current_screen = "screen1".to_string();  // Start with screen1
    let mut last_switch = get_time() - 0.02;  // Small delay for initialization

    loop {
        if get_time() - last_switch > 0.01 {  // 10ms cooldown between screen switches
            current_screen = match current_screen.as_str() {
                "screen1" => screen1::run().await,
                "screen2" => screen2::run().await,
                _ => break,  // Exit if screen name is unknown
            };
            last_switch = get_time();  // Reset cooldown timer
        }
        next_frame().await;
    }
}
```

The main.rs file acts as a screen manager that:
1. Defines which screen is currently active
2. Calls the appropriate screen's run() function
3. Gets the next screen to display from the return value
4. Implements a small cooldown to prevent rapid screen switching

#### Individual Screen Files

Each screen is contained in its own file with a `run()` function that returns the name of the next screen to show.

Example Screen 1 (screen1.rs):

```rust
use macroquad::prelude::*;

pub async fn run() -> String {
    loop {
        clear_background(BLUE);
        draw_text("Screen 1", 20.0, 40.0, 30.0, WHITE);
        
        // Handle screen-specific logic here
        
        if is_key_pressed(KeyCode::Space) {
            return "screen2".to_string();  // Switch to screen2
        }

        next_frame().await;
    }
}
```

Example Screen 2 (screen2.rs):

```rust
use macroquad::prelude::*;

pub async fn run() -> String {
    loop {
        clear_background(DARKGRAY);
        draw_text("Screen 2", 20.0, 40.0, 30.0, WHITE);
        
        // Handle screen-specific logic here
        
        if is_key_pressed(KeyCode::Space) {
            return "screen1".to_string();  // Switch back to screen1
        }

        next_frame().await;
    }
}
```

### Benefits of the Multi-Screen Approach

1. **Modularity**: Each screen's code is separated into its own file, making the codebase easier to manage.
2. **Maintainability**: Changes to one screen don't affect others, reducing the risk of bugs.
3. **Readability**: New developers can quickly understand the application flow.
4. **Extensibility**: Adding new screens is as simple as creating a new file and adding one line to the main screen manager.

### Extending the Multi-Screen System

For more complex applications, you can enhance this pattern further:

#### Passing Data Between Screens

```rust
// In screen1.rs
pub async fn run(score: i32) -> (String, i32) {
    let mut current_score = score;
    
    loop {
        // Game logic that updates current_score
        
        if is_key_pressed(KeyCode::Space) {
            return ("screen2".to_string(), current_score);  // Pass data to next screen
        }
        
        next_frame().await;
    }
}

// In main.rs
async fn main() {
    let mut current_screen = "screen1".to_string();
    let mut game_data = 0;  // Score or other data to pass between screens
    
    loop {
        (current_screen, game_data) = match current_screen.as_str() {
            "screen1" => screen1::run(game_data).await,
            "screen2" => screen2::run(game_data).await,
            _ => break,
        };
        
        next_frame().await;
    }
}
```

#### Using Enums Instead of Strings

```rust
enum Screen {
    MainMenu,
    Game,
    Settings,
    GameOver,
}

// In main.rs
async fn main() {
    let mut current_screen = Screen::MainMenu;
    
    loop {
        current_screen = match current_screen {
            Screen::MainMenu => main_menu::run().await,
            Screen::Game => game::run().await,
            Screen::Settings => settings::run().await,
            Screen::GameOver => game_over::run().await,
        };
        
        next_frame().await;
    }
}
```

#### Loading Resources for Screens

```rust
async fn main() {
    // Load shared resources once at startup
    let shared_texture = load_texture("shared.png").await.unwrap();
    
    let mut current_screen = "screen1".to_string();
    
    loop {
        current_screen = match current_screen.as_str() {
            "screen1" => screen1::run(&shared_texture).await,
            "screen2" => screen2::run(&shared_texture).await,
            _ => break,
        };
        
        next_frame().await;
    }
}
```

This multi-screen approach works well for games, interactive applications, and any project that requires different screens or states while maintaining clean code organization.

## 5. Functional Programming Features

### Map, Filter, and Collect

```rust
// Transforming a vector using map
let numbers = vec![1, 2, 3, 4, 5];
let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
println!("Squared: {:?}", squared);  // [1, 4, 9, 16, 25]

// Filtering elements
let numbers = vec![1, 2, 3, 4, 5, 6];
let even: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
println!("Even numbers: {:?}", even);  // [2, 4, 6]

// Combining map and filter
let numbers = vec![1, 2, 3, 4, 5];
let even_squared: Vec<i32> = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .collect();
println!("Even squared: {:?}", even_squared);  // [4, 16]
```

### Fold and Reduce

```rust
// Using fold to accumulate values
let numbers = vec![1, 2, 3, 4, 5];
let sum = numbers.iter().fold(0, |acc, &x| acc + x);
println!("Sum: {}", sum);  // 15

// More complex fold example (calculating product)
let numbers = vec![1, 2, 3, 4, 5];
let product = numbers.iter().fold(1, |acc, &x| acc * x);
println!("Product: {}", product);  // 120

// Using reduce (similar to fold but uses first element as initial value)
if let Some(max) = numbers.iter().reduce(|acc, item| if acc > item { acc } else { item }) {
    println!("Maximum: {}", max);  // 5
}
```

### Closures

```rust
// Simple closure
let add = |a, b| a + b;
println!("5 + 3 = {}", add(5, 3));  // 8

// Closure that captures environment
let factor = 2;
let multiply = |x| x * factor;
println!("10 * 2 = {}", multiply(10));  // 20

// Using closures with higher-order functions
let numbers = vec![1, 2, 3, 4, 5];
let custom_sum = numbers.iter()
    .filter(|&&x| x > 2)
    .map(|&x| x * 3)
    .fold(0, |sum, x| sum + x);
println!("Custom sum: {}", custom_sum);  // (3+4+5)*3 = 36
```

## 6. Implementing Traits

### Basic Trait Implementation

```rust
// Define a trait
trait Entity {
    fn new(x: f32, y: f32) -> Self;
    fn update(&mut self);
    fn draw(&self);
    fn position(&self) -> (f32, f32);
}

// Implement the trait for a struct
struct Player {
    x: f32,
    y: f32,
    speed: f32,
}

impl Entity for Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            speed: 5.0,
        }
    }
    
    fn update(&mut self) {
        // Update player position
        self.x += self.speed;
    }
    
    fn draw(&self) {
        println!("Drawing player at ({}, {})", self.x, self.y);
        // In Macroquad, you would draw a sprite or shape here
    }
    
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

// Another implementation
struct Enemy {
    x: f32,
    y: f32,
    health: i32,
}

impl Entity for Enemy {
    fn new(x: f32, y: f32) -> Self {
        Enemy {
            x,
            y,
            health: 100,
        }
    }
    
    fn update(&mut self) {
        // Update enemy behavior
        self.y += 1.0;
    }
    
    fn draw(&self) {
        println!("Drawing enemy at ({}, {})", self.x, self.y);
        // In Macroquad, you would draw a sprite or shape here
    }
    
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

// Using trait objects
fn draw_entities(entities: &[&dyn Entity]) {
    for entity in entities {
        entity.draw();
    }
}
```

### Default Trait Implementation

```rust
// Trait with default implementation
trait Collidable {
    fn bounds(&self) -> (f32, f32, f32, f32);
    
    // Default implementation using the bounds method
    fn collides_with(&self, other: &dyn Collidable) -> bool {
        let (x1, y1, w1, h1) = self.bounds();
        let (x2, y2, w2, h2) = other.bounds();
        
        x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
    }
}

// Implementing the trait
struct GameObject {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Collidable for GameObject {
    fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
    // We don't implement collides_with, so the default is used
}

// Using the trait
let player = GameObject { x: 10.0, y: 10.0, width: 50.0, height: 50.0 };
let enemy = GameObject { x: 60.0, y: 20.0, width: 40.0, height: 30.0 };

if player.collides_with(&enemy) {
    println!("Collision detected!");
}
```

### Common Traits

```rust
// Implementing Display trait for custom formatting
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

// Now we can use print! and format! with our type
let point = Point { x: 10, y: 20 };
println!("{}", point);  // Outputs: Point(10, 20)

// Implementing Debug trait
#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

let rect = Rectangle { width: 30.0, height: 50.0 };
println!("{:?}", rect);  // Outputs: Rectangle { width: 30.0, height: 50.0 }
println!("{:#?}", rect); // Pretty print

// Implementing Clone and Copy traits
#[derive(Clone, Copy)]
struct Vector2 {
    x: f32,
    y: f32,
}

let v1 = Vector2 { x: 3.0, y: 4.0 };
let v2 = v1;  // This makes a copy, not a move
println!("v1: ({}, {}), v2: ({}, {})", v1.x, v1.y, v2.x, v2.y);
```
