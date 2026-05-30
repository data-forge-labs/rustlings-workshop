# Rust for Python Data Engineers — MasterMind

*A hands-on workshop that teaches Strings, Vectors, Structs, Option, Iterators, and I/O by building a MasterMind code-breaking game.*

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

- Completed [Basic Calculator](../01-Foundations/2-BasicCalculator/workshop.md)
- Rust installed and working
- Basic familiarity with `cargo run`

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

### Next Project

Proceed to [3-TicketV1](../02-Ownership/3-TicketV1/workshop.md) to master **ownership** — Rust's most important and unique concept.
