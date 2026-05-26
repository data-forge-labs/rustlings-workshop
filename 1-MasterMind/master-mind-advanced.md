# 🦀 Advanced MasterMind — Library & Cargo Workshop

*A workshop that teaches Rust’s crate ecosystem by packaging Mastermind as a library.*

## Table of Contents

- [🦀 Advanced MasterMind — Library \& Cargo Workshop](#-advanced-mastermind--library--cargo-workshop)
  - [Table of Contents](#table-of-contents)
  - [1. Introduction](#1-introduction)
  - [2. Prerequisites](#2-prerequisites)
  - [3. Concept 1: Rust Packages, Crates, and Modules](#3-concept-1-rust-packages-crates-and-modules)
    - [Explanation](#explanation)
      - [Example](#example)
      - [Diagram](#diagram)
      - [Python Comparison](#python-comparison)
  - [4. Concept 2: Library vs Binary Crate](#4-concept-2-library-vs-binary-crate)
    - [Explanation](#explanation-1)
    - [Applying to Workshop](#applying-to-workshop)
  - [5. Concept 3: Visibility (`pub`) and Re‑exports](#5-concept-3-visibility-pub-and-reexports)
    - [Explanation](#explanation-2)
      - [Diagram](#diagram-1)
      - [Python Comparison](#python-comparison-1)
  - [6. Concept 4: Documentation (`///` \& `cargo doc`)](#6-concept-4-documentation---cargo-doc)
    - [Explanation](#explanation-3)
      - [Example](#example-1)
  - [7. Concept 5: Unit Tests (`#[test]` \& `cargo test`)](#7-concept-5-unit-tests-test--cargo-test)
    - [Explanation](#explanation-4)
      - [Example](#example-2)
  - [8. Concept 6: Command‑Line Arguments with `clap`](#8-concept-6-commandline-arguments-with-clap)
    - [Explanation](#explanation-5)
  - [9. Putting It All Together: Building the Library](#9-putting-it-all-together-building-the-library)
    - [Step 1: Create the files](#step-1-create-the-files)
    - [Step 2: `secret.rs`](#step-2-secretrs)
    - [Step 3: `game.rs`](#step-3-gamers)
    - [Step 4: `lib.rs`](#step-4-librs)
  - [10. Putting It All Together: Building the Binary](#10-putting-it-all-together-building-the-binary)
  - [11. Running the Game](#11-running-the-game)
  - [12. Additional Cargo Tricks](#12-additional-cargo-tricks)
  - [13. Summary of New Concepts](#13-summary-of-new-concepts)

---

## 1. Introduction

In the first Mastermind workshop, you wrote the whole game in a single `main.rs` file. That’s fine for a small program, but real‑world Rust projects are organised into **libraries** and **binaries**. Libraries can be shared, tested independently, and documented.

In this advanced workshop, you will:

- Split the code into a **library crate** (`src/lib.rs`) that contains all game logic.
- Keep the user interaction in a **binary crate** (`src/main.rs`) that uses the library.
- Add documentation and unit tests to the library.
- Use an external crate (`clap`) for command‑line argument parsing.

By the end, you’ll understand how Rust’s module system works and how to structure a professional project.

---

## 2. Prerequisites

- Completed the first Mastermind workshop (or understand basic Rust: structs, methods, `Vec`, `Option`, etc.)
- A working `mastermind` Cargo project

If you don’t have it, recreate it:

```bash
cargo new mastermind
cd mastermind
```

We’ll restructure it from the ground up.

---

## 3. Concept 1: Rust Packages, Crates, and Modules

### Explanation

Rust organises code into a hierarchy of **packages**, **crates**, and **modules**.

- A **package** is a Cargo project (the `Cargo.toml` folder). It can contain one or more **crates**.
- A **crate** is a compilation unit: a **binary crate** produces an executable, a **library crate** produces a `.rlib` file that can be used by other crates.
- **Modules** (`mod`) group related items and control privacy. The module tree is rooted in `crate` (for the current crate).

A typical package layout:

```
mastermind/          ← package
├── Cargo.toml
└── src/
    ├── main.rs      ← binary crate root (by default)
    └── lib.rs       ← library crate root (if present)
```

Every `.rs` file is automatically a module named after the file (minus the extension). You can also create folders with `mod.rs` (old style) or `mod.rs` alternatives.

#### Example

```rust
// src/lib.rs
pub mod game;      // tells Rust to look for src/game.rs (or src/game/mod.rs)

// src/game.rs
pub struct Game { ... }
```

#### Diagram

```
crate (root module)
├── module 'game'   (defined in src/game.rs)
│   └── struct Game
└── module 'secret' (defined in src/secret.rs)
    └── struct SecretCode
```

#### Python Comparison

Python uses files as modules and folders as packages (with `__init__.py`). Rust’s `mod` declares a module explicitly; the file name doesn’t automatically become a module without `mod` declaration.

---

## 4. Concept 2: Library vs Binary Crate

### Explanation

When you have both `lib.rs` and `main.rs`, Cargo builds:

- The library crate (named as the package, `mastermind`)
- A binary crate (also named `mastermind` by default, but can be overridden)

The binary crate can use the library crate via `use mastermind::...`. The library crate **cannot** depend on the binary crate.

Library code is reusable; binary code is the “front‑end”. All the logic should live in the library; the binary is just the `main()` function that ties things together.

### Applying to Workshop

We will move the `SecretCode` and `MastermindGame` structs into the library (`src/lib.rs`), and keep only the input/output and game loop in `main.rs`.

---

## 5. Concept 3: Visibility (`pub`) and Re‑exports

### Explanation

By default, everything in Rust is **private** to its current module. To expose items to the outside world (or to parent modules), you prefix with `pub`.

Re‑exports allow you to create a convenient public API surface. For example, you can define a module hierarchy but re‑export only the main types at the crate root.

```rust
// lib.rs
mod game;
pub use game::MastermindGame;   // now users can do `mastermind::MastermindGame`

// game.rs
pub struct MastermindGame { ... }
```

#### Diagram

```
crate (lib.rs)
├── mod game (private)
│   └── pub struct MastermindGame
└── pub use game::MastermindGame   ← re‑export, publicly visible
```

#### Python Comparison

In Python, all top‑level identifiers are public by convention; Rust requires explicit `pub`.

---

## 6. Concept 4: Documentation (`///` & `cargo doc`)

### Explanation

Rust has built‑in documentation generation. Comments starting with `///` document the item that follows, and `//!` documents the enclosing module/crate. Markdown is supported.

Run `cargo doc --open` to build and view the documentation in your browser.

#### Example

```rust
/// Represents the hidden 4-digit code.
///
/// # Examples
/// ```
/// let code = SecretCode::new();
/// ```
pub struct SecretCode { ... }
```

---

## 7. Concept 5: Unit Tests (`#[test]` & `cargo test`)

### Explanation

Testing is first‑class in Rust. Write tests in the same file as the code, inside a `mod tests` guarded by `#[cfg(test)]`. Each test function is annotated with `#[test]`.

Run all tests with `cargo test`.

#### Example

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

---

## 8. Concept 6: Command‑Line Arguments with `clap`

### Explanation

To make our binary more flexible (e.g., allow the user to set max attempts), we can parse command‑line arguments. The `clap` crate is the most popular choice.

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

---

## 9. Putting It All Together: Building the Library

Now we’ll rewrite the Mastermind code as a library crate with proper structure.

### Step 1: Create the files

Inside `src/`, we’ll have:

- `lib.rs` – the library root
- `secret.rs` – the `SecretCode` module
- `game.rs` – the `MastermindGame` module

Delete the old `main.rs` contents for now.

### Step 2: `secret.rs`

Move the `SecretCode` struct and its `impl` from the old code into `secret.rs`. Make everything necessary `pub`. Add documentation and a constructor that can accept a pre‑determined set of digits (useful for testing).

```rust
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Represents the hidden 4-digit code with unique digits (0–9).
/// Handles evaluation of guesses and controlled hint revelations.
pub struct SecretCode {
    digits: Vec<u8>,
    revealed_positions: Vec<bool>,
    revealed_digits: Vec<bool>,
}

impl SecretCode {
    /// Creates a random 4-digit secret code.
    pub fn new() -> Self {
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

    /// Creates a code from a known list of 4 digits (for testing).
    pub fn from_digits(digits: Vec<u8>) -> Self {
        SecretCode {
            digits,
            revealed_positions: vec![false; 4],
            revealed_digits: vec![false; 10],
        }
    }

    /// Compare a guess with the secret.
    /// Returns (green, yellow, red) counts.
    pub fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) {
        // (same implementation as before)
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
        // (same)
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

    pub fn give_digit_hint(&mut self) -> Option<u8> {
        // (same)
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

    /// Returns a comma-separated string of the secret digits (for game-over display).
    pub fn reveal(&self) -> String {
        self.digits.iter().map(|d| d.to_string()).collect::<Vec<_>>().join("")
    }
}
```

### Step 3: `game.rs`

Move `MastermindGame` and constants. Note that we’ll now leave the input/output (i.e., `display_welcome`, `get_user_input`, `display_feedback`, `handle_hint`) in the binary crate because they are presentation‑layer concerns. The library will just provide the core game state and logic.

```rust
use crate::secret::SecretCode;

/// High‑level controller for a Mastermind game.
/// Manages game state and turn counting, but not I/O.
pub struct MastermindGame {
    secret: SecretCode,
    attempts_left: u32,
    pub guess_count: u32,
}

impl MastermindGame {
    /// Creates a new game with a given maximum number of attempts.
    pub fn new(max_attempts: u32) -> Self {
        MastermindGame {
            secret: SecretCode::new(),
            attempts_left: max_attempts,
            guess_count: 0,
        }
    }

    /// Returns the number of attempts left.
    pub fn attempts_left(&self) -> u32 {
        self.attempts_left
    }

    /// Processes a valid guess (4 unique digits as &str).
    /// Returns `Some((green, yellow, red))`, or `None` if the game is already over.
    pub fn submit_guess(&mut self, guess: &str) -> Option<(usize, usize, usize)> {
        if self.attempts_left == 0 {
            return None;
        }
        self.guess_count += 1;
        let feedback = self.secret.evaluate_guess(guess);
        if feedback.0 == 4 {
            // Code cracked; attempts left unchanged (game will end)
            self.attempts_left = 0;
        } else {
            self.attempts_left -= 1;
        }
        Some(feedback)
    }

    /// Returns true if the user can still request hints.
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

    /// Deduct attempts (for hint costs).
    pub fn deduct_attempts(&mut self, cost: u32) {
        self.attempts_left = self.attempts_left.saturating_sub(cost);
    }

    /// Reveal the secret code string.
    pub fn reveal(&self) -> String {
        self.secret.reveal()
    }
}
```

### Step 4: `lib.rs`

The library root. Declare the modules and re‑export the public types.

```rust
pub mod secret;
pub mod game;

// Re‑export the main types so users can do `use mastermind::MastermindGame;`
pub use game::MastermindGame;
pub use secret::SecretCode;
```

---

## 10. Putting It All Together: Building the Binary

Now, `main.rs` will contain the user interaction, using the library.

Add `clap` to `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
clap = { version = "4", features = ["derive"] }
```

Now write `main.rs`:

```rust
use clap::Parser;
use mastermind::MastermindGame;
use std::io::{self, Write};

/// Command-line arguments
#[derive(Parser)]
struct Args {
    /// Maximum number of attempts
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
                    "\n🎉 Congratulations! You cracked the code in {} actual guesses.",
                    game.guess_count
                );
                return;
            }
        }
    }

    println!("\n❌ Game Over! The secret code was {}.", game.reveal());
}

fn display_welcome(max_attempts: u32) {
    println!("{}", "=".repeat(40));
    println!("   Welcome to Mastermind!");
    println!("   Guess the 4-digit code (digits 0-9, no repeats)");
    println!("   You have {} attempts. Type 'help' for hints.", max_attempts);
    println!("{}", "=".repeat(40));
}

fn display_feedback(green: usize, yellow: usize, red: usize) {
    println!("🟢: {}   🟡: {}   🔴: {}", green, yellow, red);
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

---

## 11. Running the Game

Now you can run the binary, which uses your library:

```bash
cargo run
# Or with custom attempts:
cargo run -- --max-attempts 10
```

---

## 12. Additional Cargo Tricks

- **Documentation**: Run `cargo doc --open` to see your library’s documentation generated from the doc comments.
- **Testing**: Add unit tests inside `secret.rs` and `game.rs` and run `cargo test`.
- **Workspaces**: If you later have multiple related crates, a workspace `[workspace]` in `Cargo.toml` ties them together.
- **Features**: You can define optional dependencies in `Cargo.toml` and gate code with `#[cfg(feature = "foo")]`.
- **Publishing**: If you wanted to share your library on crates.io, you’d run `cargo publish`.

---

## 13. Summary of New Concepts

| Concept | Where Used |
|---------|------------|
| Package layout (`lib.rs` + `main.rs`) | Whole project |
| Modules (`mod`, file organisation) | `secret.rs`, `game.rs`, `lib.rs` |
| `pub` visibility and re‑exports | `pub struct`, `pub use` in `lib.rs` |
| Documentation (`///`, `//!`, `cargo doc`) | Above structs and functions |
| Unit tests (`#[test]`, `#[cfg(test)]`, `cargo test`) | (suggested practice) |
| `clap` for CLI argument parsing | `main.rs` with `Args` struct |
| `derive` macros (`#[derive(Parser)]`) | On `Args` |
| `saturating_sub` | In `deduct_attempts` |
| Separation of concerns (logic in lib, I/O in bin) | Moved game logic to library |

---

Congratulations! You’ve turned a monolithic game into a well‑structured Rust project, ready for sharing, testing, and further extension.

