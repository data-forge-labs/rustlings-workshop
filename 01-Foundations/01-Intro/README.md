# 🦀 Rust for Python Data Engineers — Rust Reference

> **Test-driven approach**: This project includes a Cargo workspace with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you work through each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. The `workshop/src/main.rs` file provides a runnable demo (calling the same functions) — use `cargo run` to see your code in action. Your goal: **all 26 tests pass**.

## What Is This Reference?

A concise Rust syntax reference for Python data engineers. This project covers the fundamentals — variables, functions, control flow, tuples, and arrays — so you can read and write simple Rust programs.

### Python equivalent

```python
# Python basics — what you already know
x = 5
PI = 3.14159

def add(a: int, b: int) -> int:
    return a + b

if x > 0:
    print(f"x is {x}")

for i in range(5):
    print(i)

point = (3, 4.5)
primes = [2, 3, 5, 7]
```

In this project you'll learn to write this in Rust — and along the way
you'll discover **immutable by default**, **type inference**, **expression-based control flow**, and **fixed-size arrays**.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Variable binding | Immutable by default — opt-in mutability |
| 2 | Type inference | Compiler infers most types; explicit when needed |
| 3 | Constants | Compile-time, inlined |
| 4 | Functions | Types in signature, expression-return |
| 5 | `println!` macro | Type-checked format string |
| 6 | Tuples & arrays | Fixed-size, mixed types, stack-allocated |
| 7 | Control flow | `if` is an expression — returns a value |
| 8 | Loops | `loop`, `while`, `for` — `loop` has no Python equivalent |
| 9 | Pattern match | Exhaustive — compiler enforces all cases |
| 10 | `char` type | 4-byte Unicode scalar value |

---

## Table of Contents

1. [Why Rust for Data Engineering?](#1-why-rust-for-data-engineering)
2. [Installing Rust](#2-installing-rust)
3. [Your First Rust Program](#3-your-first-rust-program)
4. [Syntax Side-by-Side](#4-syntax-side-by-side)
5. [Functions](#5-functions)
6. [Variables and Mutability](#6-variables-and-mutability)
7. [If/Else — Making Decisions](#7-ifelse-making-decisions)
8. [Loops — Repeating Work](#8-loops-repeating-work)
9. [Tuples — Grouping Values](#9-tuples-grouping-values)
10. [Arrays — Fixed-Size Sequences](#10-arrays-fixed-size-sequences)
11. [Putting It All Together](#11-putting-it-all-together)
12. [Cargo Commands](#12-cargo-commands)
13. [Summary](#13-summary)
14. [What's Next](#14-whats-next)

---

## 1. Why Rust for Data Engineering?

| Pain Point | Python | Rust |
|---|---|---|
| **Speed** | Interpreted, can be 10–100x slower than C | Compiled, as fast as C/C++ with zero-cost abstractions |
| **Memory usage** | High per-object overhead, GC pauses | No garbage collector, predictable and lean memory use |
| **Parallelism** | GIL limits true thread parallelism | No GIL — concurrency is safe by design |
| **Deployment** | Requires Python runtime + all dependencies | Ships as a single static binary, no runtime needed |
| **Error detection** | Most bugs surface at runtime | Type errors, memory errors, and null pointer bugs caught at compile time |

### Where Rust Shines for Data Engineering

Not every tool in your stack needs to be rewritten in Rust — but some parts benefit enormously:

- **High-throughput data pipelines** — process millions of rows without GC pauses interrupting throughput
- **CLI tools for data processing** — single binary, instant startup, trivial to distribute
- **Embedded/sidecar services** — tight, predictable memory footprint alongside heavier services
- **Python extension modules** — keep your Python interface, move the hot path to Rust via [PyO3](https://pyo3.rs)
- **Streaming data** — real-time processing with deterministic, low latency

### The Trade-off

Rust earns its reputation for being strict. The compiler will push back on code that other languages would silently accept. That strictness isn't arbitrary — it's the mechanism behind memory safety without a garbage collector, data races eliminated at compile time, and abstractions that cost nothing at runtime.

The good news: **this first project only covers the gentle basics**. By the end of it you'll have written real Rust code, run a program, and passed 26 tests — all mapped to Python equivalents. The strict parts (ownership, borrowing, lifetimes) come in the next section, where they get your full attention.

---

## 2. Installing Rust

### Step 1: Install `rustup`

`rustup` is Rust's toolchain manager — it installs the compiler, `cargo`, and keeps everything up to date.

```bash
# macOS / Linux / WSL:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows (native):
# Download the installer from https://rustup.rs
```

### Step 2: Verify the installation

```bash
rustc --version   # Rust compiler
cargo --version   # Build tool and package manager
```

### Step 3: Create your first project

```bash
cargo new hello_data
cd hello_data
```

Cargo creates this layout:

```text
hello_data/
  Cargo.toml    # Package manifest — dependencies, metadata
  src/
    main.rs     # Your code lives here
```

`Cargo.toml` plays the same role as `pyproject.toml` + `requirements.txt` combined. You declare your dependencies there and Cargo handles the rest — no separate `pip install` step.

---

## 3. Your First Rust Program

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

```text
Hello, data engineers!
```

What each part means:

| Part | Meaning | Python equivalent |
|---|---|---|
| `fn main()` | Program entry point | `if __name__ == "__main__":` |
| `println!` | Print to stdout (note the `!` — it's a macro) | `print()` |
| `"..."` | String literal | `"..."` |
| `;` | Statement terminator | Not required in Python, required in Rust |

> **What's a macro?** The `!` after `println` marks it as a macro rather than a function. Macros can do things regular functions can't — like accepting a variable number of arguments with format strings. You'll see `!` on `vec![]`, `panic!`, `assert_eq!`, and others throughout this course. For now, just treat `println!` the same way you'd treat `print()`.

### `main.rs` vs `lib.rs`

The project has two source files:

- **`src/lib.rs`** — contains the public functions (`todo!()` stubs) and all unit tests. This is where you'll do your work.
- **`src/main.rs`** — a runnable demo that calls the functions from `lib.rs`. It's not tested directly; it just shows your code working end-to-end.

```bash
cargo test           # test the lib.rs functions
cargo run            # run the main.rs demo
```

---

## 4. Syntax Side-by-Side

A quick reference you can come back to. **Only the basics for now** — the advanced rows (collections, error types, references) are covered in later projects.

| Feature | Python | Rust |
|---|---|---|
| Comment | `# comment` | `// comment` |
| Function | `def add(x, y):` | `fn add(x: i32, y: i32) -> i32` |
| Variable | `x = 5` | `let x = 5;` |
| Mutable variable | `x = 5` (always mutable) | `let mut x = 5;` |
| Constant | `MAX = 100` (convention only) | `const MAX: u32 = 100;` |
| String | `"hello"` | `"hello"` (type `&str`) |
| Print | `print("hi")` | `println!("hi")` |
| Integer | `42` (one big `int`) | `42` (type `i32` by default) |
| Float | `3.14` (one big `float`) | `3.14` (type `f64` by default) |
| Boolean | `True` / `False` | `true` / `false` |
| If | `if x > 0:` | `if x > 0 { }` |
| For loop | `for i in range(5):` | `for i in 0..5 { }` |
| While | `while x > 0:` | `while x > 0 { }` |
| Tuple | `(1, "hi")` | `(1, "hi")` |
| Fixed array | `[1, 2, 3]` (size flexible) | `[1, 2, 3]` (size is part of the type) |
| Package file | `pyproject.toml` | `Cargo.toml` |

> **Coming up in later projects:** `Vec<T>` (growable list), `HashMap` (dict), `&[T]` (slice), `Option<T>` (nullable), `Result<T, E>` (errors), `panic!`, `std::io`. Don't worry about any of these yet.

The biggest practical difference: **Rust requires explicit types on function parameters and return values**. The compiler uses those types to catch mistakes before your code ever runs.

```python
# Python — types are optional hints
def is_hot(value, threshold):
    return value > threshold
```

```rust
// Rust — types are required and enforced
fn is_hot(value: f64, threshold: f64) -> bool {
    value > threshold
}
```

---

## 5. Functions

### Basic syntax

```rust
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // body
}
```

### Converting a Python function to Rust

```python
# Python
def celsius_to_fahrenheit(c):
    return (c * 9.0 / 5.0) + 32.0
```

```rust
// Rust — same logic, explicit types
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0  // no semicolon = this value is returned
}
```

That last line without a semicolon is the return value. The rule: **an expression without a semicolon at the end of a function body is what gets returned**. You can also write `return` explicitly, but it's less common in idiomatic Rust:

```rust
fn celsius_to_fahrenheit(c: f64) -> f64 {
    return (c * 9.0 / 5.0) + 32.0;  // also valid, just less idiomatic
}
```

### Functions that don't return a value

```rust
fn log_message(msg: &str) {   // no -> means returns ()
    println!("LOG: {}", msg);
}
```

Rust's `()` (the "unit type") is the equivalent of Python's `None` return — it means "nothing useful is returned."

### Your turn

Try writing `fahrenheit_to_celsius` before looking at the solution. The formula is $(f - 32) \times \frac{5}{9}$.

```python
# Python
def fahrenheit_to_celsius(f):
    return (f - 32) * 5.0 / 9.0

print(fahrenheit_to_celsius(212))  # 100.0
```

<details>
<summary>Solution</summary>

```rust
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("{}", fahrenheit_to_celsius(212.0));  // 100
}
```

</details>

Run it: `cargo run`

---

## 6. Variables and Mutability

### Immutable by default

```rust
let x = 5;
x = 6;       // ❌ compiler error: cannot assign twice to immutable variable
```

```rust
let mut y = 5;
y = 6;       // ✅ fine — y is declared mutable
```

In Python, all variables are mutable. In Rust, you opt in to mutability with `mut`. This makes intent explicit: when you see `let` without `mut`, the value never changes. It also helps the compiler catch accidental reassignment and enables better optimizations.

For data pipelines this matters — a value you didn't intend to mutate staying immutable is a guarantee, not a convention.

### Type annotations

You can let the compiler infer the type, or be explicit:

```rust
let count = 5;             // i32 by default
let count: i32 = 5;        // explicit
let ratio = 0.85;          // f64 by default
let ratio: f64 = 0.85;     // explicit
let name = "Alice";        // &str
let name: &str = "Alice";  // explicit
```

The same syntax `name: type` shows up in function parameters and return types:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Shadowing

You can redeclare a variable with `let`, creating a new binding under the same name:

```rust
let x = 5;
let x = x + 1;   // new binding, value is 6
```

Unlike `mut`, shadowing can change the type:

```rust
let value = "42";          // &str
let value = value.len();   // usize — type changed, no problem
```

This is handy when transforming a value through multiple steps and you don't want to invent new names each time. It's common in parsing pipelines.

### Constants

```rust
const MAX_ROWS: u32 = 10_000;   // type is required
const PI: f64 = 3.14159265359;
```

- Always `const`, never `let const`
- Type annotation is required
- Convention: `SCREAMING_SNAKE_CASE`
- Evaluated at compile time — good for configuration values you want to be hard constants, not runtime variables

---

## 7. If/Else — Making Decisions

### Basic syntax

```rust
if condition {
    // ...
} else if other_condition {
    // ...
} else {
    // ...
}
```

```python
# Python
if x > 0:
    print("positive")
elif x == 0:
    print("zero")
else:
    print("negative")
```

```rust
// Rust — braces instead of colons, no indentation rules
if x > 0 {
    println!("positive");
} else if x == 0 {
    println!("zero");
} else {
    println!("negative");
}
```

### Booleans only — no truthy/falsy

In Python, `if 0:` and `if []:` are valid (everything is "truthy" or "falsy"). In Rust, **conditions must be `bool`**. A bare `0` won't compile:

```rust
if 0 { }       // ❌ ERROR: expected `bool`, found integer
if x != 0 { }  // ✅ fine
```

This is one of the small adjustments when moving from Python — you can't accidentally test an empty list as "false."

### `if` as an expression

Here's the unique Rust part: `if` *returns a value*, just like Python's ternary `a if cond else b`:

```python
# Python ternary
result = "positive" if x > 0 else "non-positive"
```

```rust
// Rust — if/else returns a value directly
let result = if x > 0 { "positive" } else { "non-positive" };
```

Both branches must return the same type. The compiler will catch a mismatch.

### Example: classify a temperature

```rust
/// Return "cold" if temp < 10, "mild" if 10..30, "hot" if >= 30.
fn classify_temp(temp: i32) -> &'static str {
    if temp < 10 {
        "cold"
    } else if temp < 30 {
        "mild"
    } else {
        "hot"
    }
}
```

The whole `if/else if/else` is an expression, and its value is the last expression in the chosen branch. This is the same pattern you'll use throughout the rest of the course.

### A quick word on `&'static str`

The return type `&'static str` looks unusual — here's what each part means:

| Part | What it does | Python equivalent |
|------|--------------|-------------------|
| `&` | "I'm just looking at this, not making a copy" | Passing a string to a function — Python doesn't copy it either |
| `'static` | "This reference is valid for the entire program" | Any variable in Python — Python keeps everything alive as long as someone references it |
| `str` | A chunk of UTF-8 text | `str` in Python |

**Why `'static`?** Because the function hands back one of `"cold"`, `"mild"`, or `"hot"` — they're literally part of your compiled `.exe` file. They sit in the binary's read-only memory. Rust needs to know: "how long will this pointer be valid?" The answer here is "for the whole program" (these strings never get freed). So the full type says: *a pointer (`&`) to text (`str`) that's valid forever (`'static`)*.

**What if we just wrote `&str`?** Rust would complain: "you're returning a borrowed string, but I don't know how long it lives." The `'static` keyword is the answer — it's like telling Rust "don't worry, this data isn't going anywhere."

**The `&` symbol** — This comes from C's "address-of" operator. But Rust's `&` is safer: it can never be null, never point to freed memory, and the compiler tracks who's using it so you can't accidentally mess things up.

### Exercise

```rust
fn is_adult(age: i32) -> &'static str {
    // Return "adult" if age >= 18, else "minor"
    todo!()
}

fn main() {
    println!("{}", is_adult(25));  // adult
    println!("{}", is_adult(15));  // minor
}
```

<details>
<summary>Solution</summary>

```rust
fn is_adult(age: i32) -> &'static str {
    if age >= 18 { "adult" } else { "minor" }
}
```

</details>

---

## 8. Loops — Repeating Work

### `for` with a range

```python
# Python
for i in range(5):
    print(i)
```

```rust
// Rust — 0..5 is "0 up to 5, not including"
for i in 0..5 {
    println!("{i}");
}
// prints 0, 1, 2, 3, 4
```

Two range forms you'll use:

| Rust | Meaning | Python equivalent |
|---|---|---|
| `0..5` | 0, 1, 2, 3, 4 (exclusive end) | `range(5)` |
| `0..=5` | 0, 1, 2, 3, 4, 5 (inclusive end) | `range(6)` |

### `while`

```python
# Python
n = 3
while n > 0:
    print(n)
    n -= 1
```

```rust
// Rust
let mut n = 3;
while n > 0 {
    println!("{n}");
    n -= 1;
}
```

### Example: count positive values in a fixed array

```rust
fn count_positive(values: [i32; 5]) -> usize {
    let mut count = 0;
    for v in values {
        if v > 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    let readings = [10, -3, 25, 0, 7];
    println!("Positive readings: {}", count_positive(readings));  // 3
}
```

Three Rust-specific bits:

- `let mut count` — we *opt in* to mutability so the compiler can track changes
- The last line `count` (no semicolon) is the return value
- The return type `usize` — Rust's standard type for counts, lengths, and indices

### A quick word on `usize`

The return type `usize` looks odd at first — here's what it means:

| Part | What it does | Python equivalent |
|------|--------------|-------------------|
| `u` | "unsigned" — no negative values, range is `0` to `2ⁿ − 1` | Python's `int` is signed but arbitrary precision |
| `size` | Width matches the platform: 64 bits on a 64-bit system, 32 bits on 32-bit | N/A — Python ints are unbounded |
| (combined) | An unsigned integer that's always big enough to count anything in memory | N/A |

**Why `usize` for a count?** Because `count` represents a *quantity*, not a *measurement*. It can never be negative, and the moment you compare it to something like `readings.len()`, both sides need to match — `readings.len()` returns `usize`, and so do `vec.len()`, `string.len()`, and `hashmap.len()`. Using `usize` for counts keeps the math with those calls clean.

**Why not `i32`?** Signed would be wasteful (you'll never have a negative count) and would force you to cast whenever you compare with `len()`.

**Why not `u32`?** Technically works, but Rust's convention is: if it's a size, count, length, or array index, use `usize` — it auto-scales to the platform and matches every standard-library length method.

**Where you'll see `usize`:**
- `arr.len()` returns `usize`
- Indexing `arr[i]` requires `i: usize`
- `0..n` ranges produce `usize` when `n` is `usize`

In short: think of `usize` as Rust's "non-negative count" type. The compiler enforces the unsigned-ness so you can never accidentally produce a negative length.

### Exercise

```rust
/// Sum all the values in a fixed array of 5 i32s.
fn sum_five(values: [i32; 5]) -> i32 {
    todo!()
}

fn main() {
    println!("{}", sum_five([10, 20, 30, 40, 50]));  // 150
}
```

<details>
<summary>Solution</summary>

```rust
fn sum_five(values: [i32; 5]) -> i32 {
    let mut total = 0;
    for v in values {
        total += v;
    }
    total
}
```

</details>

---

## 9. Tuples — Grouping Values

A **tuple** is a fixed-size group of values, each with its own type. Python has tuples too; Rust's are similar but type-strict.

```python
# Python — heterogeneous types, no type enforcement
point = (3, 4.5, "origin")
name, age = ("Alice", 30)
```

```rust
// Rust — each position has a fixed type
let point: (i32, f64, &str) = (3, 4.5, "origin");
let (name, age) = ("Alice", 30);
```

### Why tuples matter for data engineering

Tuples are the simplest way to return **multiple values** from a function — a pattern you'll hit constantly in data work:

```rust
/// Returns (label, length) for a piece of text.
fn describe(text: &str) -> (&str, usize) {
    let label = if text.is_empty() { "empty" } else { "non-empty" };
    (label, text.len())
}

let (l, n) = describe("hello");
println!("{l}, {n} chars");  // non-empty, 5 chars
```

### Tuple syntax

| Operation | Syntax | Example |
|---|---|---|
| Create | `(T1, T2, ...)` | `let t: (i32, &str) = (1, "x");` |
| Access by index | `t.0`, `t.1`, ... | `let n = t.0;` |
| Destructure | `let (a, b) = t;` | `let (x, y) = (1.0, 2.0);` |
| Empty / unit | `()` | The "no return value" type |
| Single-element | `(T,)` | Note the trailing comma — `(5)` is just `5` |

### Destructuring

Destructuring assigns each tuple element to a separate variable. It works in `let`, in function parameters, and (later) in `match` arms:

```rust
let (status, code) = ("ok", 200);

// In a function parameter
fn print_point((x, y): (i32, i32)) {
    println!("({}, {})", x, y);
}
print_point((10, 20));
```

### Example: classify a data row

```rust
/// Classify a row given as (id, value, is_valid).
/// Return "ok" if is_valid && value > 0, "invalid" if !is_valid, "zero" otherwise.
fn categorize_row(row: (u32, f64, bool)) -> &'static str {
    let (_id, value, is_valid) = row;
    if !is_valid {
        "invalid"
    } else if value > 0.0 {
        "ok"
    } else {
        "zero"
    }
}

fn main() {
    println!("{}", categorize_row((1, 5.0, true)));    // ok
    println!("{}", categorize_row((2, 0.0, true)));    // zero
    println!("{}", categorize_row((3, 5.0, false)));   // invalid
}
```

Notice the destructured local — it makes the function read like Python's `def categorize_row(id, value, is_valid)`.

---

## 10. Arrays — Fixed-Size Sequences

An **array** in Rust is a fixed-size sequence of values **all of the same type**. The size is part of the type, written as `[T; N]` where `N` is the count.

```python
# Python
primes = [2, 3, 5, 7]
zeros = [0] * 1024
```

```rust
// Rust
let primes: [i32; 4] = [2, 3, 5, 7];
let zeros: [i32; 1024] = [0; 1024];   // 1024 zeros
```

### Accessing elements

```rust
let primes = [2, 3, 5, 7];
let first = primes[0];   // 2
let len = primes.len();  // 4
```

The size `N` is part of the type — `[i32; 4]` and `[i32; 5]` are different types. Trying to grow an array is a compile error:

```rust
let mut a: [i32; 3] = [1, 2, 3];
// a.push(4);   // ❌ ERROR — arrays don't have methods
```

For growable sequences, you need `Vec<T>` — that's covered in [04-MasterMind](../04-MasterMind/README.md).

### Why fixed-size arrays matter for data engineering

Many real-world data records are fixed-shape: a sensor reading is always (timestamp, value, unit); a CSV row is always the same number of fields. Fixed arrays are perfect for that — no allocation, no growth, no surprises.

### Arrays vs tuples

| Need | Use | Example |
|---|---|---|
| Multiple values of different types, no field names | **tuple** | `(i32, &str, f64)` |
| Many values of the same type, fixed size | **array** `[T; N]` | `[1, 2, 3, 4, 5]` |
| Many values of the same type, growable | **Vec** `<T>` | (next project) |
| Multiple values of different types, with field names | **struct** | (next project) |

### Exercise

```rust
/// Return the largest value in a fixed array of 5 i32s.
fn max_of_five(values: [i32; 5]) -> i32 {
    todo!()
}

fn main() {
    println!("{}", max_of_five([3, 1, 4, 1, 5]));   // 5
    println!("{}", max_of_five([-2, -8, -1, -9]));  // -1
}
```

<details>
<summary>Solution</summary>

```rust
fn max_of_five(values: [i32; 5]) -> i32 {
    let mut max = values[0];
    for i in 1..5 {
        if values[i] > max {
            max = values[i];
        }
    }
    max
}
```

</details>

---

## 10.5. A Quick Note on `char`

You saw `char` in the type table earlier. Here's the minimum you need to know for this course:

```rust
let letter: char = 'A';
let emoji: char = '🦀';
let digit: char = '7';
```

Three differences from Python:

| Aspect | Python `str[0]` | Rust `char` |
|---|---|---|
| Size | Variable (1–4 bytes per char) | **Always 4 bytes** (a full Unicode scalar value) |
| Quote | `"A"` (double quotes) | `'A'` (single quotes) |
| What it holds | A substring | Exactly one Unicode codepoint |

You'll use `char` mostly when iterating over text in later projects:

```rust
for ch in "hello".chars() {       // ch: char
    println!("{}", ch);
}
```

For now, treat `char` as a 4-byte Unicode scalar. The deeper UTF-8 story (why `String` is byte-indexed, not char-indexed) is covered in the **§4 "Common pitfalls"** of [04-MasterMind](../04-MasterMind/README.md#4-concept-string-vs-str-deeper-dive).

---

## 11. Putting It All Together

Here's a small data processing function that uses everything covered so far — a fixed-size array, a `for` loop, an `if` expression, and a tuple return value:

```rust
/// Count how many readings in a 5-element array are "hot" (>= 30).
/// Returns (count, label) where label is "few", "some", or "many".
fn hot_readings_summary(readings: [i32; 5]) -> (usize, &'static str) {
    let mut count = 0;
    for v in readings {
        if v >= 30 {
            count += 1;
        }
    }
    let label = if count == 0 {
        "few"
    } else if count <= 2 {
        "some"
    } else {
        "many"
    };
    (count, label)
}

fn main() {
    let monday = [22, 28, 31, 35, 30];
    let (n, l) = hot_readings_summary(monday);
    println!("Hot readings: {n} → {l}");  // Hot readings: 3 → many
}
```

Expected output:

```text
Hot readings: 3 → many
```

What each concept is doing here:

| Concept | Where it appears |
|---|---|
| `fn` | `hot_readings_summary()` and `main()` |
| `let mut` | `count` — needs to be reassigned |
| `let` (immutable) | `monday`, `n`, `l` |
| `for` loop | Iterating over the readings |
| `if` as expression | Counting hot values and the `label` |
| `[i32; 5]` | Fixed-size array of readings |
| Tuple return | `(count, label)` |
| Tuple destructuring | `let (n, l) = hot_readings_summary(monday);` |
| `println!` | Formatted output |

---

## 12. Cargo Commands

Cargo is Rust's all-in-one tool: package manager, build system, test runner, formatter, linter. Think `pip` + `pytest` + `black` + `make` unified into one command.

### Commands you'll use every day

```bash
cargo new my_project   # create a new project
cargo check            # check that code compiles (fast, no binary produced)
cargo build            # compile the project
cargo run              # compile and run
cargo test             # run all tests
cargo fmt              # format code
cargo clippy           # lint
```

### The development loop

```bash
cd workshop
cargo check            # quick syntax check while writing
cargo test             # see how many tests pass
# edit src/lib.rs, replace todo!() with real code
cargo test             # more tests should pass now
```

> `cargo check` is faster than `cargo build` because it skips the linking step. Use it while you're actively writing to get instant compiler feedback without waiting for a full build.

### Quick reference

| Task | Command |
|---|---|
| Build | `cargo build` |
| Check (no binary) | `cargo check` |
| Run | `cargo run` |
| Run all tests | `cargo test` |
| Run one test | `cargo test test_name` |
| Format | `cargo fmt` |
| Lint | `cargo clippy` |
| Open docs | `cargo doc --open` |

> **Adding external dependencies** (`cargo add crate_name`) is covered in a later project, when you actually need one. For this intro, the standard library is more than enough.

---

## 13. Summary

| Concept | Rust | Python equivalent |
|---|---|---|
| Define a function | `fn name(x: T) -> R { }` | `def name(x):` |
| Mutable variable | `let mut x = 5;` | `x = 5` |
| Immutable variable | `let x = 5;` | N/A (Python variables are always mutable) |
| Constant | `const MAX: u32 = 100;` | `MAX = 100` (convention only) |
| Return type | `-> f64` | Type hints, unenforced |
| Implicit return | Last expression, no `;` | `return` always required |
| Print | `println!("val = {}", x)` | `print(f"val = {x}")` |
| If/else | `if cond { } else { }` | `if cond:` |
| If as expression | `let r = if cond { a } else { b };` | `r = a if cond else b` |
| For range | `for i in 0..5 { }` | `for i in range(5):` |
| While | `while cond { }` | `while cond:` |
| Tuple | `(T1, T2, ...)` | `(1, "x")` |
| Tuple destructuring | `let (a, b) = t;` | `a, b = t` |
| Array (fixed) | `[T; N]`, e.g. `[1, 2, 3]` | `list` (size not enforced) |
| Boolean | `true` / `false` | `True` / `False` |
| Integer | `42` (i32 by default) | `42` (arbitrary precision) |
| Float | `3.14` (f64 by default) | `3.14` |

---

## 14. What's Next

You now know the Rust basics: variables, functions, control flow, tuples, and fixed-size arrays. That's enough to read and write simple Rust programs.

The next project, [02-GuessGame](../02-GuessGame/README.md), builds your **first interactive Rust program** — the classic "Guess the Number" game. It introduces six new concepts the intro project deliberately skipped:

- **`String` vs `&str`** — owned growable text vs borrowed views
- **Custom `enum`** with `#[derive(Debug, PartialEq, Eq, Copy, Clone)]`
- **`std::io::stdin().read_line(&mut buf)`** for console input
- **`Result<T, E>`**, `.parse()`, and `.expect("msg")` for fallible operations
- **Basic `match`** on `Result` and on a custom enum
- **Adding an external crate** (`rand`) via `Cargo.toml`

After that, [03-BasicCalculator](../03-BasicCalculator/README.md) takes you deeper into **integer-specific Rust**:

- Integer types: `i32` vs `u32` vs `i64` vs `usize` (Python only has one int)
- Integer overflow and the `panic!` macro
- `while` and `for` loops in practice
- The `as` keyword for type conversion
- Built-in unit testing with `#[test]` and `#[should_panic]`

Then [04-MasterMind](../04-MasterMind/README.md) introduces **structs**, **`Vec`**, **`Option`**, and **exhaustive `match`** — moving you from "tutorial snippets" to a real game project with multiple files.

Topics that come even later:

- **Ownership and borrowing** ([Section 02: Ownership](../../02-Ownership/README.md)) — Rust's central idea, including the `&` reference and `&[T]` slice
- **Error handling** with `Result<T, E>` and the `?` operator (deep dive)
- **Collections** ([Section 03: Collections](../../03-Collections/README.md)) — `Vec`, `HashMap`, `HashSet`, iterators
- **File I/O** ([Section 04: File I/O](../../04-FileIO/README.md)) — reading CSVs and Parquet
- **Concurrency** ([Section 05: Concurrency](../../05-Concurrency/README.md)) — threads, async, channels

Don't worry about any of these yet. Make sure all **26 tests pass** in `workshop/`, then move on to [02-GuessGame](../02-GuessGame/README.md).

---

## Related Projects

After this introduction, continue your Rust journey with these hands-on workshops:

- [02-GuessGame](../02-GuessGame/README.md) — `String` vs `&str`, custom enums, console I/O, `Result`, external crates
- [03-BasicCalculator](../03-BasicCalculator/README.md) — integers, branching, loops
- [04-MasterMind](../04-MasterMind/README.md) — `struct`, `Vec`, `Option`, console I/O

For deeper exploration of the foundations covered here:

- [02-Ownership/01-TicketV1](../../02-Ownership/01-TicketV1/README.md) — ownership, stack vs heap (the next big concept after syntax)
- [02-Ownership/02-Traits](../../02-Ownership/02-Traits/README.md) — traits, `derive`, bounds
- [03-Collections/01-TicketManagement](../../03-Collections/01-TicketManagement/README.md) — `Vec`, `HashMap`, iterators (where collections begin)
- [Pattern Matching: @ Bindings and Guards](../README.md#pattern-matching--bindings-and-guards) — advanced `match` patterns (`@` bindings and guards)

---

*Next up - Project 1.2: Guess the Number Game. You'll learn `String` vs `&str`, custom enums, console I/O, `Result`, and external crates by building a real interactive game.*

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

