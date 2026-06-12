# Project 18: CLI Salad -- Command-Line Parsing with clap

> **Test-driven approach**: This project includes a Cargo project with progressive

## Why Use `clap` for CLI Parsing?

---

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: CLI Basics and `std::env::args`](#3-concept-cli-basics-and-stdenvargs)
4. [Concept: Using the Clap Library](#4-concept-using-the-clap-library)
5. [Concept: Collections and Randomization](#5-concept-collections-and-randomization)
6. [Concept: Error Handling with Result](#6-concept-error-handling-with-result)
7. [Concept: CLI Function Testing](#7-concept-cli-function-testing)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Summary](#9-summary)

## 1. Introduction

Command-line tools are the bread and butter of data engineering. In Python, you use `argparse` to parse arguments. In Rust, `clap` is the standard library for building robust CLI applications with automatic help generation, type validation, and subcommands.

You will build a fruit salad CLI that takes a number of fruits as an argument, randomly selects that many fruits from a list, and prints the result. The project introduces the `clap` derive API and testing CLI functions with `Result`.

## 2. Prerequisites

- Basic Rust: functions, vectors, `match`
- The `rand` crate for randomization
- No prior CLI experience required

## 3. Concept: CLI Basics and `std::env::args`

### Explanation

In Python, you access command-line arguments with `sys.argv`:

```python
import sys
print(f"Program name: {sys.argv[0]}")
print(f"Arguments: {sys.argv[1:]}")
```

In Rust, `std::env::args()` returns an iterator over argument strings:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Program name: {}", args[0]);
    println!("Arguments: {:?}", &args[1..]);
}
```

Pattern matching on `args.len()` lets you handle different invocation patterns, just like checking `len(sys.argv)` in Python.

## 4. Concept: Using the Clap Library

### Explanation

In Python with argparse:

```python
import argparse

parser = argparse.ArgumentParser(description="Creates a random fruit salad")
parser.add_argument("-c", "--count", type=int, default=5, help="Number of fruits")
args = parser.parse_args()
print(args.count)
```

In Rust with clap (builder API):

```rust
use clap::{Arg, Command};

let matches = Command::new("fruit-salad")
    .version("1.0")
    .about("Creates a random fruit salad")
    .arg(Arg::new("count")
        .short('c')
        .long("count")
        .value_name("NUMBER")
        .help("Number of fruits to include")
        .takes_value(true))
    .get_matches();
```

With the derive API (used in later projects), you define a struct:

```rust
#[derive(clap::Parser)]
struct Args {
    #[arg(short, long, default_value_t = 5)]
    count: usize,
}
```

`clap` automatically generates `--help` output with descriptions and version info -- no manual printing needed.

## 5. Concept: Collections and Randomization

### Explanation

In Python, you shuffle a list with `random.shuffle()`:

```python
import random

fruits = ["Apple", "Banana", "Orange", "Grape"]
random.shuffle(fruits)
selected = fruits[:count]
```

In Rust with the `rand` crate:

```rust
use rand::seq::SliceRandom;
use rand::rng;

let mut fruits = vec!["Apple", "Banana", "Orange", "Grape"];
let mut rng = rng();
fruits.shuffle(&mut rng);
```

`SliceRandom` extends all slices with the `shuffle` method. `rng()` provides a thread-local random number generator. Unlike Python's `random.shuffle()` which works in-place, Rust requires explicit mutable access.

## 6. Concept: Error Handling with Result

### Explanation

CLI applications must handle invalid input gracefully. In Python:

```python
import sys

class SaladError(Exception):
    pass

def create_salad(count):
    if count <= 0:
        raise SaladError(f"Invalid count: {count}")
    return ["Apple", "Banana"]

try:
    salad = create_salad(0)
except SaladError as e:
    print(f"Error: {e}", file=sys.stderr)
    sys.exit(1)
```

In Rust:

```rust
#[derive(Debug)]
enum SaladError {
    InvalidCount(usize),
    InvalidFruit(String),
}

fn create_salad(count: usize) -> Result<Vec<String>, SaladError> {
    if count == 0 {
        return Err(SaladError::InvalidCount(count));
    }
    Ok(vec!["Apple".to_string(), "Banana".to_string()])
}

match create_salad(0) {
    Ok(salad) => println!("{:?}", salad),
    Err(SaladError::InvalidCount(n)) => eprintln!("Invalid count: {}", n),
    Err(SaladError::InvalidFruit(f)) => eprintln!("Invalid fruit: {}", f),
}
```

Rust's `Result<T, E>` is equivalent to a function that either returns a value or raises an exception. The `match` on Result is like `try/except` with pattern matching on exception types.

## 7. Concept: CLI Function Testing

### Explanation

The project's `fruit_salad_cli` function takes a `Vec<String>` (simulating `env::args()`) and returns `Result<String, String>`:

```rust
pub fn fruit_salad_cli(args: Vec<String>) -> Result<String, String>
```

This design makes CLI logic testable without actual process invocation. In Python, the equivalent would be a function like:

```python
def fruit_salad_cli(args: list[str]) -> tuple[bool, str]:
    # returns (success, message)
```

Tests pass simulated argument vectors to verify correct behavior:

```rust
#[test]
fn test_cli_valid_number() {
    let result = fruit_salad_cli(vec!["cli-salad".to_string(), "3".to_string()]);
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Fruit salad with 3 fruits"));
}
```

## 8. Putting It All Together

Open `workshop/src/lib.rs` and implement:

1. **`list_fruits`** -- return `vec!["Arbutus", "Loquat", ..., "Apple"]` (10 fruits)
2. **`create_fruit_salad`** -- shuffle all fruits, take the first `num_fruits`, return them
3. **`fruit_salad_cli`** -- parse args, validate count is a number, call `create_fruit_salad`, format output

Run `cd workshop && cargo test` after each step. Groups: `step_01_fruit_list` (3 tests), `step_02_fruit_salad` (4 tests), `step_03_cli` (3 tests).

## 9. Summary

| Concept | Rust Equivalent | Python Equivalent | Used In |
|---|---|---|---|
| CLI args | `std::env::args()` | `sys.argv` | Argument collection |
| Argument parsing | `clap::Command` builder | `argparse.ArgumentParser` | CLI setup |
| Random shuffle | `SliceRandom::shuffle` | `random.shuffle()` | `create_fruit_salad` |
| Random subset | `choose_multiple` | `random.sample()` | Random selection |
| Error handling | `Result<T, E>` + `match` | `try/except` | `fruit_salad_cli` |
| Testing CLI logic | Function taking `Vec<String>`, returning `Result` | Function taking list, returning tuple | `fruit_salad_cli` |

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

