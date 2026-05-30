# Send and Sync Marker Traits — unsafe impl for Custom Types

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 9 tests pass**.

## Why This Project?

### The Problem

In Python, there is no way to declare that a type is or isn't thread-safe. Every object can be passed between threads freely:

```python
from threading import Thread

class Counter:
    def __init__(self):
        self.count = 0

def worker(c: Counter):
    for _ in range(10000):
        c.count += 1  # Not thread-safe — but Python lets you do it anyway

c = Counter()
threads = [Thread(target=worker, args=(c,)) for _ in range(8)]
for t in threads: t.start()
for t in threads: t.join()
print(c.count)  # Wrong value — no warning, no error
```

```
Python type system:     "I don't know — and I don't care — if this is thread-safe."
                        Any type → any thread → no protection

Rust type system:       "Rc<i32> is not Send. RefCell<i32> is not Sync."
                        Compile-time protection → fearless concurrency
```

Python depends on documentation and discipline. In large teams with complex data pipelines, this causes production incidents that are hard to reproduce and fix.

### The Rust Solution

Rust's `Send` and `Sync` marker traits are automatically derived from field types. The compiler uses them to prevent thread-safety bugs:

```rust
use std::rc::Rc;
use std::sync::{Arc, Mutex};

// Rc<i32> is NOT Send — cannot transfer across threads
// let handle = thread::spawn(move || { let _ = Rc::new(42); });
// ERROR: Rc<i32> cannot be sent between threads safely

// Arc<Mutex<i32>> is Send + Sync — safe for threads
pub fn verify_send<T: Send>(val: T) -> T { val }
pub fn verify_sync<T: Sync>(val: T) -> T { val }

let safe = Arc::new(Mutex::new(42i32));
verify_send(&safe);   // ✓ Compiles
verify_sync(&safe);   // ✓ Compiles
```

Custom types can implement `Send`/`Sync` via `unsafe impl` when the compiler cannot auto-derive them (e.g., with raw pointers).

## What You'll Learn

| # | Concept | Rust Trait | Python Equivalent | Purpose |
|---|---------|------------|------------------|---------|
| 1 | Send Trait | `std::marker::Send` | No equivalent | Ownership transferable across threads |
| 2 | Sync Trait | `std::marker::Sync` | No equivalent | Shared ref transferable across threads |
| 3 | Auto-Implementation | Field composition | N/A | Compiler derives from field types |
| 4 | Unsafe Impl | `unsafe impl Send`/`Sync` | N/A | Manual implementation for custom types |
| 5 | Thread-Safe Pattern | `Arc<Mutex<T>>` | `threading.Lock` | Standard shared mutable state |
| 6 | Non-Thread-Safe Types | `Rc`, `Cell`, `RefCell` | All types are "thread-safe" | Thread-unsafe types are explicit |

## Concepts at a Glance

- **Send Trait**: A type is `Send` if ownership can be transferred between threads. `Rc<T>` is not `Send` (its reference count uses non-atomic operations). Python has no equivalent — all Python objects can be sent between threads.
- **Sync Trait**: A type is `Sync` if `&T` can be shared between threads. `Cell<T>` and `RefCell<T>` are not `Sync` (their interior mutability has no synchronization). Python has no equivalent marker.
- **Auto-Implementation**: `Send` and `Sync` are automatically implemented by the compiler based on field types. A struct is `Send` if all fields are `Send`; it is `Sync` if all fields are `Sync`. This is the foundation of Rust's fearless concurrency.
- **Unsafe Impl (`unsafe impl Send`/`Sync`)**: Allows manual implementation of `Send`/`Sync` for custom types where the compiler cannot determine safety. The `unsafe` keyword indicates a correctness guarantee the programmer must uphold.
- **Thread-Safe Pattern (`Arc<Mutex<T>>`)**: The standard Rust pattern for shared mutable state across threads. `Arc` provides shared ownership; `Mutex` provides synchronization. Python's `threading.Lock` achieves the same result but without compile-time verification.
- **Non-Thread-Safe Types**: `Rc`, `Cell`, `RefCell` are deliberately `!Send` or `!Sync`. Their thread-unsafety is part of their type signature — visible in documentation and enforced by the compiler.

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: The Send trait](#3-concept-the-send-trait)
4. [Concept: The Sync trait](#4-concept-the-sync-trait)
5. [Concept: unsafe impl Send and Sync](#5-concept-unsafe-impl-send-and-sync)
6. [Putting It All Together](#6-putting-it-all-together)
7. [Exercises](#7-exercises)
8. [Summary](#8-summary)

## 1. Introduction

Rust has two special **marker traits** — `Send` and `Sync` — that have no methods. They exist purely to mark types as thread-safe or not. The compiler uses them to prevent you from accidentally sharing data across threads in unsafe ways.

In Python, there is no equivalent concept. Any object can be shared between threads, which is why bugs are common. Rust's auto-implementation of `Send`/`Sync` based on field types is the foundation of fearless concurrency.

**Data engineering context**: When designing custom data types for parallel processing (e.g., a thread-safe accumulator or a partitioned dataset wrapper), you must understand which types are `Send` and `Sync` to ensure your type is usable across threads.

## 2. Prerequisites

- Threads from [01-Threads](../01-Threads/README.md)
- `Arc` and `Mutex` from [03-DataRace](../03-DataRace/README.md)

## 3. Concept: The Send trait

### Explanation

A type is **`Send`** if ownership of a value of that type can be transferred between threads. Almost all types are `Send` by default. The notable exceptions:

- `Rc<T>` — reference count is not atomic
- Raw pointers (`*const T`, `*mut T`)
- `UnsafeCell<T>` (and types containing it without synchronization)

```rust
pub fn verify_send<T: Send>(val: T) -> T {
    val
}
```

### Python comparison

```python
# Python — no Send equivalent. Any type can be sent between threads.
from threading import Thread

class Counter:
    def __init__(self):
        self.count = 0

def worker(c: Counter):
    c.count += 1  # No compile-time protection

c = Counter()
t = Thread(target=worker, args=(c,))
t.start()
t.join()
```

### Applying to our project

```rust
pub fn verify_send<T: Send>(val: T) -> T {
    val
}
```

This function constrains `T` with `Send`. If you try to pass an `Rc<i32>`, it will fail to compile:

```rust
// let x = verify_send(Rc::new(42));  // ERROR: Rc<i32> is not Send
```

## 4. Concept: The Sync trait

### Explanation

A type is **`Sync`** if shared references (`&T`) can be shared between threads. In other words, `T: Sync` means `&T` is `Send`. Almost all types are `Sync`. Exceptions include:

- `Cell<T>` and `RefCell<T>` — no synchronization for interior mutability
- `Rc<T>` — non-atomic reference count
- Raw pointers

```rust
pub fn verify_sync<T: Sync>(val: T) -> T {
    val
}
```

### Python comparison

```python
# Python — any type can be shared between threads
counter = 0

def reader():
    print(counter)  # No compile-time protection

t = Thread(target=reader)
t.start()
t.join()
```

### Applying to our project

```rust
pub fn verify_sync<T: Sync>(val: T) -> T {
    val
}
```

`Mutex<T>` is `Sync` because it synchronizes access. If you try to pass a `RefCell<i32>`, it will fail:

```rust
// let x = verify_sync(RefCell::new(42));  // ERROR: RefCell<i32> is not Sync
```

## 5. Concept: unsafe impl Send and Sync

### Explanation

You can manually implement `Send` and `Sync` for your types using `unsafe impl`. This tells the compiler: "I guarantee this type is thread-safe."

```rust
/// A wrapper type that explicitly implements Send and Sync.
pub struct Wrapper(pub i32);

// SAFETY: Wrapper contains only an i32, which is Send + Sync
unsafe impl Send for Wrapper {}
unsafe impl Sync for Wrapper {}
```

### Python comparison

```python
# Python — there is no way to mark a type as "thread-safe" or not
class Wrapper:
    def __init__(self, val: int):
        self.val = val
```

In Python, you rely on documentation and discipline. In Rust, the compiler enforces it.

### Why `unsafe`?

Implementing `Send` or `Sync` on a type that contains non-thread-safe fields (like raw pointers or `UnsafeCell`) could cause data races. The `unsafe` keyword is your promise that you have verified the implementation is correct.

### Applying to our project

```rust
pub fn create_thread_safe_wrapper(val: i32) -> Wrapper {
    Wrapper(val)
}

pub fn demonstrate_mutex_send_sync() -> bool {
    use std::sync::{Arc, Mutex};
    // Arc<Mutex<i32>> is Send + Sync — this is the standard pattern
    fn check_send<T: Send>(_: &T) -> bool { true }
    fn check_sync<T: Sync>(_: &T) -> bool { true }

    let val = Arc::new(Mutex::new(42i32));
    check_send(&val) && check_sync(&val)
}
```

### Auto-implementation

For most types, `Send` and `Sync` are **auto-implemented** by the compiler based on field types. A struct is `Send` if all its fields are `Send`. It is `Sync` if all its fields are `Sync`. You only need `unsafe impl` when the compiler cannot infer thread safety (e.g., with raw pointers).

## 6. Putting It All Together

Implement each function in `workshop/src/lib.rs`:

| Function | Concept | Test Module | Tests |
|---|---|---|---|
| `verify_send()` | `T: Send` bound | `step_01_send_trait` | 3 |
| `verify_sync()` | `T: Sync` bound | `step_02_sync_trait` | 2 |
| `create_thread_safe_wrapper()` | Create `Wrapper` | `step_03_unsafe_impl` | 1 |
| `demonstrate_mutex_send_sync()` | Arc\<Mutex\> is Send + Sync | `step_03_unsafe_impl` | 1 |

The `Wrapper` struct and its `unsafe impl Send for Wrapper` / `unsafe impl Sync for Wrapper` are already defined in the stub. Review them as examples of how to safely implement these traits for custom types.

## 7. Exercises

**Easy**: Create a `ThreadSafeCounter` struct that wraps `Arc<Mutex<usize>>` and implements `Send` and `Sync`.

**Medium**: Create a type `MyBox<T>` that wraps a raw pointer `*mut T` and implement `Send` only (not `Sync`) to allow transfer but not sharing across threads.

**Hard**: Analyze a real crate's source to identify which types implement `Send`/`Sync` manually and why, then write a short report.

## 8. Summary

| Concept | Rust Trait | Python Equivalent |
|---|---|---|
| Transfer ownership across threads | `Send` | No equivalent |
| Share references across threads | `Sync` | No equivalent |
| Combined marker | `T: Send + Sync` | No equivalent |
| Automatic implementation | By field composition | N/A |
| Manual implementation | `unsafe impl Send`/`Sync` | N/A |
| Standard thread-safe pattern | `Arc<Mutex<T>>` | `threading.Lock` |
