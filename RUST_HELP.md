# Rust Help Guide
*Created by: Mathew Dusome*  
*Date: April 27, 2025*

This guide contains examples of common Rust operations in Rust using Macroquad:
1. Creating variables
2. Formatting text for labels
3. If statements and control flow
4. Parsing text input from TextBox component
5. Generating and using random numbers
6. Working with vectors
7. Using loops

## 1. Creating Variables

### Basic Variable Types

```rust
// Boolean
let is_player_active: bool = true;
let game_over = false;  // Type inference works too

// Integer (i32)
let score: i32 = 100;
let lives = 3;

// Float (f32)
let player_speed: f32 = 5.5;
let gravity = 9.8;

// String and &str
let message: String = String::from("Hello, World!");
let static_message: &str = "Game Over";

// Mutable variables (can be changed)
let mut health = 100;
health -= 10;  // Now health is 90
```

### Constants

```rust
// Constants are always immutable and require type annotation
const MAX_PLAYERS: i32 = 4;
const GRAVITY: f32 = 9.8;
const GAME_NAME: &str = "My Awesome Game";
```

## 2. Formatting Text for Labels

### Basic String Formatting

```rust
// Using the format! macro to create formatted strings
let player_name = "Player1";
let score = 1250;

// Format variables into a string
let score_text = format!("Player: {} - Score: {}", player_name, score);

// Using with a label component
label.set_text(&score_text);
```

### Advanced Formatting

```rust
// Formatting numbers
let health = 75.5;
let formatted_health = format!("Health: {:.1}%", health);  // Shows one decimal place

// Padding with zeros
let timer = 9;
let timer_display = format!("Time: {:02}:00", timer);  // Shows as "Time: 09:00"

// Formatting currency
let coins = 1250;
let coin_display = format!("Coins: ${}", coins);

// Multiple values
let player = "Mario";
let lives = 3;
let coins = 150;
let status_text = format!("Player: {} | Lives: {} | Coins: {}", player, lives, coins);

// Using with a label
label.set_text(&status_text);
```

## 3. If Statements and Control Flow

### Basic If Statements

```rust
// Simple if statement
let number = 5;
if number < 10 {
    println!("Number is less than 10");
}

// If-else statement
let temperature = 25;
if temperature > 30 {
    println!("It's hot outside!");
} else {
    println!("The temperature is pleasant");
}

// If-else if-else statement
let score = 85;
if score >= 90 {
    println!("Grade: A");
} else if score >= 80 {
    println!("Grade: B");
} else if score >= 70 {
    println!("Grade: C");
} else if score >= 60 {
    println!("Grade: D");
} else {
    println!("Grade: F");
}

// Using if in a let statement (ternary-like operation)
let is_evening = true;
let greeting = if is_evening { "Good evening" } else { "Good day" };
println!("{}", greeting);
```

### Complex Conditions

```rust
// If with multiple conditions using logical operators
let age = 25;
let has_license = true;

if age >= 18 && has_license {
    println!("You can drive");
}

let is_weekday = true;
let is_holiday = false;

if is_weekday && !is_holiday {
    println!("It's a working day");
}

let has_umbrella = false;
let is_raining = true;

if is_raining && !has_umbrella {
    println!("You might get wet");
}
```

### Match Expressions (Switch Statements)

```rust
let dice_roll = 4;

match dice_roll {
    1 => println!("You rolled a one!"),
    2 => println!("You rolled a two!"),
    3 => println!("You rolled a three!"),
    4..=6 => println!("You rolled between 4 and 6"),
    _ => println!("Invalid dice roll"),
}
```

## 4. Parsing Text Input

### With Whole Numbers

Example of parsing text input from a TextBox component into a i32

```rust
 let text = textbox.get_text();

        if let Ok(num) = text.trim().parse::<i32>() {
            println!("Parsed number: {}", num);
        } else {
            println!("Invalid input: must be a number");
        }

```

### With Floats

```rust
let text = textbox.get_text();

        if let Ok(num) = text.trim().parse::<f32>() {
            println!("Parsed number: {}", num);
        } else {
            println!("Invalid input: must be a number");
        } 

```

### Simple Validation

```rust
// Simple email validation example
let is_email = input.contains('@') && input.contains('.');
println!("Is email format? {}", is_email);
```

## 5. Random Numbers

### Basic Random Numbers (Using Macroquad)

```rust
 rand::srand(miniquad::/date::now() as u64);
    
    // Random float between 0.0 and 1.0
    let random_float = rand::gen_range(0.0, 1.0);
    println!("Random float between 0 and 1: {}", random_float);
    
    // Random integer in range
    let random_int = rand::gen_range(1, 101);  // 1 to 100 inclusive
    println!("Random integer between 1 and 100: {}", random_int);
    
    // Dice roll (1-6)
    let dice = rand::gen_range(1, 7);  // 1 to 6 inclusive
    println!("Dice roll: {}", dice);
```

### Random Selection from Collections

```rust
 //Add at the top
 use macroquad::rand::ChooseRandom;
    
//Then use:
    let colors = vec!["Red", "Green", "Blue", "Yellow", "Purple"];
    let random_element = my_vec.choose().unwrap();
    println!("Random color: {}", random_element);
```

## 6. Working with Vectors

### Creating and Using Vectors

```rust
// Creating an empty vector of integers
let mut scores: Vec<i32> = Vec::new();

// Creating a vector with initial values
let colors = vec!["Red", "Green", "Blue"];

// Adding elements to a vector
scores.push(100);
scores.push(85);
scores.push(90);

// Accessing elements (indexing starts at 0)
let first_score = scores[0];  // 100
let first_color = colors[0];  // "Red"

// Safe access with get() method
if let Some(color) = colors.get(1) {
    println!("Second color: {}", color);  // "Green"
}

// Vector length
let num_scores = scores.len();  // 3

// Checking if empty
let is_empty = scores.is_empty();  // false

// Removing elements
let last_score = scores.pop();  // Removes and returns 90

// Removing elements by index
let mut fruits = vec!["Apple", "Banana", "Cherry", "Date"];
fruits.remove(1);  // Removes "Banana" (index 1)
// fruits is now ["Apple", "Cherry", "Date"]

// Removing multiple elements by range
let mut numbers = vec![1, 2, 3, 4, 5, 6];
numbers.drain(2..4);  // Removes elements at indices 2 and 3 (values 3 and 4)
// numbers is now [1, 2, 5, 6]

// Removing elements that match a condition
let mut even_numbers = vec![1, 2, 3, 4, 5, 6];
even_numbers.retain(|&x| x % 2 == 0);  // Keeps only even numbers
// even_numbers is now [2, 4, 6]
```

### Vector Operations

```rust
// Iterating through all elements
for score in &scores {
    println!("Score: {}", score);
}

// Finding the sum
let total: i32 = scores.iter().sum();

// Finding the maximum
if let Some(max_score) = scores.iter().max() {
    println!("Highest score: {}", max_score);
}

// Checking if a vector contains a value
let contains_blue = colors.contains(&"Blue");  // true

// Clearing a vector
scores.clear();  // Removes all elements
```

## 7. Using Loops

### For Loops with Ranges

```rust
// Basic range loop (0 to 9)
for i in 0..10 {
    println!("Count: {}", i);
}

// Inclusive range (1 to 10)
for i in 1..=10 {
    println!("Number: {}", i);
}

// Stepping by values
for i in (0..10).step_by(2) {
    println!("Even number: {}", i);  // 0, 2, 4, 6, 8
}

// Counting down
for i in (1..=5).rev() {
    println!("Countdown: {}", i);  // 5, 4, 3, 2, 1
```

### Looping Through Collections

```rust
// Iterating through a vector
let enemies = vec!["Goomba", "Koopa", "Piranha Plant"];
for enemy in &enemies {
    println!("Enemy: {}", enemy);
}

// Iterating with index
for (index, enemy) in enemies.iter().enumerate() {
    println!("Enemy {} is {}", index + 1, enemy);
}

// Iterating through a mutable reference
let mut health_values = vec![100, 90, 80];
for health in &mut health_values {
    *health -= 10;  // Decrease each health value by 10
}
```

### While Loops

```rust
// Basic while loop
let mut countdown = 5;
while countdown > 0 {
    println!("Countdown: {}", countdown);
    countdown -= 1;
}

// Loop with break
let mut attempts = 0;
loop {
    attempts += 1;
    println!("Attempt {}", attempts);
    
    if attempts >= 3 {
        println!("Maximum attempts reached");
        break;
    }
}
```