# 🦀 MasterMind — Python to Rust Workshop

*A beginner‑friendly workshop that teaches Rust from scratch by building a Mastermind game.*

---

## Table of Contents

1. [Prerequisites & Setup](#1-prerequisites--setup)
2. [Adding Dependencies](#2-adding-dependencies)
3. [Concept 1: Variables, Mutability, and Basic Data Types](#3-concept-1-variables-mutability-and-basic-data-types)
4. [Concept 2: Strings – `String` vs `&str`](#4-concept-2-strings--string-vs-str)
5. [Concept 3: Ownership, Borrowing, and References](#5-concept-3-ownership-borrowing-and-references)
6. [Concept 4: Vectors – `Vec<T>`](#6-concept-4-vectors--vect)
7. [Concept 5: Structs and Methods – `struct` + `impl`](#7-concept-5-structs-and-methods--struct--impl)
8. [Concept 6: `Option<T>` and Pattern Matching](#8-concept-6-optiont-and-pattern-matching)
9. [Concept 7: Iterators and Closures](#9-concept-7-iterators-and-closures)
10. [Concept 8: Constants (`const`)](#10-concept-8-constants-const)
11. [Concept 9: Input/Output – Reading from the Console](#11-concept-9-inputoutput--reading-from-the-console)
12. [Putting It All Together: The Complete `main.rs`](#12-putting-it-all-together-the-complete-mainrs)
13. [Running the Game](#13-running-the-game)
14. [Summary of Rust Concepts Used](#14-summary-of-rust-concepts-used)

---

## 1. Prerequisites & Setup

### Installing Rust (if not already done)

Open your WSL terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, verify the tools:

```bash
rustc --version
cargo --version
```

If they're not up to date, update with:

```bash
rustup update stable
```

### Creating the Project

```bash
cargo new mastermind
cd mastermind
```

This creates a folder with:

- `Cargo.toml` – project configuration
- `src/main.rs` – main source file

---

## 2. Adding Dependencies

Rust uses `cargo` to manage external libraries (called *crates*). We need `rand` for random number generation.

Open `Cargo.toml` and add:

```toml
[dependencies]
rand = "0.8"
```

Now run `cargo build`. Cargo downloads the `rand` crate and compiles your project. From now on, you can use `rand` in your code.

> **Comparison:** In Python, `import random` gives you random functions. In Rust, you declare the dependency in `Cargo.toml` and then `use rand::...` in your code.

---

## 3. Concept 1: Variables, Mutability, and Basic Data Types

### Explanation

In Rust, a variable is declared with `let`. **By default, variables are immutable** – once assigned, you cannot change their value. To allow mutation, you must add the `mut` keyword.

Rust is statically typed, but the compiler can often *infer* the type. You can also explicitly annotate types.

#### A Note on `fn main()`

You'll see `fn main() { ... }` in many examples. `fn` defines a function, and `main` is the function that runs when you execute the program — similar to `if __name__ == "__main__"` in Python. All example code goes **inside** the curly braces. Don't worry about the syntax for now; we'll cover functions properly in a later section.

#### Example

```rust
fn main() {
    let x = 5;            // immutable, type i32 (inferred)
    // x = 6;             // ERROR: cannot assign twice to immutable variable

    let mut y = 10;       // mutable
    y = 20;               // OK

    let a: u32 = 100;     // unsigned 32-bit integer
    let b: i32 = -50;     // signed 32-bit integer
    let c: f64 = 3.14;    // 64-bit floating point
    let d: bool = true;   // boolean
    let e: char = '🦀';    // Unicode character (4 bytes)
}
```

#### Diagram

```
let x = 5;          x ───── [5]  (locked, no change allowed)
let mut y = 10;     y ───── [10] (open to change)
```

#### Python Comparison

```python
x = 5   # always rebindable
x = 6   # works fine
```

### Applying to Mastermind

Our game needs variables to track attempts, guess count, etc. We'll use:

```rust
let mut attempts_left: u32 = 20;   // mutable, because we decrement it
let guess_count: u32 = 0;         // immutable, but we'll reassign with let mut later
```

---

## 4. Concept 2: Strings – `String` vs `&str`

### Explanation

Rust has two main string types:

- **`String`** – an *owned*, growable, heap‑allocated string. You can modify it (e.g., push characters).  
- **`&str`** – a *string slice*, a reference to a sequence of UTF‑8 bytes. It can point to a part of a `String` or to a string literal (which is stored in the binary).

String literals like `"hello"` are of type `&str`.

#### Example

```rust
fn main() {
    let s1: String = String::from("hello");  // heap-allocated String
    let s2: &str = "world";                  // string literal (&str)

    // You can borrow a String as a &str
    let s3: &str = &s1;
    println!("{} {}", s1, s3);  // hello hello  (s1 is still valid)

    // You can modify a String
    let mut s4 = String::from("foo");
    s4.push_str("bar");
    println!("{}", s4);  // foobar
}
```

#### Diagram

```
s1 (String)          heap
+-------+      +-----------+
| ptr   | ---> | h e l l o |
| len 5 |      +-----------+
| cap 5 |
+-------+

s3 (&str)       points to same heap data
+-------+      
| ptr   | ---> (same as s1.ptr)
| len 5 |
+-------+
```

#### Python Comparison

Python's `str` is like Rust's `String` – it's immutable but managed for you. There is no separate slice type; slicing creates a new `str`.

### Applying to Mastermind

We'll read user input into a `String`, then pass a `&str` reference to functions that only need to look at the data. For example:

```rust
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();
let input = input.trim().to_lowercase();   // returns a String
```

When we evaluate a guess, we'll borrow it:

```rust
fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) { ... }
```

---

## 5. Concept 3: Ownership, Borrowing, and References

### Explanation

Every value in Rust has exactly one *owner*. When the owner goes out of scope, the value is dropped (memory freed). You can **move** ownership or **borrow** it via references.

- **Move** – the old owner is invalidated.
- **Borrow** – you temporarily get a reference, but the owner retains ownership.

Rules:
- You can have either one mutable reference (`&mut`) **or** any number of immutable references (`&`), but not both at the same time.
- References must always be valid (no dangling pointers).

#### Example

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;             // s1 is moved to s2; s1 can no longer be used!
    // println!("{}", s1);   // ERROR

    let s3 = String::from("world");
    let len = calculate_length(&s3);   // borrow s3 immutably
    println!("{} has length {}", s3, len); // s3 is still valid
}

fn calculate_length(s: &String) -> usize {
    s.len()   // s is a reference, we can read but not modify
}
```

#### Diagram – Move

```
s1  ── owns ──> [hello]      after let s2 = s1;
s2  ── owns ──> [hello]      s1 is invalidated
```

#### Diagram – Borrow

```
s3  ── owns ──> [world]
                ↑
calculate_length(&s3) borrows s3 immutably
```

#### Python Comparison

Python never moves ownership; everything is a reference, and the garbage collector frees memory. Rust does all this at compile time with zero runtime overhead.

### Applying to Mastermind

Our struct methods will borrow `self` (either `&self` or `&mut self`) so that the caller retains ownership. For instance:

```rust
impl SecretCode {
    fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) { ... }
    fn give_position_hint(&mut self) -> Option<(usize, u8)> { ... }
}
```

This allows us to call `evaluate_guess` without giving up the secret, and to modify the secret's hint state with `&mut self`.

---

## 6. Concept 4: Vectors – `Vec<T>`

### Explanation

`Vec<T>` is a resizable, heap‑allocated array. It's similar to Python's `list`. You can push elements, iterate, and index into it.

#### Example

```rust
fn main() {
    let mut numbers: Vec<i32> = Vec::new();  // empty vector
    numbers.push(10);
    numbers.push(20);

    // Macro for initialisation
    let names = vec!["Alice", "Bob"];       // Vec<&str>

    // Accessing elements
    println!("{:?}", numbers);              // [10, 20]
    println!("{}", numbers[0]);             // 10

    // Iterating
    for n in &numbers {
        println!("{}", n);
    }
}
```

#### Diagram

```
Vec<i32> { ptr, len, cap }
          |
          v
         [10, 20, ...]   on the heap
```

#### Python Comparison

```python
nums = [10, 20]     # dynamic list
```

### Applying to Mastermind

The secret digits will be stored in a `Vec<u8>`:

```rust
let digits: Vec<u8> = vec![1, 4, 2, 7];
```

We'll also use `Vec<bool>` to track revealed hints:

```rust
let revealed_positions: Vec<bool> = vec![false; 4];
let revealed_digits: Vec<bool> = vec![false; 10];
```

---

## 7. Concept 5: Structs and Methods – `struct` + `impl`

### Explanation

Rust groups data into **structs** (like Python classes without inheritance). The behaviour (methods) is defined in a separate `impl` block.

A struct definition lists fields with their types.

#### Example

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (constructor)
    fn new(w: u32, h: u32) -> Self {
        Self { width: w, height: h }
    }

    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method that modifies the struct
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

fn main() {
    let mut rect = Rectangle::new(10, 20);
    println!("Area: {}", rect.area());   // 200
    rect.double_size();
    println!("Area: {}", rect.area());   // 800
}
```

#### Diagram

```
struct Rectangle {        impl Rectangle {
    width: u32               fn new(...) -> Self
    height: u32              fn area(&self) -> u32
}                            fn double_size(&mut self)
                             ...
                         }
```

#### Python Comparison

```python
class Rectangle:
    def __init__(self, w, h):
        self.width = w
        self.height = h
    def area(self):
        return self.width * self.height
```

### Applying to Mastermind

We'll define two structs:

- `SecretCode` – holds the secret digits and hint state.
- `MastermindGame` – manages the game loop, input, and attempts.

```rust
struct SecretCode {
    digits: Vec<u8>,
    revealed_positions: Vec<bool>,
    revealed_digits: Vec<bool>,
}

impl SecretCode {
    fn new() -> Self { ... }
    fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) { ... }
    fn give_position_hint(&mut self) -> Option<(usize, u8)> { ... }
    // ...
}
```

---

## 8. Concept 6: `Option<T>` and Pattern Matching

### Explanation

Rust has no `null`. Instead, optional values are expressed using the `Option<T>` enum, which can be either `Some(value)` or `None`.

You handle `Option` using `match` or the concise `if let` syntax.

#### Example

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let result = divide(10.0, 2.0);

    match result {
        Some(val) => println!("Result: {}", val),
        None => println!("Cannot divide by zero"),
    }

    // Shorthand with if let
    if let Some(val) = result {
        println!("Got value: {}", val);
    }
}
```

#### Diagram

```
Option<f64>  =  Some(5.0)
               or
               None
```

#### Python Comparison

Python uses `None`. The equivalent would be:

```python
def divide(a, b):
    if b == 0:
        return None
    return a / b

result = divide(10, 2)
if result is not None:
    print(f"Got value: {result}")
```

### Applying to Mastermind

Hint functions return `Option` because they may fail (all hints already revealed).

```rust
fn give_position_hint(&mut self) -> Option<(usize, u8)> {
    if !self.can_give_position_hint() {
        return None;
    }
    // ...
    Some((chosen_index, digit))
}
```

When calling, we use `if let`:

```rust
if let Some((pos, digit)) = self.secret.give_position_hint() {
    // use pos and digit
}
```

---

## 9. Concept 7: Iterators and Closures

### Explanation

Iterators allow you to process collections in a functional style. A **closure** is an anonymous function that can capture variables from its environment.

Common iterator adapters: `.map()`, `.filter()`, `.zip()`, `.enumerate()`, `.collect()`.

#### Example

```rust
fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    // Map with closure, collect into new Vec
    let doubled: Vec<i32> = nums.iter()
        .map(|x| x * 2)         // closure: |parameter| body
        .collect();
    println!("{:?}", doubled);  // [2, 4, 6, 8, 10]

    // Filter
    let evens: Vec<&i32> = nums.iter().filter(|&&x| x % 2 == 0).collect();
    println!("{:?}", evens);    // [2, 4]

    // Zip two iterators
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    for (x, y) in a.iter().zip(b.iter()) {
        println!("{} {}", x, y); // 1 4, 2 5, 3 6
    }
}
```

#### Python Comparison

```python
nums = [1, 2, 3, 4, 5]
doubled = [x * 2 for x in nums]       # list comprehension
evens = [x for x in nums if x % 2 == 0]
```

### Applying to Mastermind

We heavily use iterators to evaluate guesses:

```rust
let green = self.digits.iter()
    .zip(guess_digits.iter())
    .filter(|(s, g)| s == g)
    .count();
```

And to find unrevealed positions:

```rust
let available: Vec<usize> = self.revealed_positions.iter()
    .enumerate()
    .filter(|(_, &revealed)| !revealed)
    .map(|(i, _)| i)
    .collect();
```

---

## 10. Concept 8: Constants (`const`)

### Explanation

Constants are values that are always immutable and must be type‑annotated. They can be declared in any scope and are inlined at compile time.

#### Example

```rust
const MAX_SCORE: u32 = 100;

fn main() {
    println!("Maximum score: {}", MAX_SCORE);
}
```

#### Python Comparison

Python uses variables in `UPPER_CASE` by convention, but they can still be changed. Rust enforces immutability.

### Applying to Mastermind

We'll define game constants:

```rust
const DEFAULT_ATTEMPTS: u32 = 20;
const HINT_POSITION_COST: u32 = 5;
const HINT_DIGIT_COST: u32 = 3;
```

---

## 11. Concept 9: Input/Output – Reading from the Console

### Explanation

To read a line from standard input, we use `std::io::stdin().read_line(&mut some_string)`. This appends the input (including newline) to the string.

For prompts without a newline, we must `flush` the output because `print!` does not automatically flush.

#### Example

```rust
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter something: ");
    io::stdout().flush().unwrap();    // ensure the prompt appears

    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();         // remove whitespace/newline
    println!("You typed: {}", input);
}
```

#### Python Comparison

```python
input("Enter something: ")   # automatically prints prompt and reads line
```

### Applying to Mastermind

We'll build a `get_user_input()` method that loops until a valid guess or `"help"` is entered.

```rust
fn get_user_input(&self) -> String {
    loop {
        print!("Enter guess (or 'help'): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input == "help" {
            return input.to_string();
        }
        // validation ...
    }
}
```

---

## 12. Putting It All Together: The Complete `main.rs`

Now we'll build the entire game file step by step, incorporating all the concepts above. Replace the content of `src/main.rs` with the following blocks, in order.

### Top‑level imports and utility function

```rust
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

/// Returns true if the given string consists of 4 unique digits.
fn has_unique_digits(s: &str) -> bool {
    let mut seen = [false; 10];        // array of 10 bools, all false
    for ch in s.chars() {
        let digit = ch.to_digit(10).unwrap() as usize;
        if seen[digit] {
            return false;
        }
        seen[digit] = true;
    }
    true
}
```

### SecretCode struct and implementation

```rust
struct SecretCode {
    digits: Vec<u8>,               // the 4 unique digits
    revealed_positions: Vec<bool>, // indices 0..3 that have been shown
    revealed_digits: Vec<bool>,    // digits 0..9 that have been revealed
}

impl SecretCode {
    /// Create a new random 4-digit code with no hints revealed.
    fn new() -> Self {
        let mut rng = thread_rng();
        let mut pool: Vec<u8> = (0..=9).collect();
        pool.shuffle(&mut rng);
        let digits = pool[..4].to_vec();

        SecretCode {
            digits,
            revealed_positions: vec![false; 4],
            revealed_digits: vec![false; 10],
        }
    }

    /// Compare a guess (string of exactly 4 digits) with the secret.
    /// Returns (green, yellow, red) counts.
    fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) {
        let guess_digits: Vec<u8> = guess
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        // green = exact matches
        let green = self.digits
            .iter()
            .zip(guess_digits.iter())
            .filter(|(s, g)| s == g)
            .count();

        // Collect unmatched digits for yellow/red counting
        let mut secret_unmatched: Vec<u8> = Vec::new();
        let mut guess_unmatched: Vec<u8> = Vec::new();
        for (s, g) in self.digits.iter().zip(guess_digits.iter()) {
            if s != g {
                secret_unmatched.push(*s);
                guess_unmatched.push(*g);
            }
        }

        let mut yellow = 0;
        for g in &guess_unmatched {
            if let Some(pos) = secret_unmatched.iter().position(|&x| x == *g) {
                yellow += 1;
                secret_unmatched.remove(pos);
            }
        }

        let red = 4 - green - yellow;
        (green, yellow, red)
    }

    fn can_give_position_hint(&self) -> bool {
        self.revealed_positions.iter().any(|&revealed| !revealed)
    }

    fn can_give_digit_hint(&self) -> bool {
        self.revealed_digits.iter().any(|&revealed| !revealed)
    }

    /// Reveal one unrevealed digit with its position. Returns Some((index, digit)) or None.
    fn give_position_hint(&mut self) -> Option<(usize, u8)> {
        if !self.can_give_position_hint() {
            return None;
        }
        let available: Vec<usize> = self.revealed_positions
            .iter()
            .enumerate()
            .filter(|(_, &revealed)| !revealed)
            .map(|(i, _)| i)
            .collect();

        let mut rng = thread_rng();
        let chosen = *available.choose(&mut rng).unwrap();
        self.revealed_positions[chosen] = true;
        Some((chosen, self.digits[chosen]))
    }

    /// Reveal one unrevealed correct digit (without position). Returns Some(digit) or None.
    fn give_digit_hint(&mut self) -> Option<u8> {
        if !self.can_give_digit_hint() {
            return None;
        }
        let available: Vec<usize> = self.digits
            .iter()
            .enumerate()
            .filter(|(_, &d)| !self.revealed_digits[d as usize])
            .map(|(i, _)| i)
            .collect();

        let mut rng = thread_rng();
        let chosen_idx = *available.choose(&mut rng).unwrap();
        let digit = self.digits[chosen_idx];
        self.revealed_digits[digit as usize] = true;
        Some(digit)
    }
}
```

### MastermindGame struct and constants

```rust
const DEFAULT_ATTEMPTS: u32 = 20;
const HINT_POSITION_COST: u32 = 5;
const HINT_DIGIT_COST: u32 = 3;

struct MastermindGame {
    secret: SecretCode,
    attempts_left: u32,
    guess_count: u32,
}

impl MastermindGame {
    fn new(max_attempts: u32) -> Self {
        MastermindGame {
            secret: SecretCode::new(),
            attempts_left: max_attempts,
            guess_count: 0,
        }
    }

    fn play(&mut self) {
        self.display_welcome();

        while self.attempts_left > 0 {
            println!("\nAttempts left: {}", self.attempts_left);
            let input = self.get_user_input();

            if input == "help" {
                self.handle_hint();
                continue;
            }

            self.guess_count += 1;
            let (green, yellow, red) = self.secret.evaluate_guess(&input);
            self.display_feedback(green, yellow, red);

            if green == 4 {
                println!(
                    "\n🎉 Congratulations! You cracked the code in {} actual guesses.",
                    self.guess_count
                );
                return;
            }

            self.attempts_left -= 1;
        }

        let secret_str: String = self.secret.digits.iter().map(|d| d.to_string()).collect();
        println!("\n❌ Game Over! The secret code was {}.", secret_str);
    }

    fn display_welcome(&self) {
        println!("{}", "=".repeat(40));
        println!("   Welcome to Mastermind!");
        println!("   Guess the 4-digit code (digits 0-9, no repeats)");
        println!("   You have {} attempts. Type 'help' for hints.", self.attempts_left);
        println!("{}", "=".repeat(40));
    }

    fn display_feedback(&self, green: usize, yellow: usize, red: usize) {
        println!("🟢: {}   🟡: {}   🔴: {}", green, yellow, red);
    }

    fn get_user_input(&self) -> String {
        loop {
            print!("Enter guess (or 'help'): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if input == "help" {
                return input.to_string();
            }

            if input.len() != 4 || !input.chars().all(|c| c.is_ascii_digit()) {
                println!("Invalid input. Please enter exactly 4 digits (e.g., 1234).");
                continue;
            }
            if !has_unique_digits(&input) {
                println!("Digits must be unique (no repeats). Try again.");
                continue;
            }
            return input.to_string();
        }
    }

    fn handle_hint(&mut self) {
        if self.attempts_left == 0 {
            println!("You don't have enough attempts to use a hint.");
            return;
        }

        let pos_available = self.secret.can_give_position_hint();
        let dig_available = self.secret.can_give_digit_hint();

        if !pos_available && !dig_available {
            println!("All hints already revealed. No more help available.");
            return;
        }

        println!("\n--- Hint Menu ---");
        let mut menu_options = Vec::new();
        if pos_available {
            println!("1. Reveal one digit and its correct position (costs {} attempts)", HINT_POSITION_COST);
            menu_options.push('1');
        } else {
            println!("1. (No more position hints available)");
        }
        if dig_available {
            println!("2. Reveal a correct digit (costs {} attempts)", HINT_DIGIT_COST);
            menu_options.push('2');
        } else {
            println!("2. (No more digit hints available)");
        }

        print!("Choose 1 or 2 (or press Enter to cancel): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        if choice.is_empty() || !menu_options.contains(&choice.chars().next().unwrap_or(' ')) {
            println!("Hint cancelled.");
            return;
        }

        let choice_char = choice.chars().next().unwrap();
        if choice_char == '1' {
            if self.attempts_left < HINT_POSITION_COST {
                println!("Not enough attempts. You need {} but have {}.", HINT_POSITION_COST, self.attempts_left);
                return;
            }
            if let Some((pos, digit)) = self.secret.give_position_hint() {
                self.attempts_left -= HINT_POSITION_COST;
                println!("Hint: Digit {} is at position {}.", digit, pos + 1);
                println!("({} attempts deducted)", HINT_POSITION_COST);
            }
        } else {
            if self.attempts_left < HINT_DIGIT_COST {
                println!("Not enough attempts. You need {} but have {}.", HINT_DIGIT_COST, self.attempts_left);
                return;
            }
            if let Some(digit) = self.secret.give_digit_hint() {
                self.attempts_left -= HINT_DIGIT_COST;
                println!("Hint: The code contains the digit {}.", digit);
                println!("({} attempts deducted)", HINT_DIGIT_COST);
            }
        }

        if self.attempts_left == 0 {
            println!("\nYou've used up your last attempts on a hint.");
        }
    }
}
```

### Main entry point

```rust
fn main() {
    let mut game = MastermindGame::new(DEFAULT_ATTEMPTS);
    game.play();
}
```

---

## 13. Running the Game

Inside the `mastermind` directory, simply run:

```bash
cargo run
```

You'll see the welcome message and can play exactly as in the Python version.

---

## 14. Summary of Rust Concepts Used

| Concept | Where Used |
|---------|------------|
| Variables & mutability (`let`, `let mut`) | `attempts_left`, `guess_count`, `input` |
| Data types (`u32`, `u8`, `bool`, `usize`) | struct fields, counters, array indices |
| Strings (`String`, `&str`) | user input, function parameters |
| Ownership & borrowing (`&self`, `&mut self`, `&str`) | method signatures, passing references |
| Vectors (`Vec`) | `digits`, `revealed_positions`, `revealed_digits` |
| Structs & methods (`struct`, `impl`) | `SecretCode`, `MastermindGame` |
| `Option<T>` and `if let` | hint functions, result handling |
| Iterators & closures (`.map()`, `.filter()`, `.zip()`, etc.) | evaluating guesses, finding unrevealed hints |
| Constants (`const`) | `DEFAULT_ATTEMPTS`, hint costs |
| I/O (`stdin`, `stdout`, `flush`) | reading guesses, printing prompts |

---

You've now built a complete, working Rust application while absorbing the language's core ideas. Happy coding! 🦀