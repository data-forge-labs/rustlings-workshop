# CLI Salad Project

## Rust Fundamentals

### 1. Command Line Interface (CLI) Basics
```rust
use std::env;

// Basic argument handling
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Program name: {}", args[0]);
    println!("Arguments: {:?}", &args[1..]);
}

// Pattern matching for arguments
match args.len() {
    1 => println!("No arguments provided"),
    2 => println!("One argument: {}", args[1]),
    _ => println!("Multiple arguments: {:?}", &args[1..]),
}
```

### 2. Using the Clap Library
Clap is a powerful CLI argument parser:
```rust
use clap::{Arg, Command};

// Basic command setup
let matches = Command::new("fruit-salad")
    .version("1.0")
    .author("Your Name")
    .about("Creates a random fruit salad")
    .arg(Arg::new("count")
        .short('c')
        .long("count")
        .value_name("NUMBER")
        .help("Number of fruits to include")
        .takes_value(true))
    .get_matches();

// Accessing arguments
let count = matches.value_of("count")
    .unwrap_or("5")
    .parse::<usize>()
    .unwrap_or(5);
```

### 3. User Input Handling
```rust
use std::io;

// Reading a single line
fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

// Reading numbers
fn read_number() -> Result<usize, std::num::ParseIntError> {
    let input = read_line();
    input.parse::<usize>()
}

// Error handling with match
match read_number() {
    Ok(n) => println!("You entered: {}", n),
    Err(e) => println!("Error: {}", e),
}
```

### 4. Collections and Randomization
```rust
use rand::seq::SliceRandom;
use rand::thread_rng;

// Working with vectors
let mut fruits = vec!["Apple", "Banana", "Orange"];
fruits.push("Grape");
fruits.sort();

// Random selection
let mut rng = thread_rng();
let random_fruit = fruits.choose(&mut rng).unwrap();

// Shuffling
fruits.shuffle(&mut rng);
```

### 5. Error Handling in CLI Applications
```rust
use std::error::Error;

// Custom error type
#[derive(Debug)]
enum SaladError {
    InvalidCount(usize),
    InvalidFruit(String),
}

// Error handling with Result
fn create_salad(count: usize) -> Result<Vec<String>, SaladError> {
    if count == 0 {
        return Err(SaladError::InvalidCount(count));
    }
    // ... salad creation logic
    Ok(vec!["Apple".to_string(), "Banana".to_string()])
}

// Using the function
match create_salad(0) {
    Ok(salad) => println!("Created salad: {:?}", salad),
    Err(SaladError::InvalidCount(n)) => println!("Invalid count: {}", n),
    Err(SaladError::InvalidFruit(f)) => println!("Invalid fruit: {}", f),
}
```

## Project Implementation

### 1. Dependencies and Imports
```rust
use clap::{Arg, Command};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
```
- `clap`: Command line argument parsing
- `rand`: Random number generation
- `std::io`: Input/output operations

### 2. Command Setup
```rust
fn setup_cli() -> Command<'static> {
    Command::new("fruit-salad")
        .version("1.0")
        .about("Creates a random fruit salad")
        .arg(Arg::new("count")
            .short('c')
            .long("count")
            .help("Number of fruits to include"))
        .arg(Arg::new("interactive")
            .short('i')
            .long("interactive")
            .help("Select fruits interactively"))
}
```
- Defines CLI interface
- Sets up command-line arguments
- Configures help messages

### 3. Fruit Selection Logic
```rust
fn select_fruits(count: usize, interactive: bool) -> Vec<String> {
    let mut fruits = vec!["Apple", "Banana", "Orange", "Grape"];
    if interactive {
        select_fruits_interactive(&fruits, count)
    } else {
        select_fruits_random(&fruits, count)
    }
}
```
- Handles both interactive and random selection
- Uses pattern matching for different modes

### 4. Interactive Mode
```rust
fn select_fruits_interactive(fruits: &[&str], count: usize) -> Vec<String> {
    let mut selected = Vec::new();
    println!("Available fruits: {:?}", fruits);
    
    while selected.len() < count {
        println!("Select a fruit (1-{}):", fruits.len());
        if let Ok(choice) = read_number() {
            if choice > 0 && choice <= fruits.len() {
                selected.push(fruits[choice - 1].to_string());
            }
        }
    }
    selected
}
```
- Implements interactive fruit selection
- Handles user input validation

### 5. Random Mode
```rust
fn select_fruits_random(fruits: &[&str], count: usize) -> Vec<String> {
    let mut rng = thread_rng();
    let mut selected: Vec<_> = fruits.choose_multiple(&mut rng, count)
        .cloned()
        .collect();
    selected.sort();
    selected
}
```
- Implements random fruit selection
- Uses iterator methods for clean code

## Running the Project
1. Ensure you have Rust and Cargo installed
2. Navigate to the project directory
3. Run `cargo build` to compile
4. Run `cargo run` to execute the program
5. Use `--help` to see available options

## Learning Outcomes
- Understanding CLI application development
- Working with command-line arguments
- Implementing interactive user input
- Managing collections and randomization
- Handling errors in CLI applications
- Creating user-friendly interfaces
- Using external crates effectively 