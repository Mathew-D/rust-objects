# Rust Help Guide
*Created by: Mathew Dusome*  
*Date: April 26, 2025*

This guide contains examples of common Rust operations for game development using Macroquad:
1. If statements and control flow
2. Parsing text input from TextBox component
3. Generating and using random numbers

## 1. If Statements and Control Flow

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

## 2. Parsing Text Input

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

## 3. Random Numbers

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