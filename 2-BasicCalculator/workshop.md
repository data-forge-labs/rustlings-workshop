# Rust for Python Data Engineers — Basic Calculator

*A hands-on workshop that teaches Rust fundamentals by building a command-line calculator — all concepts mapped to Python equivalents.*

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Prerequisites](#2-prerequisites)
3. [Running the Python Version](#3-running-the-python-version)
4. [Concept: Integer Types in Rust](#4-concept-integer-types-in-rust)
5. [Concept: Variables and Mutability](#5-concept-variables-and-mutability)
6. [Concept: Arithmetic Operators](#6-concept-arithmetic-operators)
7. [Concept: Control Flow — `if`/`else`](#7-concept-control-flow--ifelse)
8. [Concept: No Truthy/Falsy — the `bool` Type](#8-concept-no-truthyfalsy--the-bool-type)
9. [Concept: Panics — Unrecoverable Errors](#9-concept-panics--unrecoverable-errors)
10. [Concept: Loops — `while` and `for`](#10-concept-loops--while-and-for)
11. [Concept: Integer Overflow](#11-concept-integer-overflow)
12. [Concept: Wrapping and Saturating Arithmetic](#12-concept-wrapping-and-saturating-arithmetic)
13. [Concept: Type Casting with `as`](#13-concept-type-casting-with-as)
14. [Putting It All Together — The Complete Calculator](#14-putting-it-all-together--the-complete-calculator)
15. [Exercises to Try](#15-exercises-to-try)
16. [Summary](#16-summary)

---

## 1. Project Overview

We'll build a **command-line calculator** that can:
- Add, subtract, multiply, and divide integers
- Compute factorials
- Detect and handle overflow gracefully

### What You'll Learn

| Rust Concept | Why It Matters for Data Engineering |
|---|---|
| Integer types (`u32`, `i32`, `i64`) | Choosing the right type for counts, IDs, timestamps |
| `let`, `let mut` | Controlling mutability for safety |
| `if`/`else` expressions | Branching logic in data pipelines |
| `while` and `for` loops | Iterating over data |
| `panic!` | Error handling basics |
| Integer overflow | Avoiding silent data corruption |
| `wrapping_` / `saturating_` | Safe arithmetic in production |
| `as` casting | Converting between types |

---

## 2. Prerequisites

- Rust installed (see [Project 0: Intro](../0-Intro/workshop.md))
- Basic Python knowledge
- Familiarity with `cargo new` and `cargo run`

---

## 3. Running the Python Version

The Python version (`project.py`) demonstrates what our Rust calculator will do:

```bash
cd rustlings-workshop/2-BasicCalculator
python project.py
```

> **Note:** If `project.py` doesn't exist yet, the workshop will guide you through writing the Rust version from scratch.

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

## 5. Concept: Variables and Mutability

### `let` — Immutable by Default

```rust
let x = 5;
x = 6;  // ❌ Compile error: "cannot assign twice to immutable variable"
```

### `let mut` — Opt In to Mutation

```rust
let mut x = 5;
x = 6;  // ✅ Allowed because x is mutable
```

### Why Does This Matter for Data Engineering?

In data pipelines, **shared mutable state** is the #1 cause of bugs. Rust forces you to be explicit about what can change, making your code easier to reason about.

```python
# Python — everything can be mutated anywhere
def clean_data(df):
    df["value"] = df["value"].fillna(0)  # Mutates the original!
```

```rust
// Rust — you control mutability
fn clean_data(df: &mut DataFrame) {   // &mut = explicit permission to mutate
    // ... do the cleaning
}
```

### Shadowing

Rust lets you reuse variable names by **shadowing**:

```rust
let x = 5;
let x = x + 1;    // Shadow — creates new variable
let x = x * 2;    // Shadow again
// x = 12
```

Shadowing is **not** mutation — it creates a new variable with the same name. You can even change the type:

```rust
let data = "42";       // data: &str
let data: i32 = data.parse().unwrap();  // data: i32
// data is now an integer!
```

### Python vs Rust — Variable Rules

| Operation | Python | Rust |
|---|---|---|
| Create variable | `x = 5` | `let x = 5;` |
| Make mutable | Always mutable | `let mut x = 5;` |
| Reassign | `x = 6` | `x = 6` (only if `mut`) |
| Reassign with type change | `x = "hello"` | Not allowed unless shadowed |
| Change type via shadow | N/A | `let x = x.to_string();` |

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

## 7. Concept: Control Flow — `if`/`else`

### Basic `if`/`else`

```rust
let number = 7;

if number < 10 {
    println!("Small number");
} else {
    println!("Big number");
}
```

### `else if` Chains

```rust
if number < 5 {
    println!("Very small");
} else if number < 10 {
    println!("Small");
} else {
    println!("Big");
}
```

### `if` is an Expression in Rust (Key Difference!)

**Python:** `if` is a **statement** — it doesn't produce a value:

```python
# Python — this is a statement
if x > 0:
    result = "positive"
else:
    result = "non-positive"
```

**Rust:** `if` is an **expression** — it produces a value:

```rust
// Rust — this is an expression
let result = if x > 0 { "positive" } else { "non-positive" };
```

### Expression vs Statement — Memory Diagram

```
Python:                        Rust:
┌────────────────┐           ┌──────────────────────┐
│ x = 5          │◄──stmt    │ let x: i32 = 5;      │◄──stmt
│ if x > 0:      │◄──stmt    │ let label = if x > 0 {│◄──expr returns value
│   y = "pos"    │           │     "positive"         │
│ else:          │           │ } else {              │
│   y = "neg"    │           │     "negative"        │
└────────────────┘           │ };                    │
                              └──────────────────────┘
```

### Exercise: Categorize Temperature

```rust
fn categorize_temp(temp: i32) -> &'static str {
    // Return "hot" if temp > 30, "cold" if temp < 10, "mild" otherwise
    if temp > 30 {
        "hot"
    } else if temp < 10 {
        "cold"
    } else {
        "mild"
    }
}

fn main() {
    println!("{}", categorize_temp(35));  // hot
    println!("{}", categorize_temp(5));   // cold
    println!("{}", categorize_temp(20));  // mild
}
```

---

## 8. Concept: No Truthy/Falsy — the `bool` Type

### Python Truthy/Falsy

```python
# Python — many types are "truthy" or "falsy"
if "":       # False (empty string)
if []:       # False (empty list)
if 0:        # False (zero)
if None:     # False
if [1, 2]:   # True (non-empty list)
```

### Rust Only Accepts `bool`

```rust
let x = 5;
// if x { }  // ❌ ERROR: expected `bool`, found integer
if x > 0 { }  // ✅ Must be a boolean condition
if true { }   // ✅ Literal bool
if false { }  // ✅
```

### Why This Matters

In data engineering pipelines, truthy/falsy bugs are common:

```python
# Python — subtle bug
count = 0
if count:        # False! Skips processing even though 0 is valid
    process(count)
```

```rust
// Rust — explicit, no ambiguity
let count: u32 = 0;
if count > 0 {    // Must write the explicit condition
    process(count);
}
if count != 0 {   // Another option
    process(count);
}
```

### Comparisons Produce `bool`

```rust
let a = 5;
let b = 10;
let is_greater: bool = a > b;  // false
let is_equal: bool = a == b;   // false
let is_not_equal: bool = a != b; // true
```

| Operator | Meaning | Python |
|---|---|---|
| `==` | Equal | `==` |
| `!=` | Not equal | `!=` |
| `<` | Less than | `<` |
| `>` | Greater than | `>` |
| `<=` | Less or equal | `<=` |
| `>=` | Greater or equal | `>=` |
| `&&` | Logical AND | `and` |
| `\|\|` | Logical OR | `or` |
| `!` | Logical NOT | `not` |

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
| `dev` (default for `cargo run`) | `true` | **Panics** on overflow |
| `release` (`cargo run --release`) | `false` | **Wraps** silently |

```bash
cargo run              # Panics on overflow
cargo run --release    # Wraps silently (256 → 0 for u8)
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

---

## 14. Putting It All Together — The Complete Calculator

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
cargo new 2-BasicCalculator
cd 2-BasicCalculator
# Replace src/main.rs with the code above
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

## 15. Exercises to Try

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

## 16. Summary

| Concept | Description | Python Equivalent |
|---|---|---|
| Integer types | `u8`, `i32`, `u64`, etc. — choose precision | Single `int` type |
| `mut` | Opt-in mutability | Everything is mutable |
| `if` as expression | `if` returns a value | `if` is a statement |
| No truthy/falsy | Conditions must be `bool` | Any value can be truthy/falsy |
| `panic!` | Unrecoverable error | `raise Exception` |
| `while` loop | Loop with condition | `while` |
| `for` + range | Loop over range | `for i in range(n)` |
| Overflow | Result exceeds type's max | Not possible (arbitrary precision) |
| `saturating_*` | Clamp at max/min | N/A |
| `wrapping_*` | Wrap around on overflow | N/A |
| `as` casting | Explicit type conversion | Implicit conversion |

### Key Takeaways for Data Engineers

1. **Choose your integer types carefully** — `u32` for counts, `i64` for timestamps, `u64` for large datasets
2. **Enable overflow checks in release** — silent data corruption is worse than a crash
3. **Use saturating arithmetic** for production data pipelines where overflow is possible
4. **Rust's strict type system** prevents an entire class of bugs — lean into it
5. **`if` as expression** enables clean, functional-style code

### Next Project

Proceed to [1-MasterMind](../1-MasterMind/master_mind.md) to solidify these concepts with a game, then [3-TicketV1](../3-TicketV1/workshop.md) to learn about **ownership** — Rust's most unique and important feature.
