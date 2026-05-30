# Project 33: Advanced CLI with clap + CSV Reading -- Modules and Project Structure

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 10 tests pass**.

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: CSV String Parsing](#3-concept-csv-string-parsing)
4. [Concept: Random Shuffling with `rand`](#4-concept-random-shuffling-with-rand)
5. [Concept: Display Formatting](#5-concept-display-formatting)
6. [Concept: Module Structure (lib/main split)](#6-concept-module-structure-libmain-split)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Summary](#8-summary)

## 1. Introduction

Real-world Rust data engineering projects follow a clear module structure: the `lib.rs` file contains all business logic and tests, while `main.rs` is a thin CLI entry point that uses `clap` for argument parsing. This separation makes the code testable and reusable.

In this project, you will build a fruit salad generator that:
- Reads fruits from a CSV string or a CLI argument
- Shuffles them randomly using the `rand` crate
- Displays the result in a formatted string

In Python, you would use `argparse` + `csv` modules. In Rust, you use `clap` (derive API) for parsing and implement CSV splitting manually.

## 2. Prerequisites

- `Vec<String>` manipulation
- `rand` crate basics (shuffle)
- Function signatures and return types
- Understanding of `lib.rs`/`main.rs` project structure

## 3. Concept: CSV String Parsing

### Explanation

Parse a comma-separated string into a `Vec<String>`. In Python:

```python
def csv_to_vec(csv_string):
    return [item.strip() for item in csv_string.split(",")]
```

In Rust, you use `split`, `map`, and `collect`:

```rust
pub fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}
```

The function:
1. `split(',')` returns an iterator over substrings
2. `.map(|s| s.trim().to_string())` strips whitespace and converts to owned `String`
3. `.collect()` gathers into a `Vec<String>`

### Applying to Our Project

`csv_to_vec("apple, pear, banana")` returns `vec!["apple", "pear", "banana"]`. An empty string `""` yields `vec![""]` (one empty string).

## 4. Concept: Random Shuffling with `rand`

### Explanation

Shuffling a `Vec` in-place using the `rand` crate. In Python:

```python
import random

def create_fruit_salad(fruits):
    random.shuffle(fruits)
    return fruits
```

In Rust:

```rust
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(mut fruits: Vec<String>) -> Vec<String> {
    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);
    fruits
}
```

Key details:
- `SliceRandom` trait extends slices with the `shuffle` method
- `thread_rng()` returns a thread-local random number generator
- The `mut` parameter allows in-place mutation of the `Vec`

### Applying to Our Project

`create_fruit_salad(vec!["apple", "pear", "banana"])` returns the same fruits in random order. The test verifies all original fruits are present by sorting both the input and result.

## 5. Concept: Display Formatting

### Explanation

Format a list of fruits into a readable string. In Python:

```python
def display_fruit_salad(fruits):
    result = "Your fruit salad contains:\n"
    for fruit in fruits:
        result += f"{fruit}\n"
    return result
```

In Rust, you build the string using `String::from` and `push_str`:

```rust
pub fn display_fruit_salad(fruits: &[String]) -> String {
    let mut result = String::from("Your fruit salad contains:\n");
    for fruit in fruits {
        result.push_str(&format!("{}\n", fruit));
    }
    result
}
```

Returns a `String` (owned), not `&str`, because it constructs new content. The `&format!()` macro creates a temporary `String` that `push_str` borrows.

### Applying to Our Project

`display_fruit_salad(&["apple", "pear"])` produces:

```
Your fruit salad contains:
apple
pear
```

## 6. Concept: Module Structure (lib/main split)

### Explanation

A well-structured Rust project separates library code from executable code:

- **`workshop/src/lib.rs`**: Contains public functions, type definitions, and all unit tests under `#[cfg(test)] mod tests`. This is the crate root for the library.
- **`workshop/src/main.rs`**: Contains `fn main()`, uses `clap` to parse arguments, calls functions from `lib.rs`, and handles I/O.

In Python, the equivalent is separating reusable functions into a module file (e.g., `salad.py`) and the CLI entry point into `__main__.py` or a script.

The `main.rs` has access to the crate via `use custom_cli_fruit_salad::function_name;` (using the package name from `Cargo.toml`).

### Applying to Our Project

Your `workshop/src/main.rs` will use `clap`'s derive API to define a CLI:

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "apple,pear,banana")]
    fruits: String,
}

fn main() {
    let args = Args::parse();
    let fruits = custom_cli_fruit_salad::csv_to_vec(&args.fruits);
    let salad = custom_cli_fruit_salad::create_fruit_salad(fruits);
    let output = custom_cli_fruit_salad::display_fruit_salad(&salad);
    println!("{}", output);
}
```

This keeps `main.rs` as a thin wrapper, with all logic testable through `lib.rs`.

## 7. Putting It All Together

Open `workshop/src/lib.rs` and implement:

1. **`csv_to_vec`** -- `split(',')`, `map(|s| s.trim().to_string())`, `collect()`
2. **`create_fruit_salad`** -- accept `mut Vec<String>`, `shuffle(&mut thread_rng())`, return
3. **`display_fruit_salad`** -- build string with header + each fruit on its own line

Run `cd workshop && cargo test` after each step. Groups: `step_01_csv_parsing` (4 tests), `step_02_fruit_salad` (3 tests), `step_03_display` (3 tests).

## 8. Summary

| Concept | Rust Equivalent | Python Equivalent | Used In |
|---|---|---|---|
| CSV string parsing | `split(',')`, `map(str::trim)`, `collect()` | `str.split(",")` + list comprehension | `csv_to_vec` |
| Random shuffle | `SliceRandom::shuffle` | `random.shuffle()` | `create_fruit_salad` |
| Thread-local RNG | `thread_rng()` | `random.Random()` | `create_fruit_salad` |
| String building | `String::from` + `push_str` + `format!` | String concatenation | `display_fruit_salad` |
| CLI argument parsing | `clap::Parser` derive | `argparse` | `main.rs` |
| Library/executable split | `lib.rs` (logic + tests) / `main.rs` (CLI) | Module + `__main__.py` | Project structure |
