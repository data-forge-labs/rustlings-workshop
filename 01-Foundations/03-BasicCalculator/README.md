# Rust for Python Data Engineers — Basic Calculator

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each `workshop/src/lib.rs` function starts as a `todo!()` stub. As you follow each section of this tutorial, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 35 tests pass**.

---

## What Is This Calculator?

A command-line integer calculator that demonstrates Rust's fixed-size integer types, overflow behavior, and built-in unit testing.

### Python equivalent

```python
# Python — one int type, no overflow
def add(a, b):
    return a + b

def factorial(n):
    result = 1
    for i in range(1, n + 1):
        result *= i
    return result

x = 2 ** 1000  # Python handles arbitrarily large integers
```

In this project you'll learn to build this in Rust — and along the way
you'll discover **fixed-size integer types**, **overflow and saturating arithmetic**, **`panic!`**, and **built-in unit testing with `#[test]`**.

> **Note:** Variables, mutability, `if`/`else` expressions, and the `bool`-only rule are taught in [Project 0: Intro](../01-Intro/README.md). This project focuses only on **integer-specific** Rust.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Integer types | `u8`, `i32`, `u64`, `usize` — fixed-size, prevent silent overflow |
| 2 | Arithmetic & division | `/` truncates toward zero, not floor |
| 3 | `panic!` | Unrecoverable runtime error for programmer bugs |
| 4 | `while` & `for` loops | Condition and range-based iteration |
| 5 | Integer overflow | Debug: panics, Release: wraps — never silent |
| 6 | Wrapping / saturating | `.wrapping_add()`, `.saturating_add()` — controlled overflow |
| 7 | Type casting | `x as u64` — explicit, truncates wide → narrow |
| 8 | Unit testing | `#[test]`, `#[should_panic]` — built-in, no extra crate |

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Prerequisites](#2-prerequisites)
3. [Running the Python Version](#3-running-the-workshop)
4. [Concept: Integer Types in Rust](#4-concept-integer-types-in-rust)
5. [Concept: Variables and Mutability (Recap)](#5-concept-variables-and-mutability-recap)
6. [Concept: Arithmetic Operators](#6-concept-arithmetic-operators)
7. [Concept: Control Flow — `if`/`else` (Recap)](#7-concept-control-flow-ifelse-recap)
8. [Concept: No Truthy/Falsy — the `bool` Type (Recap)](#8-concept-no-truthyfalsy-the-bool-type-recap)
9. [Concept: Panics — Unrecoverable Errors](#9-concept-panics-unrecoverable-errors)
10. [Concept: Loops — `while` and `for`](#10-concept-loops-while-and-for)
11. [Concept: Integer Overflow](#11-concept-integer-overflow)
12. [Concept: Wrapping and Saturating Arithmetic](#12-concept-wrapping-and-saturating-arithmetic)
13. [Concept: Type Casting with `as`](#13-concept-type-casting-with-as)
14. [Concept: Unit Testing in Rust](#14-concept-unit-testing-in-rust)
15. [Putting It All Together — The Complete Calculator](#15-putting-it-all-together-the-complete-calculator)
16. [Exercises to Try](#16-exercises-to-try)
17. [Summary](#17-summary)

---

## 1. Project Overview

We'll build a **command-line calculator** that can:
- Add, subtract, multiply, and divide integers
- Compute factorials
- Detect and handle overflow gracefully

> Variables, mutability, and `if`/`else` expressions are covered in [Project 0: Intro](../01-Intro/README.md).

---

## 2. Prerequisites

- Rust installed (see [Project 0: Intro](../01-Intro/README.md))
- Basic Python knowledge
- Familiarity with `cargo new` and `cd workshop && cargo run`

---

## 3. Running the Workshop

This is a **Type B** Cargo project — there is no `project.py`. The workshop lives in `workshop/`:

```bash
cd workshop
cargo test        # Run the 35 progressive tests
cargo run         # Run the calculator REPL
```

If `workshop/` does not exist yet, follow the [Setup section](#create-the-project) below to create it from scratch.

---

## 4. Concept: Integer Types in Rust

### Python vs Rust Integers

```python
# Python — one integer type, arbitrary precision
x = 42
y = 2 ** 1000  # Works fine, any size
```

```rust
// Rust — multiple integer types, fixed precision
let x: u32 = 42;      // Unsigned 32-bit: 0 to 4,294,967,295
let y: i32 = -42;     // Signed 32-bit: -2,147,483,648 to 2,147,483,647
let z: u64 = 100;     // Unsigned 64-bit: 0 to 18,446,744,073,709,551,615
```

### Why So Many Types?

In Python, one integer type is convenient but wastes memory. In data engineering:

- **Counts of rows**: `u32` or `u64` (never negative)
- **Timestamps (Unix epoch)**: `i64`
- **IDs**: `u64`
- **Temperatures**: `i32` (can be negative)
- **Network port**: `u16` (0–65535)

Choosing a smaller type saves memory — critical when processing millions of records.

### Integer Type Family

| Bit width | Signed (positive + negative) | Unsigned (0 and positive) | Max value (unsigned) |
|---|---|---|---|
| 8-bit | `i8` | `u8` | 255 |
| 16-bit | `i16` | `u16` | 65,535 |
| 32-bit | `i32` | `u32` | ~4.3 billion |
| 64-bit | `i64` | `u64` | ~18.4 quintillion |
| 128-bit | `i128` | `u128` | huge |

**Memory diagram** (how `u32` is stored):

```
u32 = 32 bits = 4 bytes
┌──────────────────────────────────────────────┐
│ 0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│1│0│1│0│1│
└──────────────────────────────────────────────┘
                    Value: 42
```

### Inference and Defaults

```rust
let x = 42;        // Defaults to i32
let y: u64 = 42;   // Explicit type annotation
let z = 42u64;     // Suffix syntax — same as above
let w = 1_000_000; // Underscores for readability
```

### `usize` and `isize`

Special types that match your system's pointer size:
- On 64-bit systems: `usize` = `u64`, `isize` = `i64`
- Used for: array/vector indices, sizes

```rust
let arr = [1, 2, 3];
let len: usize = arr.len();  // Returns usize
```

### Exercise: Declare Data Types

Write a Rust program that declares variables matching these data types:

```rust
fn main() {
    // TODO: Declare these with the correct types
    // 1. Number of rows in a dataset (always positive, could be large)
    // 2. Temperature in Celsius (can be negative)
    // 3. A single byte of data
    // 4. The year as a 16-bit number
}
```

<details>
<summary>Solution</summary>

```rust
fn main() {
    let row_count: u64 = 1_000_000;   // Large, never negative
    let temp_c: i32 = -5;              // Can be negative
    let byte: u8 = 255;                // Single byte: 0-255
    let year: u16 = 2026;              // Year fits in 16 bits
}
```
</details>

---

## 5. Concept: Variables and Mutability (Recap)

> **Already covered in [Project 0: Intro](../../01-Intro/README.md#6-variables-and-mutability).** Below is a quick refresher — open the Intro project for the full teaching with Python comparison, ASCII diagram, and shadowing example.

Variables are **immutable by default**. Use `mut` to opt in:

```rust
let x = 5;        // immutable
let mut y = 5;    // mutable
y += 1;           // ✅ works because y is mut
// x += 1;        // ❌ compile error
```

You can also **shadow** a variable with a new `let` binding, even changing its type:

```rust
let data = "42";                  // &str
let data: i32 = data.parse().unwrap();  // i32 — shadowed with new type
```

In data engineering, **explicit mutability is a feature, not a restriction** — it makes code easier to reason about and prevents bugs from shared mutable state. The full Python comparison and detailed examples live in the Intro project.

---

## 6. Concept: Arithmetic Operators

### The Operators

| Operator | Meaning | Python Equivalent | Notes |
|---|---|---|---|
| `+` | Addition | `+` | Same |
| `-` | Subtraction | `-` | Same |
| `*` | Multiplication | `*` | Same |
| `/` | Division | `/` | In Rust: integer division truncates toward zero |
| `%` | Remainder | `%` | Same |
| `+=` | Add and assign | Not available directly | `x += 1` is shorthand for `x = x + 1` |

### Integer Division (Important!)

```python
# Python 3 — division always returns float
print(5 / 2)    # 2.5
print(5 // 2)   # 2 (floor division)
```

```rust
// Rust — integer division truncates toward zero
let x = 5 / 2;    // 2 (like Python //)
let y = 5.0 / 2.0; // 2.5 (floating point)
```

### Type Consistency

Rust requires both operands of an arithmetic operation to be the **same type**:

```rust
let x: i32 = 5;
let y: u32 = 3;
// let z = x + y;   // ❌ ERROR: mismatched types!
```

Python handles this implicitly:

```python
x = 5     # int
y = 3.0   # float
z = x + y # 8.0 — Python auto-promotes to float
```

In Rust, you must convert explicitly:

```rust
let z = x + (y as i32);  // Convert y to i32 first
```

### Exercise: Speed Calculator

Write a function that calculates speed given distance and time:

```rust
fn speed(distance: u32, time: u32) -> u32 {
    // Your code here
    distance / time
}

fn main() {
    println!("Speed: {}", speed(100, 20));  // Should print 5
}
```

---

## 7. Concept: Control Flow — `if`/`else` (Recap)

> **Already covered in [Project 0: Intro §7](../../01-Intro/README.md#7-ifelse-making-decisions).** Quick refresher:

In Rust, `if` is an **expression** (it returns a value), not a statement:

```rust
let label = if temp > 30 { "hot" } else { "cold" };
```

Both branches must return the same type. The compiler enforces this.

```rust
// if/else chain
let category = if temp > 30 {
    "hot"
} else if temp < 10 {
    "cold"
} else {
    "mild"
};
```

Use this in the calculator's `divide` function (next section) to guard against division by zero. See Intro §7 for the full Python comparison and memory diagram.

---

## 8. Concept: No Truthy/Falsy — the `bool` Type (Recap)

> **Already covered in [Project 0: Intro](../../01-Intro/README.md#4-syntax-side-by-side).** Quick refresher:

Rust conditions must be `bool` — there are no truthy/falsy values like Python:

```rust
let count: u32 = 0;
// if count { }    // ❌ ERROR: expected `bool`, found integer
if count > 0 { }   // ✅ must use an explicit comparison
```

This eliminates a whole class of Python bugs where `if 0:` or `if None:` silently does the wrong thing.

### Comparison operators

| Rust | Python | Meaning |
|------|--------|---------|
| `==` | `==` | Equal |
| `!=` | `!=` | Not equal |
| `<`, `>`, `<=`, `>=` | same | Ordering |
| `&&` | `and` | Logical AND |
| `\|\|` | `or` | Logical OR |
| `!` | `not` | Logical NOT |

---

## 9. Concept: Panics — Unrecoverable Errors

### What Is a Panic?

A **panic** is Rust's way of saying "something went wrong and we can't continue."

```rust
fn main() {
    panic!("Something terrible happened!");
    // This line never executes
}
```

### Division by Zero

```rust
let x = 5 / 0;  // ⚡ Panic: "attempt to divide by zero"
```

Output:
```
thread 'main' panicked at src/main.rs:2:17:
attempt to divide by zero
```

### Index Out of Bounds

```rust
let arr = [1, 2, 3];
let x = arr[5];  // ⚡ Panic: "index out of bounds"
```

### Python Comparison

```python
# Python — raises exceptions that CAN be caught
raise ValueError("bad data")
```

```rust
// Rust — panics CANNOT be caught (in normal flow)
panic!("bad data");
```

> **Why?** Panics are for **programmer errors** (bugs). For expected failures (file not found, invalid input), Rust uses `Result<T, E>` (covered in later sections).

### When to `panic!` vs When Not To

| Scenario | Use |
|---|---|
| Index out of bounds | Let it panic (it's a bug) |
| Division by zero | Check first, or panic |
| File not found | DON'T panic — use `Result` |
| Invalid user input | DON'T panic — return error |
| Unreachable code | `unreachable!()` macro |

### The `assert!` Macro

```rust
fn main() {
    let x = 2 + 2;
    assert!(x == 4);     // Passes silently
    assert!(x == 5);     // ⚡ Panics: "assertion failed: x == 5"
}
```

`assert_eq!` shows both values:

```rust
assert_eq!(2 + 2, 4);    // ✅
assert_eq!(2 + 2, 5);    // ⚡ "assertion failed: `(left == right)` left: `4`, right: `5`"
```

### `panic!` vs `Result` — Side by Side

For division-by-zero, you have two choices. Here's the same logic in both styles:

```rust
// Version A: panic! — for internal/CLI tools where the caller is "you, the developer"
fn divide_panic(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero!");  // crash with a message
    }
    a / b
}

// Version B: Result — for libraries where the caller is "another Rust programmer"
fn divide_result(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by zero!"));  // hand back the error
    }
    Ok(a / b)
}

fn main() {
    // Version A usage — the program just dies
    let result = divide_panic(10, 2);  // 5

    // Version B usage — the caller decides what to do
    match divide_result(10, 0) {
        Ok(v)  => println!("Got {}", v),
        Err(e) => println!("Bad input: {}", e),  // "Bad input: Cannot divide by zero!"
    }
}
```

You saw `Result` briefly in [02-GuessGame §7](../02-GuessGame/README.md#7-concept-resultt-e-and-parse). The deep dive — including the `?` operator and `From` conversions — is in [Section 02: Ownership](../../02-Ownership/README.md). For this project, the `divide` function uses `panic!` (it's an internal tool, not a library).

---

## 10. Concept: Loops — `while` and `for`

### `while` Loop

```rust
let mut counter = 0;
while counter < 5 {
    println!("Count: {}", counter);
    counter += 1;
}
// Prints: Count: 0, Count: 1, ..., Count: 4
```

### `for` Loop with Ranges

```rust
// Exclusive range (0..5 includes 0,1,2,3,4)
for i in 0..5 {
    println!("{}", i);
}

// Inclusive range (0..=5 includes 0,1,2,3,4,5)
for i in 0..=5 {
    println!("{}", i);
}
```

### Range Types

```rust
0..5      // Range: 0,1,2,3,4 (exclusive end)
0..=5     // RangeInclusive: 0,1,2,3,4,5
0..       // RangeFrom: 0 to infinity (be careful!)
..5       // RangeTo: up to but not including 5
..=5      // RangeToInclusive: up to and including 5
```

### Python vs Rust Loops

```python
# Python
for i in range(5):      # 0,1,2,3,4
    print(i)

i = 0
while i < 5:
    print(i)
    i += 1
```

```rust
// Rust
for i in 0..5 {        // 0,1,2,3,4
    println!("{}", i);
}

let mut i = 0;
while i < 5 {
    println!("{}", i);
    i += 1;
}
```

### Visual: Range vs RangeInclusive

```
0..5:     [0] [1] [2] [3] [4]         ← 5 elements
           └─────────────────┘
           Stop before 5

0..=5:    [0] [1] [2] [3] [4] [5]     ← 6 elements
           └─────────────────────┘
           Include 5
```

### Exercise: Factorial with Loop

```rust
fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;   // Same as: result = result * i
    }
    result
}

fn main() {
    println!("5! = {}", factorial(5));  // 120
    println!("10! = {}", factorial(10)); // 3,628,800
}
```

---

## 11. Concept: Integer Overflow

### What Is Overflow?

When the result of an arithmetic operation exceeds the maximum value for the type:

```rust
let x: u8 = 255;     // Max value for u8
let y = x + 1;       // ❗ Overflow! 256 doesn't fit in u8
```

### Two Behaviors: Debug vs Release

| Profile | `overflow-checks` | Behavior |
|---|---|---|
| `dev` (default for `cd workshop && cargo run`) | `true` | **Panics** on overflow |
| `release` (`cd workshop && cargo run --release`) | `false` | **Wraps** silently |

```bash
cd workshop && cargo run              # Panics on overflow
cd workshop && cargo run --release    # Wraps silently (256 → 0 for u8)
```

### The Danger of Silent Wrapping

```rust
// In release mode, this runs without error
let mut balance: u16 = 65535;  // Max u16
balance += 1;                   
println!("Balance: {}", balance); // Prints: 0 !!
```

**This is a data corruption bug!** Your bank account just went from $65,535 to $0 without any error.

### Why Rust Does This

- **Debug mode:** Catch bugs early — crash loudly
- **Release mode:** Performance — checking every operation costs CPU cycles
- **Best practice:** Enable overflow checks in release too:

```toml
# Cargo.toml
[profile.release]
overflow-checks = true
```

### Underflow (Same Problem, Opposite Direction)

```rust
let x: u8 = 0;
let y = x - 1;  // Underflow! 0 - 1 doesn't fit in u8
```

With wrapping: `0 - 1 = 255` for `u8`

### Python comparison

In Python, integer overflow is **impossible** — Python's `int` type uses arbitrary precision:

```python
x = 2 ** 1000  # works fine, no overflow
balance = 65535 + 1  # 65536, no problem
```

This is convenient but has a cost: every arithmetic operation allocates memory for a new big-int. Rust's fixed-size integers (`u8`, `u32`, `i64`, etc.) are far more efficient — the trade-off is that you must reason about ranges. For data engineering (row counts, byte sizes, timestamps), the Rust approach is usually what you want: the type encodes the constraint.

---

## 12. Concept: Wrapping and Saturating Arithmetic

### Per-Operation Control

Instead of relying on profile settings, you can handle overflow **per operation**:

### Wrapping Methods

Wraps around like a clock:

```rust
let x: u8 = 255;
let y = x.wrapping_add(1);  // 255 + 1 = 0 (wraps around)
let z = x.wrapping_mul(2);  // 255 * 2 = 254 (wraps around)
```

Visual:
```
Wrapping circle for u8:
        255 ── 0
      254      1
    253          2
    .             .
   .               .
  .                 .
 128 ──────────── 127
```

### Saturating Methods

Stops at the minimum or maximum value:

```rust
let x: u8 = 255;
let y = x.saturating_add(1);  // 255 + 1 = 255 (stays at max)

let z: u8 = 0;
let w = z.saturating_sub(1);  // 0 - 1 = 0 (stays at min)
```

Visual:
```
Saturating:
        255 ── 255 (stuck at max)
      254      255
       ...      255
       0        0 (stuck at min)
```

### Available Methods

| Method | Description |
|---|---|
| `x.wrapping_add(y)` | Wrapping addition |
| `x.wrapping_sub(y)` | Wrapping subtraction |
| `x.wrapping_mul(y)` | Wrapping multiplication |
| `x.saturating_add(y)` | Saturating addition |
| `x.saturating_sub(y)` | Saturating subtraction |
| `x.saturating_mul(y)` | Saturating multiplication |

### Python vs Rust Overflow

```python
# Python — no overflow, arbitrary precision
x = 2 ** 1000  # Perfectly fine
```

```rust
// Rust — must handle overflow explicitly
fn safe_add(a: u32, b: u32) -> u32 {
    a.saturating_add(b)  // Never panics
}
```

### Data Engineering Application

```rust
// Counting rows across partitions — use saturating to avoid overflow
let mut total_rows: u64 = 0;
for partition in partitions {
    total_rows = total_rows.saturating_add(partition.row_count());
}
```

---

## 13. Concept: Type Casting with `as`

### Why Casting Is Needed

Rust won't mix types:

```rust
let a: u32 = 5;
let b: u64 = 10;
// let c = a + b;  // ❌ ERROR: mismatched types
let c = a as u64 + b;  // ✅ Convert a to u64 first
```

### The `as` Operator

```rust
let x = 5u32;
let y = x as u64;   // 5 as u64 = 5
let z = x as f64;   // 5 as f64 = 5.0
```

### Safe Direction: Smaller → Larger

```rust
let a: u8 = 100;
let b = a as u16;   // Always safe: all u8 values fit in u16
let c = a as u32;   // Always safe
let d = a as u64;   // Always safe
```

### Dangerous Direction: Larger → Smaller (Truncation)

```rust
let a: u16 = 256;
let b = a as u8;    // b = 0 ! Truncation occurs
```

Memory diagram of truncation:

```
u16 value 256 in binary:
┌──────────────┬──────────────┐
│ 0000 0001    │ 0000 0000    │
│  High byte   │  Low byte    │
└──────────────┴──────────────┘

as u8 keeps only the low byte:
┌──────────────┐
│ 0000 0000    │  ← 0
└──────────────┘
```

### Best Practice

```rust
// Safe: smaller to larger
let x: u8 = 50;
let y: u64 = x as u64;

// Dangerous: larger to smaller
let a: u64 = 500;
let b: u8 = a as u8;  // Truncation — b != a!

// For larger → smaller, use TryFrom (taught later):
// let b: u8 = a.try_into().unwrap();  // Panics if out of range
```

### Suffix Syntax

```rust
let x = 42u8;        // Equivalent to let x: u8 = 42;
let y = 100u64;      // Equivalent to let y: u64 = 100;
let z = 3.14f32;     // Equivalent to let z: f32 = 3.14;
```

### Python comparison

Python is **dynamically typed** — the same variable can hold an `int` or a `float` without any conversion:

```python
x = 5          # int
x = 5.0        # float — no cast needed
x = "hello"    # str — Python doesn't care
```

In Rust, types are static and explicit. When you need to convert, you write `as` (for primitives) or `try_into` (for fallible conversions, taught in 02-Ownership). For data engineering this matters: when you read a CSV column as `i64` and want to log it as `f64`, you have to write the conversion — the type system makes it visible in code review.

---

## 14. Concept: Unit Testing in Rust

### Why Testing in Rust Is Different from Python

In Python, testing is an afterthought — you reach for pytest or unittest as an external library, and tests live in separate files (`test_*.py`). It works, but it's bolted on.

In Rust, **testing is a first-class language feature**. The compiler, the build system, and the standard library all support testing out of the box.

**Python approach:**
```python
# test_calculator.py — separate file, external framework
import pytest
from calculator import add

def test_add():
    assert add(2, 3) == 5
```
```bash
pytest test_calculator.py  # Need pytest installed
```

**Rust approach:**
```rust
// src/lib.rs — lives right next to the code, no external framework
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```
```bash
cd workshop && cargo test  # Built in, no extra dependencies
```

### The Test-Driven Architecture of This Course

Every project in this course follows the same pattern:

1. **`workshop/src/lib.rs`** contains all public functions, each starting as `pub fn foo() { todo!() }` — a stub that will panic if called.
2. **`workshop/src/main.rs`** is a thin CLI entry point that calls functions from `lib.rs`.
3. **Tests** live in `lib.rs` inside a `#[cfg(test)]` module, organized by tutorial step.
4. **Your job:** As you read each section of the tutorial, replace `todo!()` with real code. Run `cd workshop && cargo test` after each step — more tests pass each time.

```
Step 1:  0/35 tests pass  ← only todo!() stubs
Step 2:  3/35 tests pass  ← first function implemented
Step 3:  8/35 tests pass  ← more functions working
...
Final:  35/35 tests pass  ← all green!
```

### How to Write Tests in Rust

#### The `#[test]` Attribute

In Python, any function whose name starts with `test_` is picked up by pytest:

```python
def test_add():      # pytest finds this automatically
    assert add(2, 3) == 5
```

In Rust, you mark test functions with `#[test]`:

```rust
#[test]              // <-- attribute marks this as a test
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
```

#### Assertion Macros vs Python `assert`

| Python | Rust | What it does |
|--------|------|-------------|
| `assert x == y` | `assert_eq!(x, y)` | Panics with `left: X, right: Y` if not equal |
| `assert x != y` | `assert_ne!(x, y)` | Panics with values if equal |
| `assert condition` | `assert!(condition)` | Panics with message if false |
| `with pytest.raises(ZeroDivisionError)` | `#[should_panic]` | Test passes only if the code panics |

```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);          // Like assert add(2,3) == 5
}

#[test]
fn test_divide_by_zero() {
    // This test passes only if divide() panics
    divide(5, 0);  // This will panic!
}

#[test]
fn test_not_overflow() {
    let result = saturating_add(100, 50);
    assert!(result == 150);             // Like assert result == 150
}
```

#### `#[should_panic]` for Expected Failures

Python's `pytest.raises` checks that a specific exception is raised:

```python
import pytest

def test_divide_by_zero():
    with pytest.raises(ZeroDivisionError):
        divide(5, 0)
```

In Rust, `#[should_panic]` checks that the test panics:

```rust
#[test]
#[should_panic(expected = "Cannot divide by zero!")]
fn test_divide_by_zero() {
    divide(5, 0);
}
```

The `expected` parameter is optional — it checks that the panic message contains the given string.

#### Tests as `Result<T, E>`

Instead of panicking on failure, tests can return a `Result`:

```rust
#[test]
fn test_add_returns_result() -> Result<(), String> {
    if add(2, 3) == 5 {
        Ok(())
    } else {
        Err(String::from("addition failed"))
    }
}
```

This pattern is useful when you want to use the `?` operator inside tests.

### Test Organization

#### `#[cfg(test)]` Module

Tests in Rust are wrapped in a module annotated with `#[cfg(test)]`. This tells the compiler: "only compile this module when running `cd workshop && cargo test`, not when building the final binary."

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;  // Bring parent module items into scope

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, 3), 1);
    }
}
```

The `use super::*;` line imports everything from the parent module (the `lib.rs` file) so you can call `add()` directly.

#### Nested Modules for Progressive Steps

This course organizes tests into nested modules that correspond to tutorial steps:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod step_04_integers {
        use super::*;

        #[test]
        fn test_u32_max() {
            let x: u32 = 4_294_967_295;
            assert_eq!(x, u32::MAX);
        }
    }

    mod step_11_overflow {
        use super::*;

        #[test]
        #[should_panic]
        fn test_overflow_panics_in_debug() {
            let x: u8 = 255;
            let _y = x + 1;  // Panics in debug mode
        }
    }
}
```

Each `mod step_N_name` block contains the tests that start passing when you complete that section of the tutorial.

#### Private Items Are Testable

One advantage of inline tests: private functions are accessible from within the module:

```rust
// Private helper — only used internally
fn validate_input(x: i32) -> bool {
    x >= 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input() {
        assert!(validate_input(5));   // Can test private functions!
        assert!(!validate_input(-1));
    }
}
```

In Python, testing private functions (those starting with `_`) is possible but considered bad practice. In Rust, since tests live in the same module, testing private functions is perfectly idiomatic.

#### Integration Tests in `tests/`

For end-to-end testing of your public API, Rust supports a `tests/` directory:

```rust
// tests/integration_test.rs — compiled as a separate crate
use my_calculator::add;

#[test]
fn test_add_integration() {
    assert_eq!(add(2, 3), 5);
}
```

Each file in `tests/` is compiled as its own crate and can only access `pub` items from your library. This project focuses on unit tests, but you'll see integration tests in later projects.

### Running Tests

| Command | What it does |
|---------|-------------|
| `cd workshop && cargo test` | Runs all tests in the project |
| `cd workshop && cargo test test_add` | Runs only tests whose name contains `test_add` |
| `cd workshop && cargo test step_04` | Runs only tests in the `step_04` module |
| `cd workshop && cargo test -- --nocapture` | Shows `println!` output from tests (off by default) |
| `cd workshop && cargo test -- --test-threads=1` | Runs tests one at a time (useful for shared state) |

**Pro tip:** Use `cd workshop && cargo test` often — it's fast because Rust only recompiles changed code:

```bash
# After every step:
cd workshop && cargo test           # See how many tests pass now
cd workshop && cargo test step_04   # Just test the integers section
```

### The Test-Driven Workflow for This Course

Here's exactly how you'll use testing in every project:

1. **Open the project** — `workshop/src/lib.rs` has functions like this:
   ```rust
   /// Adds two numbers
   pub fn add(a: i32, b: i32) -> i32 {
       todo!()  // ← You'll replace this
   }
   ```

2. **Read the README section** — e.g., Section 6 explains arithmetic operators.

3. **Replace `todo!()` with real code:**
   ```rust
   pub fn add(a: i32, b: i32) -> i32 {
       a + b
   }
   ```

4. **Run the tests:**
   ```bash
   cd workshop && cargo test
   ```
   You'll see something like:
   ```
   running 4 tests
   test tests::step_06_operators::test_add ... ok
   test tests::step_06_operators::test_subtract ... ok
   test tests::step_06_operators::test_multiply ... FAILED
   test tests::step_06_operators::test_divide ... ok
   ```

5. **Fix failing tests** — maybe `multiply` still has `todo!()`. Fix it, re-run.

6. **Repeat** — each section unlocks more tests. The counter climbs: 4/35, 8/35, 16/35, 35/35.

```
   ✅✅✅✅  ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜   ← 4/35 after Section 6
   ✅✅✅✅✅✅✅✅  ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜   ← 8/35 after Section 7
   ...until...
   ✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅✅   ← 35/35 done!
```

### Concrete Example

Looking at this project's own `workshop/src/lib.rs`, you'll find tests like these:

```rust
// In src/lib.rs

#[cfg(test)]
mod tests {
    use super::*;

    mod step_04_integers {
        use super::*;

        #[test]
        fn test_u32_max() {
            // Tests that u32 can hold 4,294,967,295
            let max: u32 = 4_294_967_295;
            assert_eq!(max, u32::MAX);
        }
    }

    mod step_11_overflow {
        use super::*;

        #[test]
        #[should_panic(expected = "attempt to add with overflow")]
        fn test_u8_overflow() {
            let x: u8 = 255;
            let _y = x + 1;  // Should panic in debug mode
        }
    }
}
```

Each `step_XX_name` module corresponds directly to a numbered section in this README. When you finish Section 4, the `step_04_integers` tests pass. When you finish Section 11, the `step_11_overflow` tests pass. And so on until all 35 tests pass.

### Data Engineering Context

In data engineering, testing is critical:
- **Pipeline validation:** Test that your row-counting logic doesn't overflow
- **Schema checks:** Test that type conversions produce correct values
- **ETL correctness:** Test that your data transformations are accurate
- **Regression prevention:** Tests ensure that adding new features doesn't break existing pipelines

Rust's built-in testing, combined with its type safety, means you catch bugs at compile time and verify behavior at test time — a powerful combination for production data systems.

---

## 15. Putting It All Together — The Complete Calculator

Now let's build the complete calculator that uses everything we've learned:

```rust
/// A basic calculator for integer arithmetic
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero!");
    }
    a / b
}

/// Safe factorial using saturating arithmetic
fn factorial_safe(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..=n {
        result = result.saturating_mul(i);
    }
    result
}

/// Factorial using wrapping arithmetic
fn factorial_wrapping(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..=n {
        result = result.wrapping_mul(i);
    }
    result
}

/// Demonstrate numeric type sizes
fn demonstrate_types() {
    // usize/isize match your system
    println!("usize = {} bits", std::mem::size_of::<usize>() * 8);

    // Type range limits
    println!("u8:     {} to {}", u8::MIN, u8::MAX);
    println!("u32:    {} to {}", u32::MIN, u32::MAX);
    println!("i32:    {} to {}", i32::MIN, i32::MAX);
    println!("u64:    {} to {}", u64::MIN, u64::MAX);

    // Working with usize for indices
    let data = [10, 20, 30, 40, 50];
    let index: usize = 2;
    println!("data[{}] = {}", index, data[index]);
}

fn main() {
    println!("=== Basic Calculator ===\n");

    // Basic operations
    println!("10 + 5 = {}", add(10, 5));
    println!("10 - 5 = {}", subtract(10, 5));
    println!("10 * 5 = {}", multiply(10, 5));
    println!("10 / 5 = {}", divide(10, 5));

    // Safe factorial
    println!("\n--- Safe Factorial ---");
    for i in 0..=20 {
        let fact = factorial_safe(i);
        if fact == 0 && i > 0 {
            println!("{}! = OVERFLOW (returned 0)", i);
        } else {
            println!("{}! = {}", i, fact);
        }
    }

    // Demonstrate type ranges
    println!("\n--- Type Information ---");
    demonstrate_types();

    // Demonstrate integer division vs float
    println!("\n--- Integer vs Float Division ---");
    println!("5 / 2 = {}", 5 / 2);         // 2 (truncates)
    println!("5.0 / 2.0 = {}", 5.0 / 2.0); // 2.5

    // Demonstrate `as` casting
    println!("\n--- Type Casting ---");
    let a: u8 = 200;
    let b: u32 = a as u32;
    println!("u8 {} cast to u32: {}", a, b);

    // Temperature categorization using if/else expression
    println!("\n--- Temperature Check ---");
    let temps = [-5, 15, 35];
    for temp in temps {
        let category = if temp > 30 {
            "hot"
        } else if temp < 10 {
            "cold"
        } else {
            "mild"
        };
        println!("{}°C: {}", temp, category);
    }
}
```

### Create the Project

```bash
# The complete calculator is also available as a stretch goal in workshop/
cd workshop
# Compare your src/main.rs with the reference solution
cargo run
```

Expected output:

```
=== Basic Calculator ===

10 + 5 = 15
10 - 5 = 5
10 * 5 = 50
10 / 5 = 2

--- Safe Factorial ---
0! = 1
1! = 1
2! = 2
3! = 6
4! = 24
5! = 120
...
12! = 479001600
13! = OVERFLOW (returned 0)

--- Type Information ---
usize = 64 bits
u8:     0 to 255
u32:    0 to 4294967295
i32:    -2147483648 to 2147483647
u64:    0 to 18446744073709551615

--- Integer vs Float Division ---
5 / 2 = 2
5.0 / 2.0 = 2.5
```

---

## 16. Exercises to Try

### Exercise 1: Find the Bug

```rust
fn average(a: u32, b: u32) -> u32 {
    (a + b) / 2
}
```

What happens when `a = u32::MAX` and `b = 1`? Fix it using saturating arithmetic.

<details>
<summary>Solution</summary>

```rust
fn average(a: u32, b: u32) -> u32 {
    a.saturating_add(b) / 2
}
```
</details>

### Exercise 2: CSV Row Counter

Write a function that safely adds row counts from multiple CSV files:

```rust
fn total_rows(counts: &[u64]) -> u64 {
    // Use saturating arithmetic
    let mut total = 0u64;
    for &count in counts {
        total = total.saturating_add(count);
    }
    total
}
```

### Exercise 3: Temperature Stats

Write functions to compute min, max, and average of a list of temperatures using what you've learned:

```rust
fn min_temp(temps: &[i32]) -> i32 {
    let mut min = temps[0];
    for &t in temps {
        if t < min {
            min = t;
        }
    }
    min
}

fn max_temp(temps: &[i32]) -> i32 {
    let mut max = temps[0];
    for &t in temps {
        if t > max {
            max = t;
        }
    }
    max
}

fn avg_temp(temps: &[i32]) -> f64 {
    let mut sum: i32 = 0;
    for &t in temps {
        sum = sum.saturating_add(t);  // Safe from overflow
    }
    sum as f64 / temps.len() as f64
}

fn main() {
    let temps = [23, 25, 19, 30, 28, 22, 21];
    println!("Min: {}", min_temp(&temps));
    println!("Max: {}", max_temp(&temps));
    println!("Avg: {:.1}", avg_temp(&temps));
}
```

### Exercise 4: Overflow Detector

Write a function that checks if adding two numbers would overflow:

```rust
fn would_overflow(a: u32, b: u32) -> bool {
    a > u32::MAX - b
}

fn main() {
    assert!(!would_overflow(100, 50));
    assert!(would_overflow(u32::MAX, 1));
}
```

---

## 17. Summary

This project focused on **integer-specific Rust**. For variables, mutability, `if`/`else` expressions, and `bool` rules, see [Project 0: Intro](../01-Intro/README.md).

| Concept | Description | Python Equivalent |
|---|---|---|
| Integer types | `u8`, `i32`, `u64`, etc. — choose precision | Single `int` type |
| Arithmetic operators | `+`, `-`, `*`, `/`, `%` on integers | Same operators, with implicit promotion |
| `panic!` | Unrecoverable error | `raise Exception` |
| `while` loop | Loop with condition | `while` |
| `for` + range | Loop over range | `for i in range(n)` |
| Overflow | Result exceeds type's max | Not possible (arbitrary precision) |
| `saturating_*` | Clamp at max/min | N/A |
| `wrapping_*` | Wrap around on overflow | N/A |
| `as` casting | Explicit type conversion | Implicit conversion |
| `#[test]`, `#[should_panic]`, `assert_eq!` | First-class unit testing | `pytest`, `unittest` (external) |

### Next Project

Proceed to [04-MasterMind](../04-MasterMind/README.md) to solidify these concepts with a game, then [02-Ownership/01-TicketV1](../../02-Ownership/01-TicketV1/README.md) to learn about **ownership** — Rust's most unique and important feature.

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

