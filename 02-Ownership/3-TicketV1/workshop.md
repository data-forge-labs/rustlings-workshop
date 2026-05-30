# Rust for Python Data Engineers — TicketV1: Structs & Ownership

*The most important workshop in this course. Master Rust's ownership system — the concept that makes Rust unique — by building a ticket tracking system.*

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Prerequisites](#2-prerequisites)
3. [Running the Python Version](#3-running-the-python-version)
4. [Concept: Structs — Rust's Custom Types](#4-concept-structs--rusts-custom-types)
5. [Concept: Methods with `impl`](#5-concept-methods-with-impl)
6. [Concept: Validation — Enforcing Invariants](#6-concept-validation--enforcing-invariants)
7. [Concept: Modules and Visibility](#7-concept-modules-and-visibility)
8. [Concept: Encapsulation](#8-concept-encapsulation)
9. [Concept: Ownership — The Key to Rust](#9-concept-ownership--the-key-to-rust)
10. [Concept: Stack vs Heap — Where Data Lives](#10-concept-stack-vs-heap--where-data-lives)
11. [Concept: References and Borrowing](#11-concept-references-and-borrowing)
12. [Concept: Destructors and Drop](#12-concept-destructors-and-drop)
13. [Putting It All Together — The Complete Ticket System](#13-putting-it-all-together--the-complete-ticket-system)
14. [Exercises](#14-exercises)
15. [Summary](#15-summary)

---

## 1. Project Overview

We'll build a **ticket tracking system** (like Jira or Trello) that:

- Creates tickets with title, description, and status
- Validates ticket data (title not empty, status is valid)
- Controls access to fields via getters and setters
- Enforces rules at compile time using Rust's type system

### What You'll Learn

| Rust Concept | Python Equivalent | Why It Matters |
|---|---|---|
| `struct` | `class` (dataclass) | Model data in pipelines |
| `impl` | Methods inside class | Attach behavior to data |
| `pub` / `mod` | Public/private convention | Organize large codebases |
| **Ownership** | Nothing — Python uses GC | **Memory safety without GC** |
| **Borrowing** | Nothing — Python uses GC | **Zero-cost abstractions** |
| Stack vs Heap | `id()` / `is` | Understand performance |
| `Drop` | `__del__` / context manager | Resource cleanup |
| References `&` | Pass-by-reference semantics | Efficient data access |

---

## 2. Prerequisites

- Completed [Basic Calculator](../01-Foundations/2-BasicCalculator/workshop.md)
- Understand integers, `if/else`, loops
- Familiar with `cargo run`, `cargo test`

---

## 3. Running the Python Version

The Python version (`project.py`) shows what we're building:

```bash
cd rustlings-workshop/3-TicketV1
python project.py
```

---

## 4. Concept: Structs — Rust's Custom Types

### From Python Classes to Rust Structs

```python
# Python — class with __init__
class Ticket:
    def __init__(self, title, description, status):
        self.title = title
        self.description = description
        self.status = status
```

```rust
// Rust — struct definition
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

### Creating an Instance

```python
# Python — just call the class
ticket = Ticket("Bug", "Fix this bug", "Open")
```

```rust
// Rust — use struct literal syntax
let ticket = Ticket {
    title: String::from("Bug"),
    description: String::from("Fix this bug"),
    status: String::from("Open"),
};
```

### Accessing Fields

```python
# Python
print(ticket.title)
ticket.title = "New title"
```

```rust
// Rust
println!("{}", ticket.title);
ticket.title = String::from("New title");  // Only if mutable!
```

### Memory Layout of a Struct

```
struct Ticket {
    title: String,       // 24 bytes (ptr + len + cap)
    description: String, // 24 bytes
    status: String,      // 24 bytes
}
// Total: 72 bytes on stack

Stack:
┌──────────────────────────────┐
│ Ticket instance (72 bytes)   │
│ ┌──────────────────────────┐ │
│ │ title:                   │ │
│ │   [ptr] ───→ "Bug" (heap)│ │
│ │   [len:3] [cap:3]        │ │
│ ├──────────────────────────┤ │
│ │ description:             │ │
│ │   [ptr] ───→ "Fix" (heap)│ │
│ │   [len:3] [cap:3]        │ │
│ ├──────────────────────────┤ │
│ │ status:                  │ │
│ │   [ptr] ───→ "Open" (h.) │ │
│ │   [len:4] [cap:4]        │ │
│ └──────────────────────────┘ │
└──────────────────────────────┘
```

### Exercise: Define Your First Struct

```rust
// Step 1: Define the struct
struct Ticket {
    title: String,
    description: String,
    status: String,
}

// Step 2: Create an instance
fn main() {
    let ticket = Ticket {
        title: String::from("Build login page"),
        description: String::from("Create the login page UI"),
        status: String::from("In Progress"),
    };

    // Step 3: Access fields
    println!("Title: {}", ticket.title);
    println!("Status: {}", ticket.status);
}
```

---

## 5. Concept: Methods with `impl`

### Python vs Rust Methods

```python
# Python
class Ticket:
    def __init__(self, title, desc, status):
        self.title = title
        self.desc = desc
        self.status = status

    def is_open(self):      # 'self' is implicit
        return self.status == "Open"
```

```rust
// Rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // Method — takes &self (borrow, don't take ownership)
    fn is_open(&self) -> bool {
        self.status == "Open"
    }

    // Static method — no self (like @classmethod without cls)
    fn default() -> Ticket {
        Ticket {
            title: String::from("Default"),
            description: String::from(""),
            status: String::from("Open"),
        }
    }
}
```

### Calling Methods

```rust
let ticket = Ticket::default();          // Static method call
let open = ticket.is_open();             // Method call
let open2 = Ticket::is_open(&ticket);    // Function call syntax (same thing)
```

### The Three Forms of `self`

| Parameter | Owns? | Can Modify? | Use Case |
|---|---|---|---|
| `self` | Takes ownership (moves) | Yes | Consuming the value |
| `&self` | Borrows (shared ref) | No | Reading only |
| `&mut self` | Borrows (mutable ref) | Yes | Mutating |

```rust
impl Ticket {
    // Read-only access
    fn title(&self) -> &String {
        &self.title
    }

    // Mutable access
    fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    // Consumes the ticket
    fn into_summary(self) -> String {
        format!("{}: {}", self.title, self.status)
        // `self` is gone after this call
    }
}
```

---

## 6. Concept: Validation — Enforcing Invariants

### The Problem

```rust
let ticket = Ticket {
    title: String::from(""),        // Empty title — bad!
    description: String::from(""),
    status: String::from("NOPE"),   // Invalid status — bad!
};
```

### Solution: Constructor with Validation

```rust
impl Ticket {
    fn new(title: String, description: String, status: String) -> Ticket {
        // Validate title
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title too long (max 50 chars)");
        }

        // Validate description
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description too long (max 500 chars)");
        }

        // Validate status
        let valid_statuses = ["Open", "In Progress", "Resolved", "Closed"];
        if !valid_statuses.contains(&status.as_str()) {
            panic!("Invalid status: {}", status);
        }

        Ticket { title, description, status }
    }
}
```

### Python Comparison

```python
# Python — validation in __init__
class Ticket:
    VALID_STATUSES = ["Open", "In Progress", "Resolved", "Closed"]

    def __init__(self, title, description, status):
        if not title:
            raise ValueError("Title cannot be empty")
        if not description:
            raise ValueError("Description cannot be empty")
        if status not in self.VALID_STATUSES:
            raise ValueError(f"Invalid status: {status}")
        self.title = title
        self.description = description
        self.status = status
```

| Python | Rust |
|---|---|
| `raise ValueError` | `panic!("message")` |
| `if not title:` | `if title.is_empty()` |
| `status not in list` | `!list.contains(&status)` |

---

## 7. Concept: Modules and Visibility

### What is a Module?

A module groups related code under a namespace:

```rust
// src/lib.rs
mod ticket {
    pub struct Ticket {
        pub title: String,
        description: String,  // private!
        status: String,       // private!
    }
}
```

### Module File Structure

```
src/
  lib.rs           ← Root module (crate root)
  ticket.rs        ← module `ticket`
  └── ...          ← or ticket/mod.rs with submodules
```

### Visibility Rules

| Keyword | Accessible from |
|---|---|
| `(none)` | Private — only within current module |
| `pub` | Public — accessible everywhere |
| `pub(crate)` | Public within the crate |
| `pub(super)` | Public within parent module |

```rust
mod ticket {
    pub struct Ticket {
        pub title: String,       // Anyone can read/write
        pub(crate) description: String, // Only within this crate
        status: String,          // Private even within crate!
    }

    pub fn create_ticket(title: String) -> Ticket {
        // This function is public — anyone can call it
        Ticket {
            title,
            description: String::new(),
            status: String::from("Open"),
        }
    }
}
```

### Python vs Rust Visibility

```python
# Python — naming convention
class Ticket:
    def __init__(self):
        self.title = "Bug"       # Public
        self._status = "Open"    # "Private" (convention)
        self.__secret = "x"      # Name mangled (still accessible)
```

```rust
// Rust — compiler-enforced
struct Ticket {
    pub title: String,           // Public
    status: String,              // Private — can't access from outside
}
```

> **Key difference:** Python's privacy is by convention. Rust's is enforced by the compiler.

---

## 8. Concept: Encapsulation

### The Problem

Even with `Ticket::new` doing validation, nothing stops someone from:

```rust
let ticket = Ticket {
    title: String::from(""),    // Bypasses validation!
    description: String::from(""),
    status: String::from("Bad"),
};
```

### The Solution

Make fields private so they can **only** be set through the constructor:

```rust
// src/ticket.rs
pub struct Ticket {
    title: String,       // Private — not `pub`
    description: String, // Private
    status: String,      // Private
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        // Validation logic...
        Ticket { title, description, status }
    }
}
```

Now users **must** use `Ticket::new()`:

```rust
// This won't compile:
let t = Ticket { title: "...", description: "...", status: "..." };
// ❌ ERROR: field `title` of struct `Ticket` is private

// This works:
let t = Ticket::new("...".into(), "...".into(), "...".into());
```

### Why Encapsulation Matters for Data Engineers

In data pipelines, you want to ensure data integrity:

```rust
// Encapsulated data row — can only be created with valid data
pub struct DataRow {
    id: u64,
    timestamp: i64,
    value: f64,
}

impl DataRow {
    pub fn new(id: u64, timestamp: i64, value: f64) -> Result<DataRow, String> {
        if timestamp < 0 {
            return Err("Negative timestamp".into());
        }
        if value.is_nan() || value.is_infinite() {
            return Err("Invalid value".into());
        }
        Ok(DataRow { id, timestamp, value })
    }
}
```

---

## 9. Concept: Ownership — The Key to Rust

### What Is Ownership?

**Every value in Rust has exactly one owner.** When the owner goes out of scope, the value is dropped (memory freed).

### The Three Rules

1. **Each value has exactly one owner**
2. **When the owner goes out of scope, the value is dropped**
3. **There can be many readers OR one writer at a time**

### Ownership Transfer (Move)

```python
# Python — multiple references to same object
a = [1, 2, 3]
b = a            # b and a both point to the SAME list
a.append(4)      # b is also affected!
print(b)         # [1, 2, 3, 4]
```

```rust
// Rust — ownership MOVES
let a = vec![1, 2, 3];
let b = a;           // Ownership moves from a to b
// println!("{:?}", a); // ❌ ERROR: a is no longer valid!
println!("{:?}", b);    // ✅ b owns the data now
```

### Visual: Move in Memory

```
Before move:
a ──→ [1, 2, 3] (heap)

After `let b = a;`:
a ──→ (invalid — compiler prevents access)
b ──→ [1, 2, 3] (heap)
```

### Why Does This Matter?

No garbage collector needed! When `b` goes out of scope, the memory is freed instantly — no GC pause.

```python
# Python — GC needs to figure out when to clean up
def process():
    data = load_large_dataset()  # Millions of rows
    result = transform(data)
    return result
    # `data` might be freed now, or later — GC decides
```

```rust
// Rust — deterministic cleanup
fn process() -> Vec<f64> {
    let data = load_large_dataset();  // Millions of rows
    let result = transform(data);    // Ownership moves to transform
    // `data` is gone! transform now owns it
    result  // Ownership moves to caller
}  // result is dropped here (unless caller takes ownership)
```

### The Ticket Problem

```rust
impl Ticket {
    // BAD: takes ownership of self!
    fn title(self) -> String {
        self.title
    }
}

fn main() {
    let ticket = Ticket::new("Bug".into(), "Fix it".into(), "Open".into());
    let title = ticket.title();   // ticket is CONSUMED here!
    // println!("{}", ticket.status()); // ❌ ERROR: ticket was moved!
}
```

### Solution: Borrow Instead of Move

```rust
impl Ticket {
    // GOOD: borrows self (doesn't take ownership)
    fn title(&self) -> &String {
        &self.title
    }
}

fn main() {
    let ticket = Ticket::new("Bug".into(), "Fix it".into(), "Open".into());
    let title = ticket.title();   // ticket is BORROWED, not moved
    println!("{}", ticket.status()); // ✅ Still works!
}
```

---

## 10. Concept: Stack vs Heap — Where Data Lives

### The Stack

- **LIFO** (Last In, First Out)
- Super fast allocation/deallocation
- Size must be known at compile time
- Used for: local variables, function arguments

```
Stack layout when calling nested functions:

    main() calls greet() calls format()

    ┌──────────────────────┐
    │ format's frame       │
    ├──────────────────────┤
    │ greet's frame        │
    │   name: ptr → "Alice"│
    ├──────────────────────┤
    │ main's frame         │
    │   greeting: ptr      │
    └──────────────────────┘
    ↑ stack pointer
```

### The Heap

- Dynamic memory — slower to allocate
- Size can change at runtime
- Used for: `String`, `Vec<T>`, boxed values
- Must be freed when no longer needed

```
Heap layout for a String "Hello":

Stack (3 × 8 bytes = 24 bytes):   Heap:
┌──────────────────────────┐      ┌──────────────────┐
│ String metadata          │      │ 'H' │ 'e' │ 'l'  │
│ [ptr] ───────────────────────→  │ 'l' │ 'o' │      │
│ [len: 5]                  │      └──────────────────┘
│ [cap: 5]                  │
└──────────────────────────┘
```

### Python vs Rust: Memory Management

```python
# Python — everything on heap, GC manages
def process():
    data = [1, 2, 3]  # List object on heap, refcounted
    # ... Python tracks references, frees when refcount=0
```

```rust
// Rust — stack by default, explicit heap
fn process() {
    let x = 42;                     // i32 — stack allocated
    let s = String::from("hello");  // String — metadata on stack, chars on heap
    let v = vec![1, 2, 3];          // Vec — metadata on stack, elements on heap
    // When function returns, x, s, v all cleaned up (no GC needed)
}
```

### `size_of` — How Big Is Each Type?

```rust
println!("i32:    {} bytes", std::mem::size_of::<i32>());      // 4
println!("f64:    {} bytes", std::mem::size_of::<f64>());      // 8
println!("bool:   {} bytes", std::mem::size_of::<bool>());     // 1
println!("String: {} bytes", std::mem::size_of::<String>());   // 24 (on stack)
println!("&String:{} bytes", std::mem::size_of::<&String>());  // 8 (pointer size)
println!("Vec<i32>:{} bytes", std::mem::size_of::<Vec<i32>>());// 24
```

> A `String` is only 24 bytes on the stack — it's the **pointer to the heap** data, plus length and capacity.

---

## 11. Concept: References and Borrowing

### What Is a Reference?

A reference (`&T`) is a **non-owning pointer**. It lets you access data without taking ownership.

```rust
let s = String::from("hello");
let r = &s;           // r BORROWS s — doesn't own it
println!("{}", r);    // "hello"
println!("{}", s);    // ✅ Still valid! s still owns the data
```

### Visual: Reference in Memory

```
let s = String::from("Hey");
let r = &s;

Stack:
┌──────────────────────────────────────┐
│ s (String, 24 bytes):                │
│   ptr:  ──────────────────┐          │
│   len: 3                  │          │
│   cap: 5                  │          │
├──────────────────────────────────────┤
│ r (&String, 8 bytes):     │          │
│   ptr: ───────────────────┘          │
└──────────────────────────────────────┘
          │
          v
Heap:
┌──────────────────────────────┐
│ 'H' │ 'e' │ 'y' │   │   │   │
└──────────────────────────────┘
```

### Mutable References (`&mut`)

```rust
let mut s = String::from("hello");
let r = &mut s;       // Mutable borrow
r.push_str(", world"); // Modify through the reference
println!("{}", r);    // "hello, world"
```

### The Borrowing Rules (MOST IMPORTANT)

> **At any given time, you can have EITHER:**
> - **One mutable reference, OR**
> - **Any number of immutable references**

```rust
let mut s = String::from("hello");

let r1 = &s;     // ✅ One immutable ref
let r2 = &s;     // ✅ Another immutable ref — fine!
println!("{}, {}", r1, r2);
// r1 and r2 are no longer used here

let r3 = &mut s;  // ✅ Mutable ref — fine since r1, r2 are done
r3.push_str(" world");
```

```rust
let mut s = String::from("hello");

let r1 = &s;      // ✅ Immutable borrow
let r2 = &mut s;  // ❌ ERROR: can't borrow as mutable while immutable borrow exists!

println!("{}", r1); // r1 is still live here
```

### Why These Rules?

**Data races** — two threads reading and writing the same memory unpredictably — are eliminated at compile time.

```python
# Python — possible data race (GIL helps but doesn't eliminate)
def update(list, index, value):
    list[index] = value  # What if another thread is reading?

# This runs fine until it doesn't
```

```rust
// Rust — data race caught at compile time
let mut data = vec![1, 2, 3];
let r1 = &data;
let r2 = &mut data;  // ❌ Compiler: "You already have an immutable borrow!"
// If this compiled, r1 could see data change under it
```

### The Three Forms of `self` Revisited

```rust
impl Ticket {
    // 1. Immutable borrow: read-only, can have many
    fn title(&self) -> &String { &self.title }

    // 2. Mutable borrow: read+write, exclusive
    fn set_title(&mut self, new: String) { self.title = new; }

    // 3. Move: consumes self, can't use afterward
    fn consume(self) -> String { self.title }
}
```

| Method signature | Can call on `ticket: Ticket` | Can call on `&ticket` | Can call on `&mut ticket` |
|---|---|---|---|
| `fn title(&self)` | ✅ | ✅ | ✅ |
| `fn set_title(&mut self)` | ✅ (but can't use after) | ❌ | ✅ |
| `fn consume(self)` | ✅ (but can't use after) | ❌ | ❌ |

---

## 12. Concept: Destructors and Drop

### Scopes and Lifetime

```rust
fn main() {
    let x = 5;     // x's scope starts
    {              // inner block starts
        let y = 10;  // y's scope starts
        println!("{}", y);
    }              // y goes out of scope — DROPPED
    println!("{}", x);
}                  // x goes out of scope — DROPPED
```

### The `Drop` Trait

Rust calls `drop` on every value when it goes out of scope:

```rust
// Equivalent to what the compiler does:
fn main() {
    let x = String::from("hello");
    let y = String::from("world");
    drop(y);  // compiler inserts this at end of scope
    drop(x);  // in reverse order of declaration
}
```

### Python Comparison

```python
# Python — __del__ is unreliable
class File:
    def __del__(self):    # May never be called!
        self.handle.close()
```

```rust
// Rust — Drop is deterministic
struct File {
    handle: i32,
}

impl Drop for File {
    fn drop(&mut self) {
        println!("File {} closed immediately!", self.handle);
    }
}

fn main() {
    let f = File { handle: 42 };
    // ... use file ...
    // f is dropped HERE, not whenever GC decides
}
```

### For Data Engineers: Predictable Cleanup

```rust
struct Dataset {
    rows: Vec<Vec<f64>>,
    path: String,
}

impl Drop for Dataset {
    fn drop(&mut self) {
        println!("Closing dataset: {}", self.path);
        self.rows.clear();  // Free memory immediately
    }
}
```

No `with` statement, no `finally` block — Rust handles it automatically.

---

## 13. Putting It All Together — The Complete Ticket System

### Project Structure

```
3-TicketV1/
  src/
    lib.rs       ← Public API, re-exports
    ticket.rs    ← Ticket struct and implementation
  Cargo.toml
```

### `src/ticket.rs`

```rust
/// Represents a ticket in the tracking system.
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    /// Create a new Ticket with validation.
    pub fn new(title: String, description: String, status: String) -> Ticket {
        // Validate title
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title too long (max 50 chars)");
        }
        if title.contains('\n') {
            panic!("Title cannot contain newlines");
        }

        // Validate description
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description too long (max 500 chars)");
        }

        // Validate status
        let valid = ["Open", "In Progress", "Resolved", "Closed"];
        if !valid.contains(&status.as_str()) {
            panic!("Invalid status: '{}'", status);
        }

        Ticket { title, description, status }
    }

    // Getters — borrow, don't own
    pub fn title(&self) -> &String { &self.title }
    pub fn description(&self) -> &String { &self.description }
    pub fn status(&self) -> &String { &self.status }

    // Setter — mutable borrow
    pub fn set_title(&mut self, title: String) {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title too long (max 50 chars)");
        }
        self.title = title;
    }

    pub fn set_description(&mut self, description: String) {
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description too long (max 500 chars)");
        }
        self.description = description;
    }

    pub fn set_status(&mut self, status: String) {
        let valid = ["Open", "In Progress", "Resolved", "Closed"];
        if !valid.contains(&status.as_str()) {
            panic!("Invalid status: '{}'", status);
        }
        self.status = status;
    }
}
```

### `src/lib.rs`

```rust
mod ticket;

pub use ticket::Ticket;
```

### `src/main.rs`

```rust
use ticket_system::Ticket;

fn main() {
    // Create a validated ticket
    let mut ticket = Ticket::new(
        String::from("Fix login bug"),
        String::from("Users cannot log in with SSO"),
        String::from("Open"),
    );

    // Read through getters
    println!("Title: {}", ticket.title());
    println!("Status: {}", ticket.status());

    // Update through setters
    ticket.set_status(String::from("In Progress"));
    println!("New status: {}", ticket.status());

    // Borrow checker in action
    let title = ticket.title();       // Immutable borrow
    let status = ticket.status();     // Another immutable borrow — OK
    println!("{title} — {status}");
    // Immutable borrows end here

    ticket.set_status(String::from("Resolved"));  // Mutable borrow — OK now
}
```

---

## 14. Exercises

### Exercise 1: Fix the Move Error

```rust
struct Ticket {
    title: String,
}

impl Ticket {
    // Fix this method so it doesn't consume the ticket
    fn title(self) -> String {
        self.title
    }
}

fn main() {
    let t = Ticket { title: String::from("Bug") };
    let title = t.title();
    println!("{:?}", t.title());  // ❌ ERROR: t was moved
}
```

<details>
<summary>Solution</summary>

```rust
fn title(&self) -> &String {
    &self.title
}
```
</details>

### Exercise 2: Add a Priority Field

Add a `priority` field to `Ticket`:

1. Add `priority: String` to the struct
2. Validate it in `new()` — must be "Low", "Medium", or "High"
3. Add getter and setter
4. Ensure encapsulation (field is private)

<details>
<summary>Solution</summary>

```rust
pub struct Ticket {
    title: String,
    description: String,
    status: String,
    priority: String,  // NEW
}

impl Ticket {
    pub fn new(title: String, description: String, status: String, priority: String) -> Ticket {
        // ... existing validation ...
        let valid_priorities = ["Low", "Medium", "High"];
        if !valid_priorities.contains(&priority.as_str()) {
            panic!("Invalid priority: '{}'", priority);
        }
        Ticket { title, description, status, priority }
    }

    pub fn priority(&self) -> &String { &self.priority }

    pub fn set_priority(&mut self, priority: String) {
        let valid = ["Low", "Medium", "High"];
        if !valid.contains(&priority.as_str()) {
            panic!("Invalid priority: '{}'", priority);
        }
        self.priority = priority;
    }
}
```
</details>

### Exercise 3: Ownership Chain

Trace the ownership in this code — what's valid and what's not?

```rust
fn consume_string(s: String) -> usize {
    s.len()
    // s is dropped here
}

fn main() {
    let a = String::from("hello");
    let b = a;                     // What happens to a?
    let len = consume_string(b);   // What happens to b?
    // Can we use a here? Can we use b here?
    println!("Length: {}", len);
}
```

<details>
<summary>Answer</summary>

```rust
let a = String::from("hello");
let b = a;                     // a's ownership MOVES to b. a is dead.
let len = consume_string(b);   // b's ownership MOVES to consume_string. b is dead.
// Can't use a or b — both were moved
println!("Length: {}", len);   // ✅ len is valid (copy)
```
</details>

---

## 15. Summary

### Concept Reference

| Concept | Description | Python Equivalent |
|---|---|---|
| `struct` | Define a custom type | `class` / `dataclass` |
| `impl` | Attach methods to a struct | Methods inside class |
| `pub` / private | Visibility control | Naming convention (`_`) |
| **Ownership** | Each value has exactly one owner | Nothing (GC) |
| **Move** | Transfer ownership | `b = a` (both reference same object) |
| **Borrow** (`&`) | Temporary, non-owning access | Pass-by-reference |
| **Mutable borrow** (`&mut`) | Exclusive, writable access | N/A (everything can be mutated) |
| **Scope** | Region where a variable is alive | Variable lifetime |
| **Drop** | Cleanup when owner goes out of scope | `__del__` (unreliable) |
| **Stack** | Fast, fixed-size memory | Local variables |
| **Heap** | Dynamic, slower memory | All objects |

### The Ownership Rules (Memorize These)

1. **Each value has exactly one owner**
2. **Either: one mutable reference, OR many immutable references**
3. **References must always be valid** (no dangling pointers)

### Next Project

Proceed to [4-Traits](../02-Ownership/4-Traits/workshop.md) to learn about **traits** — Rust's version of interfaces and protocols.
