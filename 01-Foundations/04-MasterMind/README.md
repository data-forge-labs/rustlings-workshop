# Rust for Python Data Engineers — MasterMind

> **Test-driven approach**: This project includes two Cargo projects with progressive unit tests. The **basic** workshop (`workshop/`) implements the core game; the **advanced** workshop (`workshop/advanced/`) adds modules, CLI args with `clap`, and documentation. Each function in `src/lib.rs` starts as a `todo!()` stub. Run `cd workshop && cargo test` (basic) or `cd workshop/advanced && cargo test` (advanced) to watch the pass count grow. Your goal: **all 30 tests pass (basic) and all tests pass (advanced)**.

---

## What Is This Game?

A classic code-breaking game where the computer generates a 4-digit secret code and you guess it with bull/cow feedback. It introduces `struct`, `Vec`, `Option`, and exhaustive `match`.

### Python equivalent

```python
from enum import Enum

class Status(Enum):
    OPEN = "open"
    CLOSED = "closed"

def evaluate_guess(secret, guess):
    green = sum(s == g for s, g in zip(secret, guess))
    return green

# No compile-time check on status values or field names
record = {"status": "open", "value": 42}
```

In this project you'll learn to build this in Rust — and along the way
you'll discover **structs with `impl`**, **`Vec<T>`**, **`Option<T>`**, **exhaustive `match`**, and **iterators**.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | `String` vs `&str` | Owned (heap) vs borrowed (fixed) |
| 2 | `Vec<T>` | Dynamic typed array |
| 3 | `struct` + `impl` | Custom data types with methods |
| 4 | `Option<T>` | Nullable values the compiler forces you to check |
| 5 | `match` | Exhaustive pattern dispatch |
| 6 | Ownership basics | `&self` — memory safety without GC |
| 7 | Iterators | `.chars()`, `.enumerate()`, `.filter()` |
| 8 | Console I/O | `read_line`, `println!` |
| 9 | `rand` crate | Random shuffling |
| 10 | `pub` visibility | Items private unless explicitly exported |

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Prerequisites](#2-prerequisites)
3. [How to Use This Workshop](#3-how-to-use-this-workshop)
4. [Concept: `String` vs `&str` (Deeper Dive)](#4-concept-string-vs-str-deeper-dive)
5. [Concept: `Vec<T>` — Dynamic Arrays](#5-concept-vect-dynamic-arrays)
6. [Concept: `struct` and `impl` — Custom Data Types](#6-concept-struct-and-impl-custom-data-types)
7. [Concept: `Option<T>` — Handling Missing Data](#7-concept-optiont-handling-missing-data)
8. [Concept: Iterators and Closures](#8-concept-iterators-and-closures)
9. [Concept: `&self` vs `&mut self` — Method Receivers](#9-concept-self-vs-mut-self-method-receivers)
10. [Concept: `pub` Visibility](#10-concept-pub-visibility)
11. [Detailed Step-by-Step Guide (Basic)](#11-detailed-step-by-step-guide-basic)
12. [Advanced Exercise Guide](#12-advanced-exercise-guide)
13. [Summary](#13-summary)

---

## 1. Project Overview

MasterMind is a classic code-breaking game:
- The computer generates a secret 4-digit code
- The player guesses the code
- After each guess, the computer gives feedback: correct digits in the right position (A), and correct digits in the wrong position (B)
- The player wins by guessing the code in as few tries as possible

---

## 2. Prerequisites

- Completed [01-Intro](../01-Intro/README.md) — variables, mutability, `if`/`else`, loops, tuples, arrays
- Completed [02-GuessGame](../02-GuessGame/README.md) — `String` vs `&str`, `enum`, `Result`, `.parse()`, `match`
- Completed [03-BasicCalculator](../03-BasicCalculator/README.md) — integer types, overflow, `#[test]`
- Rust installed and working
- Basic familiarity with `cd workshop && cargo run`

---

## 3. How to Use This Workshop

This project has two separate workshops:

### Basic — `workshop/`

Build the core MasterMind game with structs, Vec, Option, and iterators. Start here.

1. **Read the concept sections below** (Sections 5–10) — each teaches a Rust concept with Python comparison, standalone example, and MasterMind application
2. **Follow the detailed guide** in [Section 11](#11-detailed-step-by-step-guide-basic) for the full step-by-step implementation
3. **Build the game** with `cd workshop && cargo run`

### Advanced — `workshop/advanced/`

Refactor the game into a library + binary crate with `clap` CLI args and documentation. Complete the basic version first.

1. **Read** [Section 12](#12-advanced-exercise-guide) for module organization, `clap`, and doc concepts
2. **Browse the stub files** in `workshop/advanced/src/` (lib.rs, main.rs, secret.rs, game.rs)
3. **Build** with `cd workshop/advanced && cargo run -- --max-attempts 15`

---

## 4. Concept: `String` vs `&str` (Deeper Dive)

You met `String` and `&str` briefly in [02-GuessGame §5](../02-GuessGame/README.md#5-concept-string-vs-str). This section deepens the distinction for data-engineering work.

### Explanation

Rust has two string types that you'll use together constantly:

| Type | Owns its data? | Grows? | Use for |
|------|----------------|--------|---------|
| `String` | Yes (heap) | Yes | Building, modifying, returning new text |
| `&str` | No (borrowed view) | No | Reading text, function parameters |

### Example (standalone)

```rust
fn main() {
    // String literal → &str (points into the binary's read-only memory)
    let greeting: &str = "hello";

    // Owned, growable String
    let mut name: String = String::from("Alice");
    name.push_str(" Smith");          // grows the heap buffer
    let combined: String = format!("{}, {}!", greeting, name);

    // &str can borrow from String
    let slice: &str = &name[0..5];    // "Alice"
    println!("{} | {} | {}", slice, name, combined);
}
```

### Python comparison

```python
# Python — one type, all behaviors
name = "Alice"
name += " Smith"      # creates a new str, rebinds
greeting = "hello"
combined = f"{greeting}, {name}!"
print(combined)
```

In Python, `str` is always immutable; the runtime allocates a new string each time you "modify" it. Rust gives you a **choice**: cheap borrowed views (`&str`) when you only read, owned buffers (`String`) when you need to grow.

### Applying to our project

In MasterMind, the player's guess enters as a `String` (mutable buffer for `read_line`), and every function that *reads* the guess takes `&str` — that way it works with string literals in tests too:

```rust
// In lib.rs
pub fn has_unique_digits(s: &str) -> bool { ... }       // accepts both
pub fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) { ... }

// In main.rs
let mut input = String::new();       // owned, growable buffer
io::stdin().read_line(&mut input).unwrap();
if has_unique_digits(input.trim()) {  // &str coercion: &String → &str
    let (g, y, r) = secret.evaluate_guess(input.trim());
    ...
}
```

The pattern: **read into `String`, pass `&str` to validators**. This is the idiomatic shape for any text-processing pipeline you'll build in Rust.

### Common pitfalls

- `s.len()` returns **bytes**, not characters. `"café".len() == 5` because `é` is 2 bytes in UTF-8. For character counts use `s.chars().count()`.
- `s[0..2]` is **byte-indexed** and will panic on a non-ASCII boundary. For safe slicing use `s.chars().take(n).collect::<String>()`.
- `String` does not implement `Copy`. Assigning moves the value; clone explicitly with `.clone()` when you need a second owner.

---

## 5. Concept: `Vec<T>` — Dynamic Arrays

### Explanation

`Vec<T>` is Rust's growable, type-homogeneous array — the equivalent of Python's `list`. The big difference: **every element has the same type, enforced at compile time**.

```
┌────────────────────────────────────┐
│  Vec<u8>   digits: [3, 1, 4, 1]    │
│  ────────  ─────────────────────   │
│  length:   4                       │
│  capacity: 4  (may be > length)    │
│  element:  u8 (1 byte each)         │
└────────────────────────────────────┘
```

### Example (standalone)

```rust
fn main() {
    // Three ways to create a Vec
    let a: Vec<i32> = Vec::new();            // empty
    let b: Vec<i32> = vec![10, 20, 30];      // with values (the `vec!` macro)
    let c: Vec<i32> = (0..5).collect();      // from an iterator

    // Mutation
    let mut nums: Vec<i32> = vec![1, 2];
    nums.push(3);         // [1, 2, 3]
    nums.push(4);         // [1, 2, 3, 4]
    let last = nums.pop(); // Some(4) — Vec grows AND shrinks

    // Access
    println!("len={}, first={}", nums.len(), nums[0]);  // len=3, first=1
}
```

### Python comparison

```python
nums = [1, 2]
nums.append(3)
nums.append(4)
last = nums.pop()      # 4
print(len(nums), nums[0])  # 3 1
```

Same operations, but Rust's `Vec<i32>` would reject `nums.push("hello")` at compile time.

### Applying to our project

The secret code and hint-tracking state are both `Vec`s:

```rust
pub struct SecretCode {
    pub digits: Vec<u8>,              // the 4-digit code
    pub revealed_positions: Vec<bool>, // [false, false, false, false]
    pub revealed_digits: Vec<bool>,    // [false; 10] — which digits 0-9 revealed
}
```

You'll build these with `vec![]`, `.push()`, and indexed assignment (`self.revealed_positions[i] = true`).

---

## 6. Concept: `struct` and `impl` — Custom Data Types

This is your **first encounter with `struct`** in the course. Take your time here.

### Explanation

A `struct` groups related fields into a single type. An `impl` block attaches methods to it. The two are always written separately — unlike Python where a `class` body holds both data and methods.

```rust
// 1. Declare the data shape
struct Player {
    name: String,
    attempts: u32,
}

// 2. Attach behavior
impl Player {
    // Constructor: returns Self (the type name)
    fn new(name: String) -> Self {
        Self { name, attempts: 0 }
    }

    // Read-only method: &self borrows the data
    fn status(&self) -> String {
        format!("{}: {} attempts left", self.name, self.attempts)
    }

    // Mutating method: &mut self can change fields
    fn use_attempt(&mut self) {
        self.attempts += 1;
    }
}
```

### Why split data (`struct`) from behavior (`impl`)?

- **You can have multiple `impl` blocks** for the same struct (split across files in larger projects).
- **Traits** (covered in section 02-Ownership) can attach behavior to structs you don't own.
- The compiler knows which methods mutate (`&mut self`) and which don't (`&self`), enabling safer concurrency.

### Example (standalone)

```rust
#[derive(Debug)]  // auto-implement Debug so we can println!("{:?}")
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

fn main() {
    let mut p = Point::new(3.0, 4.0);
    println!("{:?} → distance = {}", p, p.distance_from_origin());  // 5.0
    p.translate(1.0, 1.0);
    println!("After translate: {:?}", p);  // Point { x: 4.0, y: 5.0 }
}
```

### Python comparison

```python
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def distance_from_origin(self):
        return (self.x**2 + self.y**2) ** 0.5

    def translate(self, dx, dy):
        self.x += dx
        self.y += dy
```

| Aspect | Python `class` | Rust `struct` + `impl` |
|---|---|---|
| Data fields | Assigned in `__init__` | Declared in `struct` |
| Methods | All inside the class body | In one or more `impl` blocks |
| Constructor | `__init__` | `fn new(...) -> Self` (convention) |
| Read method | `self` (read by convention) | `&self` (enforced immutable borrow) |
| Mutating method | `self` (no enforcement) | `&mut self` (required to change fields) |
| `self` / `Self` | `self` is an argument | `&self` is a *reference*; `Self` is the type name |
| Visibility | Public by default | Private by default; add `pub` to expose |

The last row is critical: in Python, anything not prefixed with `_` is public. In Rust, fields and methods are **private** to their module unless you write `pub`. You'll see `pub struct`, `pub fn`, and `pub` on every field in our library code.

### Applying to our project

MasterMind uses two structs and two `impl` blocks:

```rust
// Data: the secret code
pub struct SecretCode {
    pub digits: Vec<u8>,
    pub revealed_positions: Vec<bool>,
    pub revealed_digits: Vec<bool>,
}

// Behavior: methods on SecretCode
impl SecretCode {
    pub fn new() -> Self { ... }
    pub fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) { ... }
    pub fn give_position_hint(&mut self) -> Option<(usize, u8)> { ... }
}

// Data: the game state
pub struct MastermindGame {
    pub secret: SecretCode,
    pub attempts_left: u32,
    pub guess_count: u32,
}

// Behavior: methods on MastermindGame
impl MastermindGame {
    pub fn new(max_attempts: u32) -> Self { ... }
    pub fn play(&mut self) { ... }
}
```

Notice the `pub` on each item — without it, `main.rs` (which is a separate module) couldn't see `SecretCode` or `MastermindGame`.

### Exercise

Before you read further, implement `has_unique_digits` in `workshop/src/lib.rs`. It takes `s: &str` and returns `bool`. The test stubs are already there in `step_01_validation`. This is the smallest possible function that combines `&str`, `Vec`, and a `for` loop — the rest of the project builds on this pattern.

---

## 7. Concept: `Option<T>` — Handling Missing Data

This is your **first encounter with `Option`** in the course. Section 02-Ownership will deepen it with `Result` and combinators; here you only need the basics.

### Explanation

`Option<T>` represents a value that *might not exist*. It has exactly two variants:

```rust
enum Option<T> {
    Some(T),  // there is a value of type T
    None,     // there is no value
}
```

The compiler **forces** you to handle both variants before you can use the inner value.

### Example (standalone)

```rust
fn find_index(haystack: &[&str], needle: &str) -> Option<usize> {
    for (i, item) in haystack.iter().enumerate() {
        if *item == needle {
            return Some(i);
        }
    }
    None
}

fn main() {
    let data = ["apple", "banana", "cherry"];

    // Three ways to handle Option
    // 1. match (exhaustive — compiler checks every variant)
    match find_index(&data, "banana") {
        Some(i) => println!("Found at index {}", i),
        None    => println!("Not found"),
    }

    // 2. if let (only handle one variant)
    if let Some(i) = find_index(&data, "apple") {
        println!("Apple is at index {}", i);
    }

    // 3. .unwrap_or (provide a default)
    let i = find_index(&data, "kiwi").unwrap_or(999);
    println!("Kiwi at {} (or 999 if missing)", i);
}
```

### Python comparison

```python
def find_index(haystack, needle):
    for i, item in enumerate(haystack):
        if item == needle:
            return i
    return None  # the implicit "no answer"

result = find_index(data, "banana")
if result is not None:
    print("Found at index", result)
```

| Python | Rust |
|---|---|
| `None` | `Option::None` |
| Return value might be `None` | Return type is `Option<T>` |
| `if x is not None:` | `if let Some(x) = value` |
| `x = func() or default` | `x = func().unwrap_or(default);` |
| `result["key"]` (KeyError) | `map.get("key")` returns `Option<&V>` |

The crucial difference: in Python, `None` is a runtime value you check for. In Rust, the **type** is `Option<T>`. You literally cannot call `Option<T>` as if it were a `T` — the compiler will stop you.

### Applying to our project

`SecretCode::give_position_hint` returns `Option<(usize, u8)>`: it gives back `(position, digit)` if a hint is still available, or `None` if all hints have been revealed.

```rust
pub fn give_position_hint(&mut self) -> Option<(usize, u8)> {
    if !self.can_give_position_hint() {
        return None;                  // nothing left to reveal
    }
    // ... pick a position ...
    self.revealed_positions[chosen] = true;
    Some((chosen, self.digits[chosen]))  // hand back the hint
}
```

The caller uses `if let` to consume the hint:

```rust
if let Some((pos, digit)) = self.secret.give_position_hint() {
    self.attempts_left -= HINT_POSITION_COST;
    println!("Hint: Digit {} is at position {}.", digit, pos + 1);
}
// if it was None, we just skip — no crash, no NoneType error
```

### `if let` vs `match`

`if let Some(x) = opt` is a shortcut for `match opt { Some(x) => ..., None => () }`. Use `match` when you care about both arms; use `if let` when you only care about one.

### Exercise

Implement `SecretCode::can_give_position_hint` and `can_give_digit_hint` next. They return `bool` (not `Option`), but they feed the `if !...` check that decides whether to return `None` from the actual hint functions. The test stubs are in `step_03_hints`.

---

## 8. Concept: Iterators and Closures

### Explanation

An **iterator** is anything that produces a sequence of values. A **closure** is an anonymous function you can pass around. Together they form Rust's equivalent of Python's `map`/`filter`/`reduce`.

The most common iterator method is `.iter()`, which produces a sequence of references to each element:

```rust
let nums = vec![1, 2, 3, 4];
for n in nums.iter() {        // n: &i32
    println!("{}", n);
}
```

Iterator adapters (`.map`, `.filter`, `.enumerate`, `.zip`, `.sum`, `.count`) build a **chain** that runs when you call a **consumer** (`.collect()`, `.sum()`, `.count()`, `for`):

```rust
let nums = vec![1, 2, 3, 4, 5, 6];

// Sum of all even numbers
let even_sum: i32 = nums.iter()
    .filter(|n| *n % 2 == 0)  // keep evens
    .sum();

println!("Even sum: {}", even_sum);  // 12
```

| Iterator method | What it does | Python equivalent |
|---|---|---|
| `.iter()` | Iterate by reference | implicit in `for x in lst` |
| `.map(closure)` | Transform each item | `map(func, lst)` |
| `.filter(closure)` | Keep items where closure returns `true` | `filter(func, lst)` |
| `.enumerate()` | Pair each item with its index | `enumerate(lst)` |
| `.zip(other)` | Pair items from two iterators | `zip(a, b)` |
| `.count()` | Count items | `len(lst)` |
| `.sum()` | Add all items | `sum(lst)` |
| `.collect::<Vec<_>>()` | Build a `Vec` from the iterator | `list(lst)` |

### Example (standalone)

```rust
fn main() {
    let secrets = vec![1, 2, 3, 4];
    let guesses = vec![1, 0, 2, 0];

    // Count exact-position matches (zip + filter + count)
    let green = secrets.iter()
        .zip(guesses.iter())
        .filter(|(s, g)| s == g)
        .count();
    println!("Exact matches: {}", green);  // 2
}
```

### Python comparison

```python
secrets = [1, 2, 3, 4]
guesses = [1, 0, 2, 0]
green = sum(1 for s, g in zip(secrets, guesses) if s == g)
print("Exact matches:", green)  # 2
```

The shape is identical: pair them up, filter, count. The Rust version is **lazier** (nothing runs until `.count()` is called) and **type-checked** (`.zip` enforces both iterators have the same item type).

### Closures: `|arg| body`

```rust
let double = |x: i32| x * 2;
let is_even = |x: &i32| *x % 2 == 0;

println!("{}", double(5));      // 10
println!("{}", is_even(&4));    // true
```

Closures can capture variables from their surrounding scope (like Python lambdas). The pipe `|` syntax is just shorthand for `fn` arguments.

### Applying to our project

The core scoring algorithm in `evaluate_guess` uses exactly this chain:

```rust
let green = self.digits
    .iter()                          // iterate the secret
    .zip(guess_digits.iter())        // pair with the guess
    .filter(|(s, g)| s == g)         // keep exact matches
    .count();                        // count them
```

You'll also see `enumerate()` when picking a random hint position:

```rust
let available: Vec<usize> = self.revealed_positions
    .iter()
    .enumerate()                          // (index, &bool) pairs
    .filter(|(_, &revealed)| !revealed)  // keep unrevealed
    .map(|(i, _)| i)                      // drop the bool, keep index
    .collect();
```

### Exercise

Implement `SecretCode::evaluate_guess` next. The algorithm is in the README, the test stubs are in `step_02_secret_code`. You'll use `.iter().zip().filter().count()` for the green count, and a manual loop for the yellow count (because yellow requires removing matched digits from the secret pool).

---

## 9. Concept: `&self` vs `&mut self` — Method Receivers

### Explanation

In Python, every method takes `self` and *might* mutate. In Rust, the method signature declares whether mutation is allowed:

| Receiver | What it means | When to use |
|----------|---------------|-------------|
| `&self` | Borrow the value (read-only) | Method only reads fields |
| `&mut self` | Borrow the value mutably | Method changes fields |
| `self` | Take ownership of the value | Method consumes the value (rare) |
| No `self` | Associated function, not a method | Constructors, utilities |

```rust
impl MastermindGame {
    pub fn new(max_attempts: u32) -> Self {           // no self — constructor
        MastermindGame { ... }
    }

    pub fn attempts_left(&self) -> u32 {             // reads only
        self.attempts_left
    }

    pub fn submit_guess(&mut self, guess: &str)      // changes state
        -> Option<(usize, usize, usize)>
    {
        self.attempts_left -= 1;                     // OK — we have &mut
        ...
    }
}
```

The compiler enforces this: a `&self` method **cannot** modify fields, and you cannot call a `&mut self` method while a `&self` borrow is alive.

### Python comparison

```python
class MastermindGame:
    def new(max_attempts):           # @staticmethod
        return MastermindGame(...)

    def attempts_left(self):         # convention: doesn't mutate
        return self.attempts_left

    def submit_guess(self, guess):   # convention: might mutate
        self.attempts_left -= 1      # but nothing stops accidental mutation
```

In Python, the convention is "name your mutating methods carefully." In Rust, the type system **stops you from accidentally mutating through a read-only reference**.

### Applying to our project

| Method | Receiver | Why |
|--------|----------|-----|
| `SecretCode::new` | none | Constructor |
| `SecretCode::evaluate_guess` | `&self` | Only reads `digits` |
| `SecretCode::can_give_position_hint` | `&self` | Only reads `revealed_positions` |
| `SecretCode::give_position_hint` | `&mut self` | Sets `revealed_positions[chosen] = true` |
| `MastermindGame::new` | none | Constructor |
| `MastermindGame::play` | `&mut self` | Decrements `attempts_left`, calls `&mut self` methods |

You'll see the compiler complain if you try to call `give_position_hint` through a `&self` reference — that's the safety net working.

---

## 10. Concept: `pub` Visibility

### Explanation

By default, every item in Rust is **private** to the module where it's defined. To expose something to other modules (or to `main.rs`), you prefix it with `pub`:

```rust
// In lib.rs
pub struct SecretCode { ... }       // visible to main.rs
pub fn new() -> SecretCode { ... }  // visible to main.rs

struct Helper { ... }               // PRIVATE — only this file can use it
fn internal() { ... }               // PRIVATE
```

Three levels you'll use in this course:

| Visibility | Syntax | Where it's visible |
|------------|--------|---------------------|
| Private (default) | `fn` | Only the current module |
| Public to anyone | `pub fn` | Anyone who can see the module |
| Public to current crate only | `pub(crate) fn` | Anything in the same crate (rare) |

### Why default to private?

It forces you to think about the **public API** of your module. In Python, every name not prefixed with `_` is public, which makes it easy to depend on internals that later change. Rust makes you opt in to "this is part of my API."

### Example (standalone)

```rust
mod game {
    pub struct Player {
        pub name: String,        // public field
        health: u32,             // PRIVATE field — outside code can't read it
    }

    impl Player {
        pub fn new(name: String) -> Self {  // public constructor
            Self { name, health: 100 }
        }

        pub fn health(&self) -> u32 {       // public getter (private field)
            self.health
        }
    }
}

fn main() {
    let p = game::Player::new("Alice".to_string());
    println!("{} has {} HP", p.name, p.health());  // OK — both are public
    // println!("{}", p.health);                  // ERROR — field is private
}
```

### Python comparison

```python
class Player:
    def __init__(self, name):
        self.name = name        # public by convention
        self._health = 100      # "private" by underscore convention
```

In Python, the underscore is a hint, not a wall. `p._health = 999` still works. In Rust, `p.health` is a *compile error* if the field isn't `pub`.

### Applying to our project

In the basic workshop, **every** item in `lib.rs` is `pub` because the tests live in the same module. In the advanced workshop, we deliberately make some items `pub` (the public API) and others private (helpers) — that exercise is in section 12.

For now, the rule is simple: **prefix with `pub` if it should be usable from `main.rs` or from tests in another module**.

---

## 11. Detailed Step-by-Step Guide (Basic)

Build the core MasterMind game. Work in the `workshop/` directory.

### Table of Contents

1. [Prerequisites & Setup](#1-prerequisites-setup)
2. [Adding Dependencies](#2-adding-dependencies)
3. [Setup: Create the Project from Scratch](#3-setup-create-the-project-from-scratch)
4. [Step-by-Step Implementation](#4-step-by-step-implementation)
5. [Running the Game](#5-running-the-game)
6. [Summary of Rust Concepts Used](#6-summary-of-rust-concepts-used)

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

### 2. Adding Dependencies

Rust uses `cargo` to manage external libraries (called *crates*). We need `rand` for random number generation.

Open `Cargo.toml` and add:

```toml
[dependencies]
rand = "0.10"
```

Now run `cargo build`. Cargo downloads the `rand` crate and compiles your project.

> **Comparison:** In Python, `import random` gives you random functions. In Rust, you declare the dependency in `Cargo.toml` and then `use rand::...` in your code.

### 3. Setup: Create the Project from Scratch

If the `workshop/` directory is empty, create the project:

```bash
cargo new mastermind
cd mastermind
```

This creates a folder with:

- `Cargo.toml` – project configuration
- `src/main.rs` – main source file

Replace `Cargo.toml` with the project's deps (just `rand`), then create the `src/lib.rs` stub (or copy it from the workshop folder). The test stubs in `lib.rs` are already organized into `step_01_validation`, `step_02_secret_code`, `step_03_hints`, `step_04_game_setup`, and `step_05_integration`.

### 4. Step-by-Step Implementation

The test stubs in `workshop/src/lib.rs` are organized into five progressive modules. Open the README for each concept first, implement the matching function, then run `cargo test` to watch the test count grow.

| Step | Concept (read this first) | Function to implement | Tests that pass |
|------|---------------------------|----------------------|-----------------|
| 1 | [§4 String vs &str](../02-GuessGame/README.md#5-concept-string-vs-str) | `has_unique_digits(s: &str) -> bool` | `step_01_validation` (7 tests) |
| 2 | [§6 struct + impl](#6-concept-struct-and-impl-custom-data-types), [§8 iterators](#8-concept-iterators-and-closures) | `SecretCode::new`, `SecretCode::evaluate_guess` | `step_02_secret_code` (8 tests) |
| 3 | [§7 Option](#7-concept-optiont-handling-missing-data) | `SecretCode::can_give_*_hint`, `SecretCode::give_*_hint` | `step_03_hints` (8 tests) |
| 4 | [§6 struct + impl](#6-concept-struct-and-impl-custom-data-types) | `MastermindGame::new` | `step_04_game_setup` (5 tests) |
| 5 | Integration test | (just run) | `step_05_integration` (2 tests) |

After all 30 tests pass, your `lib.rs` is complete. Now write the thin `main.rs` that calls into it (shown below).

#### Reference: Complete `lib.rs` (Step 5 final form)

For reference only — you build this up function-by-function as you complete each step. After Step 5, your `workshop/src/lib.rs` should match the following (the test modules are abbreviated here; see the workshop folder for the full file):

```rust
use rand::seq::SliceRandom;
use rand::rng;

pub const DEFAULT_ATTEMPTS: u32 = 20;
pub const HINT_POSITION_COST: u32 = 5;
pub const HINT_DIGIT_COST: u32 = 3;

/// Returns true if the given string consists of 4 unique digits.
pub fn has_unique_digits(s: &str) -> bool {
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

pub struct SecretCode {
    pub digits: Vec<u8>,
    pub revealed_positions: Vec<bool>,
    pub revealed_digits: Vec<bool>,
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

    pub fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) {
        // ... (full implementation follows the algorithm in §8)
    }

    pub fn can_give_position_hint(&self) -> bool {
        self.revealed_positions.iter().any(|&r| !r)
    }

    pub fn can_give_digit_hint(&self) -> bool {
        self.revealed_digits.iter().any(|&r| !r)
    }

    pub fn give_position_hint(&mut self) -> Option<(usize, u8)> {
        // ... (full implementation follows §7 + §8)
    }

    pub fn give_digit_hint(&mut self) -> Option<u8> {
        // ... (full implementation follows §7 + §8)
    }
}

pub struct MastermindGame {
    pub secret: SecretCode,
    pub attempts_left: u32,
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
}
```

> **Note:** The `play()` method and full hint logic live in `main.rs` in the basic workshop (no test depends on them). The full version of `evaluate_guess` and the hint methods is shown in §8 and §7 above. Type it in yourself as you complete each step — don't paste.

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

#### The `main.rs` you'll write

Once all 30 tests pass, replace `workshop/src/main.rs` with the game loop. This is the only file that does I/O — all logic lives in `lib.rs`:

```rust
use std::io::{self, Write};
use mastermind::{has_unique_digits, MastermindGame, DEFAULT_ATTEMPTS};

fn main() {
    let mut game = MastermindGame::new(DEFAULT_ATTEMPTS);

    println!("{}", "=".repeat(40));
    println!("   Welcome to Mastermind!");
    println!("   Guess the 4-digit code (digits 0-9, no repeats)");
    println!("   You have {} attempts. Type 'help' for hints.", DEFAULT_ATTEMPTS);
    println!("{}", "=".repeat(40));

    while game.attempts_left > 0 {
        println!("\nAttempts left: {}", game.attempts_left);
        let input = get_user_input();

        if input == "help" {
            handle_hint(&mut game);
            continue;
        }

        game.guess_count += 1;
        let (green, yellow, red) = game.secret.evaluate_guess(&input);
        println!("Green: {}   Yellow: {}   Red: {}", green, yellow, red);

        if green == 4 {
            println!(
                "\nCongratulations! You cracked the code in {} actual guesses.",
                game.guess_count
            );
            return;
        }

        game.attempts_left -= 1;
    }

    let secret_str: String = game.secret.digits.iter().map(|d| d.to_string()).collect();
    println!("\nGame Over! The secret code was {}.", secret_str);
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
        return input;
    }
}

fn handle_hint(game: &mut MastermindGame) {
    const HINT_POSITION_COST: u32 = 5;
    const HINT_DIGIT_COST: u32 = 3;

    if game.attempts_left == 0 {
        println!("You don't have enough attempts to use a hint.");
        return;
    }

    let pos_available = game.secret.can_give_position_hint();
    let dig_available = game.secret.can_give_digit_hint();

    if !pos_available && !dig_available {
        println!("All hints already revealed. No more help available.");
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
        if game.attempts_left < HINT_POSITION_COST {
            println!("Not enough attempts.");
            return;
        }
        if let Some((pos, digit)) = game.secret.give_position_hint() {
            game.attempts_left -= HINT_POSITION_COST;
            println!("Hint: Digit {} is at position {}.", digit, pos + 1);
        }
    } else if choice == "2" && dig_available {
        if game.attempts_left < HINT_DIGIT_COST {
            println!("Not enough attempts.");
            return;
        }
        if let Some(digit) = game.secret.give_digit_hint() {
            game.attempts_left -= HINT_DIGIT_COST;
            println!("Hint: The code contains the digit {}.", digit);
        }
    } else {
        println!("Invalid choice.");
    }
}
```

This `main.rs` uses every concept you learned in §4–§10: `&str` parameters, `Vec` (in the secret code), `if let Some(...)` to consume hint results, iterator methods for hint selection, `pub` to make items visible across the `lib.rs`/`main.rs` boundary, and `&mut self` on `MastermindGame` to decrement attempts.

### 5. Running the Game

Inside the `mastermind` directory, run:

```bash
cargo run
```

### 6. Summary of Rust Concepts Used

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
| `pub` visibility | every item in `lib.rs` exposed to `main.rs` |

---

> ### ⏭️ What's Next: Section 2 — Ownership
>
> You've used `&self`, `&mut self`, and `pub` here, but we never asked **why** Rust has them. In **Section 2: Ownership**, you'll go from *using* these features to *understanding* the memory model behind them:
>
> | You just used (MasterMind) | You'll deeply understand in (Section 2) |
> |---------------------------|------------------------------------------|
> | `&self` and `&mut self` on methods | [01-TicketV1 §9-11](../../02-Ownership/01-TicketV1/README.md#9-concept-ownership-the-key-to-rust) — why Rust enforces "many readers OR one writer" |
> | `pub` on every item in `lib.rs` | [01-TicketV1 §7-8](../../02-Ownership/01-TicketV1/README.md#7-concept-modules-and-visibility) — module trees, `pub(crate)`, `pub(super)` |
> | `&str` vs `String` (implicit) | [01-TicketV1 §9](../../02-Ownership/01-TicketV1/README.md#9-concept-ownership-the-key-to-rust) — *why* `&str` doesn't own its data, and what "borrowing" means |
> | `Drop` mentioned briefly | [04-OBRM §4](../../02-Ownership/04-OBRM/README.md) — the full `impl Drop` pattern and RAII |
> | `panic!`-free `Option` handling | [03-TicketV2 §6-7](../../02-Ownership/03-TicketV2/README.md) — `Result<T, E>` and the `?` operator for production error handling |
>
> **If you want to proceed with the full mental model before tackling Section 2**, spend 5 minutes re-reading §9 (method receivers) and §10 (`pub` visibility) above. The borrow-checker errors in TicketV1 will feel like guardrails instead of walls.

---

## 12. Advanced Exercise Guide

Refactor the game into a library + binary crate. Work in the `workshop/advanced/` directory.

### Table of Contents

- [12. Advanced Exercise Guide](#12-advanced-exercise-guide)
  - [1. Introduction](#1-introduction)
  - [2. Prerequisites](#2-prerequisites)
  - [3. Concept 1: Rust Packages, Crates, and Modules](#3-concept-1-rust-packages-crates-and-modules)
  - [4. Concept 2: Library vs Binary Crate](#4-concept-2-library-vs-binary-crate)
  - [5. Concept 3: Visibility (`pub`) and Re-exports](#5-concept-3-visibility-pub-and-re-exports)
  - [6. Concept 4: Documentation (`///` & `cargo doc`)](#6-concept-4-documentation-cargo-doc)
  - [7. Concept 5: Unit Tests (`#[test]` & `cargo test`)](#7-concept-5-unit-tests-test-cargo-test)
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
├── Cargo.toml
└── src/
    ├── main.rs      ← binary crate root (by default)
    └── lib.rs       ← library crate root (if present)
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
- `lib.rs` — the library root
- `secret.rs` — the `SecretCode` module
- `game.rs` — the `MastermindGame` module

#### `secret.rs`

Move the `SecretCode` struct and its `impl` from the old code into `secret.rs`:

```rust
use rand::seq::SliceRandom;
use rand::rng;

pub struct SecretCode {
    // pub(crate) so game.rs tests (in a sibling module) can read digits
    // — see §5 "Visibility (pub) and Re-exports" above
    pub(crate) digits: Vec<u8>,
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

---

## 13. Summary

### Concepts taught in this project (first appearances)

| # | Concept | Section | Where applied |
|---|---------|---------|----------------|
| 1 | `String` vs `&str` (deeper) | §4 | Player input, function parameters |
| 2 | `Vec<T>` (deeper) | §5 | `digits`, `revealed_positions`, `revealed_digits` |
| 3 | `struct` + `impl` | §6 | `SecretCode`, `MastermindGame` |
| 4 | `Self` constructor | §6 | `SecretCode::new() -> Self` |
| 5 | `Option<T>` + `if let` | §7 | `give_position_hint` returns `Option<(usize, u8)>` |
| 6 | Iterators (`.iter()`, `.zip()`, `.filter()`, `.count()`, `.enumerate()`, `.collect()`) | §8 | `evaluate_guess`, hint selection |
| 7 | Closures (`|x| ...`) | §8 | `filter(|(_, &revealed)| !revealed)` |
| 8 | `&self` vs `&mut self` | §9 | Read-only vs mutating methods |
| 9 | `pub` visibility | §10 | Every public item in `lib.rs` |
| 10 | `Self` type alias | §6 | `fn new() -> Self` |

### Concepts used but taught earlier (review pointers)

| Concept | First taught in | Used here for |
|---------|-----------------|----------------|
| Variables, mutability, basic types | [01-Intro §6](../01-Intro/README.md#6-variables-and-mutability) | `let mut attempts_left: u32 = 20;` |
| `const` | [01-Intro §6](../01-Intro/README.md#6-variables-and-mutability) | `DEFAULT_ATTEMPTS`, `HINT_*_COST` |
| `if`/`else` as expressions | [01-Intro §7](../01-Intro/README.md#7-ifelse-making-decisions) | Hint cost checks |
| `while` loop | [01-Intro §8](../01-Intro/README.md#8-loops-repeating-work) | Game loop in `play()` |
| `String`/`&str` (intro) | [02-GuessGame §5](../02-GuessGame/README.md#5-concept-string-vs-str) | `String::new()` buffer, `&str` params |
| `Result`, `.expect()` | [02-GuessGame §7](../02-GuessGame/README.md#7-concept-resultt-e-and-parse) | `io::stdin().read_line(...).unwrap()` |
| `match` | [02-GuessGame §7](../02-GuessGame/README.md#7-concept-resultt-e-and-parse) | `match input.trim().parse()` |
| `panic!` (covered lightly) | [03-BasicCalculator §9](../03-BasicCalculator/README.md#9-concept-panics-unrecoverable-errors) | `unwrap()` on I/O |
| `#[test]`, `assert_eq!` | [03-BasicCalculator §14](../03-BasicCalculator/README.md#14-concept-unit-testing-in-rust) | 30 tests in `workshop/src/lib.rs` |
| External crates | [02-GuessGame §4](../02-GuessGame/README.md#4-concept-adding-an-external-crate) | `rand = "0.10"` in `Cargo.toml` |
| Random numbers | [02-GuessGame §4](../02-GuessGame/README.md#4-concept-adding-an-external-crate) | `rand::seq::SliceRandom` for shuffling |

### What you should be able to do now

After completing both workshops (basic + advanced), you can:

- Model a small domain with `struct` and `impl`, choosing `&self` vs `&mut self` correctly.
- Use `Vec<T>` to hold dynamic collections, and `Option<T>` for values that might not exist.
- Chain iterator adapters (`.iter().zip().filter().count()`) for data processing.
- Decide when a function should take `&str` (read) vs `String` (own).
- Mark items `pub` so they're visible across module boundaries.
- Refactor a single-file program into a library + binary crate with `clap` for CLI args.

### What's next

Proceed to [Section 02: Ownership](../../02-Ownership/README.md) to learn the **borrow checker** — Rust's central feature. You'll revisit `&self`/`&mut self` and learn the rules that make Rust memory-safe without a garbage collector.

---

*You finished Project 1.4 — MasterMind. You now know how to model domain types, handle missing data, and process collections with iterators. The borrow checker in Section 02 will make all of this feel even safer.*

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

