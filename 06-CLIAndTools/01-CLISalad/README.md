# Project 18: CLI Salad -- Command-Line Parsing with clap

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 10 tests pass**.

## Why This Project?

### The Problem

Python's `argparse` is the standard for CLI parsing, but it comes with repetitive boilerplate that grows with every argument:

```python
import argparse

parser = argparse.ArgumentParser(description="Fruit salad maker")
parser.add_argument("-c", "--count", type=int, default=5, help="Number of fruits")
parser.add_argument("--fruits", nargs="+", default=["Apple", "Banana"])
args = parser.parse_args()
```

Every project repeats this pattern. There is no compile-time validation -- a typo in argument names only appears at runtime. Python CLI tools also pay 100-300ms interpreter startup overhead, which adds up in pipelined data workflows running hundreds of small CLI tools.

### The Rust Solution

Rust's `clap` derive API turns argument parsing into a single struct definition with automatic `--help` generation and compile-time validation:

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 5)]
    count: usize,
}

fn main() {
    let args = Args::parse();
    println!("Count: {}", args.count);
}
```

No manual validation, no boilerplate, no runtime surprises. Startup time is single-digit milliseconds -- fast enough for tight data pipelines.

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | CLI argument iteration | `std::env::args()` | `sys.argv` | Collect raw CLI argument strings |
| 2 | Builder API parsing | `clap::Command` builder | `argparse.ArgumentParser` | Define CLI arguments with help text |
| 3 | Derive API parsing | `clap::Parser` derive macro | `argparse.Namespace` | Auto-generate CLI parser from struct |
| 4 | Random shuffle | `rand::seq::SliceRandom` | `random.shuffle()` | Randomize fruit order in-place |
| 5 | Random subset | `.choose_multiple()` | `random.sample()` | Pick N distinct random fruits |
| 6 | Custom error type | `enum SaladError` | Custom `Exception` subclass | Model domain-specific errors |
| 7 | Pattern matching on errors | `match` on `Result` | `try/except` with type checks | Handle each error variant distinctly |
| 8 | Testable CLI logic | `fn(Vec<String>) -> Result<String, String>` | Function returning `(bool, str)` | Unit-test CLI without spawning process |

## Concepts at a Glance

**std::env::args** -- Rust's `env::args()` returns an iterator over argument strings, like Python's `sys.argv`. Collect into `Vec<String>` to inspect argument count and values. **clap Command builder** -- The builder API chains `.short('c')`, `.long("count")`, and `.help(...)` calls, mirroring `parser.add_argument()` in argparse but with a fluent style. **clap Parser derive** -- `#[derive(Parser)]` on a struct auto-generates a CLI parser from field attributes, eliminating all manual `parser.parse_args()` plumbing. **SliceRandom::shuffle** -- This trait extends every `&mut [T]` with `.shuffle(&mut rng)`, analogous to Python's `random.shuffle()` operating in-place. **choose_multiple** -- A `SliceRandom` method that picks N random elements without replacement, parallel to `random.sample(population, k)`. **Custom error enum** -- `enum SaladError { InvalidCount(usize), InvalidFruit(String) }` defines typed error variants, like subclassing Python's `Exception` with specific fields. **match on Result** -- `match result { Ok(v) => ..., Err(e) => ... }` mirrors Python's `try/except`. Each `Err` variant is a separate `except` clause. **Testable CLI functions** -- By accepting `Vec<String>` and returning `Result`, CLI logic is testable without subprocess invocation, unlike Python's argparse which requires mocking `sys.argv`.

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
use rand::thread_rng;

let mut fruits = vec!["Apple", "Banana", "Orange", "Grape"];
let mut rng = thread_rng();
fruits.shuffle(&mut rng);
```

`SliceRandom` extends all slices with the `shuffle` method. `thread_rng()` provides a thread-local random number generator. Unlike Python's `random.shuffle()` which works in-place, Rust requires explicit mutable access.

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
