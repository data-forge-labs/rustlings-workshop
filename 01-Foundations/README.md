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

### 5. `bool` — Boolean
`bool` is true/false (same as Python). Note that Rust conditions must be `bool` — there is no truthy/falsy like Python's `if 0:` or `if []:`.

> `char` (a 4-byte Unicode scalar) is briefly introduced in the type table but is **not exercised** in this section. You'll see it as the element type of strings (`String.chars()`) in 02-GuessGame and 04-MasterMind.

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

| # | Project | Concepts | Tests | Format |
|---|---------|----------|-------|--------|
| 01 | **Intro** — Rust primer, syntax, `fn main` | `fn main()`, `let`, `mut`, `println!`, basic types, `&str`, tuples, arrays `[T; N]`, `if`/`else`, loops | 26 | Reference |
| 02 | **GuessGame** — interactive "guess the number" with `std::io` and `Result` | `String` vs `&str`, custom `enum`, `derive`, `read_line`, `Result<T, E>`, `.parse()`, `.expect()`, basic `match`, `?` operator, external crates (`rand`) | 20 | Tutorial + Project |
| 03 | **BasicCalculator** — integers, branching, loops, overflow | `i32`/`u32`/`i64`/`usize`, `if`/`else`, `while`/`for`, `panic!`, `Result` (vs `panic!`), overflow, saturating/wrapping arithmetic, `as` casting, `#[test]` | 35 | Tutorial |
| 04a | **MasterMind (Basic)** — guess a 4-digit secret code | `struct`, `impl`, `Vec<T>`, `Option<T>`, exhaustive `match`, `if let`, `String`/`&str` (deeper), `rand`, iterators (`.iter()`, `.zip()`, `.filter()`, `.count()`), closures, `&self` vs `&mut self`, `pub` visibility | 30 | Project |
| 04b | **MasterMind (Advanced)** — refactor into library + binary with `clap` | `mod` / file organization, library vs binary crate, `pub` re-exports, `///` docs, `cargo doc --open`, `clap` derive CLI parsing | (suggested) | Project (optional) |

## Learning Path

1. Start with **01-Intro** to get Rust installed and write your first program (26 tests)
2. Move to **02-GuessGame** to learn I/O, `String` vs `&str`, `Result`, and `match` by building an interactive game (20 tests)
3. Build **03-BasicCalculator** to deepen integers, control flow, overflow handling, and Rust's built-in `#[test]` framework (35 tests)
4. Build **04-MasterMind (Basic)** to learn `struct` + `impl`, `Vec`, `Option`, iterators, and the `&self`/`&mut self` distinction (30 tests)
5. (Optional) Refactor in **04-MasterMind (Advanced)** to learn modules, `pub` visibility, and `clap` CLI parsing

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

