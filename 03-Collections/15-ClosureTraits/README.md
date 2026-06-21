# Closure Traits & Custom Iterators — Python to Rust Workshop

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 18 tests pass**.

---

## What Is This Project?

Closure traits and custom iterator implementations — the two features that make Rust's functional style both expressive and zero-cost.

### Python equivalent

```python
from functools import reduce

# Lambdas are just objects — no trait distinction
add = lambda x, y: x + y
apply_twice = lambda f, x: f(f(x))

# Custom iterators via __iter__ / __next__
class Counter:
    def __init__(self, max):
        self.count = 0
        self.max = max
    def __iter__(self):
        return self
    def __next__(self):
        if self.count >= self.max:
            raise StopIteration
        self.count += 1
        return self.count

# Chaining: filter + map + reduce
result = reduce(lambda acc, x: acc + x, map(lambda x: x**2, filter(lambda x: x % 2 == 0, [1,2,3,4,5])))
```

In this project you'll learn to build this in Rust — and along the way
you'll discover **`Fn`/`FnMut`/`FnOnce` traits**, **`move` closures**,
**custom `Iterator` implementations**, and **`IntoIterator`**.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | `FnOnce` trait | Closures that consume captured values (thread callbacks) |
| 2 | `FnMut` trait | Closures that mutate captured state (accumulators) |
| 3 | `Fn` trait | Closures that only borrow (predicates, transforms) |
| 4 | `move` closures | Transferring ownership into closures for threads/storage |
| 5 | Custom `Iterator` impl | Making your own types work with all iterator adapters |
| 6 | `IntoIterator` | Making types work with `for` loops |
| 7 | Closure-returning closures | Factory patterns, currying |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: The Three Closure Traits](#3-concept-the-three-closure-traits)
4. [Concept: `move` Closures](#4-concept-move-closures)
5. [Concept: Custom Iterator Implementations](#5-concept-custom-iterator-implementations)
6. [Concept: Closures Returning Closures](#6-concept-closures-returning-closures)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Exercises](#8-exercises)
9. [Summary](#9-summary)

## 1. Introduction

Closures are everywhere in Rust — they power iterator adapters, thread spawning, async tasks, and callback-based APIs. But unlike Python lambdas (which are just anonymous functions), Rust closures have **three distinct traits** that describe how they interact with captured variables.

Understanding these traits matters for data pipelines because:
- **`Fn`** closures are used in `filter`, `map`, and most iterator adapters
- **`FnMut`** closures are used in `fold`, `scan`, and `sort_by`
- **`FnOnce`** closures are used in `thread::spawn`, `tokio::spawn`, and one-shot callbacks

Similarly, implementing `Iterator` for your own types unlocks all the adapter methods (`map`, `filter`, `fold`, `take`, `collect`) for free — a powerful pattern for custom data sources.

## 2. Prerequisites

- Closures basics from [01-Foundations/04-MasterMind](../../01-Foundations/04-MasterMind/README.md)
- Iterator adapters from [12-RustIterators](../12-RustIterators/README.md)
- Traits from [02-Ownership/02-Traits](../../02-Ownership/02-Traits/README.md)

## 3. Concept: The Three Closure Traits

### Explanation

Every Rust closure implements one or more of three traits. The compiler automatically determines which trait(s) a closure satisfies based on how it uses captured variables:

```
┌─────────────────────────────────────────────────────┐
│  Trait Hierarchy                                     │
│                                                      │
│  FnOnce ── (can be called once)                      │
│    ↑                                                 │
│  FnMut ── (can be called multiple times, may mutate) │
│    ↑                                                 │
│  Fn ──── (can be called multiple times, immutable)   │
│                                                      │
│  FnMut also implements FnOnce                        │
│  Fn also implements FnMut and FnOnce                 │
└─────────────────────────────────────────────────────┘
```

**Python comparison**: In Python, all lambdas are equivalent — they're just functions. Rust distinguishes between closures that borrow, mutate, or consume captured values. This distinction enables the compiler to enforce memory safety without a garbage collector.

| Trait | Can call multiple times? | Can mutate captures? | Typical use |
|-------|-------------------------|---------------------|-------------|
| `FnOnce` | No (consumes captures) | N/A (consumed) | `thread::spawn`, one-shot callbacks |
| `FnMut` | Yes | Yes | `sort_by`, `fold`, accumulators |
| `Fn` | Yes | No (immutable borrow) | `filter`, `map`, predicates |

### `Fn` — Immutable borrow

```rust
let threshold = 10;
let is_big = |n| n > threshold; // borrows `threshold` immutably

// Can call many times — threshold is still there
assert!(is_big(15));
assert!(!is_big(5));
```

### `FnMut` — Mutable borrow

```rust
let mut count = 0;
let mut increment = || { count += 1; count }; // mutably borrows `count`

assert_eq!(increment(), 1);
assert_eq!(increment(), 2);
assert_eq!(count, 2); // count was modified
```

### `FnOnce` — Consumes captures

```rust
let name = String::from("hello");
let consume = || {
    let _moved = name; // `name` is moved into this block
    println!("consumed!");
};

consume();
// consume(); // ERROR: can't call twice — `name` was moved
```

### Applying to Our Project

```rust
pub fn apply_fn<F: Fn(i32) -> bool>(f: &F, val: i32) -> bool {
    f(val)
}

pub fn apply_fn_mut<F: FnMut(i32) -> i32>(f: &mut F, val: i32) -> i32 {
    f(val)
}

pub fn apply_fn_once<F: FnOnce() -> String>(f: F) -> String {
    f()
}
```

## 4. Concept: `move` Closures

### Explanation

The `move` keyword forces a closure to **take ownership** of captured variables, regardless of how the closure uses them:

```rust
let name = String::from("Rust");
let greet = move || println!("Hello, {}!", name);
greet();
// println!("{}", name); // ERROR: name was moved into the closure
```

**When to use `move`:**
- Passing closures to `thread::spawn` (threads may outlive the scope)
- Storing closures in structs or collections
- When you want the closure to own the data

**Python comparison**: Python doesn't have this concept because the garbage collector handles lifetime. In Rust, `move` is necessary when the closure must outlive the scope where variables are defined.

### Applying to Our Project

Move closures are essential for thread callbacks:

```rust
let data = vec![1, 2, 3];
std::thread::spawn(move || {
    println!("{:?}", data); // data is owned by the thread
});
// data is no longer accessible here
```

## 5. Concept: Custom Iterator Implementations

### Explanation

Any type can become an iterator by implementing the `Iterator` trait. The trait requires one method:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Return `Some(value)` for each item, or `None` when done.

**Python comparison**: In Python, you implement `__iter__` and `__next__`. The pattern is similar, but Rust's trait system gives you all adapters (`map`, `filter`, `fold`, `collect`) for free once you implement `next()`.

### Example: Counter

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Now we can use all iterator adapters:
let sum: u32 = Counter::new(5).sum();        // 15
let doubled: Vec<u32> = Counter::new(3).map(|x| x * 2).collect(); // [2, 4, 6]
```

### Example: Fibonacci

```rust
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.a;
        let new_b = self.a + self.b;
        self.a = self.b;
        self.b = new_b;
        Some(result) // infinite iterator — always returns Some
    }
}

// Take first 10 Fibonacci numbers
let fib: Vec<u64> = Fibonacci::new().take(10).collect();
// [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

### Applying to Our Project

```rust
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.a;
        let new_b = self.a + self.b;
        self.a = self.b;
        self.b = new_b;
        Some(result)
    }
}
```

## 6. Concept: Closures Returning Closures

### Explanation

Closures can return other closures. The returned closure captures values from the outer scope:

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

let add_five = make_adder(5);
assert_eq!(add_five(3), 8);
assert_eq!(add_five(10), 15);
```

**Python comparison**: This is like a closure factory:

```python
def make_adder(n):
    return lambda x: x + n

add_five = make_adder(5)
assert add_five(3) == 8
```

The `move` keyword ensures the returned closure owns `n`, so it can be used after `make_adder` returns.

### Applying to Our Project

```rust
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}
```

## 7. Putting It All Together

Open `workshop/src/lib.rs` and implement all functions. Run tests step by step:

```bash
# Step 1: Closure traits
cd workshop && cargo test step_01

# Step 2: Closure returning closure
cd workshop && cargo test step_02

# Step 3: FnMut with sort_by
cd workshop && cargo test step_03

# Step 4: Custom Iterator — Counter
cd workshop && cargo test step_04

# Step 5: Custom Iterator — Fibonacci
cd workshop && cargo test step_05

# Step 6: Data pipeline
cd workshop && cargo test step_06

# All tests
cd workshop && cargo test
```

All 18 tests should pass.

## 8. Exercises

### Easy
Write a function `make_multiplier(n: i32) -> impl Fn(i32) -> i32` that returns a closure which multiplies its argument by `n`. Test with `make_multiplier(3)(4) == 12`.

### Medium
Implement a `struct Primes` that implements `Iterator<Item = u64>` and yields prime numbers (2, 3, 5, 7, 11, ...). Use trial division up to `sqrt(n)` to check primality. Collect the first 20 primes.

### Hard
Implement a `struct Window<T>` that wraps a `Vec<T>` and implements `Iterator` yielding sliding windows of size `n`. For example, `Window::new(vec![1,2,3,4,5], 3)` yields `[1,2,3]`, `[2,3,4]`, `[3,4,5]`.

## 9. Summary

| Concept | Description | Python equivalent |
|---------|------------|-------------------|
| `FnOnce` | Closure that consumes captured values | N/A (GC handles lifetime) |
| `FnMut` | Closure that mutates captured values | Lambda with nonlocal |
| `Fn` | Closure that immutably borrows captures | Lambda (default) |
| `move` keyword | Forces ownership transfer into closure | N/A (GC handles lifetime) |
| `Iterator` trait | Custom iteration via `next() -> Option<Item>` | `__iter__` / `__next__` |
| `IntoIterator` | Makes types work with `for` loops | `__iter__` |
| Closure returning closure | Factory pattern, currying | Nested lambda |

Closures and iterators are the backbone of idiomatic Rust. The three closure traits ensure memory safety without runtime overhead, and custom iterators unlock the full power of Rust's adapter chains for your own data types.
