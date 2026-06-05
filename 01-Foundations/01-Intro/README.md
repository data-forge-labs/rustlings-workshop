# Rust for Python Data Engineers

*A beginner-friendly introduction to Rust, designed for data engineers who already know Python.*

If you've written Python pipelines long enough, you've hit the familiar walls: a job that's too slow to scale, a service eating memory under load, or a bug that only shows up in production at 2 AM. Rust was built to eliminate exactly these problems — and this tutorial is built to make Rust approachable for people who already think in Python.

You don't need any systems programming background. Every concept is introduced alongside its Python equivalent.

> **Test-driven approach**: This project includes a Cargo workspace with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you work through each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. The `workshop/src/main.rs` file provides a runnable demo (calling the same functions) — use `cargo run` to see your code in action. Your goal: **all 31 tests pass**.

---

## Table of Contents

1. [Why Rust for Data Engineering?](#1-why-rust-for-data-engineering)
2. [Installing Rust](#2-installing-rust)
3. [Your First Rust Program](#3-your-first-rust-program)
4. [Syntax Side-by-Side](#4-syntax-side-by-side)
5. [Functions](#5-functions)
6. [Variables and Mutability](#6-variables-and-mutability)
7. [Expressions vs Statements](#7-expressions-vs-statements)
8. [Tuples — Grouping Values of Different Types](#8-tuples--grouping-values-of-different-types)
9. [Arrays and Slices — Fixed and Dynamic Sequences](#9-arrays-and-slices--fixed-and-dynamic-sequences)
10. [Putting It All Together](#10-putting-it-all-together)
11. [Cargo Commands](#11-cargo-commands)
12. [Summary](#12-summary)
13. [Exercise: Guess the Number Game](#13-exercise-guess-the-number-game)

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

Rust earns its reputation for being strict. The compiler will push back on code that other languages would silently accept, and concepts like ownership and lifetimes will feel unfamiliar at first. That strictness isn't arbitrary — it's the mechanism behind memory safety without a garbage collector, data races eliminated at compile time, and abstractions that cost nothing at runtime.

This course introduces every Rust concept alongside its Python equivalent, so you're always building on what you already know.

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

A quick reference you can come back to as you work through exercises.

| Feature | Python | Rust |
|---|---|---|
| Comment | `# comment` | `// comment` |
| Function | `def add(x, y):` | `fn add(x: u32, y: u32) -> u32` |
| Variable | `x = 5` | `let x = 5;` |
| Mutable variable | `x = 5` (always mutable) | `let mut x = 5;` |
| Constant | `MAX = 100` (convention only) | `const MAX: u32 = 100;` |
| String | `"hello"` | `"hello"` (type `&str`) |
| Print | `print("hi")` | `println!("hi")` |
| If | `if x > 0:` | `if x > 0 { }` |
| For loop | `for i in range(5):` | `for i in 0..5 { }` |
| While | `while x > 0:` | `while x > 0 { }` |
| List | `[1, 2, 3]` | `vec![1, 2, 3]` (growable) or `[1, 2, 3]` (fixed) |
| Tuple | `(1, "hi")` | `(1, "hi")` |
| Dict | `{"a": 1}` | `HashMap::from([("a", 1)])` |
| None / null | `None` | `None` (wrapped in `Option`) |
| Error | `raise ValueError("msg")` | `panic!("msg")` or `Err(...)` |
| Import | `import os` | `use std::fs;` |
| Package file | `pyproject.toml` | `Cargo.toml` |

The biggest practical difference: Rust requires **explicit types** on function parameters and return values. The compiler uses those types to catch mistakes before your code ever runs.

```python
# Python — types are optional hints
def process_data(df, threshold):
    return df[df["value"] > threshold]
```

```rust
// Rust — types are required and enforced
fn process_data(data: Vec<f64>, threshold: f64) -> Vec<f64> {
    data.into_iter()
        .filter(|&x| x > threshold)
        .collect()
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

## 7. Expressions vs Statements

This is one of the more unfamiliar ideas coming from Python, but it directly affects how you write Rust code every day.

| | Statement | Expression |
|---|---|---|
| **What it does** | Performs an action | Produces a value |
| **Ends with `;`** | Yes | No |
| **Examples** | `let x = 5;` | `x + 1`, `if a { b } else { c }` |

The key idea: in Rust, `if` and block `{ }` are expressions — they produce values. This means you can assign their result directly to a variable instead of writing separate mutation logic.

### `if` as an expression

```python
# Python ternary
result = "positive" if x > 0 else "non-positive"
```

```rust
// Rust — if/else returns a value directly
let result = if x > 0 { "positive" } else { "non-positive" };
```

Both branches must return the same type. The compiler will catch a mismatch — this is a common source of early errors to be aware of.

### Blocks as expressions

```rust
let x = {
    let a = 2;
    let b = 3;
    a + b      // no semicolon — this is the block's value
};
// x == 5
```

Any block `{ }` is an expression. Its value is the last expression inside it (no semicolon). This is exactly how function return values work — a function body is just a block.

**Why this matters for data work:** you'll often write transformation logic inline without needing temporary mutable variables. The more functional style Rust encourages maps naturally onto data pipeline thinking.

---

## 8. Tuples — Grouping Values of Different Types

A **tuple** is a fixed-size group of values, each with its own type. Python has tuples too; Rust's are similar but type-strict.

```python
# Python — heterogeneous types, no type enforcement
point = (3, 4.5, "origin")
name, age = ("Alice", 30)
```

```rust
// Rust — each position has a fixed type
let point: (i32, f64, &str) = (3, 4.5, "origin");
let (name, age) = ("Alice", 30i32);
```

### Why tuples matter for data engineering

Tuples are the simplest way to return **multiple values** from a function — a pattern you'll hit constantly in data work:

```rust
/// Returns (min, max, mean) of a dataset.
/// Python has no direct equivalent — you'd return a dict or unpack via tuple.
fn describe(values: &[f64]) -> (f64, f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0;
    for &v in values {
        if v < min { min = v; }
        if v > max { max = v; }
        sum += v;
    }
    (min, max, sum / values.len() as f64)
}

let (lo, hi, avg) = describe(&[1.0, 2.0, 3.0]);
println!("range: {}..{}, avg: {}", lo, hi, avg);
// range: 1..3, avg: 2
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

Destructuring assigns each tuple element to a separate variable. It works in `let`, in function parameters, and in `match` arms:

```rust
let (status, code) = ("ok", 200);

// In a function parameter
fn print_point((x, y): (i32, i32)) {
    println!("({}, {})", x, y);
}
print_point((10, 20));
```

You can ignore parts with `_`:

```rust
let (_, _, avg) = describe(&[1.0, 2.0, 3.0]);  // keep only the mean
```

### Tuples vs arrays vs structs

| Need | Use | Example |
|---|---|---|
| Multiple values of different types, no field names | **tuple** | `(i32, &str, f64)` |
| Many values of the same type, indexed by number | **array** `[T; N]` | `[1, 2, 3, 4, 5]` |
| Multiple values of different types, with field names | **struct** | `Point { x: 1, y: 2 }` |

### Exercise

```rust
fn categorize_row(row: (u32, f64, bool)) -> &'static str {
    // row.0 = id, row.1 = value, row.2 = is_valid
    // Return "ok" if is_valid && value > 0, "invalid" if !is_valid, "zero" otherwise
    todo!()
}

fn main() {
    println!("{}", categorize_row((1, 5.0, true)));    // ok
    println!("{}", categorize_row((2, 0.0, true)));    // zero
    println!("{}", categorize_row((3, 5.0, false)));   // invalid
}
```

<details>
<summary>Solution</summary>

```rust
fn categorize_row((_id, value, is_valid): (u32, f64, bool)) -> &'static str {
    if !is_valid {
        "invalid"
    } else if value > 0.0 {
        "ok"
    } else {
        "zero"
    }
}
```

Notice the destructured parameter — it makes the function read like Python's
`def categorize_row(id, value, is_valid)`.

</details>

---

## 9. Arrays and Slices — Fixed and Dynamic Sequences

Both arrays and slices hold a sequence of values **of the same type**, indexed by `usize`. The difference is **size**:

| Type | Size | When to use |
|---|---|---|
| **Array** `[T; N]` | Fixed at compile time (`N` is part of the type) | Known-length data: `[1, 2, 3]`, `[0u8; 1024]` buffer |
| **Slice** `&[T]` | Dynamic — a *view* into data already owned by something else | Function parameters that accept any sequence |

### Arrays

```rust
// Fixed-size, on the stack
let primes: [u32; 4] = [2, 3, 5, 7];
let zeros: [u8; 1024] = [0; 1024];  // 1024 zeros

let first = primes[0];   // 2
let len = primes.len();  // 4
```

The size `N` is part of the type — `[u32; 4]` and `[u32; 5]` are different types. Trying to grow an array is a compile error.

```rust
let mut a: [i32; 3] = [1, 2, 3];
// a.push(4);   // ❌ ERROR — arrays don't have methods
```

For growable sequences, use `Vec<T>` (covered in [03-Collections/01-TicketManagement](../03-Collections/01-TicketManagement/README.md)).

### Slices

A **slice** is a fat pointer — `(pointer, length)` — that *borrows* a contiguous run of elements. You can take a slice of an array, a `Vec`, or even a `String` (giving a `&str`).

```rust
let data = [10, 20, 30, 40, 50];

// Borrow the whole array as a slice
let all: &[i32] = &data;

// Borrow a sub-range
let middle: &[i32] = &data[1..4];   // [20, 30, 40]
let head:   &[i32] = &data[..3];    // [10, 20, 30]
let tail:   &[i32] = &data[3..];    // [40, 50]
```

> **Why is this useful for data engineering?** A function that takes `&[T]` accepts **any** sequence — array, `Vec`, or sub-range — without copying. This is the Rust equivalent of a Python function taking `Sequence[T]`.

```rust
fn first_n(values: &[f64], n: usize) -> &[f64] {
    &values[..n.min(values.len())]
}
```

### Slices vs arrays in function signatures

| Signature | Accepts | Rejects |
|---|---|---|
| `fn f(arr: [i32; 4])` | Only `[i32; 4]` (size matters) | Other sizes, `Vec`, slices |
| `fn f(s: &[i32])` | Any `&[i32]` (array, `Vec`, sub-range) | Owned values — caller must borrow |
| `fn f(s: &Vec<i32>)` | Only `Vec<i32>` (anti-pattern) | Arrays, slices |

> **Idiom**: prefer `&[T]` over `&Vec<T>` in function parameters. It's more flexible (accepts arrays too) and is the convention in the standard library.

### Exercise

```rust
/// Return the second half of a dataset (rounded up for odd lengths).
fn second_half(data: &[f64]) -> &[f64] {
    // TODO: return the slice from len/2 to end
    todo!()
}

fn main() {
    let values = [1.0, 2.0, 3.0, 4.0, 5.0];
    println!("{:?}", second_half(&values));   // [3.0, 4.0, 5.0]

    let small = [10.0, 20.0];
    println!("{:?}", second_half(&small));     // [20.0]
}
```

<details>
<summary>Solution</summary>

```rust
fn second_half(data: &[f64]) -> &[f64] {
    let mid = data.len() / 2;
    &data[mid..]
}
```

Note the return type is `&[f64]` — we're returning a borrowed view into the input, not a new collection. The borrow checker guarantees that the returned slice lives at most as long as the input does, so this is safe.

</details>

---

## 10. Putting It All Together

Here's a small data processing function that uses everything covered so far:

```rust
/// Calculate (min, max, mean) of a slice of f64 values.
/// Returns a tuple — three values from one function.
/// &[f64] means "a reference to a sequence of f64 values" —
/// it works with both arrays and Vecs without copying the data.
fn describe(values: &[f64]) -> (f64, f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0;

    for &v in values {
        if v < min { min = v; }
        if v > max { max = v; }
        sum += v;
    }

    (min, max, sum / values.len() as f64)  // tuple returned implicitly
}

fn main() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    let (lo, hi, avg) = describe(&data);  // tuple destructured
    println!("min: {}, max: {}, mean: {}", lo, hi, avg);

    let status = if avg > 3.0 { "above midpoint" } else { "below midpoint" };
    println!("Status: {}", status);
}
```

Expected output:

```text
min: 1, max: 5, mean: 3
Status: below midpoint
```

> **What is `&[f64]`?** It's a *slice reference* — a view into a contiguous sequence of `f64` values. We covered this in detail in [§9 — Arrays and Slices](#9-arrays-and-slices--fixed-and-dynamic-sequences). The practical point: passing `&data` instead of `data` lets the function read the values without taking ownership of them or copying them. You'll see `&` frequently in Rust; ownership is covered in depth in a later section.

What each concept is doing here:

| Concept | Where it appears |
|---|---|
| `fn` | `describe()` and `main()` |
| `let mut` | `min`, `max`, `sum` — all need to be reassigned |
| `let` (immutable) | `data`, `status` |
| `for` loop | Iterating over values |
| `&[f64]` | Slice reference — a view into the array |
| `if` as expression | `min`/`max` updates and `status` assignment |
| `f64::INFINITY` / `f64::NEG_INFINITY` | Initial sentinels for `min`/`max` |
| Tuple return | `(min, max, sum / values.len() as f64)` |
| Tuple destructuring | `let (lo, hi, avg) = describe(&data);` |
| `println!` | Formatted output |

---

## 11. Cargo Commands

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
cargo add rand         # add a dependency to Cargo.toml
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
| Add dependency | `cargo add crate_name` |
| Build optimized | `cargo build --release` |
| Open docs | `cargo doc --open` |

---

## 12. Summary

| Concept | Rust | Python equivalent |
|---|---|---|
| Define a function | `fn name(x: T) -> R { }` | `def name(x):` |
| Immutable variable | `let x = 5;` | N/A (Python variables are always mutable) |
| Mutable variable | `let mut x = 5;` | `x = 5` |
| Constant | `const MAX: u32 = 100;` | `MAX = 100` (convention only) |
| Return type | `-> f64` | Type hints, unenforced |
| Implicit return | Last expression, no `;` | `return` always required |
| Print | `println!("val = {}", x)` | `print(f"val = {x}")` |
| If expression | `if cond { a } else { b }` | `a if cond else b` |
| For loop | `for i in 0..5 { }` | `for i in range(5):` |
| Tuple | `(T1, T2, ...)` | `(1, "x")` |
| Tuple destructuring | `let (a, b) = t;` | `a, b = t` |
| Tuple return value | `fn f() -> (i32, &str) { ... }` | Return a dict or unpack via tuple |
| Array (fixed-size) | `[T; N]`, e.g. `[1, 2, 3]` | `list` (size not enforced) |
| Slice (borrowed view) | `&[T]`, e.g. `&arr[1..4]` | `Sequence[T]` (no equivalent — pass-by-reference) |
| `&[T]` in function params | Accepts arrays, `Vec`, sub-ranges | N/A |

---

## 13. Exercise: Guess the Number Game

Let's put it all together with a small interactive program. This covers `let`/`mut`, `fn`, `println!`, `std::io`, loops, `if/else`, and using an external crate.

### What it does

1. Generates a random 2-digit number ($10$–$99$)
2. Gives the player $5$ chances to guess
3. After each guess: "Too high!" or "Too low!"
4. Correct guess: "You win!" and exit
5. No guesses left: reveal the number

### Python version

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

### Rust version

Create a new project and add the `rand` crate:

```bash
cargo new guess_game
cd guess_game
cargo add rand
```

> **Version note:** `rand::random_range` is available from `rand 0.10` onward. Running `cargo add rand` will pull the latest version, so this should work as-is. If you see a `no method named gen_range` error, it means an older version was resolved — run `cargo add rand@0.10` to be explicit.

Write `src/main.rs`:

```rust
use std::io;

fn main() {
    // Generate a random number between 10 and 99 (inclusive)
    let secret: u32 = rand::random_range(10..=99);
    let attempts = 5;

    println!("Guess the 2-digit number!");

    for i in 0..attempts {
        println!("Attempt {}/{}:", i + 1, attempts);

        // Read a line of input into a mutable String
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim whitespace and parse to u32
        let guess: u32 = input.trim().parse().expect("Please enter a number");

        if guess == secret {
            println!("You win!");
            return;  // exit main — we're done
        } else if guess < secret {
            println!("Too low!");
        } else {
            println!("Too high!");
        }
    }

    println!("Game over! The number was {}.", secret);
}
```

Run it:

```bash
cargo run
```

> **What does `.expect()` do?** Both `read_line` and `parse` can fail — `read_line` if there's an IO error, `parse` if the user types something that isn't a number. `.expect("message")` says: "if this fails, crash immediately and print this message." It's the simplest form of error handling and fine for a small exercise. In production Rust you'd handle errors more gracefully — that's covered in a later section.

### Python vs Rust — what changed

| Task | Python | Rust |
|---|---|---|
| Random number | `random.randint(10, 99)` | `rand::random_range(10..=99)` |
| Read input | `input()` | `io::stdin().read_line(&mut String)` |
| Parse input | `int(input())` | `.trim().parse::<u32>()` |
| Loop | `for i in range(5)` | `for i in 0..5` |
| Early exit | `break` | `return` (exits `main`) |
| Error on bad input | `ValueError` at runtime | `.expect("msg")` — panics with a message |

### Things to try

- Change the range to $100$–$999$ with $8$ attempts
- Add a "warm/cold" hint: print "Getting warm!" if the guess is within $5$ of the secret
- Track wins across multiple rounds

---

*Next up — Project 1: Basic Calculator. You'll go deeper on integer types, arithmetic operators, control flow, and error handling.*