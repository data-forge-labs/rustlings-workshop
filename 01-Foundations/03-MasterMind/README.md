# Rust for Python Data Engineers â€” MasterMind

*A hands-on workshop that teaches Strings, Vectors, Structs, Option, Iterators, and I/O by building a MasterMind code-breaking game.*

> **Test-driven approach**: This project includes two Cargo projects with progressive unit tests. The **basic** workshop (`workshop/`) implements the core game; the **advanced** workshop (`workshop/advanced/`) adds modules, CLI args with `clap`, and documentation. Each function in `src/lib.rs` starts as a `todo!()` stub. Run `cd workshop && cargo test` (basic) or `cd workshop/advanced && cargo test` (advanced) to watch the pass count grow. Your goal: **all 30 tests pass (basic) and all tests pass (advanced)**.

---

## Why This Project?

### The Problem

In data engineering, you constantly deal with **collections of structured data** â€” rows in a DataFrame, records in JSON, chunks of a CSV file. You need to model real-world entities (a "Record", a "Customer", a "Transaction") with clear fields and behaviors. You need to handle **missing data** gracefully. And you need to process text efficiently without performance surprises.

Python makes this easy â€” almost too easy:

```python
def process_records(data):
    result = []
    for record in data:
        result.append(record["value"] * 2)  # Does 'record' have "value"?
    return result

value = maybe_get_value()  # Returns None 10% of the time
processed = value * 2       # TypeError at runtime!
```

There's no guarantee a "record" has the expected fields. A `None` can silently propagate through a pipeline until it crashes at the worst moment. Python's `str` type hides the distinction between immutable text (most of your data) and mutable string buffers (what you build during processing).

In this project, you'll build a **MasterMind code-breaking game** â€” a perfect vehicle for learning Rust's solutions to these exact problems. You'll model game state with `struct`, handle missing guesses with `Option` (no more `None` surprises), process characters with iterators, and distinguish between `String` (owned, growable) and `&str` (borrowed, fixed).

### The Rust Solution

Rust gives you precise control over data modeling and memory:

```rust
struct Record {
    id: u64,
    value: f64,
    label: Option<String>,  // Missing data is explicit
}

fn process(record: Record) -> f64 {
    match record.label {
        Some(label) => record.value * 2.0,
        None => record.value,  // Compiler forces this branch!
    }
}

// String vs &str â€” be explicit about ownership
fn analyze(text: &str) -> usize {
    text.chars().count()  // Borrow, don't own
}
let owned: String = String::from("data");
analyze(&owned);  // Borrow without taking ownership
```

---

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | `String` vs `&str` | `String`, `&str` | `str` | Owned vs borrowed string types |
| 2 | `Vec<T>` | `Vec<T>` | `list` | Dynamic typed array |
| 3 | `struct` | `struct Name { fields }` | `@dataclass` / `class` | Custom data types with named fields |
| 4 | `impl` blocks | `impl MyStruct { fn method(&self) {} }` | Methods inside a `class` | Attach behavior to structs |
| 5 | `Option<T>` | `Option::Some(val)` / `Option::None` | `None` / `Optional[T]` | Handle nullable/missing values |
| 6 | `match` | `match val { Some(x) => ..., None => ... }` | `if/elif/else` chains | Exhaustive pattern matching |
| 7 | Ownership basics | Move vs borrow (`&self`, function params) | N/A (GC) | Memory safety without garbage collection |
| 8 | Iterators | `.chars()`, `.enumerate()`, `.filter()` | `for ch in s`, `enumerate(s)` | Lazy functional iteration over sequences |
| 9 | Console I/O | `io::stdin().read_line()`, `println!` | `input()`, `print()` | Read user input and print output |
| 10 | `rand` crate | `rand::rng().random_range()` | `random.randint()` | Random number generation |
| 11 | `pub` visibility | `pub fn`, `pub struct` | Public by default | Control the public API surface |
| 12 | `Self` constructor | `fn new(...) -> Self` | `__init__(self)` | Idiomatic constructor pattern |

## Concepts at a Glance

### 1. `String` vs `&str`
`String` is an owned, heap-allocated, growable UTF-8 string. `&str` is a borrowed view â€” the default for function parameters that only need to read text. **Python:** one `str` type, always immutable.

### 2. `Vec<T>`
`vec![1, 2, 3]` creates a typed, growable array. Use `.push()` to add, `[i]` to index. **Python:** `list` â€” but Rust's `Vec` is typed; you can't mix unrelated types.

### 3. `struct`
`struct Guess { value: String }` defines a new data type with named fields, validated at compile time. **Python:** `@dataclass class Guess: value: str`.

### 4. `impl` blocks
`impl Guess { fn new(v: String) -> Self { ... } }` attaches methods to a struct. **Python:** methods live inside the `class` body. Rust separates data and behavior.

### 5. `Option<T>`
`Option<T>` represents a value that may be present (`Some(val)`) or absent (`None`). The compiler forces you to check. **Python:** `None` can appear anywhere without warning.

### 6. `match`
`match value { Some(x) => x * 2, None => 0 }` performs exhaustive pattern matching. The compiler checks every variant is handled. **Python:** `if x is not None: ... else: ...` â€” no compiler verification.

### 7. Ownership basics
When a value is assigned to a new variable or passed to a function, ownership moves. Use `&` to borrow. **Python:** everything is reference-counted â€” no ownership concept.

### 8. Iterators
`s.chars()` returns an iterator over characters; `.enumerate()` pairs each element with its index. Iterators are lazy and composable. **Python:** `for ch in s:` and `enumerate(s)` work similarly but are eager.

### 9. Console I/O
`io::stdin().read_line(&mut buf)` reads a line into a buffer. `println!` writes to stdout. **Python:** `input()` and `print()`.

### 10. `rand` crate
`rand::rng().random_range(10..=99)` generates a random number. Add `rand = "0.10"` to `Cargo.toml`. **Python:** `random.randint(10, 99)`.

### 11. `pub` visibility
Items are private by default. Prefix with `pub` to expose them. Controls the public API surface. **Python:** everything is public; `_` prefix is a convention.

### 12. `Self` constructor
`fn new(value: String) -> Self { Self { value } }` is the idiomatic constructor. `Self` abbreviates the struct's type. **Python:** `def __init__(self, value): self.value = value`.

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Prerequisites](#2-prerequisites)
3. [How to Use This Workshop](#3-how-to-use-this-workshop)
4. [Python vs Rust Concepts in This Project](#4-python-vs-rust-concepts-in-this-project)
5. [Basic Workshop (workshop/)](#5-basic-workshop-workshop)
6. [Advanced Workshop (workshop/advanced/)](#6-advanced-workshop-workshopadvanced)
7. [Detailed Step-by-Step Guide (Basic)](#7-detailed-step-by-step-guide-basic)
8. [Advanced Exercise Guide](#8-advanced-exercise-guide)
9. [Summary](#9-summary)

---

## 1. Project Overview

MasterMind is a classic code-breaking game:
- The computer generates a secret 4-digit code
- The player guesses the code
- After each guess, the computer gives feedback: correct digits in the right position (A), and correct digits in the wrong position (B)
- The player wins by guessing the code in as few tries as possible

### Rust Concepts Covered

| Concept | Python Equivalent | Why It Matters for Data Engineering |
|---|---|---|
| `String` vs `&str` | `str` | Text processing in data pipelines |
| Ownership & Borrowing | N/A (GC handles this) | Memory safety without GC pauses |
| `Vec<T>` | `list` | Dynamic collections for data |
| `struct` + `impl` | `class` | Organizing data and behavior |
| `Option<T>` | `None` / `Optional` | Handling missing data |
| Pattern matching (`match`) | `if`/`elif`/`else` | Clean branching logic |
| Iterators & Closures | `for` loops, `map`/`filter` | Functional data processing |
| Console I/O | `input()` / `print()` | CLI tools for data engineering |

---

## 2. Prerequisites

- Completed [Basic Calculator](../01-Foundations/02-BasicCalculator/README.md)
- Rust installed and working
- Basic familiarity with `cd workshop && cargo run`

---

## 3. How to Use This Workshop

This project has two separate workshops:

### Basic â€” `workshop/`

Build the core MasterMind game with structs, Vec, Option, and iterators. Start here.

1. **Read the concept overview** in Section 4 below â€” maps each Rust concept to Python
2. **Follow the detailed guide** in [Section 7](#7-detailed-step-by-step-guide-basic) for the full step-by-step implementation
3. **Build the game** with `cd workshop && cargo run`

### Advanced â€” `workshop/advanced/`

Refactor the game into a library + binary crate with `clap` CLI args and documentation. Complete the basic version first.

1. **Read** [Section 8](#8-advanced-exercise-guide) for module organization, `clap`, and doc concepts
2. **Browse the stub files** in `workshop/advanced/src/` (lib.rs, main.rs, secret.rs, game.rs)
3. **Build** with `cd workshop/advanced && cargo run -- --max-attempts 15`

---

## 4. Python vs Rust Concepts in This Project

### Strings: `String` vs `&str`

```python
# Python â€” one string type
name = "Alice"
name += " Smith"   # Creates a new string
```

```rust
// Rust â€” two string types
let literal: &str = "Alice";       // Immutable, fixed, efficient
let mut owned: String = String::from("Alice");  // Heap-allocated, growable
owned.push_str(" Smith");
```

| Characteristic | `&str` (string slice) | `String` (owned string) |
|---|---|---|
| Mutability | Immutable | Mutable |
| Where stored | Read-only memory or borrowed | Heap |
| Use case | Read-only access, parameters | Building, modifying text |
| Python analog | `str` (immutable) | No direct equivalent |

### Vectors: `Vec<T>`

```python
# Python â€” dynamic list
fruits = ["apple", "banana"]
fruits.append("cherry")
```

```rust
// Rust â€” typed vector
let mut fruits: Vec<&str> = vec!["apple", "banana"];
fruits.push("cherry");
```

| Operation | Python `list` | Rust `Vec<T>` |
|---|---|---|
| Create | `items = []` | `let items: Vec<T> = Vec::new();` |
| With values | `items = [1, 2, 3]` | `let items = vec![1, 2, 3];` |
| Add | `items.append(x)` | `items.push(x);` |
| Remove last | `items.pop()` | `items.pop();` |
| Length | `len(items)` | `items.len()` |
| Access | `items[0]` | `items[0]` (panics if out of bounds) |

### Structs and Methods: `struct` + `impl`

```python
# Python class
class Guess:
    def __init__(self, value: str):
        self.value = value

    def is_valid(self) -> bool:
        return len(self.value) == 4
```

```rust
// Rust struct + impl
struct Guess {
    value: String,
}

impl Guess {
    fn new(value: String) -> Self {
        Self { value }
    }

    fn is_valid(&self) -> bool {
        self.value.len() == 4
    }
}
```

| Aspect | Python `class` | Rust `struct` + `impl` |
|---|---|---|
| Data fields | `self.field` in `__init__` | Fields in `struct` definition |
| Methods | All in class body | Separate `impl` block |
| Constructor | `__init__` | `fn new(...) -> Self` |
| Visibility | Public by default | Private by default (`pub` to expose) |

### Option and Pattern Matching

```python
# Python â€” None handling
def find_item(items, target):
    for item in items:
        if item == target:
            return item
    return None

result = find_item(data, "x")
if result is not None:
    print(result)
```

```rust
// Rust â€” Option + match
fn find_item(items: &[&str], target: &str) -> Option<&str> {
    for &item in items {
        if item == target {
            return Some(item);
        }
    }
    None
}

match find_item(&data, "x") {
    Some(item) => println!("Found: {}", item),
    None => println!("Not found"),
}
```

| Python | Rust |
|---|---|
| `None` | `Option::None` |
| `if x is not None` | `if let Some(x) = value` |
| `x = func() or default` | `func().unwrap_or(default)` |
| `x = func()` (might be None) | `func()` returns `Option<T>` |

---

## 5. Basic Workshop (workshop/)

The `workshop/` directory contains the basic MasterMind game. Follow the detailed guide in [Section 7](#7-detailed-step-by-step-guide-basic). Key sections:

1. **Setup:** Create project, add dependencies (`rand` crate)
2. **Variables & Types:** Declare game constants and state
3. **Strings:** Handle player input with `String` and `&str`
4. **Ownership & Borrowing:** Pass data between functions
5. **Vectors:** Store the secret code and guess history
6. **Structs:** Model the `Guess` and `Game` types
7. **Option:** Handle cases where data might not exist
8. **Iterators:** Process guesses functionally
9. **I/O:** Read guesses, print feedback

---

## 6. Summary

| Concept | How Used in MasterMind |
|---|---|
| `String` / `&str` | Player input strings, string slicing for digits |
| Ownership | Function parameters â€” know when to move vs borrow |
| `Vec<char>` | Store the 4-digit code as vector of characters |
| `struct Guess` | Model a single guess with validation |
| `struct Game` | Model the game state (secret, attempts) |
| `impl` | Methods on `Guess` and `Game` |
| `Option<&str>` | Parse input, may fail |
| `match` | Branch on `Option` variants |
| Iterators | `chars()`, `enumerate()` for digit comparison |
| Console I/O | `io::stdin().read_line()`, `println!` |
| `rand` crate | Generate random secret code |

**Advanced workshop extra concepts:**
| Concept | How Used |
|---------|----------|
| `mod` / `pub` | Split code into `secret.rs`, `game.rs`, `lib.rs` modules |
| `clap::Parser` | Parse `--max-attempts` CLI argument |
| `///` docs | Document structs and methods, `cargo doc --open` |
| `#[cfg(test)]` | Unit tests alongside implementation code |

### Further Reading

| Section | Workshop | Topics |
|---------|----------|--------|
| [Section 7](#7-detailed-step-by-step-guide-basic) | Basic (`workshop/`) | Full step-by-step guide: structs, Vec, Option, iterators, I/O |
| [Section 8](#8-advanced-exercise-guide) | Advanced (`workshop/advanced/`) | Module organization, `clap` CLI args, documentation, unit tests |

### Next Project

Proceed to [3-TicketV1](../02-Ownership/01-TicketV1/README.md) to master **ownership** â€” Rust's most important and unique concept.

---

## 7. Detailed Step-by-Step Guide (Basic)

Build the core MasterMind game. Work in the `workshop/` directory.

### Table of Contents

1. [Prerequisites & Setup](#1-prerequisites--setup)
2. [Adding Dependencies](#2-adding-dependencies)
3. [Concept 1: Variables, Mutability, and Basic Data Types](#3-concept-1-variables-mutability-and-basic-data-types)
4. [Concept 2: Strings â€“ `String` vs `&str`](#4-concept-2-strings--string-vs-str)
5. [Concept 3: Ownership, Borrowing, and References](#5-concept-3-ownership-borrowing-and-references)
6. [Concept 4: Vectors â€“ `Vec<T>`](#6-concept-4-vectors--vect)
7. [Concept 5: Structs and Methods â€“ `struct` + `impl`](#7-concept-5-structs-and-methods--struct--impl)
8. [Concept 6: `Option<T>` and Pattern Matching](#8-concept-6-optiont-and-pattern-matching)
9. [Concept 7: Iterators and Closures](#9-concept-7-iterators-and-closures)
10. [Concept 8: Constants (`const`)](#10-concept-8-constants-const)
11. [Concept 9: Input/Output â€“ Reading from the Console](#11-concept-9-inputoutput--reading-from-the-console)
12. [Putting It All Together: The Complete `main.rs`](#12-putting-it-all-together-the-complete-mainrs)
13. [Running the Game](#13-running-the-game)
14. [Summary of Rust Concepts Used](#14-summary-of-rust-concepts-used)

### 1. Prerequisites & Setup

#### Installing Rust (if not already done)

Open your WSL terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, verify the tools:

```bash
rustc --version
cargo --version
```

#### Creating the Project

```bash
cargo new mastermind
cd mastermind
```

This creates a folder with:

- `Cargo.toml` â€“ project configuration
- `src/main.rs` â€“ main source file

### 2. Adding Dependencies

Rust uses `cargo` to manage external libraries (called *crates*). We need `rand` for random number generation.

Open `Cargo.toml` and add:

```toml
[dependencies]
rand = "0.10"
```

Now run `cargo build`. Cargo downloads the `rand` crate and compiles your project.

> **Comparison:** In Python, `import random` gives you random functions. In Rust, you declare the dependency in `Cargo.toml` and then `use rand::...` in your code.

### 3. Concept 1: Variables, Mutability, and Basic Data Types

In Rust, a variable is declared with `let`. **By default, variables are immutable** â€“ once assigned, you cannot change their value. To allow mutation, you must add the `mut` keyword.

Rust is statically typed, but the compiler can often *infer* the type. You can also explicitly annotate types.

```rust
fn main() {
    let x = 5;            // immutable, type i32 (inferred)
    // x = 6;             // ERROR: cannot assign twice to immutable variable

    let mut y = 10;       // mutable
    y = 20;               // OK

    let a: u32 = 100;     // unsigned 32-bit integer
    let b: i32 = -50;     // signed 32-bit integer
    let c: f64 = 3.14;    // 64-bit floating point
    let d: bool = true;   // boolean
    let e: char = 'A';    // Unicode character (4 bytes)
}
```

#### Python Comparison

```python
x = 5   # always rebindable
x = 6   # works fine
```

#### Applying to Mastermind

```rust
let mut attempts_left: u32 = 20;   // mutable, because we decrement it
let guess_count: u32 = 0;         // immutable, but we'll reassign with let mut later
```

### 4. Concept 2: Strings â€“ `String` vs `&str`

Rust has two main string types:

- **`String`** â€“ an *owned*, growable, heap-allocated string. You can modify it (e.g., push characters).
- **`&str`** â€“ a *string slice*, a reference to a sequence of UTF-8 bytes. It can point to a part of a `String` or to a string literal.

String literals like `"hello"` are of type `&str`.

```rust
fn main() {
    let s1: String = String::from("hello");  // heap-allocated String
    let s2: &str = "world";                  // string literal (&str)
    let s3: &str = &s1;                      // borrow String as &str
    println!("{} {}", s1, s3);               // hello hello

    let mut s4 = String::from("foo");
    s4.push_str("bar");
    println!("{}", s4);                      // foobar
}
```

#### Python Comparison

Python's `str` is like Rust's `String` â€“ it's immutable but managed for you. There is no separate slice type.

#### Applying to Mastermind

```rust
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();
let input = input.trim().to_lowercase();

fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) { ... }
```

### 5. Concept 3: Ownership, Borrowing, and References

Every value in Rust has exactly one *owner*. When the owner goes out of scope, the value is dropped (memory freed). You can **move** ownership or **borrow** it via references.

- **Move** â€“ the old owner is invalidated.
- **Borrow** â€“ you temporarily get a reference, but the owner retains ownership.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;             // s1 is moved to s2; s1 can no longer be used!
    // println!("{}", s1);   // ERROR

    let s3 = String::from("world");
    let len = calculate_length(&s3);   // borrow s3 immutably
    println!("{} has length {}", s3, len); // s3 is still valid
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

#### Python Comparison

Python never moves ownership; everything is a reference, and the garbage collector frees memory. Rust does all this at compile time with zero runtime overhead.

#### Applying to Mastermind

```rust
impl SecretCode {
    fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) { ... }
    fn give_position_hint(&mut self) -> Option<(usize, u8)> { ... }
}
```

### 6. Concept 4: Vectors â€“ `Vec<T>`

`Vec<T>` is a resizable, heap-allocated array. It's similar to Python's `list`. You can push elements, iterate, and index into it.

```rust
fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);
    numbers.push(20);

    let names = vec!["Alice", "Bob"];       // Vec<&str>

    println!("{:?}", numbers);              // [10, 20]
    println!("{}", numbers[0]);             // 10

    for n in &numbers {
        println!("{}", n);
    }
}
```

#### Python Comparison

```python
nums = [10, 20]     # dynamic list
```

#### Applying to Mastermind

```rust
let digits: Vec<u8> = vec![1, 4, 2, 7];
let revealed_positions: Vec<bool> = vec![false; 4];
let revealed_digits: Vec<bool> = vec![false; 10];
```

### 7. Concept 5: Structs and Methods â€“ `struct` + `impl`

Rust groups data into **structs** (like Python classes without inheritance). The behaviour (methods) is defined in a separate `impl` block.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(w: u32, h: u32) -> Self {
        Self { width: w, height: h }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

fn main() {
    let mut rect = Rectangle::new(10, 20);
    println!("Area: {}", rect.area());   // 200
    rect.double_size();
    println!("Area: {}", rect.area());   // 800
}
```

#### Python Comparison

```python
class Rectangle:
    def __init__(self, w, h):
        self.width = w
        self.height = h
    def area(self):
        return self.width * self.height
```

#### Applying to Mastermind

```rust
struct SecretCode {
    digits: Vec<u8>,
    revealed_positions: Vec<bool>,
    revealed_digits: Vec<bool>,
}

impl SecretCode {
    fn new() -> Self { ... }
    fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) { ... }
    fn give_position_hint(&mut self) -> Option<(usize, u8)> { ... }
}
```

### 8. Concept 6: `Option<T>` and Pattern Matching

Rust has no `null`. Instead, optional values are expressed using the `Option<T>` enum, which can be either `Some(value)` or `None`.

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let result = divide(10.0, 2.0);

    match result {
        Some(val) => println!("Result: {}", val),
        None => println!("Cannot divide by zero"),
    }

    if let Some(val) = result {
        println!("Got value: {}", val);
    }
}
```

#### Python Comparison

```python
def divide(a, b):
    if b == 0:
        return None
    return a / b

result = divide(10, 2)
if result is not None:
    print(f"Got value: {result}")
```

#### Applying to Mastermind

```rust
fn give_position_hint(&mut self) -> Option<(usize, u8)> {
    if !self.can_give_position_hint() {
        return None;
    }
    Some((chosen_index, digit))
}

if let Some((pos, digit)) = self.secret.give_position_hint() {
    // use pos and digit
}
```

### 9. Concept 7: Iterators and Closures

Iterators allow you to process collections in a functional style. A **closure** is an anonymous function that can capture variables from its environment.

```rust
fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = nums.iter()
        .map(|x| x * 2)
        .collect();
    println!("{:?}", doubled);  // [2, 4, 6, 8, 10]

    let evens: Vec<&i32> = nums.iter().filter(|&&x| x % 2 == 0).collect();
    println!("{:?}", evens);    // [2, 4]

    let a = [1, 2, 3];
    let b = [4, 5, 6];
    for (x, y) in a.iter().zip(b.iter()) {
        println!("{} {}", x, y);
    }
}
```

#### Python Comparison

```python
nums = [1, 2, 3, 4, 5]
doubled = [x * 2 for x in nums]
evens = [x for x in nums if x % 2 == 0]
```

#### Applying to Mastermind

```rust
// Count exact matches
let green = self.digits.iter()
    .zip(guess_digits.iter())
    .filter(|(s, g)| s == g)
    .count();

// Find unrevealed positions
let available: Vec<usize> = self.revealed_positions.iter()
    .enumerate()
    .filter(|(_, &revealed)| !revealed)
    .map(|(i, _)| i)
    .collect();
```

### 10. Concept 8: Constants (`const`)

Constants are always immutable and must be type-annotated. They can be declared in any scope and are inlined at compile time.

```rust
const MAX_SCORE: u32 = 100;

fn main() {
    println!("Maximum score: {}", MAX_SCORE);
}
```

#### Python Comparison

Python uses variables in `UPPER_CASE` by convention, but Rust enforces immutability.

#### Applying to Mastermind

```rust
const DEFAULT_ATTEMPTS: u32 = 20;
const HINT_POSITION_COST: u32 = 5;
const HINT_DIGIT_COST: u32 = 3;
```

### 11. Concept 9: Input/Output â€“ Reading from the Console

To read a line from standard input, use `std::io::stdin().read_line(&mut some_string)`. For prompts without a newline, you must `flush` the output.

```rust
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter something: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    println!("You typed: {}", input);
}
```

#### Python Comparison

```python
input("Enter something: ")   # automatically prints prompt and reads line
```

#### Applying to Mastermind

```rust
fn get_user_input(&self) -> String {
    loop {
        print!("Enter guess (or 'help'): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input == "help" {
            return input.to_string();
        }
        // validation ...
    }
}
```

### 12. Putting It All Together: The Complete `main.rs`

Now build the entire game file step by step. Replace the content of `src/main.rs` with the following blocks.

#### Top-level imports and utility function

```rust
use rand::seq::SliceRandom;
use rand::rng;
use std::io::{self, Write};

/// Returns true if the given string consists of 4 unique digits.
fn has_unique_digits(s: &str) -> bool {
    let mut seen = [false; 10];
    for ch in s.chars() {
        let digit = ch.to_digit(10).unwrap() as usize;
        if seen[digit] {
            return false;
        }
        seen[digit] = true;
    }
    true
}
```

#### SecretCode struct and implementation

```rust
struct SecretCode {
    digits: Vec<u8>,
    revealed_positions: Vec<bool>,
    revealed_digits: Vec<bool>,
}

impl SecretCode {
    fn new() -> Self {
        let mut rng = rng();
        let mut pool: Vec<u8> = (0..=9).collect();
        pool.shuffle(&mut rng);
        let digits = pool[..4].to_vec();

        SecretCode {
            digits,
            revealed_positions: vec![false; 4],
            revealed_digits: vec![false; 10],
        }
    }

    fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) {
        let guess_digits: Vec<u8> = guess
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let green = self.digits
            .iter()
            .zip(guess_digits.iter())
            .filter(|(s, g)| s == g)
            .count();

        let mut secret_unmatched: Vec<u8> = Vec::new();
        let mut guess_unmatched: Vec<u8> = Vec::new();
        for (s, g) in self.digits.iter().zip(guess_digits.iter()) {
            if s != g {
                secret_unmatched.push(*s);
                guess_unmatched.push(*g);
            }
        }

        let mut yellow = 0;
        for g in &guess_unmatched {
            if let Some(pos) = secret_unmatched.iter().position(|&x| x == *g) {
                yellow += 1;
                secret_unmatched.remove(pos);
            }
        }

        let red = 4 - green - yellow;
        (green, yellow, red)
    }

    fn can_give_position_hint(&self) -> bool {
        self.revealed_positions.iter().any(|&revealed| !revealed)
    }

    fn can_give_digit_hint(&self) -> bool {
        self.revealed_digits.iter().any(|&revealed| !revealed)
    }

    fn give_position_hint(&mut self) -> Option<(usize, u8)> {
        if !self.can_give_position_hint() {
            return None;
        }
        let available: Vec<usize> = self.revealed_positions
            .iter()
            .enumerate()
            .filter(|(_, &revealed)| !revealed)
            .map(|(i, _)| i)
            .collect();

        let mut rng = rng();
        let chosen = *available.choose(&mut rng).unwrap();
        self.revealed_positions[chosen] = true;
        Some((chosen, self.digits[chosen]))
    }

    fn give_digit_hint(&mut self) -> Option<u8> {
        if !self.can_give_digit_hint() {
            return None;
        }
        let available: Vec<usize> = self.digits
            .iter()
            .enumerate()
            .filter(|(_, &d)| !self.revealed_digits[d as usize])
            .map(|(i, _)| i)
            .collect();

        let mut rng = rng();
        let chosen_idx = *available.choose(&mut rng).unwrap();
        let digit = self.digits[chosen_idx];
        self.revealed_digits[digit as usize] = true;
        Some(digit)
    }
}
```

#### MastermindGame struct and constants

```rust
const DEFAULT_ATTEMPTS: u32 = 20;
const HINT_POSITION_COST: u32 = 5;
const HINT_DIGIT_COST: u32 = 3;

struct MastermindGame {
    secret: SecretCode,
    attempts_left: u32,
    guess_count: u32,
}

impl MastermindGame {
    fn new(max_attempts: u32) -> Self {
        MastermindGame {
            secret: SecretCode::new(),
            attempts_left: max_attempts,
            guess_count: 0,
        }
    }

    fn play(&mut self) {
        self.display_welcome();

        while self.attempts_left > 0 {
            println!("\nAttempts left: {}", self.attempts_left);
            let input = self.get_user_input();

            if input == "help" {
                self.handle_hint();
                continue;
            }

            self.guess_count += 1;
            let (green, yellow, red) = self.secret.evaluate_guess(&input);
            self.display_feedback(green, yellow, red);

            if green == 4 {
                println!(
                    "\nCongratulations! You cracked the code in {} actual guesses.",
                    self.guess_count
                );
                return;
            }

            self.attempts_left -= 1;
        }

        let secret_str: String = self.secret.digits.iter().map(|d| d.to_string()).collect();
        println!("\nGame Over! The secret code was {}.", secret_str);
    }

    fn display_welcome(&self) {
        println!("{}", "=".repeat(40));
        println!("   Welcome to Mastermind!");
        println!("   Guess the 4-digit code (digits 0-9, no repeats)");
        println!("   You have {} attempts. Type 'help' for hints.", self.attempts_left);
        println!("{}", "=".repeat(40));
    }

    fn display_feedback(&self, green: usize, yellow: usize, red: usize) {
        println!("Green: {}   Yellow: {}   Red: {}", green, yellow, red);
    }

    fn get_user_input(&self) -> String {
        loop {
            print!("Enter guess (or 'help'): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if input == "help" {
                return input.to_string();
            }

            if input.len() != 4 || !input.chars().all(|c| c.is_ascii_digit()) {
                println!("Invalid input. Please enter exactly 4 digits (e.g., 1234).");
                continue;
            }
            if !has_unique_digits(&input) {
                println!("Digits must be unique (no repeats). Try again.");
                continue;
            }
            return input.to_string();
        }
    }

    fn handle_hint(&mut self) {
        if self.attempts_left == 0 {
            println!("You don't have enough attempts to use a hint.");
            return;
        }

        let pos_available = self.secret.can_give_position_hint();
        let dig_available = self.secret.can_give_digit_hint();

        if !pos_available && !dig_available {
            println!("All hints already revealed. No more help available.");
            return;
        }

        println!("\n--- Hint Menu ---");
        let mut menu_options = Vec::new();
        if pos_available {
            println!("1. Reveal one digit and its correct position (costs {} attempts)", HINT_POSITION_COST);
            menu_options.push('1');
        } else {
            println!("1. (No more position hints available)");
        }
        if dig_available {
            println!("2. Reveal a correct digit (costs {} attempts)", HINT_DIGIT_COST);
            menu_options.push('2');
        } else {
            println!("2. (No more digit hints available)");
        }

        print!("Choose 1 or 2 (or press Enter to cancel): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        if choice.is_empty() || !menu_options.contains(&choice.chars().next().unwrap_or(' ')) {
            println!("Hint cancelled.");
            return;
        }

        let choice_char = choice.chars().next().unwrap();
        if choice_char == '1' {
            if self.attempts_left < HINT_POSITION_COST {
                println!("Not enough attempts. You need {} but have {}.", HINT_POSITION_COST, self.attempts_left);
                return;
            }
            if let Some((pos, digit)) = self.secret.give_position_hint() {
                self.attempts_left -= HINT_POSITION_COST;
                println!("Hint: Digit {} is at position {}.", digit, pos + 1);
                println!("({} attempts deducted)", HINT_POSITION_COST);
            }
        } else {
            if self.attempts_left < HINT_DIGIT_COST {
                println!("Not enough attempts. You need {} but have {}.", HINT_DIGIT_COST, self.attempts_left);
                return;
            }
            if let Some(digit) = self.secret.give_digit_hint() {
                self.attempts_left -= HINT_DIGIT_COST;
                println!("Hint: The code contains the digit {}.", digit);
                println!("({} attempts deducted)", HINT_DIGIT_COST);
            }
        }

        if self.attempts_left == 0 {
            println!("\nYou've used up your last attempts on a hint.");
        }
    }
}
```

#### Main entry point

```rust
fn main() {
    let mut game = MastermindGame::new(DEFAULT_ATTEMPTS);
    game.play();
}
```

### 13. Running the Game

Inside the `mastermind` directory, run:

```bash
cargo run
```

### 14. Summary of Rust Concepts Used

| Concept | Where Used |
|---------|------------|
| Variables & mutability (`let`, `let mut`) | `attempts_left`, `guess_count`, `input` |
| Data types (`u32`, `u8`, `bool`, `usize`) | struct fields, counters, array indices |
| Strings (`String`, `&str`) | user input, function parameters |
| Ownership & borrowing (`&self`, `&mut self`, `&str`) | method signatures, passing references |
| Vectors (`Vec`) | `digits`, `revealed_positions`, `revealed_digits` |
| Structs & methods (`struct`, `impl`) | `SecretCode`, `MastermindGame` |
| `Option<T>` and `if let` | hint functions, result handling |
| Iterators & closures | evaluating guesses, finding unrevealed hints |
| Constants (`const`) | `DEFAULT_ATTEMPTS`, hint costs |
| I/O (`stdin`, `stdout`, `flush`) | reading guesses, printing prompts |

---

## 8. Advanced Exercise Guide

Refactor the game into a library + binary crate. Work in the `workshop/advanced/` directory.

### Table of Contents

- [8. Advanced Exercise Guide](#8-advanced-exercise-guide)
  - [Table of Contents](#table-of-contents-1)
  - [1. Introduction](#1-introduction-1)
  - [2. Prerequisites](#2-prerequisites-1)
  - [3. Concept 1: Rust Packages, Crates, and Modules](#3-concept-1-rust-packages-crates-and-modules)
  - [4. Concept 2: Library vs Binary Crate](#4-concept-2-library-vs-binary-crate)
  - [5. Concept 3: Visibility (`pub`) and Re-exports](#5-concept-3-visibility-pub-and-re-exports)
  - [6. Concept 4: Documentation (`///` & `cargo doc`)](#6-concept-4-documentation--cargo-doc)
  - [7. Concept 5: Unit Tests (`#[test]` & `cargo test`)](#7-concept-5-unit-tests-test--cargo-test)
  - [8. Concept 6: Command-Line Arguments with `clap`](#8-concept-6-command-line-arguments-with-clap)
  - [9. Putting It All Together: Building the Library](#9-putting-it-all-together-building-the-library)
  - [10. Putting It All Together: Building the Binary](#10-putting-it-all-together-building-the-binary)
  - [11. Running the Game](#11-running-the-game)
  - [12. Additional Cargo Tricks](#12-additional-cargo-tricks)
  - [13. Summary of New Concepts](#13-summary-of-new-concepts)

### 1. Introduction

In the first Mastermind workshop, you wrote the whole game in a single `main.rs` file. That's fine for a small program, but real-world Rust projects are organised into **libraries** and **binaries**. Libraries can be shared, tested independently, and documented.

In this advanced workshop, you will:

- Split the code into a **library crate** (`src/lib.rs`) that contains all game logic.
- Keep the user interaction in a **binary crate** (`src/main.rs`) that uses the library.
- Add documentation and unit tests to the library.
- Use an external crate (`clap`) for command-line argument parsing.

### 2. Prerequisites

- Completed the first Mastermind workshop (or understand basic Rust: structs, methods, `Vec`, `Option`, etc.)
- A working `mastermind` Cargo project

### 3. Concept 1: Rust Packages, Crates, and Modules

Rust organises code into a hierarchy of **packages**, **crates**, and **modules**.

- A **package** is a Cargo project (the `Cargo.toml` folder). It can contain one or more **crates**.
- A **crate** is a compilation unit: a **binary crate** produces an executable, a **library crate** produces a `.rlib` file that can be used by other crates.
- **Modules** (`mod`) group related items and control privacy. The module tree is rooted in `crate`.

A typical package layout:

```
mastermind/
â”œâ”€â”€ Cargo.toml
â””â”€â”€ src/
    â”œâ”€â”€ main.rs      â† binary crate root (by default)
    â””â”€â”€ lib.rs       â† library crate root (if present)
```

```rust
// src/lib.rs
pub mod game;      // tells Rust to look for src/game.rs

// src/game.rs
pub struct Game { ... }
```

**Python Comparison:** Python uses files as modules and folders as packages (with `__init__.py`). Rust's `mod` declares a module explicitly.

### 4. Concept 2: Library vs Binary Crate

When you have both `lib.rs` and `main.rs`, Cargo builds:
- The library crate (named as the package, `mastermind`)
- A binary crate (also named `mastermind` by default)

The binary crate can use the library crate via `use mastermind::...`. The library crate **cannot** depend on the binary crate.

**Applying to Workshop:** We will move the `SecretCode` and `MastermindGame` structs into the library (`src/lib.rs`), and keep only the input/output and game loop in `main.rs`.

### 5. Concept 3: Visibility (`pub`) and Re-exports

By default, everything in Rust is **private** to its current module. To expose items to the outside world, prefix with `pub`.

Re-exports allow you to create a convenient public API surface:

```rust
// lib.rs
mod game;
pub use game::MastermindGame;   // now users can do `mastermind::MastermindGame`

// game.rs
pub struct MastermindGame { ... }
```

**Python Comparison:** In Python, all top-level identifiers are public by convention; Rust requires explicit `pub`.

### 6. Concept 4: Documentation (`///` & `cargo doc`)

Rust has built-in documentation generation. Comments starting with `///` document the item that follows, and `//!` documents the enclosing module/crate.

Run `cargo doc --open` to build and view the documentation in your browser.

```rust
/// Represents the hidden 4-digit code.
///
/// # Examples
/// ```
/// let code = SecretCode::new();
/// ```
pub struct SecretCode { ... }
```

### 7. Concept 5: Unit Tests (`#[test]` & `cargo test`)

Testing is first-class in Rust. Write tests in the same file as the code, inside a `mod tests` guarded by `#[cfg(test)]`.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_guess() {
        let code = SecretCode::from_digits(vec![1,2,3,4]);
        let (g, y, r) = code.evaluate_guess("1243");
        assert_eq!(g, 2);
        assert_eq!(y, 2);
        assert_eq!(r, 0);
    }
}
```

### 8. Concept 6: Command-Line Arguments with `clap`

Add to `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

Then use derive macros to define arguments:

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Maximum number of attempts
    #[arg(short, long, default_value_t = 20)]
    max_attempts: u32,
}
```

### 9. Putting It All Together: Building the Library

Now rewrite the Mastermind code as a library crate with proper structure.

#### Create the files

Inside `src/`, we'll have:
- `lib.rs` â€“ the library root
- `secret.rs` â€“ the `SecretCode` module
- `game.rs` â€“ the `MastermindGame` module

#### `secret.rs`

Move the `SecretCode` struct and its `impl` from the old code into `secret.rs`:

```rust
use rand::seq::SliceRandom;
use rand::rng;

pub struct SecretCode {
    digits: Vec<u8>,
    revealed_positions: Vec<bool>,
    revealed_digits: Vec<bool>,
}

impl SecretCode {
    pub fn new() -> Self {
        let mut rng = rng();
        let mut pool: Vec<u8> = (0..=9).collect();
        pool.shuffle(&mut rng);
        let digits = pool[..4].to_vec();

        SecretCode {
            digits,
            revealed_positions: vec![false; 4],
            revealed_digits: vec![false; 10],
        }
    }

    pub fn from_digits(digits: Vec<u8>) -> Self {
        SecretCode {
            digits,
            revealed_positions: vec![false; 4],
            revealed_digits: vec![false; 10],
        }
    }

    pub fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) {
        let guess_digits: Vec<u8> = guess
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let green = self.digits
            .iter()
            .zip(guess_digits.iter())
            .filter(|(s, g)| s == g)
            .count();

        let mut secret_unmatched = Vec::new();
        let mut guess_unmatched = Vec::new();
        for (s, g) in self.digits.iter().zip(guess_digits.iter()) {
            if s != g {
                secret_unmatched.push(*s);
                guess_unmatched.push(*g);
            }
        }

        let mut yellow = 0;
        for g in &guess_unmatched {
            if let Some(pos) = secret_unmatched.iter().position(|&x| x == *g) {
                yellow += 1;
                secret_unmatched.remove(pos);
            }
        }

        let red = 4 - green - yellow;
        (green, yellow, red)
    }

    pub fn can_give_position_hint(&self) -> bool {
        self.revealed_positions.iter().any(|&r| !r)
    }

    pub fn can_give_digit_hint(&self) -> bool {
        self.revealed_digits.iter().any(|&r| !r)
    }

    pub fn give_position_hint(&mut self) -> Option<(usize, u8)> {
        if !self.can_give_position_hint() {
            return None;
        }
        let available: Vec<usize> = self.revealed_positions
            .iter()
            .enumerate()
            .filter(|(_, &revealed)| !revealed)
            .map(|(i, _)| i)
            .collect();
        let mut rng = rng();
        let chosen = *available.choose(&mut rng).unwrap();
        self.revealed_positions[chosen] = true;
        Some((chosen, self.digits[chosen]))
    }

    pub fn give_digit_hint(&mut self) -> Option<u8> {
        if !self.can_give_digit_hint() {
            return None;
        }
        let available: Vec<usize> = self.digits
            .iter()
            .enumerate()
            .filter(|(_, &d)| !self.revealed_digits[d as usize])
            .map(|(i, _)| i)
            .collect();
        let mut rng = rng();
        let chosen_idx = *available.choose(&mut rng).unwrap();
        let digit = self.digits[chosen_idx];
        self.revealed_digits[digit as usize] = true;
        Some(digit)
    }

    pub fn reveal(&self) -> String {
        self.digits.iter().map(|d| d.to_string()).collect::<Vec<_>>().join("")
    }
}
```

#### `game.rs`

Move `MastermindGame` and constants. Note that input/output is left in the binary crate:

```rust
use crate::secret::SecretCode;

pub struct MastermindGame {
    secret: SecretCode,
    attempts_left: u32,
    pub guess_count: u32,
}

impl MastermindGame {
    pub fn new(max_attempts: u32) -> Self {
        MastermindGame {
            secret: SecretCode::new(),
            attempts_left: max_attempts,
            guess_count: 0,
        }
    }

    pub fn attempts_left(&self) -> u32 {
        self.attempts_left
    }

    pub fn submit_guess(&mut self, guess: &str) -> Option<(usize, usize, usize)> {
        if self.attempts_left == 0 {
            return None;
        }
        self.guess_count += 1;
        let feedback = self.secret.evaluate_guess(guess);
        if feedback.0 == 4 {
            self.attempts_left = 0;
        } else {
            self.attempts_left -= 1;
        }
        Some(feedback)
    }

    pub fn can_use_hints(&self) -> bool {
        self.attempts_left > 0
    }

    pub fn can_give_position_hint(&self) -> bool {
        self.secret.can_give_position_hint()
    }

    pub fn can_give_digit_hint(&self) -> bool {
        self.secret.can_give_digit_hint()
    }

    pub fn give_position_hint(&mut self) -> Option<(usize, u8)> {
        self.secret.give_position_hint()
    }

    pub fn give_digit_hint(&mut self) -> Option<u8> {
        self.secret.give_digit_hint()
    }

    pub fn deduct_attempts(&mut self, cost: u32) {
        self.attempts_left = self.attempts_left.saturating_sub(cost);
    }

    pub fn reveal(&self) -> String {
        self.secret.reveal()
    }
}
```

#### `lib.rs`

```rust
pub mod secret;
pub mod game;

pub use game::MastermindGame;
pub use secret::SecretCode;
```

### 10. Putting It All Together: Building the Binary

Add `clap` to `Cargo.toml`:

```toml
[dependencies]
rand = "0.10"
clap = { version = "4", features = ["derive"] }
```

Now write `main.rs`:

```rust
use clap::Parser;
use mastermind::MastermindGame;
use std::io::{self, Write};

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 20)]
    max_attempts: u32,
}

fn main() {
    let args = Args::parse();
    let mut game = MastermindGame::new(args.max_attempts);
    display_welcome(args.max_attempts);

    while game.attempts_left() > 0 {
        println!("\nAttempts left: {}", game.attempts_left());
        let input = get_user_input();

        if input == "help" {
            handle_hint(&mut game);
            continue;
        }

        if let Some((green, yellow, red)) = game.submit_guess(&input) {
            display_feedback(green, yellow, red);
            if green == 4 {
                println!(
                    "\nCongratulations! You cracked the code in {} actual guesses.",
                    game.guess_count
                );
                return;
            }
        }
    }

    println!("\nGame Over! The secret code was {}.", game.reveal());
}

fn display_welcome(max_attempts: u32) {
    println!("{}", "=".repeat(40));
    println!("   Welcome to Mastermind!");
    println!("   Guess the 4-digit code (digits 0-9, no repeats)");
    println!("   You have {} attempts. Type 'help' for hints.", max_attempts);
    println!("{}", "=".repeat(40));
}

fn display_feedback(green: usize, yellow: usize, red: usize) {
    println!("Green: {}   Yellow: {}   Red: {}", green, yellow, red);
}

fn get_user_input() -> String {
    loop {
        print!("Enter guess (or 'help'): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input == "help" {
            return input;
        }

        if input.len() != 4 || !input.chars().all(|c| c.is_ascii_digit()) {
            println!("Invalid input. Please enter exactly 4 digits (e.g., 1234).");
            continue;
        }
        if !has_unique_digits(&input) {
            println!("Digits must be unique (no repeats). Try again.");
            continue;
        }
        return input.to_string();
    }
}

fn has_unique_digits(s: &str) -> bool {
    let mut seen = [false; 10];
    for ch in s.chars() {
        let digit = ch.to_digit(10).unwrap() as usize;
        if seen[digit] {
            return false;
        }
        seen[digit] = true;
    }
    true
}

fn handle_hint(game: &mut MastermindGame) {
    const HINT_POSITION_COST: u32 = 5;
    const HINT_DIGIT_COST: u32 = 3;

    if !game.can_use_hints() {
        println!("No attempts left to use hints.");
        return;
    }

    let pos_available = game.can_give_position_hint();
    let dig_available = game.can_give_digit_hint();

    if !pos_available && !dig_available {
        println!("All hints already revealed.");
        return;
    }

    println!("\n--- Hint Menu ---");
    if pos_available {
        println!("1. Reveal one digit and its correct position (costs {} attempts)", HINT_POSITION_COST);
    } else {
        println!("1. (No more position hints available)");
    }
    if dig_available {
        println!("2. Reveal a correct digit (costs {} attempts)", HINT_DIGIT_COST);
    } else {
        println!("2. (No more digit hints available)");
    }

    print!("Choose 1 or 2 (or press Enter to cancel): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    if choice.is_empty() {
        return;
    }

    if choice == "1" && pos_available {
        if game.attempts_left() < HINT_POSITION_COST {
            println!("Not enough attempts.");
            return;
        }
        if let Some((pos, digit)) = game.give_position_hint() {
            game.deduct_attempts(HINT_POSITION_COST);
            println!("Hint: Digit {} is at position {}.", digit, pos + 1);
        }
    } else if choice == "2" && dig_available {
        if game.attempts_left() < HINT_DIGIT_COST {
            println!("Not enough attempts.");
            return;
        }
        if let Some(digit) = game.give_digit_hint() {
            game.deduct_attempts(HINT_DIGIT_COST);
            println!("Hint: The code contains the digit {}.", digit);
        }
    } else {
        println!("Invalid choice.");
    }
}
```

### 11. Running the Game

```bash
cargo run
# Or with custom attempts:
cargo run -- --max-attempts 10
```

### 12. Additional Cargo Tricks

- **Documentation**: Run `cargo doc --open` to see your library's documentation generated from the doc comments.
- **Testing**: Add unit tests inside `secret.rs` and `game.rs` and run `cargo test`.
- **Workspaces**: If you later have multiple related crates, a workspace `[workspace]` in `Cargo.toml` ties them together.
- **Features**: You can define optional dependencies in `Cargo.toml` and gate code with `#[cfg(feature = "foo")]`.
- **Publishing**: If you wanted to share your library on crates.io, you'd run `cargo publish`.

### 13. Summary of New Concepts

| Concept | Where Used |
|---------|------------|
| Package layout (`lib.rs` + `main.rs`) | Whole project |
| Modules (`mod`, file organisation) | `secret.rs`, `game.rs`, `lib.rs` |
| `pub` visibility and re-exports | `pub struct`, `pub use` in `lib.rs` |
| Documentation (`///`, `//!`, `cargo doc`) | Above structs and functions |
| Unit tests (`#[test]`, `#[cfg(test)]`, `cargo test`) | (suggested practice) |
| `clap` for CLI argument parsing | `main.rs` with `Args` struct |
| `derive` macros (`#[derive(Parser)]`) | On `Args` |
| `saturating_sub` | In `deduct_attempts` |
| Separation of concerns (logic in lib, I/O in bin) | Moved game logic to library |
