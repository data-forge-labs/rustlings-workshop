# Safe vs Unsafe Rust — Python to Rust Workshop

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 13 tests pass**.

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Safe Rust Guarantees](#3-concept-safe-rust-guarantees)
4. [Concept: The unsafe Keyword](#4-concept-the-unsafe-keyword)
5. [Concept: Raw Pointers](#5-concept-raw-pointers)
6. [Concept: split_at_mut Pattern](#6-concept-split_at_mut-pattern)
7. [Concept: When to Use unsafe](#7-concept-when-to-use-unsafe)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Complete Code Reference](#9-complete-code-reference)
10. [Summary](#10-summary)

## 1. Introduction

In this workshop you will explore Rust's safety boundaries. Rust is famous for
its ability to guarantee memory safety without a garbage collector. But
sometimes you need to step outside those guarantees: talking to hardware,
calling C code, or implementing performance-critical data structures. Rust
gives you the `unsafe` keyword for exactly these situations.

In Python, you never have to think about this -- Python is always "safe" because
its runtime checks every memory access. The cost is performance and
predictability. Rust lets you opt out of safety checks in explicit,
auditable blocks.

**Data-engineering motivation**: When processing large files or implementing
custom zero-copy parsers, you may reach for `unsafe` to skip bounds checks on
hot paths. Understanding when and how to do this safely is critical.

## 2. Prerequisites

- Completed [Section 2: Ownership](../../02-Ownership/README.md) -- references and
  borrowing concepts
- Completed [Section 3: Collections](../../03-Collections/README.md) -- slices and
  indexing

## 3. Concept: Safe Rust Guarantees

### Explanation

Safe Rust is the default. The compiler enforces:

- **No null pointer dereferences** -- `Option` replaces null
- **No buffer overflows** -- bounds checking on array/slice access
- **No use-after-free** -- ownership and borrowing prevent dangling references
- **No data races** -- the borrow checker enforces either one mutable or many
  immutable references

In Python, you get these guarantees from the interpreter and the GC. A
list access like `lst[999]` for a 3-element list raises `IndexError` at
runtime, just like Rust's `Option` return -- but Rust does it at the type
level.

```python
# Python: runtime check
def safe_index(lst, idx):
    try:
        return lst[idx]
    except IndexError:
        return None
```

```rust
// Rust: compile-time guarantee via Option
pub fn safe_index(slice: &[i32], index: usize) -> Option<i32> {
    slice.get(index).copied()
}
```

### Applying to Our Project

The `safe_add` and `safe_index` functions demonstrate safe Rust. They accept
inputs and return `Option` for fallible operations, letting the compiler
enforce that callers handle missing values.

```rust
pub fn safe_add(a: i32, b: i32) -> i32 { a + b }

pub fn safe_index(slice: &[i32], index: usize) -> Option<i32> {
    slice.get(index).copied()
}
```

## 4. Concept: The unsafe Keyword

### Explanation

`unsafe` is not a toggle that turns off all safety. It grants you access to
**five superpowers**:

1. Dereference raw pointers (`*const T`, `*mut T`)
2. Call `unsafe` functions (including foreign functions / FFI)
3. Access or modify mutable static variables
4. Implement `unsafe` traits
5. Access fields of unions

The rest of Rust's safety guarantees remain in force: the borrow checker still
applies, types are still checked, and the rest of your code is still safe.

In Python, there is no equivalent -- you cannot opt out of interpreter checks.
The closest analogue is calling into C via `ctypes` or `cffi`, which is
inherently unsafe and unenforced by the compiler.

### Applying to Our Project

```rust
pub unsafe fn unsafe_dereference(ptr: *const i32) -> i32 {
    if ptr.is_null() { 0 } else { *ptr }
}

pub unsafe fn unsafe_write(ptr: *mut i32, val: i32) {
    *ptr = val;
}
```

The caller must wrap these in their own `unsafe { }` block, taking
responsibility for correctness.

## 5. Concept: Raw Pointers

### Explanation

Rust has two raw pointer types, which have no safety guarantees:

- `*const T` -- immutable raw pointer (cannot be used to mutate)
- `*mut T` -- mutable raw pointer (can be used to mutate)

Unlike references (`&T` and `&mut T`), raw pointers:
- Can be null
- Can dangle
- Ignore aliasing rules
- Can be created from integers (e.g., hardware addresses)

In Python, there are no pointers at all. Everything is a reference to a
heap-allocated object. Rust's raw pointers give you the same power as C
pointers, but wrapped in `unsafe` so they are auditable.

```
Memory layout comparison:

  Python:   var  -->  PyObject { ob_refcnt, ob_type, ... }
                     (always heap, always tracked)

  Rust:     let x = 42;     Stack: [ 42 ]
            let ptr = &x;   Stack: [ ptr -> 42 ]  (safe reference)
            let raw = &x as *const i32;  (raw pointer, no borrowing rules)
```

### Applying to Our Project

The `unsafe_dereference` function takes a `*const i32` and reads its value,
checking for null first. The `unsafe_write` function takes a `*mut i32` and
writes through it. Both require `unsafe` to dereference.

## 6. Concept: split_at_mut Pattern

### Explanation

One classic example of a safe API that internally uses `unsafe` is
`slice::split_at_mut`. It lets you split a mutable slice into two mutable
sub-slices -- something the borrow checker normally forbids.

```rust
let mut arr = [1, 2, 3, 4];
let (left, right) = arr.split_at_mut(2);
// left = &mut [1, 2], right = &mut [3, 4]
```

The borrow checker cannot prove these don't overlap, so `split_at_mut` uses
`unsafe` internally but exposes a safe interface.

In Python, you would use slicing: `left = arr[:2]; right = arr[2:]`. Python
copies the data (or creates views in NumPy), so there is no aliasing conflict.

### Applying to Our Project

The `safe_split_sum` function splits a slice at the midpoint and sums each
half independently:

```rust
pub fn safe_split_sum(slice: &mut [i32]) -> (i32, i32) {
    let mid = slice.len() / 2;
    let (left, right) = slice.split_at_mut(mid);
    (left.iter().sum(), right.iter().sum())
}
```

## 7. Concept: When to Use unsafe

### Explanation

Use `unsafe` only when:

1. **You must** -- FFI with C libraries, inline assembly, memory-mapped I/O
2. **Performance** -- skipping bounds checks on a hot loop after proving
   correctness
3. **Custom data structures** -- implementing lock-free data structures or
   custom allocators that the borrow checker cannot verify

Before writing `unsafe`, ask:
- Can I achieve this with safe Rust (iterators, `split_at_mut`, `Cell`,
  `RefCell`)?
- Have I proven the invariants that the `unsafe` block relies on?
- Is there a well-reviewed crate that already does this safely?

In Python, you never make this choice. The interpreter handles everything.
The tradeoff is that Python cannot match Rust's performance in tight loops
or low-level systems scenarios.

### Applying to Our Project

The `safety_concepts` function returns the list of concepts you explored:

```rust
pub fn safety_concepts() -> Vec<&'static str> {
    vec![
        "safe Rust guarantees",
        "unsafe keyword",
        "raw pointers",
        "split_at_mut pattern",
    ]
}
```

## 8. Putting It All Together

Open `workshop/src/lib.rs` and replace each `todo!()`:

**Step 1 (Safe functions):** Implement `safe_add` (addition) and
`safe_index` (safe slice access returning `Option`). Tests: 5 pass.

**Step 2 (Unsafe functions):** Implement `unsafe_dereference` (dereference
`*const i32`, check null) and `unsafe_write` (dereference `*mut i32` and
write). Tests: 3 more pass (total 8).

**Step 3 (Memory safety):** Implement `safe_split_sum` using
`slice::split_at_mut` and `sum()`. Tests: 3 more pass (total 11).

**Step 4 (Concepts):** Implement `safety_concepts` returning the concepts
list. Tests: 2 more pass (total 13).

Run `cd workshop && cargo test` after each step.

## 9. Complete Code Reference

```rust
pub fn safe_add(a: i32, b: i32) -> i32 { a + b }

pub unsafe fn unsafe_dereference(ptr: *const i32) -> i32 {
    if ptr.is_null() { 0 } else { *ptr }
}

pub unsafe fn unsafe_write(ptr: *mut i32, val: i32) { *ptr = val; }

pub fn safe_split_sum(slice: &mut [i32]) -> (i32, i32) {
    let mid = slice.len() / 2;
    let (left, right) = slice.split_at_mut(mid);
    (left.iter().sum(), right.iter().sum())
}

pub fn safe_index(slice: &[i32], index: usize) -> Option<i32> {
    slice.get(index).copied()
}

pub fn safety_concepts() -> Vec<&'static str> {
    vec!["safe Rust guarantees", "unsafe keyword", "raw pointers", "split_at_mut pattern"]
}
```

## 10. Summary

| Concept | Python Equivalent | Where Used |
|---|---|---|
| Safe Rust guarantees | Interpreter checks (runtime) | `safe_add`, `safe_index` |
| `unsafe` keyword | No equivalent | `unsafe_dereference`, `unsafe_write` |
| Raw pointers `*const T` / `*mut T` | No equivalent (ctypes is closest) | Parameter types in unsafe functions |
| `split_at_mut` | List slicing (copies) | `safe_split_sum` |
| When to use unsafe | N/A (always safe) | `safety_concepts` discussion |
