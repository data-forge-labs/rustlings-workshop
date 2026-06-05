# Section 1: Foundations — From Python Loops to Rust Safety

*Getting started: syntax, types, control flow, and your first Rust programs.*

---

## Why This Section?

### The Problem

As a Python data engineer, you're used to writing:

```python
def process_scores(scores):
    total = 0
    for s in scores:
        total += s       # Python happily adds int + float + str...
    return total

process_scores([10, "20", 30])  # Runtime TypeError!
```

Python's dynamic typing is flexible but **unforgiving at runtime**. A production ETL pipeline can run for hours before hitting a `TypeError` or `AttributeError` that crashes the whole job. Worse:

```python
counter = 0
while True:
    counter += 1  # Silent overflow? Nope — Python grows the int forever
```

Python's big ints handle overflow gracefully, but they also **allocate memory on every arithmetic operation** and run **10-100x slower** than native machine integers.

### The Rust Solution

Rust catches these bugs **at compile time** — before your pipeline ever touches data:

```rust
fn process_scores(scores: &[i32]) -> i32 {
    scores.iter().sum()  // Type-safe: only i32 allowed
}

fn main() {
    // process_scores(&[10, "20", 30]);  // Compile error!
    println!("{}", process_scores(&[10, 20, 30]));  // 60
}
```

Rust's type system makes **every assumption explicit**:

```
┌─────────────────────────────────────────────┐
│  Python:  "just works" until it doesn't     │
│  ──────────────────────────────────────      │
│  x = "5" + 3     # TypeError at runtime      │
│  lst[999]        # IndexError at runtime     │
│  None.method()   # AttributeError at runtime │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│  Rust:    "annoying" at first, safe forever │
│  ──────────────────────────────────────      │
│  "5" + 3           # compile error            │
│  vec[999]          # compile-time check       │
│  Option::None      # must handle None case    │
└─────────────────────────────────────────────┘
```

This section gives you the foundational Rust toolkit to write **safe, fast, predictable** data code from day one.

---

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | Program entry | `fn main()` | `if __name__ == "__main__"` | Where execution begins |
| 2 | Variable binding | `let`, `let mut` | Variable assignment | Immutable by default — no accidental mutation |
| 3 | Integer types | `i32`, `u32`, `i64`, `usize`, `u8` | `int` | Fixed-width, no hidden cost of big ints |
| 4 | Float types | `f64`, `f32` | `float` | IEEE 754, single or double precision |
| 5 | Boolean & char | `bool`, `char` | `bool`, `str[0]` | True/false + Unicode characters |
| 6 | String types | `String`, `&str` | `str` | Owned vs borrowed string data |
| 7 | Branching | `if` / `else` | `if` / `else` | Expressions, not statements |
| 8 | Loops | `loop`, `while`, `for` | `while`, `for` | Three loop forms for different needs |
| 9 | Unrecoverable errors | `panic!` | `raise Exception` | Crash with a message |
| 10 | Safe overflow | saturating / wrapping methods | N/A (unbounded) | No silent overflow bugs |
| 11 | Type conversion | `as` casting | `int()`, `float()`, `str()` | Explicit numeric conversion |
| 12 | Custom data | `struct`, `impl` | `class`, dataclass | Bundle data + methods |
| 13 | Dynamic arrays | `Vec<T>` | `list` | Growable, type-safe collection |
| 14 | Null safety | `Option<T>` | `None`, `Optional` | No null pointer exceptions |
| 15 | Pattern matching | `match` | `match`/`case` (3.10+) | Exhaustive, safe branching |
| 16 | Console output | `println!`, `print!` | `print()` | Formatted output macro |
| 17 | Console input | `std::io::stdin()` | `input()` | Read user input |
| 18 | Random numbers | `rand` crate | `random` module | Shuffling, sampling, generation |
| 19 | Ranges | `0..n`, `0..=n` | `range(n)` | Sequence generation |
| 20 | String manipulation | `.trim()`, `.chars()`, `.to_digit()` | `str.strip()`, list(str) | String processing |

---

## Concepts at a Glance

### 1. `fn main()` — Program Entry
Every Rust program starts at `fn main()`. Unlike Python where any top-level code runs immediately, Rust requires an explicit entry point.

```rust
// Rust
fn main() {
    println!("Hello, data pipeline!");
}
```

```python
# Python
if __name__ == "__main__":
    print("Hello, data pipeline!")
```

### 2. `let` / `let mut` — Variable Binding
Variables are **immutable by default**. This is Rust's biggest design choice: it prevents accidental mutation that causes bugs.

```rust
let count = 42;        // immutable — cannot change
let mut total = 0;     // mutable — allowed to change
total += 100;           // OK
// count += 1;         // compile error!
```

In Python, everything is mutable by default — Rust flips this.

### 3. Integer Types — `i32`, `u32`, `i64`, `usize`, `u8`
Python's `int` is unbounded; Rust uses **fixed-width** integers. This makes arithmetic **predictable and fast**:

```
┌─────────────────────────────────────────────┐
│  Python int:  variable size, heap-allocated │
│  ──────────────────────────────────────      │
│  x = 42        # 28 bytes                    │
│  x = 10**100   # grows dynamically           │
│  Performance:  ~50ns per addition            │
└─────────────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────────────┐
│  Rust i32:    fixed 4 bytes, on the stack   │
│  ──────────────────────────────────────      │
│  let x: i32 = 42;  # 4 bytes, stack-alloc   │
│  Performance:  ~1ns per addition             │
└─────────────────────────────────────────────┘
```

### 4. `f64` — Floating Point
`f64` is the default (double precision), same as Python's `float`. Use `f32` when memory matters (e.g., large arrays).

### 5. `bool` and `char`
`bool` is true/false (same as Python). `char` is a **Unicode scalar value** (4 bytes), unlike Python's single-character string.

### 6. `String` vs `&str` — Two String Types
This is the most confusing concept for Python programmers:

```
┌───────────────┐     ┌───────────────┐
│   String      │     │    &str       │
│  (owned)      │     │  (borrowed)   │
├───────────────┤     ├───────────────┤
│  Can grow     │     │  Fixed view   │
│  Heap memory  │     │  Into String  │
│  You OWN it   │     │  You BORROW   │
└───────────────┘     └───────────────┘
```

```rust
let owned = String::from("hello");  // String — heap allocated
let borrowed: &str = "hello";       // &str — string literal
let slice = &owned[0..2];            // &str — view into String
```

### 7. `if` / `else` as Expressions
In Rust, `if` returns a value:

```rust
let status = if score >= 60 { "pass" } else { "fail" };
```

In Python, you'd write: `status = "pass" if score >= 60 else "fail"`

### 8. `loop`, `while`, `for`
Three loop constructs:

| Loop | When to Use | Example |
|------|-------------|---------|
| `loop` | Run forever (break explicitly) | Game loops, server listeners |
| `while` | Check condition each iteration | `while n > 0 { ... }` |
| `for` | Iterate over collection | `for x in vec.iter()` |

### 9. `panic!` — Unrecoverable Errors
`panic!` is like raising an unhandled exception — the program crashes:

```rust
panic!("Something went wrong: {}", reason);
```

### 10. Saturating / Wrapping Arithmetic
Python's `int` never overflows. Rust's fixed integers **can overflow** — but Rust gives you explicit control:

```rust
let x = u8::MAX;          // 255
// let y = x + 1;          // debug: panics, release: wraps
let y = x.saturating_add(1);  // 255 (clamped)
let z = x.wrapping_add(1);    // 0 (wrapped around)
```

### 11. Type Casting with `as`
```rust
let x: i32 = 10;
let y: f64 = x as f64;        // explicit cast
```

### 12. `struct` and `impl`
Like a Python dataclass with methods:

```rust
struct Record {
    id: u32,
    value: f64,
}

impl Record {
    fn double(&self) -> f64 {
        self.value * 2.0
    }
}
```

### 13. `Vec<T>` — Dynamic Array
Rust's equivalent of Python's `list`, but type-homogeneous:

```rust
let mut v: Vec<i32> = Vec::new();
v.push(10);
v.push(20);
println!("{}", v[0]);  // 10
```

### 14. `Option<T>` — Null Safety
No `NoneType` errors at runtime:

```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 { Some("Alice".to_string()) }
    else { None }
}

let user = find_user(2);
// user.method();  // compile error — must handle None!
```

### 15. `match` — Exhaustive Pattern Matching
Like Python 3.10+ `match`/`case`, but the compiler **enforces exhaustiveness**:

```rust
match result {
    Some(value) => println!("Got: {}", value),
    None => println!("Nothing found"),
}
```

---

## Prerequisites

- Rust installed (see [01-Intro](./01-Intro/README.md))
- Basic Python knowledge
- Familiarity with terminal/command line

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 01 | **Intro** — Rust primer, syntax, `fn main` | `fn main()`, `let`, `mut`, `println!`, basic types, `&str`, tuples, arrays `[T; N]`, `if`/`else`, loops | Reference |
| 02 | **GuessGame** — interactive "guess the number" with `std::io` and `Result` | `String` vs `&str`, custom `enum`, `derive`, `read_line`, `Result<T, E>`, `.parse()`, `.expect()`, basic `match`, `?` operator, external crates (`rand`) | Tutorial + Project |
| 03 | **BasicCalculator** — integers, branching, loops, overflow | `i32`/`u32`/`i64`/`usize`, `if`/`else`, `while`/`for`, `panic!`, overflow, saturating/wrapping arithmetic, `as` casting, `#[test]` | Tutorial |
| 04 | **MasterMind** — guess a 4-digit secret code | `struct`, `impl`, `Vec<T>`, `Option<T>`, exhaustive `match`, `if let`, `String`/`&str` (deeper), `rand`, iterators, console I/O | Project |

## Learning Path

1. Start with **01-Intro** to get Rust installed and write your first program
2. Move to **02-GuessGame** to learn I/O, `String` vs `&str`, `Result`, and `match` by building an interactive game
3. Build **03-BasicCalculator** to deepen integers, control flow, and overflow handling
4. Build **04-MasterMind** to apply everything in a structured game with `struct`, `Vec`, and `Option`
