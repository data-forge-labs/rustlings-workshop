# Rust for Python Data Engineers — MasterMind

*A hands-on workshop that teaches Strings, Vectors, Structs, Option, Iterators, and I/O by building a MasterMind code-breaking game.*

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 30 tests pass**.

---

## Why This Project?

### The Problem

In data engineering, you constantly deal with **collections of structured data** — rows in a DataFrame, records in JSON, chunks of a CSV file. You need to model real-world entities (a "Record", a "Customer", a "Transaction") with clear fields and behaviors. You need to handle **missing data** gracefully. And you need to process text efficiently without performance surprises.

Python makes this easy — almost too easy:

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

In this project, you'll build a **MasterMind code-breaking game** — a perfect vehicle for learning Rust's solutions to these exact problems. You'll model game state with `struct`, handle missing guesses with `Option` (no more `None` surprises), process characters with iterators, and distinguish between `String` (owned, growable) and `&str` (borrowed, fixed).

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

// String vs &str — be explicit about ownership
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
| 10 | `rand` crate | `rand::thread_rng().gen_range()` | `random.randint()` | Random number generation |
| 11 | `pub` visibility | `pub fn`, `pub struct` | Public by default | Control the public API surface |
| 12 | `Self` constructor | `fn new(...) -> Self` | `__init__(self)` | Idiomatic constructor pattern |

## Concepts at a Glance

### 1. `String` vs `&str`
`String` is an owned, heap-allocated, growable UTF-8 string. `&str` is a borrowed view — the default for function parameters that only need to read text. **Python:** one `str` type, always immutable.

### 2. `Vec<T>`
`vec![1, 2, 3]` creates a typed, growable array. Use `.push()` to add, `[i]` to index. **Python:** `list` — but Rust's `Vec` is typed; you can't mix unrelated types.

### 3. `struct`
`struct Guess { value: String }` defines a new data type with named fields, validated at compile time. **Python:** `@dataclass class Guess: value: str`.

### 4. `impl` blocks
`impl Guess { fn new(v: String) -> Self { ... } }` attaches methods to a struct. **Python:** methods live inside the `class` body. Rust separates data and behavior.

### 5. `Option<T>`
`Option<T>` represents a value that may be present (`Some(val)`) or absent (`None`). The compiler forces you to check. **Python:** `None` can appear anywhere without warning.

### 6. `match`
`match value { Some(x) => x * 2, None => 0 }` performs exhaustive pattern matching. The compiler checks every variant is handled. **Python:** `if x is not None: ... else: ...` — no compiler verification.

### 7. Ownership basics
When a value is assigned to a new variable or passed to a function, ownership moves. Use `&` to borrow. **Python:** everything is reference-counted — no ownership concept.

### 8. Iterators
`s.chars()` returns an iterator over characters; `.enumerate()` pairs each element with its index. Iterators are lazy and composable. **Python:** `for ch in s:` and `enumerate(s)` work similarly but are eager.

### 9. Console I/O
`io::stdin().read_line(&mut buf)` reads a line into a buffer. `println!` writes to stdout. **Python:** `input()` and `print()`.

### 10. `rand` crate
`rand::thread_rng().gen_range(10..=99)` generates a random number. Add `rand = "0.8"` to `Cargo.toml`. **Python:** `random.randint(10, 99)`.

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
5. [Step-by-Step Guide](#5-step-by-step-guide)
6. [Summary](#6-summary)

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

This project has a **detailed pre-existing guide** in `master_mind.md` (981 lines). Here's the quick path:

1. **Read the concept overview** in Section 4 below — maps each Rust concept to Python
2. **Follow the detailed guide** in [master_mind.md](./master_mind.md) for the full step-by-step implementation
3. **Build the game** yourself, referring back to concepts as needed

---

## 4. Python vs Rust Concepts in This Project

### Strings: `String` vs `&str`

```python
# Python — one string type
name = "Alice"
name += " Smith"   # Creates a new string
```

```rust
// Rust — two string types
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
# Python — dynamic list
fruits = ["apple", "banana"]
fruits.append("cherry")
```

```rust
// Rust — typed vector
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
# Python — None handling
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
// Rust — Option + match
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

## 5. Step-by-Step Guide

Follow the detailed guide in [master_mind.md](./master_mind.md) to build the game. Key sections:

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
| Ownership | Function parameters — know when to move vs borrow |
| `Vec<char>` | Store the 4-digit code as vector of characters |
| `struct Guess` | Model a single guess with validation |
| `struct Game` | Model the game state (secret, attempts) |
| `impl` | Methods on `Guess` and `Game` |
| `Option<&str>` | Parse input, may fail |
| `match` | Branch on `Option` variants |
| Iterators | `chars()`, `enumerate()` for digit comparison |
| Console I/O | `io::stdin().read_line()`, `println!` |
| `rand` crate | Generate random secret code |

### Further Reading

The following lesson files in this folder provide deeper coverage:

| File | Topics |
|------|--------|
| [master_mind.md](./master_mind.md) | Full 981-line step-by-step guide with all concept explanations |
| [master-mind-advanced.md](./master-mind-advanced.md) | Advanced features and extensions for the game |

### Next Project

Proceed to [3-TicketV1](../02-Ownership/01-TicketV1/README.md) to master **ownership** — Rust's most important and unique concept.
