# Rust for Python Data Engineers - Introduction

*A beginner-friendly introduction to Rust designed specifically for data engineers coming from Python.*

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 21 tests pass**.

---

## Why Rust 4 Data Engineers?

### The Problem

You know Python. You love Python. But when you process 10 million rows of CSV data, Python slows to a crawl. When you deploy a data pipeline, you need a Python runtime, virtualenvs, and dependency management. When you need to parallelize a task, the GIL limits you to one CPU core at a time.

Worst of all, many bugs only surface at runtime — a `None` sneaks into your pipeline, a column name changes, a type mismatch crashes your ETL at 3 AM. Python's dynamism is great for exploration but fragile in production.

```python
def process_row(row):
    return row["value"] * 2  # Runtime error if value is string or None

counts = {}
for chunk in data_chunks:
    counts[chunk.id] = counts.get(chunk.id, 0) + 1
```

### The Rust Solution

Rust catches these issues at **compile time**, before your pipeline ever runs:

```rust
fn process_row(row: &HashMap<String, f64>) -> f64 {
    row["value"] * 2.0  // Compiler ensures "value" exists and is f64
}

let mut counts: HashMap<u32, u64> = HashMap::new();
for chunk in data_chunks {
    let counter = counts.entry(chunk.id).or_insert(0);
    *counter = counter.saturating_add(1);
}
```

Every type is explicit, every potential error is handled, and the compiler guarantees correctness before deployment. No more 3 AM panics from `AttributeError` or `TypeError` in production pipelines.

---

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | Function declarations | `fn` keyword | `def` | Define functions with typed parameters |
| 2 | Variable bindings | `let`, `let mut` | `x = value` | Declare immutable or mutable variables |
| 3 | Shadowing | `let x = x + 1` | Simple reassignment | Rebind a name (possibly changing type) |
| 4 | Constants | `const NAME: Type = val;` | Convention `MAX = val` | Compile-time constant values |
| 5 | Expressions vs Statements | Expression produces value | Most Python is statements | Functional-style control flow |
| 6 | `if` as expression | `if` / `else` blocks | Ternary `a if cond else b` | Conditionally produce a value |
| 7 | Block `{}` expressions | `{ let x = 1; x + 2 }` | N/A | Blocks return their last expression |
| 8 | `for` loops with ranges | `0..n` syntax | `for i in range(n):` | Iterate over numeric ranges |
| 9 | `println!` macro | `println!` | `print()` | Print formatted output to stdout |
| 10 | Macros | `!` suffix (e.g., `vec!`, `panic!`) | N/A (special forms) | Metaprogramming with compile-time code gen |
| 11 | Module imports | `use std::io;` | `import io` | Import standard library modules |
| 12 | Cargo build system | `Cargo.toml`, `cargo new` | `pyproject.toml` + `pip` | Package management and builds |
| 13 | Console input | `io::stdin().read_line()` | `input()` | Read user input from terminal |
| 14 | String parsing | `.trim().parse::<T>().expect()` | `int(x)` / `float(x)` | Convert string to number with error handling |

## Concepts at a Glance

### 1. Function declarations
Use `fn` instead of `def`. Every parameter and return value must have an explicit type. **Python:** `def add(x, y):` -> **Rust:** `fn add(x: u32, y: u32) -> u32 { x + y }`.

### 2. Variable bindings
`let` introduces a variable that is **immutable by default**. Add `mut` to allow changes. **Python:** all variables are mutable. **Rust:** `let x = 5;` cannot reassign; `let mut x = 5;` can reassign.

### 3. Shadowing
Redeclare a variable with `let` to create a new binding. Unlike mutation, shadowing can change the type. **Python:** simple reassignment `x = "hello"` works but doesn't create a new binding.

### 4. Constants
`const MAX: u32 = 1000;` declares a compile-time constant. Type is required; naming is `SCREAMING_SNAKE_CASE`. **Python:** `MAX = 1000` is a convention only.

### 5. Expressions vs Statements
An **expression** produces a value; a **statement** performs an action. In Rust, nearly everything is an expression. **Python:** most constructs are statements (except ternary and comprehensions).

### 6. `if` as expression
`let status = if x > 0 { "pos" } else { "neg" };` — the `if`/`else` block returns a value. **Python:** only the ternary `"pos" if x > 0 else "neg"` works this way.

### 7. Block `{}` expressions
`{ let x = 2; x * 3 }` evaluates to `6`. The last expression is the block's value. **Python:** no equivalent — you'd need a separate function.

### 8. `for` loops with ranges
`for i in 0..5 { }` iterates `0,1,2,3,4`. Use `0..=5` for inclusive. **Python:** `for i in range(5):`.

### 9. `println!` macro
`println!("val = {}", x)` prints with format interpolation. The `!` marks it as a macro. **Python:** `print(f"val = {x}")`.

### 10. Macros
Rust macros (`!`) perform compile-time code generation. `println!`, `vec!`, `panic!` are common. **Python:** no direct equivalent — closest are decorators or `eval`/`exec`.

### 11. Module imports
`use std::io;` imports the `io` module. **Python:** `import io`.

### 12. Cargo build system
`Cargo.toml` is the package manifest (dependencies, metadata). `cargo new`, `cargo build`, `cargo run` manage the lifecycle. **Python:** `pyproject.toml` + `pip install` + `python -m`.

### 13. Console input
`io::stdin().read_line(&mut buf)` reads a line of user input into a mutable `String`. **Python:** `input()`.

### 14. String parsing
`.trim().parse::<u32>()` converts a string to a `u32`. Combine with `.expect("msg")` to panic on parse failure. **Python:** `int(x)` raises `ValueError`.

---

## Table of Contents

1. [Why Rust for Data Engineering?](#1-why-rust-for-data-engineering)
2. [Installing Rust](#2-installing-rust)
3. [Your First Rust Program: Hello, Data!](#3-your-first-rust-program-hello-data)
4. [Concept: Rust vs Python - Syntax Side-by-Side](#4-concept-rust-vs-python--syntax-side-by-side)
5. [Concept: Functions in Rust](#5-concept-functions-in-rust)
6. [Concept: Variables and Mutability](#6-concept-variables-and-mutability)
7. [Concept: Expressions vs Statements](#7-concept-expressions-vs-statements)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Cargo Workshop — Commands for the Course](#9-cargo-workshop--commands-for-the-course)
10. [Appendix: Rust Syntax Reference](#10-appendix-rust-syntax-reference)
11. [Summary](#11-summary)
12. [Exercise: Guess the Number Game](#12-exercise-guess-the-number-game)

---

## 1. Why Rust for Data Engineering?

If you're a Python data engineer, you know the pain points:

| Pain Point | Python | Rust |
|---|---|---|
| **Speed** | Interpreted, can be 10-100x slower than C | Compiled, as fast as C/C++ (zero-cost abstractions) |
| **Memory usage** | High overhead per object, GC pauses | No garbage collector, predictable memory |
| **Parallelism** | GIL limits true parallelism | No GIL, fearless concurrency |
| **Deployment** | Requires Python runtime + dependencies | Single static binary, no runtime needed |
| **Error detection** | Most bugs caught at runtime | Compiler catches type errors, memory errors, null pointer errors at compile time |

### Where Rust Shines for Data Engineering

- **High-throughput data pipelines** - process millions of rows without GC pauses
- **CLI tools for data processing** - single binary, fast startup, easy distribution
- **Embedded/sidecar services** - predictable memory footprint
- **Python extension modules** - write performance-critical code in Rust, call it from Python (PyO3)
- **Streaming data** - real-time processing with deterministic latency

### The Trade-off

Rust has a **steep learning curve** - the compiler is strict, and concepts like ownership and lifetimes are new. But every concept exists to solve real problems: memory safety without GC, data races eliminated at compile time, and zero-cost abstractions.

This course is designed to **flatten that curve** by mapping every Rust concept back to something you already know from Python.

---

## 2. Installing Rust

### Step 1: Install `rustup`

```bash
# Windows (WSL):
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows (PowerShell):
# Download from https://rustup.rs
```

### Step 2: Verify Installation

```bash
rustc --version   # Rust compiler
cargo --version   # Package manager + build tool
```

### Step 3: Your First Cargo Project

```bash
cargo new hello_data
cd hello_data
```

Cargo creates this structure:

```
hello_data/
  Cargo.toml    # Package manifest (like setup.py/pyproject.toml)
  src/
    main.rs     # Source code
```

> **Python comparison:** `Cargo.toml` is like `pyproject.toml` + `requirements.txt` + `setup.py` combined.

---

## 3. Your First Rust Program: Hello, Data!

Open `workshop/src/main.rs` and replace its contents:

```rust
fn main() {
    println!("Hello, data engineers!");
}
```

Run it:

```bash
cd workshop && cargo run
```

Output:
```
Hello, data engineers!
```

### What Just Happened?

| Part | Meaning | Python Equivalent |
|---|---|---|
| `fn main()` | Program entry point | `if __name__ == "__main__":` |
| `println!` | Macro (note the `!`) that prints to stdout | `print()` |
| `"..."` | String literal | `"..."` |
| `;` | Statement terminator (optional in Python) | Required in Rust |

> **`!` means macro.** `println!` is a macro, not a function. Macros can do things functions can't - like accepting a variable number of arguments. You'll see `!` on `vec!`, `panic!`, `assert_eq!` and many others.

---

## 4. Concept: Rust vs Python - Syntax Side-by-Side

Here's a quick reference table. Don't memorize it - refer back as you go through exercises.

| Feature | Python | Rust |
|---|---|---|
| **Comments** | `# comment` | `// comment` |
| **Function** | `def add(x, y):` | `fn add(x: u32, y: u32) -> u32` |
| **Variable** | `x = 5` | `let x = 5;` |
| **Mutable** | All variables mutable | `let mut x = 5;` |
| **Constant** | `MAX = 100` (convention) | `const MAX: u32 = 100;` |
| **String** | `"hello"` | `"hello"` (type `&str`) |
| **Print** | `print("hi")` | `println!("hi")` |
| **If** | `if x > 0:` | `if x > 0 { }` |
| **For loop** | `for i in range(5):` | `for i in 0..5 { }` |
| **While** | `while x > 0:` | `while x > 0 { }` |
| **List/Array** | `[1, 2, 3]` | `[1, 2, 3]` (fixed) or `vec![1, 2, 3]` (dynamic) |
| **Tuple** | `(1, "hi")` | `(1, "hi")` |
| **Dict/HashMap** | `{"a": 1}` | `HashMap::from([("a", 1)])` |
| **None/Null** | `None` | `Option::None` (or just `None`) |
| **Exception** | `raise ValueError("msg")` | `panic!("msg")` or `Result::Err` |
| **Import** | `import os` | `use std::fs;` |
| **Package file** | `pyproject.toml` / `requirements.txt` | `Cargo.toml` |

### Key Differences for Data Engineers

```python
# Python - dynamic typing, flexible
def process_data(df, threshold):
    return df[df["value"] > threshold]
```

```rust
// Rust - static typing, explicit
fn process_data(data: Vec<f64>, threshold: f64) -> Vec<f64> {
    data.into_iter()
        .filter(|&x| x > threshold)
        .collect()
}
```

In Rust you must specify **types** - the compiler enforces them at compile time. No more `AttributeError: 'NoneType' object has no attribute 'shape'` at 3 AM.

---

## 5. Concept: Functions in Rust

### Syntax

```rust
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // body
}
```

### Example: Convert Python to Rust

```python
# Python
def celsius_to_fahrenheit(c):
    return c * 9.0 / 5.0 + 32.0
```

```rust
// Rust - same logic, explicit types
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
```

### Multiple Parameters

```rust
fn calculate_speed(distance: f64, time: f64) -> f64 {
    distance / time
}
```

### No Return Type (Unit Type `()`)

```rust
fn log_message(msg: &str) {     // No -> means return type is ()
    println!("LOG: {}", msg);
}
```

Like Python returning `None`, except Rust's `()` is a real type.

### The Last Expression Is Returned

Rust functions return the value of the **last expression** (no `return` keyword needed):

```rust
fn double(x: i32) -> i32 {
    x * 2   // No semicolon = expression, this is the return value
}
```

With `return` (less idiomatic):

```rust
fn double(x: i32) -> i32 {
    return x * 2;   // Semicolon = statement
}
```

> **Python vs Rust:** In Python, everything is a statement. In Rust, most things are **expressions** (they produce a value). Even `if` and `match` are expressions in Rust!

### Exercise: Write a Temperature Converter

Create a function that converts Fahrenheit to Celsius:

<table>
<tr>
<th>Python</th>
<th>Rust</th>
</tr>
<tr>
<td>

```python
def f_to_c(f):
    return (f - 32) * 5.0 / 9.0

print(f_to_c(212))  # 100.0
```
</td>
<td>

```rust
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("{}", fahrenheit_to_celsius(212.0));
}
```
</td>
</tr>
</table>

Try it: `cd workshop && cargo run`

---

## 6. Concept: Variables and Mutability

### `let` Binding

```rust
let x = 5;     // Immutable - cannot change
x = 6;         // ❌ Compiler error!
```

```rust
let mut y = 5; // Mutable - can change
y = 6;         // ✅ OK
```

### Why Immutable by Default?

Rust defaults to immutability for **safety**. In concurrent code, shared mutable state is the root of all bugs. Rust makes you opt in to mutability explicitly.

### Python Comparison

```python
# Python - always mutable
x = 5
x = 6  # Fine
```

```rust
// Rust - opt in to mutation
let x = 5;
x = 6;  // ❌ "cannot assign twice to immutable variable"
```

```rust
let mut x = 5;
x = 6;  // ✅
```

### Shadowing

Rust lets you **shadow** a variable by redeclaring it with `let`:

```rust
let x = 5;
let x = x + 1;   // Shadowing - creates a new variable
```

This is different from `mut` - shadowing creates a new variable (can change type too):

```rust
let x = "hello";  // x is &str
let x = x.len();  // x is usize - type changed!
```

> **Python comparison:** Python `x = 5; x = "hello"` works the same way, but Rust makes the rebinding explicit with `let`.

### Constants

```rust
const MAX_ROWS: u32 = 10_000;    // Always immutable, type required
const PI: f64 = 3.14159265359;
```

- Always `const`, never `let const`
- Type must be specified
- Convention: `SCREAMING_SNAKE_CASE`
- Evaluated at compile time

---

## 7. Concept: Expressions vs Statements

This is one of the most important Rust concepts that has no direct Python equivalent.

| | Statement | Expression |
|---|---|---|
| **Does** | Performs an action | Produces a value |
| **Has `;`** | Yes (usually) | No |
| **Examples** | `let x = 5;`, `println!();` | `x + 1`, `if true { 5 } else { 6 }` |
| **Python** | Everything is (mostly) a statement | Some things are expressions |

### `if` is an Expression

```rust
// In Python:
// if x > 0: result = "positive"
// else: result = "non-positive"

// In Rust - `if` returns a value:
let result = if x > 0 { "positive" } else { "non-positive" };
```

### Block `{}` is an Expression

```rust
let x = {
    let a = 2;
    let b = 3;
    a + b   // No semicolon - this IS the return value
};
// x = 5
```

This is the foundation for Rust's functional style - you can assign the result of any control flow to a variable.

---

## 8. Putting It All Together

Let's build a simple data processing function that demonstrates everything so far:

```rust
/// Calculate the mean of a slice of f64 values
fn mean(values: &[f64]) -> f64 {
    let mut sum = 0.0;
    let count = values.len() as f64;

    for value in values {
        sum += value;
    }

    sum / count
}

fn main() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    let result = mean(&data);
    println!("Mean: {}", result);

    // If/else as expression
    let status = if result > 2.0 { "above average" } else { "below average" };
    println!("Status: {}", status);
}
```

Run with: `cd workshop && cargo run`

Expected output:
```
Mean: 3
Status: above average
```

### What We Used

| Concept | Where |
|---|---|
| `fn` function | `mean()` and `main()` |
| `let mut` | Mutable `sum` variable |
| `let` (immutable) | `count`, `data`, `result`, `status` |
| `for` loop | Iterating over values |
| `&[f64]` | Function parameter type (slice reference) |
| `if` as expression | Assigning `status` based on condition |
| `println!` macro | Printing output |

---

## 9. Cargo Workshop — Commands for the Course

Cargo is Rust's package manager and build tool — like `pip` + `setuptools` + `pytest` + `black` combined into one command. You'll use it constantly throughout this course.

For a full reference, see [`cargo-cheatsheet.md`](./cargo-cheatsheet.md) in this folder.

### Essential Commands

```bash
# Create a new project
cargo new my_project
cd my_project

# Build (compile) without running
cargo build

# Build and run
cargo run

# Check compilation quickly (no binary produced)
cargo check

# Run tests
cargo test

# Add a dependency
cargo add rand

# Format code
cargo fmt

# Lint
cargo clippy
```

### The Build-Run-Test Loop

Every project in this course follows this pattern:

```bash
cd workshop          # Enter the project directory
cargo check          # Check your code compiles
cargo test           # Run tests — see how many pass
# ... edit src/lib.rs to replace todo!() calls ...
cargo test           # More tests should pass now
cargo run            # Run the final program
```

### Quick Reference

| Task | Command |
|------|---------|
| Build project | `cargo build` |
| Check without building | `cargo check` |
| Run project | `cargo run` |
| Run all tests | `cargo test` |
| Run a specific test | `cargo test test_name` |
| Format code | `cargo fmt` |
| Lint code | `cargo clippy` |
| Add dependency | `cargo add crate_name` |
| Build docs | `cargo doc --open` |
| Build optimized | `cargo build --release` |

> **Tip:** Use `cargo check` during development — it's faster than `cargo build` because it skips the final linking step.

---

## 10. Appendix: Rust Syntax Reference

### Comments

You can use `//` for single-line comments:

```rust
// This is a single-line comment
// Followed by another single-line comment
```

### Functions

Functions in Rust are defined using the `fn` keyword, followed by the function's name, its input parameters, and its return type. The function's body is enclosed in curly braces `{}`.

```rust
// `fn` <function_name> ( <input params> ) -> <return_type> { <body> }
fn greeting() -> &'static str {
    "I'm ready to learn Rust!"
}
```

`greeting` has no input parameters and returns a reference to a string slice (`&'static str`).

### Return type

The return type can be omitted from the signature if the function doesn't return anything (i.e. if it returns `()`, Rust's unit type):

```rust
fn test_welcome() {
    assert_eq!(greeting(), "I'm ready to learn Rust!");
}
```

The above is equivalent to:

```rust
fn test_welcome() -> () {
    assert_eq!(greeting(), "I'm ready to learn Rust!");
}
```

### Returning values

The last expression in a function is implicitly returned:

```rust
fn greeting() -> &'static str {
    "I'm ready to learn Rust!"  // Last expression — returned implicitly
}
```

You can also use the `return` keyword to return a value early:

```rust
fn greeting() -> &'static str {
    return "I'm ready to learn Rust!";
}
```

It is considered idiomatic to omit the `return` keyword when possible.

### Input parameters

Input parameters are declared inside the parentheses `()` that follow the function's name. Each parameter is declared with its name, followed by a colon `:`, followed by its type.

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

If there are multiple input parameters, they must be separated with commas.

### Type annotations

Rust is a **statically typed language**. Every single value in Rust has a type and that type must be known to the compiler at compile-time. Types are a form of **static analysis** — the compiler attaches a type tag to every value and enforces rules (e.g., you can't add a string to a number). If leveraged correctly, types can prevent whole classes of runtime bugs.

---

## 11. Summary

| Concept | Description | Python Equivalent |
|---|---|---|
| `fn` | Define a function | `def` |
| `let` | Bind a variable (immutable by default) | `x = value` |
| `let mut` | Bind a mutable variable | `x = value` (always mutable) |
| `const` | Compile-time constant | `SCREAMING_SNAKE_CASE` convention |
| `-> Type` | Function return type | Type hints (but not enforced) |
| `;` | Statement terminator | Optional in Python |
| Expression | Produces a value | Most Python constructs are statements |
| `println!` | Print to stdout | `print()` |

---

## 12. Exercise: Guess the Number Game

Let's build a small game to practice everything you've learned: `let`, `mut`, `fn`, `println!`, `std::io`, loops, `if/else`, and external crates.

### How It Works

The program:
1. Generates a random 2-digit number (10–99)
2. Gives the user **5 chances** to guess it
3. After each guess, tells the user "Too high!" or "Too low!"
4. If the user guesses correctly, prints "You win!" and exits
5. If all 5 guesses are used, prints the secret number

### Python Version First

```python
import random

secret = random.randint(10, 99)
attempts = 5

print("Guess the 2-digit number!")
for i in range(attempts):
    guess = int(input(f"Attempt {i+1}/{attempts}: "))
    if guess == secret:
        print("You win!")
        break
    elif guess < secret:
        print("Too low!")
    else:
        print("Too high!")
else:
    print(f"Game over! The number was {secret}.")
```

### Rust Version - Step by Step

Create a new Cargo project:

```bash
cargo new guess_game
cd guess_game
```

Add the `rand` crate to `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
```

Now write `workshop/src/main.rs`:

```rust
use rand::Rng;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(10..=99);
    let attempts = 5;

    println!("Guess the 2-digit number!");

    for i in 0..attempts {
        println!("Attempt {}/{}:", i + 1, attempts);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please enter a number");

        if guess == secret {
            println!("You win!");
            return;
        } else if guess < secret {
            println!("Too low!");
        } else {
            println!("Too high!");
        }
    }

    println!("Game over! The number was {}.", secret);
}
```

### Rust vs Python - Side by Side

| Aspect | Python | Rust |
|--------|--------|------|
| Random number | `random.randint(10, 99)` | `rand::thread_rng().gen_range(10..=99)` |
| Read input | `input()` | `io::stdin().read_line(&mut String)` |
| Parse input | `int(input())` | `guess.trim().parse::<u32>()` |
| Loop | `for i in range(5)` | `for i in 0..5` |
| Loop `else` (no break) | `for...else` | Not directly available |

Run it:

```bash
cd workshop && cargo run
```

### Key Takeaways for This Exercise

- **`rand::thread_rng().gen_range(10..=99)`** - generating random numbers with an external crate
- **`io::stdin().read_line(&mut guess)`** - reading user input into a mutable `String`
- **`.trim().parse::<u32>()`** - trimming whitespace and parsing a string into a number
- **`for i in 0..attempts`** - range-based loop (Python `range()` equivalent)
- **`return;`** - early exit from the function (like `break` but exits `main`)
- **`expect("message")`** - crash with a helpful message on error (like not catching an exception)

### Experiment

Try modifying the game:
- Change the range to 3-digit (100–999) with 8 attempts
- Add a hint: "You're freezing!" if within 3 of the secret
- Keep score: count how many guesses the player has taken across multiple rounds

### Further Reading

The following topic is now covered in the [Appendix: Rust Syntax Reference](#9-appendix-rust-syntax-reference) section of this README:

| Section | Topics |
|---------|--------|
| Appendix | Functions, return types, expressions vs statements, comments, type annotations |

### Next Steps

You're ready to start Project 1: **Basic Calculator**, where you'll learn about integer types, arithmetic, control flow, loops, and error handling in depth.
