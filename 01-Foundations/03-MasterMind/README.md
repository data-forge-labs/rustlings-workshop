# Rust for Python Data Engineers â€” MasterMind

*A hands-on workshop that teaches Strings, Vectors, Structs, Option, Iterators, and I/O by building a MasterMind code-breaking game.*

> **Test-driven approach**: This project includes two Cargo projects with progressive unit tests. The **basic** workshop (`workshop/`) implements the core game; the **advanced** workshop (`workshop/advanced/`) adds modules, CLI args with `clap`, and documentation. Each function in `src/lib.rs` starts as a `todo!()` stub. Run `cd workshop && cargo test` (basic) or `cd workshop/advanced && cargo test` (advanced) to watch the pass count grow. Your goal: **all 30 tests pass (basic) and all tests pass (advanced)**.

---

## Why Build a Code-Breaking Game?

**Python pain:** A pipeline like `record["value"] * 2` is just a dictionary lookup disguised as a typed operation — typos, missing fields, and `None` propagating through five layers of code all crash at runtime, far from the cause. There is no way to ask the type system "what fields does this record have?".

**Rust fix:** `struct Guess { value: String }` validates shape at compile time; `Option<T>` forces every `None` to be handled; `match` exhausts every variant before the code compiles. The same code expresses the domain directly:

```rust
match record.label {
    Some(_) => record.value * 2.0,
    None    => record.value,  // compiler forces this branch
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | `String` vs `&str` | `String`, `&str` | `str` | Owned (heap, growable) vs borrowed (fixed) — one `str` in Python |
| 2 | `Vec<T>` | `Vec<T>` | `list` | Dynamic typed array — `Vec` is *typed*, no mixed types |
| 3 | `struct` | `struct Name { fields }` | `@dataclass` / `class` | Custom data types validated at compile time |
| 4 | `impl` blocks | `impl Guess { fn new(...) -> Self { ... } }` | methods inside `class` | Rust separates *data* (struct) from *behavior* (impl) |
| 5 | `Option<T>` | `Some(val)` / `None` | `None` / `Optional[T]` | Nullable values the compiler forces you to check |
| 6 | `match` | `match val { A => ..., B => ... }` | `if/elif/else` | Exhaustive — compiler verifies every variant is handled |
| 7 | Ownership basics | `&self`, function params | N/A (GC) | Memory safety *without* garbage collection |
| 8 | Iterators | `.chars()`, `.enumerate()`, `.filter()` | `for ch in s`, `enumerate(s)` | Lazy, composable functional iteration |
| 9 | Console I/O | `io::stdin().read_line(&mut buf)`, `println!` | `input()`, `print()` | Read user input and print output |
| 10 | `rand` crate | `rand::rng().random_range(10..=99)` | `random.randint(10, 99)` | Random number generation (add `rand = "0.10"`) |
| 11 | `pub` visibility | `pub fn`, `pub struct` | Public by default | Items are private unless explicitly exported |
| 12 | `Self` constructor | `fn new(...) -> Self` | `__init__(self)` | Idiomatic constructor pattern; `Self` abbreviates the type |

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Prerequisites](#2-prerequisites)
3. [How to Use This Workshop](#3-how-to-use-this-workshop)
4. [Python vs Rust Concepts in This Project](#4-python-vs-rust-concepts-in-this-project)
5. [Basic Workshop (workshop/)](#5-basic-workshop-workshop)
6. [Advanced Workshop (workshop/advanced/)](#6-advanced-workshop-workshopadvanced)
7. [Detailed Step-by-Step Guide (Basic)](#7-detailed-step-by-step-guide-basic)
8. [Advanced Exercise Guide](#8-advanced-exercise-guide)
9. [Summary](#9-summary)

---

## 1. Project Overview

MasterMind is a classic code-breaking game:
- The computer generates a secret 4-digit code
- The player guesses the code
- After each guess, the computer gives feedback: correct digits in the right position (A), and correct digits in the wrong position (B)
- The player wins by guessing the code in as few tries as possible

### Rust Concepts Covered

| Concept | Python Equivalent | Why It Matters for Data Engineering |
|---|---|---|
| `String` vs `&str` | `str` | Text processing in data pipelines |
| Ownership & Borrowing | N/A (GC handles this) | Memory safety without GC pauses |
| `Vec<T>` | `list` | Dynamic collections for data |
| `struct` + `impl` | `class` | Organizing data and behavior |
| `Option<T>` | `None` / `Optional` | Handling missing data |
| Pattern matching (`match`) | `if`/`elif`/`else` | Clean branching logic |
| Iterators & Closures | `for` loops, `map`/`filter` | Functional data processing |
| Console I/O | `input()` / `print()` | CLI tools for data engineering |

---

## 2. Prerequisites

- Completed [Basic Calculator](../01-Foundations/02-BasicCalculator/README.md)
- Rust installed and working
- Basic familiarity with `cd workshop && cargo run`

---

## 3. How to Use This Workshop

This project has two separate workshops:

### Basic â€” `workshop/`

Build the core MasterMind game with structs, Vec, Option, and iterators. Start here.

1. **Read the concept overview** in Section 4 below â€” maps each Rust concept to Python
2. **Follow the detailed guide** in [Section 7](#7-detailed-step-by-step-guide-basic) for the full step-by-step implementation
3. **Build the game** with `cd workshop && cargo run`

### Advanced â€” `workshop/advanced/`

Refactor the game into a library + binary crate with `clap` CLI args and documentation. Complete the basic version first.

1. **Read** [Section 8](#8-advanced-exercise-guide) for module organization, `clap`, and doc concepts
2. **Browse the stub files** in `workshop/advanced/src/` (lib.rs, main.rs, secret.rs, game.rs)
3. **Build** with `cd workshop/advanced && cargo run -- --max-attempts 15`

---

## 4. Python vs Rust Concepts in This Project

### Strings: `String` vs `&str`

```python
# Python â€” one string type
name = "Alice"
name += " Smith"   # Creates a new string
```

```rust
// Rust â€” two string types
let literal: &str = "Alice";       // Immutable, fixed, efficient
let mut owned: String = String::from("Alice");  // Heap-allocated, growable
owned.push_str(" Smith");
```

| Characteristic | `&str` (string slice) | `String` (owned string) |
|---|---|---|
| Mutability | Immutable | Mutable |
| Where stored | Read-only memory or borrowed | Heap |
| Use case | Read-only access, parameters | Building, modifying text |
| Python analog | `str` (immutable) | No direct equivalent |

### Vectors: `Vec<T>`

```python
# Python â€” dynamic list
fruits = ["apple", "banana"]
fruits.append("cherry")
```

```rust
// Rust â€” typed vector
let mut fruits: Vec<&str> = vec!["apple", "banana"];
fruits.push("cherry");
```

| Operation | Python `list` | Rust `Vec<T>` |
|---|---|---|
| Create | `items = []` | `let items: Vec<T> = Vec::new();` |
| With values | `items = [1, 2, 3]` | `let items = vec![1, 2, 3];` |
| Add | `items.append(x)` | `items.push(x);` |
| Remove last | `items.pop()` | `items.pop();` |
| Length | `len(items)` | `items.len()` |
| Access | `items[0]` | `items[0]` (panics if out of bounds) |

### Structs and Methods: `struct` + `impl`

```python
# Python class
class Guess:
    def __init__(self, value: str):
        self.value = value

    def is_valid(self) -> bool:
        return len(self.value) == 4
```

```rust
// Rust struct + impl
struct Guess {
    value: String,
}

impl Guess {
    fn new(value: String) -> Self {
        Self { value }
    }

    fn is_valid(&self) -> bool {
        self.value.len() == 4
    }
}
```

| Aspect | Python `class` | Rust `struct` + `impl` |
|---|---|---|
| Data fields | `self.field` in `__init__` | Fields in `struct` definition |
| Methods | All in class body | Separate `impl` block |
| Constructor | `__init__` | `fn new(...) -> Self` |
| Visibility | Public by default | Private by default (`pub` to expose) |

### Option and Pattern Matching

```python
# Python â€” None handling
def find_item(items, target):
    for item in items:
        if item == target:
            return item
    return None

result = find_item(data, "x")
if result is not None:
    print(result)
```

```rust
// Rust â€” Option + match
fn find_item(items: &[&str], target: &str) -> Option<&str> {
    for &item in items {
        if item == target {
            return Some(item);
        }
    }
    None
}

match find_item(&data, "x") {
    Some(item) => println!("Found: {}", item),
    None => println!("Not found"),
}
```

| Python | Rust |
|---|---|
| `None` | `Option::None` |
| `if x is not None` | `if let Some(x) = value` |
| `x = func() or default` | `func().unwrap_or(default)` |
| `x = func()` (might be None) | `func()` returns `Option<T>` |

---

## 5. Basic Workshop (workshop/)

The `workshop/` directory contains the basic MasterMind game. Follow the detailed guide in [Section 7](#7-detailed-step-by-step-guide-basic). Key sections:

1. **Setup:** Create project, add dependencies (`rand` crate)
2. **Variables & Types:** Declare game constants and state
3. **Strings:** Handle player input with `String` and `&str`
4. **Ownership & Borrowing:** Pass data between functions
5. **Vectors:** Store the secret code and guess history
6. **Structs:** Model the `Guess` and `Game` types
7. **Option:** Handle cases where data might not exist
8. **Iterators:** Process guesses functionally
9. **I/O:** Read guesses, print feedback

---

## 6. Summary

| Concept | How Used in MasterMind |
|---|---|
| `String` / `&str` | Player input strings, string slicing for digits |
| Ownership | Function parameters â€” know when to move vs borrow |
| `Vec<char>` | Store the 4-digit code as vector of characters |
| `struct Guess` | Model a single guess with validation |
| `struct Game` | Model the game state (secret, attempts) |
| `impl` | Methods on `Guess` and `Game` |
| `Option<&str>` | Parse input, may fail |
| `match` | Branch on `Option` variants |
| Iterators | `chars()`, `enumerate()` for digit comparison |
| Console I/O | `io::stdin().read_line()`, `println!` |
| `rand` crate | Generate random secret code |

**Advanced workshop extra concepts:**
| Concept | How Used |
|---------|----------|
| `mod` / `pub` | Split code into `secret.rs`, `game.rs`, `lib.rs` modules |
| `clap::Parser` | Parse `--max-attempts` CLI argument |
| `///` docs | Document structs and methods, `cargo doc --open` |
| `#[cfg(test)]` | Unit tests alongside implementation code |

### Further Reading

| Section | Workshop | Topics |
|---------|----------|--------|
| [Section 7](#7-detailed-step-by-step-guide-basic) | Basic (`workshop/`) | Full step-by-step guide: structs, Vec, Option, iterators, I/O |
| [Section 8](#8-advanced-exercise-guide) | Advanced (`workshop/advanced/`) | Module organization, `clap` CLI args, documentation, unit tests |

### Next Project

Proceed to [3-TicketV1](../02-Ownership/01-TicketV1/README.md) to master **ownership** â€” Rust's most important and unique concept.

---

## 7. Detailed Step-by-Step Guide (Basic)

Build the core MasterMind game. Work in the `workshop/` directory.

### Table of Contents

1. [Prerequisites & Setup](#1-prerequisites--setup)
2. [Adding Dependencies](#2-adding-dependencies)
3. [Concept Recap — Where to Learn Each Concept](#3-concept-recap--where-to-learn-each-concept)
4. [Putting It All Together: The Complete `main.rs`](#4-putting-it-all-together-the-complete-mainrs)
5. [Running the Game](#5-running-the-game)
6. [Summary of Rust Concepts Used](#6-summary-of-rust-concepts-used)

### 1. Prerequisites & Setup

#### Installing Rust (if not already done)

Open your WSL terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, verify the tools:

```bash
rustc --version
cargo --version
```

#### Creating the Project

```bash
cargo new mastermind
cd mastermind
```

This creates a folder with:

- `Cargo.toml` â€“ project configuration
- `src/main.rs` â€“ main source file

### 2. Adding Dependencies

Rust uses `cargo` to manage external libraries (called *crates*). We need `rand` for random number generation.

Open `Cargo.toml` and add:

```toml
[dependencies]
rand = "0.10"
```

Now run `cargo build`. Cargo downloads the `rand` crate and compiles your project.

> **Comparison:** In Python, `import random` gives you random functions. In Rust, you declare the dependency in `Cargo.toml` and then `use rand::...` in your code.

### 3. Concept Recap — Where to Learn Each Concept

The 8 concepts below are covered in detail in earlier projects. The full teaching with Python comparisons, ASCII diagrams, and exercises lives there. Below is **only the MasterMind-specific application** of each.

| # | Concept | Canonical Source | Used in MasterMind as |
|---|---------|------------------|----------------------|
| 1 | Variables, mutability, basic types | [01-Intro §6](../01-Intro/README.md#6-variables-and-mutability) | `let mut attempts_left: u32 = 20;` |
| 2 | `String` vs `&str` | [01-Intro §4](../01-Intro/README.md#4-syntax-side-by-side) (mention) — full teaching in this project §4 below | `let mut input = String::new();` and `fn evaluate_guess(&self, guess: &str) -> ...` |
| 3 | Ownership, borrowing, references | [02-Ownership/01-TicketV1](../02-Ownership/01-TicketV1/README.md) | `&self` for read-only methods, `&mut self` for hint methods |
| 4 | `Vec<T>` | [03-Collections/01-TicketManagement](../03-Collections/01-TicketManagement/README.md) | `let digits: Vec<u8> = vec![1, 4, 2, 7];` |
| 5 | `struct` + `impl` | This project §4 (Python vs Rust Concepts) above | `struct SecretCode { ... } impl SecretCode { ... }` |
| 6 | `Option<T>`, `if let`, `match` | This project §4 above | `fn give_position_hint(&mut self) -> Option<(usize, u8)>` |
| 7 | Iterators, closures | This project §4 above | `.iter().zip().filter().count()` chains for guess evaluation |
| 8 | `const` | [01-Intro §6](../01-Intro/README.md#6-variables-and-mutability) (Constants) | `const DEFAULT_ATTEMPTS: u32 = 20;` |

**If you haven't completed the Intro and BasicCalculator projects, do that first** — this project assumes you know how to declare variables, write functions, read input, and match on `Option`. If you have, just skim the recap table above and skip to the next section.

### 4. Putting It All Together: The Complete `main.rs`

Now build the entire game file step by step. Replace the content of `src/main.rs` with the following blocks.

#### Top-level imports and utility function

```rust
use rand::seq::SliceRandom;
use rand::rng;
use std::io::{self, Write};

/// Returns true if the given string consists of 4 unique digits.
fn has_unique_digits(s: &str) -> bool {
    let mut seen = [false; 10];
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

#### SecretCode struct and implementation

```rust
struct SecretCode {
    digits: Vec<u8>,
    revealed_positions: Vec<bool>,
    revealed_digits: Vec<bool>,
}

impl SecretCode {
    fn new() -> Self {
        let mut rng = rng();
        let mut pool: Vec<u8> = (0..=9).collect();
        pool.shuffle(&mut rng);
        let digits = pool[..4].to_vec();

        SecretCode {
            digits,
            revealed_positions: vec![false; 4],
            revealed_digits: vec![false; 10],
        }
    }

    fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) {
        let guess_digits: Vec<u8> = guess
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let green = self.digits
            .iter()
            .zip(guess_digits.iter())
            .filter(|(s, g)| s == g)
            .count();

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

        let mut rng = rng();
        let chosen = *available.choose(&mut rng).unwrap();
        self.revealed_positions[chosen] = true;
        Some((chosen, self.digits[chosen]))
    }

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

        let mut rng = rng();
        let chosen_idx = *available.choose(&mut rng).unwrap();
        let digit = self.digits[chosen_idx];
        self.revealed_digits[digit as usize] = true;
        Some(digit)
    }
}
```

#### MastermindGame struct and constants

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
                    "\nCongratulations! You cracked the code in {} actual guesses.",
                    self.guess_count
                );
                return;
            }

            self.attempts_left -= 1;
        }

        let secret_str: String = self.secret.digits.iter().map(|d| d.to_string()).collect();
        println!("\nGame Over! The secret code was {}.", secret_str);
    }

    fn display_welcome(&self) {
        println!("{}", "=".repeat(40));
        println!("   Welcome to Mastermind!");
        println!("   Guess the 4-digit code (digits 0-9, no repeats)");
        println!("   You have {} attempts. Type 'help' for hints.", self.attempts_left);
        println!("{}", "=".repeat(40));
    }

    fn display_feedback(&self, green: usize, yellow: usize, red: usize) {
        println!("Green: {}   Yellow: {}   Red: {}", green, yellow, red);
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

#### Main entry point

```rust
fn main() {
    let mut game = MastermindGame::new(DEFAULT_ATTEMPTS);
    game.play();
}
```

### 5. Running the Game

Inside the `mastermind` directory, run:

```bash
cargo run
```

### 6. Summary of Rust Concepts Used

| Concept | Where Used |
|---------|------------|
| Variables & mutability (`let`, `let mut`) | `attempts_left`, `guess_count`, `input` |
| Data types (`u32`, `u8`, `bool`, `usize`) | struct fields, counters, array indices |
| Strings (`String`, `&str`) | user input, function parameters |
| Ownership & borrowing (`&self`, `&mut self`, `&str`) | method signatures, passing references |
| Vectors (`Vec`) | `digits`, `revealed_positions`, `revealed_digits` |
| Structs & methods (`struct`, `impl`) | `SecretCode`, `MastermindGame` |
| `Option<T>` and `if let` | hint functions, result handling |
| Iterators & closures | evaluating guesses, finding unrevealed hints |
| Constants (`const`) | `DEFAULT_ATTEMPTS`, hint costs |
| I/O (`stdin`, `stdout`, `flush`) | reading guesses, printing prompts |

---

## 8. Advanced Exercise Guide

Refactor the game into a library + binary crate. Work in the `workshop/advanced/` directory.

### Table of Contents

- [8. Advanced Exercise Guide](#8-advanced-exercise-guide)
  - [Table of Contents](#table-of-contents-1)
  - [1. Introduction](#1-introduction-1)
  - [2. Prerequisites](#2-prerequisites-1)
  - [3. Concept 1: Rust Packages, Crates, and Modules](#3-concept-1-rust-packages-crates-and-modules)
  - [4. Concept 2: Library vs Binary Crate](#4-concept-2-library-vs-binary-crate)
  - [5. Concept 3: Visibility (`pub`) and Re-exports](#5-concept-3-visibility-pub-and-re-exports)
  - [6. Concept 4: Documentation (`///` & `cargo doc`)](#6-concept-4-documentation--cargo-doc)
  - [7. Concept 5: Unit Tests (`#[test]` & `cargo test`)](#7-concept-5-unit-tests-test--cargo-test)
  - [8. Concept 6: Command-Line Arguments with `clap`](#8-concept-6-command-line-arguments-with-clap)
  - [9. Putting It All Together: Building the Library](#9-putting-it-all-together-building-the-library)
  - [10. Putting It All Together: Building the Binary](#10-putting-it-all-together-building-the-binary)
  - [11. Running the Game](#11-running-the-game)
  - [12. Additional Cargo Tricks](#12-additional-cargo-tricks)
  - [13. Summary of New Concepts](#13-summary-of-new-concepts)

### 1. Introduction

In the first Mastermind workshop, you wrote the whole game in a single `main.rs` file. That's fine for a small program, but real-world Rust projects are organised into **libraries** and **binaries**. Libraries can be shared, tested independently, and documented.

In this advanced workshop, you will:

- Split the code into a **library crate** (`src/lib.rs`) that contains all game logic.
- Keep the user interaction in a **binary crate** (`src/main.rs`) that uses the library.
- Add documentation and unit tests to the library.
- Use an external crate (`clap`) for command-line argument parsing.

### 2. Prerequisites

- Completed the first Mastermind workshop (or understand basic Rust: structs, methods, `Vec`, `Option`, etc.)
- A working `mastermind` Cargo project

### 3. Concept 1: Rust Packages, Crates, and Modules

Rust organises code into a hierarchy of **packages**, **crates**, and **modules**.

- A **package** is a Cargo project (the `Cargo.toml` folder). It can contain one or more **crates**.
- A **crate** is a compilation unit: a **binary crate** produces an executable, a **library crate** produces a `.rlib` file that can be used by other crates.
- **Modules** (`mod`) group related items and control privacy. The module tree is rooted in `crate`.

A typical package layout:

```
mastermind/
â”œâ”€â”€ Cargo.toml
â””â”€â”€ src/
    â”œâ”€â”€ main.rs      â† binary crate root (by default)
    â””â”€â”€ lib.rs       â† library crate root (if present)
```

```rust
// src/lib.rs
pub mod game;      // tells Rust to look for src/game.rs

// src/game.rs
pub struct Game { ... }
```

**Python Comparison:** Python uses files as modules and folders as packages (with `__init__.py`). Rust's `mod` declares a module explicitly.

### 4. Concept 2: Library vs Binary Crate

When you have both `lib.rs` and `main.rs`, Cargo builds:
- The library crate (named as the package, `mastermind`)
- A binary crate (also named `mastermind` by default)

The binary crate can use the library crate via `use mastermind::...`. The library crate **cannot** depend on the binary crate.

**Applying to Workshop:** We will move the `SecretCode` and `MastermindGame` structs into the library (`src/lib.rs`), and keep only the input/output and game loop in `main.rs`.

### 5. Concept 3: Visibility (`pub`) and Re-exports

By default, everything in Rust is **private** to its current module. To expose items to the outside world, prefix with `pub`.

Re-exports allow you to create a convenient public API surface:

```rust
// lib.rs
mod game;
pub use game::MastermindGame;   // now users can do `mastermind::MastermindGame`

// game.rs
pub struct MastermindGame { ... }
```

**Python Comparison:** In Python, all top-level identifiers are public by convention; Rust requires explicit `pub`.

### 6. Concept 4: Documentation (`///` & `cargo doc`)

Rust has built-in documentation generation. Comments starting with `///` document the item that follows, and `//!` documents the enclosing module/crate.

Run `cargo doc --open` to build and view the documentation in your browser.

```rust
/// Represents the hidden 4-digit code.
///
/// # Examples
/// ```
/// let code = SecretCode::new();
/// ```
pub struct SecretCode { ... }
```

### 7. Concept 5: Unit Tests (`#[test]` & `cargo test`)

Testing is first-class in Rust. Write tests in the same file as the code, inside a `mod tests` guarded by `#[cfg(test)]`.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_guess() {
        let code = SecretCode::from_digits(vec![1,2,3,4]);
        let (g, y, r) = code.evaluate_guess("1243");
        assert_eq!(g, 2);
        assert_eq!(y, 2);
        assert_eq!(r, 0);
    }
}
```

### 8. Concept 6: Command-Line Arguments with `clap`

Add to `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

Then use derive macros to define arguments:

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Maximum number of attempts
    #[arg(short, long, default_value_t = 20)]
    max_attempts: u32,
}
```

### 9. Putting It All Together: Building the Library

Now rewrite the Mastermind code as a library crate with proper structure.

#### Create the files

Inside `src/`, we'll have:
- `lib.rs` â€“ the library root
- `secret.rs` â€“ the `SecretCode` module
- `game.rs` â€“ the `MastermindGame` module

#### `secret.rs`

Move the `SecretCode` struct and its `impl` from the old code into `secret.rs`:

```rust
use rand::seq::SliceRandom;
use rand::rng;

pub struct SecretCode {
    digits: Vec<u8>,
    revealed_positions: Vec<bool>,
    revealed_digits: Vec<bool>,
}

impl SecretCode {
    pub fn new() -> Self {
        let mut rng = rng();
        let mut pool: Vec<u8> = (0..=9).collect();
        pool.shuffle(&mut rng);
        let digits = pool[..4].to_vec();

        SecretCode {
            digits,
            revealed_positions: vec![false; 4],
            revealed_digits: vec![false; 10],
        }
    }

    pub fn from_digits(digits: Vec<u8>) -> Self {
        SecretCode {
            digits,
            revealed_positions: vec![false; 4],
            revealed_digits: vec![false; 10],
        }
    }

    pub fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) {
        let guess_digits: Vec<u8> = guess
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let green = self.digits
            .iter()
            .zip(guess_digits.iter())
            .filter(|(s, g)| s == g)
            .count();

        let mut secret_unmatched = Vec::new();
        let mut guess_unmatched = Vec::new();
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

    pub fn can_give_position_hint(&self) -> bool {
        self.revealed_positions.iter().any(|&r| !r)
    }

    pub fn can_give_digit_hint(&self) -> bool {
        self.revealed_digits.iter().any(|&r| !r)
    }

    pub fn give_position_hint(&mut self) -> Option<(usize, u8)> {
        if !self.can_give_position_hint() {
            return None;
        }
        let available: Vec<usize> = self.revealed_positions
            .iter()
            .enumerate()
            .filter(|(_, &revealed)| !revealed)
            .map(|(i, _)| i)
            .collect();
        let mut rng = rng();
        let chosen = *available.choose(&mut rng).unwrap();
        self.revealed_positions[chosen] = true;
        Some((chosen, self.digits[chosen]))
    }

    pub fn give_digit_hint(&mut self) -> Option<u8> {
        if !self.can_give_digit_hint() {
            return None;
        }
        let available: Vec<usize> = self.digits
            .iter()
            .enumerate()
            .filter(|(_, &d)| !self.revealed_digits[d as usize])
            .map(|(i, _)| i)
            .collect();
        let mut rng = rng();
        let chosen_idx = *available.choose(&mut rng).unwrap();
        let digit = self.digits[chosen_idx];
        self.revealed_digits[digit as usize] = true;
        Some(digit)
    }

    pub fn reveal(&self) -> String {
        self.digits.iter().map(|d| d.to_string()).collect::<Vec<_>>().join("")
    }
}
```

#### `game.rs`

Move `MastermindGame` and constants. Note that input/output is left in the binary crate:

```rust
use crate::secret::SecretCode;

pub struct MastermindGame {
    secret: SecretCode,
    attempts_left: u32,
    pub guess_count: u32,
}

impl MastermindGame {
    pub fn new(max_attempts: u32) -> Self {
        MastermindGame {
            secret: SecretCode::new(),
            attempts_left: max_attempts,
            guess_count: 0,
        }
    }

    pub fn attempts_left(&self) -> u32 {
        self.attempts_left
    }

    pub fn submit_guess(&mut self, guess: &str) -> Option<(usize, usize, usize)> {
        if self.attempts_left == 0 {
            return None;
        }
        self.guess_count += 1;
        let feedback = self.secret.evaluate_guess(guess);
        if feedback.0 == 4 {
            self.attempts_left = 0;
        } else {
            self.attempts_left -= 1;
        }
        Some(feedback)
    }

    pub fn can_use_hints(&self) -> bool {
        self.attempts_left > 0
    }

    pub fn can_give_position_hint(&self) -> bool {
        self.secret.can_give_position_hint()
    }

    pub fn can_give_digit_hint(&self) -> bool {
        self.secret.can_give_digit_hint()
    }

    pub fn give_position_hint(&mut self) -> Option<(usize, u8)> {
        self.secret.give_position_hint()
    }

    pub fn give_digit_hint(&mut self) -> Option<u8> {
        self.secret.give_digit_hint()
    }

    pub fn deduct_attempts(&mut self, cost: u32) {
        self.attempts_left = self.attempts_left.saturating_sub(cost);
    }

    pub fn reveal(&self) -> String {
        self.secret.reveal()
    }
}
```

#### `lib.rs`

```rust
pub mod secret;
pub mod game;

pub use game::MastermindGame;
pub use secret::SecretCode;
```

### 10. Putting It All Together: Building the Binary

Add `clap` to `Cargo.toml`:

```toml
[dependencies]
rand = "0.10"
clap = { version = "4", features = ["derive"] }
```

Now write `main.rs`:

```rust
use clap::Parser;
use mastermind::MastermindGame;
use std::io::{self, Write};

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 20)]
    max_attempts: u32,
}

fn main() {
    let args = Args::parse();
    let mut game = MastermindGame::new(args.max_attempts);
    display_welcome(args.max_attempts);

    while game.attempts_left() > 0 {
        println!("\nAttempts left: {}", game.attempts_left());
        let input = get_user_input();

        if input == "help" {
            handle_hint(&mut game);
            continue;
        }

        if let Some((green, yellow, red)) = game.submit_guess(&input) {
            display_feedback(green, yellow, red);
            if green == 4 {
                println!(
                    "\nCongratulations! You cracked the code in {} actual guesses.",
                    game.guess_count
                );
                return;
            }
        }
    }

    println!("\nGame Over! The secret code was {}.", game.reveal());
}

fn display_welcome(max_attempts: u32) {
    println!("{}", "=".repeat(40));
    println!("   Welcome to Mastermind!");
    println!("   Guess the 4-digit code (digits 0-9, no repeats)");
    println!("   You have {} attempts. Type 'help' for hints.", max_attempts);
    println!("{}", "=".repeat(40));
}

fn display_feedback(green: usize, yellow: usize, red: usize) {
    println!("Green: {}   Yellow: {}   Red: {}", green, yellow, red);
}

fn get_user_input() -> String {
    loop {
        print!("Enter guess (or 'help'): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input == "help" {
            return input;
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

fn has_unique_digits(s: &str) -> bool {
    let mut seen = [false; 10];
    for ch in s.chars() {
        let digit = ch.to_digit(10).unwrap() as usize;
        if seen[digit] {
            return false;
        }
        seen[digit] = true;
    }
    true
}

fn handle_hint(game: &mut MastermindGame) {
    const HINT_POSITION_COST: u32 = 5;
    const HINT_DIGIT_COST: u32 = 3;

    if !game.can_use_hints() {
        println!("No attempts left to use hints.");
        return;
    }

    let pos_available = game.can_give_position_hint();
    let dig_available = game.can_give_digit_hint();

    if !pos_available && !dig_available {
        println!("All hints already revealed.");
        return;
    }

    println!("\n--- Hint Menu ---");
    if pos_available {
        println!("1. Reveal one digit and its correct position (costs {} attempts)", HINT_POSITION_COST);
    } else {
        println!("1. (No more position hints available)");
    }
    if dig_available {
        println!("2. Reveal a correct digit (costs {} attempts)", HINT_DIGIT_COST);
    } else {
        println!("2. (No more digit hints available)");
    }

    print!("Choose 1 or 2 (or press Enter to cancel): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    if choice.is_empty() {
        return;
    }

    if choice == "1" && pos_available {
        if game.attempts_left() < HINT_POSITION_COST {
            println!("Not enough attempts.");
            return;
        }
        if let Some((pos, digit)) = game.give_position_hint() {
            game.deduct_attempts(HINT_POSITION_COST);
            println!("Hint: Digit {} is at position {}.", digit, pos + 1);
        }
    } else if choice == "2" && dig_available {
        if game.attempts_left() < HINT_DIGIT_COST {
            println!("Not enough attempts.");
            return;
        }
        if let Some(digit) = game.give_digit_hint() {
            game.deduct_attempts(HINT_DIGIT_COST);
            println!("Hint: The code contains the digit {}.", digit);
        }
    } else {
        println!("Invalid choice.");
    }
}
```

### 11. Running the Game

```bash
cargo run
# Or with custom attempts:
cargo run -- --max-attempts 10
```

### 12. Additional Cargo Tricks

- **Documentation**: Run `cargo doc --open` to see your library's documentation generated from the doc comments.
- **Testing**: Add unit tests inside `secret.rs` and `game.rs` and run `cargo test`.
- **Workspaces**: If you later have multiple related crates, a workspace `[workspace]` in `Cargo.toml` ties them together.
- **Features**: You can define optional dependencies in `Cargo.toml` and gate code with `#[cfg(feature = "foo")]`.
- **Publishing**: If you wanted to share your library on crates.io, you'd run `cargo publish`.

### 13. Summary of New Concepts

| Concept | Where Used |
|---------|------------|
| Package layout (`lib.rs` + `main.rs`) | Whole project |
| Modules (`mod`, file organisation) | `secret.rs`, `game.rs`, `lib.rs` |
| `pub` visibility and re-exports | `pub struct`, `pub use` in `lib.rs` |
| Documentation (`///`, `//!`, `cargo doc`) | Above structs and functions |
| Unit tests (`#[test]`, `#[cfg(test)]`, `cargo test`) | (suggested practice) |
| `clap` for CLI argument parsing | `main.rs` with `Args` struct |
| `derive` macros (`#[derive(Parser)]`) | On `Args` |
| `saturating_sub` | In `deduct_attempts` |
| Separation of concerns (logic in lib, I/O in bin) | Moved game logic to library |
