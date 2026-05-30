# Rust for Python Data Engineers — Introduction

*A beginner-friendly introduction to Rust designed specifically for data engineers coming from Python.*

---

## Table of Contents

1. [Why Rust for Data Engineering?](#1-why-rust-for-data-engineering)
2. [Installing Rust](#2-installing-rust)
3. [Your First Rust Program: Hello, Data!](#3-your-first-rust-program-hello-data)
4. [Concept: Rust vs Python — Syntax Side-by-Side](#4-concept-rust-vs-python--syntax-side-by-side)
5. [Concept: Functions in Rust](#5-concept-functions-in-rust)
6. [Concept: Variables and Mutability](#6-concept-variables-and-mutability)
7. [Concept: Expressions vs Statements](#7-concept-expressions-vs-statements)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Summary](#9-summary)

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

- **High-throughput data pipelines** — process millions of rows without GC pauses
- **CLI tools for data processing** — single binary, fast startup, easy distribution
- **Embedded/sidecar services** — predictable memory footprint
- **Python extension modules** — write performance-critical code in Rust, call it from Python (PyO3)
- **Streaming data** — real-time processing with deterministic latency

### The Trade-off

Rust has a **steep learning curve** — the compiler is strict, and concepts like ownership and lifetimes are new. But every concept exists to solve real problems: memory safety without GC, data races eliminated at compile time, and zero-cost abstractions.

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

Open `src/main.rs` and replace its contents:

```rust
fn main() {
    println!("Hello, data engineers!");
}
```

Run it:

```bash
cargo run
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

> **`!` means macro.** `println!` is a macro, not a function. Macros can do things functions can't — like accepting a variable number of arguments. You'll see `!` on `vec!`, `panic!`, `assert_eq!` and many others.

---

## 4. Concept: Rust vs Python — Syntax Side-by-Side

Here's a quick reference table. Don't memorize it — refer back as you go through exercises.

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
# Python — dynamic typing, flexible
def process_data(df, threshold):
    return df[df["value"] > threshold]
```

```rust
// Rust — static typing, explicit
fn process_data(data: Vec<f64>, threshold: f64) -> Vec<f64> {
    data.into_iter()
        .filter(|&x| x > threshold)
        .collect()
}
```

In Rust you must specify **types** — the compiler enforces them at compile time. No more `AttributeError: 'NoneType' object has no attribute 'shape'` at 3 AM.

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
// Rust — same logic, explicit types
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

Try it: `cargo run`

---

## 6. Concept: Variables and Mutability

### `let` Binding

```rust
let x = 5;     // Immutable — cannot change
x = 6;         // ❌ Compiler error!
```

```rust
let mut y = 5; // Mutable — can change
y = 6;         // ✅ OK
```

### Why Immutable by Default?

Rust defaults to immutability for **safety**. In concurrent code, shared mutable state is the root of all bugs. Rust makes you opt in to mutability explicitly.

### Python Comparison

```python
# Python — always mutable
x = 5
x = 6  # Fine
```

```rust
// Rust — opt in to mutation
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
let x = x + 1;   // Shadowing — creates a new variable
```

This is different from `mut` — shadowing creates a new variable (can change type too):

```rust
let x = "hello";  // x is &str
let x = x.len();  // x is usize — type changed!
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

// In Rust — `if` returns a value:
let result = if x > 0 { "positive" } else { "non-positive" };
```

### Block `{}` is an Expression

```rust
let x = {
    let a = 2;
    let b = 3;
    a + b   // No semicolon — this IS the return value
};
// x = 5
```

This is the foundation for Rust's functional style — you can assign the result of any control flow to a variable.

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

Run with: `cargo run`

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

## 9. Summary

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

### Further Reading

The following lesson files in this folder provide additional context:

| File | Topics |
|------|--------|
| [00_welcome.md](./00_welcome.md) | Course overview, methodology, tools, `wr` workshop runner |
| [01_syntax.md](./01_syntax.md) | Functions, return types, expressions vs statements, comments |

### Next Steps

You're ready to start Project 1: **Basic Calculator**, where you'll learn about integer types, arithmetic, control flow, loops, and error handling in depth.
