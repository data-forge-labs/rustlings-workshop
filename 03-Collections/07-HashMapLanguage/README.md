# HashMap Language Project

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 6 tests pass**.

## Objective

This project demonstrates the practical use of Rust's HashMap data structure by creating a program that calculates and normalizes weights for programming languages based on their creation years. Through this project, you'll learn how to:

- Create and manipulate HashMaps with complex data
- Implement normalization algorithms
- Work with mutable references
- Handle date-based calculations
- Perform statistical operations on collections

## Rust Concepts Covered

1. **HashMap**: Key-value storage with String keys
2. **Mutable References**: Using values_mut for in-place updates
3. **Normalization**: Implementing statistical normalization
4. **Type Conversion**: Converting between numeric types
5. **Iterators**: Working with map iterators
6. **Error Handling**: Using unwrap_or for safe value access

## Rust Fundamentals

### 1. Understanding HashMap

HashMap is a key-value store implementation in Rust:

```rust
use std::collections::HashMap;

// Creating a HashMap
let mut map: HashMap<String, i32> = HashMap::new();
let mut map_with_capacity = HashMap::with_capacity(10);

// Inserting values
map.insert(String::from("Rust"), 2010);
map.insert(String::from("Python"), 1991);

// Accessing values
let rust_year = map.get("Rust");  // Returns Option<&i32>
let python_year = map["Python"];  // Returns &i32 (panics if key doesn't exist)

// Updating values
map.insert("Rust", 2015);  // Overwrites existing value
map.entry("Go").or_insert(2009);  // Only inserts if key doesn't exist
```

### 2. HashMap Operations

```rust
let mut languages = HashMap::new();

// Basic operations
languages.insert("Rust", 2010);
languages.insert("Python", 1991);
languages.insert("Go", 2009);

// Checking existence
if languages.contains_key("Rust") {
    println!("Rust exists!");
}

// Removing entries
languages.remove("Go");

// Iterating over entries
for (lang, year) in &languages {
    println!("{} was created in {}", lang, year);
}

// Getting all keys or values
let lang_names: Vec<&str> = languages.keys().cloned().collect();
let years: Vec<i32> = languages.values().cloned().collect();
```

### 3. HashMap Entry API

```rust
let mut map = HashMap::new();

// Using entry API for conditional insertion
map.entry("Rust").or_insert(2010);
map.entry("Rust").or_insert(2015);  // Won't change value

// Using entry API for modification
map.entry("Rust").and_modify(|year| *year = 2015);

// Complex operations with entry
map.entry("Rust")
   .and_modify(|year| *year += 1)
   .or_insert(2010);
```

### 4. HashMap Performance

```rust
// Creating with capacity
let mut map = HashMap::with_capacity(100);

// Reserving additional space
map.reserve(50);

// Shrinking to fit
map.shrink_to_fit();

// Getting capacity information
let capacity = map.capacity();
let len = map.len();
```

### 5. Type Conversion and Normalization

```rust
// Converting between types
let mut map = HashMap::new();
map.insert("Rust", 2010);

// Converting to Vec
let entries: Vec<(&str, i32)> = map.into_iter().collect();

// Normalizing values
let max_year = map.values().max().unwrap_or(&0);
let normalized: HashMap<&str, f64> = map.iter()
    .map(|(lang, year)| (lang, *year as f64 / *max_year as f64))
    .collect();
```

## Project Implementation

### 1. Dependencies and Imports

```rust
use std::collections::HashMap;
```

- `HashMap`: Key-value store implementation

### 2. Main Function Structure

```rust
fn main() {
    let mut languages = HashMap::new();
    languages.insert("Rust", 2010);
    languages.insert("Python", 1991);
    languages.insert("Go", 2009);
    // ...
}
```

- Creates a new HashMap
- Demonstrates basic insert operations

### 3. Weight Calculation

```rust
fn calculate_weights(languages: &HashMap<&str, i32>) -> HashMap<&str, f64> {
    let max_year = languages.values().max().unwrap_or(&0);
    languages.iter()
        .map(|(lang, year)| (lang, *year as f64 / *max_year as f64))
        .collect()
}
```

- Calculates normalized weights based on creation years
- Uses type conversion for floating-point calculations

### 4. Output Formatting

```rust
fn print_language_weights(weights: &HashMap<&str, f64>) {
    println!("Language Weights:");
    for (lang, weight) in weights {
        println!("{}: {:.2}", lang, weight);
    }
}
```

- Formats output with proper decimal places
- Uses iterator methods for clean code

### 5. Error Handling

```rust
fn get_language_year(languages: &HashMap<&str, i32>, lang: &str) -> Option<i32> {
    languages.get(lang).copied()
}

// Using the function
match get_language_year(&languages, "Rust") {
    Some(year) => println!("Rust was created in {}", year),
    None => println!("Language not found"),
}
```

- Handles missing keys gracefully
- Uses Option type for safe access

## Running the Project

1. Ensure you have Rust and Cargo installed
2. Navigate to the project directory
3. Run `cargo build` to compile
4. Run `cd workshop && cargo run` to execute the program

## Learning Outcomes

- Understanding HashMap operations and use cases
- Working with key-value stores
- Implementing normalization algorithms
- Managing type conversions
- Using the Entry API effectively
- Understanding HashMap performance characteristics
- Handling errors and edge cases 