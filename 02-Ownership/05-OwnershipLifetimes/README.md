# Lifetime Annotations and Move Semantics -- Mastering the Borrow Checker

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 14 tests pass**.

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Why Lifetimes? Python vs Rust](#3-concept-why-lifetimes-python-vs-rust)
4. [Concept: Lifetime Annotations -- `'a`](#4-concept-lifetime-annotations---a)
5. [Concept: Lifetime Elision Rules](#5-concept-lifetime-elision-rules)
6. [Concept: Structs with Lifetime Parameters](#6-concept-structs-with-lifetime-parameters)
7. [Concept: Move vs Copy Semantics](#7-concept-move-vs-copy-semantics)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Exercises](#9-exercises)
10. [Summary](#10-summary)

---

## 1. Introduction

Every reference in Rust has a **lifetime** -- the scope for which that reference is valid. Most of the time lifetimes are implicit and inferred, just like types are inferred. But when the compiler cannot figure out the correct lifetime, you must annotate it.

In this workshop you will:

- Learn **why lifetimes exist** by contrasting Python's garbage-collected model with Rust's ownership model.
- Write functions with **explicit lifetime annotations** (`'a`) that connect the lifetimes of inputs and outputs.
- Build a **`Bookmark` struct** that holds borrowed string slices -- requiring a lifetime parameter on the struct itself.
- Understand **move semantics** (types like `String` that are moved by default) vs **copy semantics** (types like `i32` that implement `Copy`).
- Implement helper functions that demonstrate each concept and a final **concept-listing function**.

**Data-engineering motivation**: In data pipelines you frequently process record-like structures where borrowing avoids unnecessary allocations. For example, when iterating over a large CSV file and extracting fields, you want to borrow from the input buffer rather than clone every string. Lifetimes let the compiler guarantee that your borrowed references never outlive the data they point to, preventing the kind of use-after-free bugs that would corrupt data in a production pipeline.

---

## 2. Prerequisites

Before starting, you should be comfortable with:

- **Structs and `impl` blocks** -- covered in `../01-MasterMind/README.md`.
- **References (`&T`, `&mut T`)** -- covered in `../02-Ownership/01-TicketV1/README.md`.
- **Ownership basics** -- moving, borrowing, and the three ownership rules from `../02-Ownership/01-TicketV1/`.
- Basic **function syntax** and **string slices (`&str`)** .

If any of these feel unfamiliar, revisit the prerequisite projects first.

---

## 3. Concept: Why Lifetimes? Python vs Rust

### Explanation

In **Python**, memory management is handled by a garbage collector (reference counting + cycle detection). You never think about whether a reference is still valid:

```python
def longest(x, y):
    return x if len(x) >= len(y) else y

a = "hello"
b = "world!!!"
result = longest(a, b)  # Works fine -- GC ensures both strings live as long as needed
print(result)
```

Python keeps a reference count on every object. As long as at least one reference exists, the object stays alive. This is convenient, but it means:

- You pay a runtime cost (reference-count updates, cycle detection).
- Memory is freed non-deterministically -- you cannot know exactly when `__del__` runs.
- There is no compile-time guarantee that a reference is valid.

In **Rust**, there is no garbage collector. Memory is freed the moment the **owner** goes out of scope. Consider this dangerous pattern (which would be a use-after-free bug in C, but Rust prevents it at compile time):

```rust
fn problematic() -> &str {
    let s = String::from("hello");
    &s  // ERROR: returns reference to local variable
}   // s is dropped here -- the reference would dangle
```

Rust rejects this at compile time. The compiler uses **lifetimes** to track how long each reference is valid and ensures you never create a dangling reference.

**Python** lets you write the equivalent without complaint (though the string lives as long as needed because of reference counting):

```python
def problematic():
    s = "hello"
    return s  # Fine -- Python keeps s alive
```

The trade-off: Python's convenience costs runtime overhead and unpredictability. Rust's lifetimes enforce safety at compile time with zero runtime cost.

### ASCII Diagram: Reference Validity

```
Python model (GC):

  variable a ──> "hello"  ←── count = 2
  variable b ──> "world!!!" ←─ count = 2
  result ──────> "world!!!" ←─ count = 3

  All references are valid as long as the object
  has at least one reference. No concept of
  "lifetime" -- the GC handles everything.

Rust model (ownership + lifetimes):

     owner: a    ──> "hello"
     owner: b    ──> "world!!!"
     borrows: result ──> borrows b (lifetime tied to b's scope)

     When b goes out of scope, result becomes invalid.
     The compiler enforces that result is never used
     after b is dropped.
```

### Applying to Our Project

The `longest` function in `workshop/src/lib.rs` is the classic lifetime example. It takes two string slices and returns the longer one. Because the return value could come from either input, Rust needs to know how the lifetimes of the inputs relate to the lifetime of the output. You will annotate this with `'a`.

---

## 4. Concept: Lifetime Annotations -- `'a`

### Explanation

A **lifetime annotation** does not change how long a reference lives. It describes the **relationship** between the lifetimes of multiple references. Think of it as a label that connects inputs to outputs.

Syntax: a tick followed by a name, like `'a`, `'b`, or `'ctx`. By convention, short lowercase names are used (`'a`, `'b`, `'c`).

```
fn function<'a>(param1: &'a T, param2: &'a T) -> &'a T
```

This signature says: "Both `param1` and `param2` share the same lifetime `'a`, and the return value also lives at least as long as `'a`." In practical terms: the returned reference is valid as long as **both** inputs are valid (because the narrower lifetime wins).

### Example (stand-alone)

```rust
fn choose_first<'a>(x: &'a str, y: &'a str) -> &'a str {
    x  // Always returns x, but the signature still ties the lifetimes together
}
```

Without `'a`, the compiler would not know if the returned reference came from `x`, `y`, or some static data.

### ASCII Diagram: Lifetime Relationship

```
Memory timeline:

    'a starts ────────────────────────────────── 'a ends
    │                                              │
    ├── x: &'a str (borrows from outer data)       │
    ├── y: &'a str (borrows from outer data)       │
    │                                              │
    return value: &'a str                          │
    (valid anywhere inside 'a)                     │

    'b (shorter): ────────── 'b ends
    │                         │
    ├── local_value: &'b str  │
    │                         │
    (cannot return &'b str as &'a str
     unless 'b outlives 'a)
```

The key insight: the annotation `'a` on the return value means "the returned reference is valid for at least the duration of `'a`". If the actual reference points to data with a shorter lifetime, the compiler rejects the code.

### Applying to Our Project

In `workshop/src/lib.rs`, implement `longest`:

```rust
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}
```

The `'a` annotation tells the compiler: "The returned reference is valid as long as both `x` and `y` are valid." When you call `longest(s1, s2)`, the concrete lifetime `'a` is the overlap of the lifetimes of `s1` and `s2`.

Similarly, implement `first`, which returns a reference to the first element of a slice:

```rust
pub fn first<'a>(items: &'a [i32]) -> &'a i32 {
    &items[0]
}
```

This uses the same `'a` annotation: the returned reference lives as long as the input slice.

**Run `cd workshop && cargo test` after implementing each function** to see the step_01 tests turn green.

---

## 5. Concept: Lifetime Elision Rules

### Explanation

Rust has **lifetime elision rules** that allow you to omit lifetime annotations in common patterns. The compiler applies these rules automatically:

1. Each input reference gets its own lifetime parameter.
2. If there is exactly one input lifetime, it is assigned to all output references.
3. If there are multiple input lifetimes but one is `&self` or `&mut self`, the output references get the `self` lifetime.

These rules mean that many common functions need no annotation at all.

### Example

```rust
fn first_word(s: &str) -> &str
// Elided: the compiler expands this to:
// fn first_word<'a>(s: &'a str) -> &'a str

fn print_and_return(s: &str, flag: bool) -> &str
// ERROR: two input references, no self -- must annotate
```

The `longest` function cannot use elision because there are **two** input references with no `self`. The compiler does not know which lifetime the output should have -- it could be `x`'s lifetime or `y`'s. You must annotate.

The `first` function **could** use elision because there is only one input reference:

```rust
pub fn first(items: &[i32]) -> &i32
// Elided to: pub fn first<'a>(items: &'a [i32]) -> &'a i32
```

However, in this project the signature is written with explicit `'a` to make the concept visible.

### Python Comparison

```python
# Python has no concept of lifetime elision because
# it has no lifetime annotations at all.
def first_word(s):
    return s.split()[0] if s else ""
```

In Rust, even when elision lets you skip annotations, the compiler still enforces the lifetimes internally. Python simply has no equivalent mechanism.

---

## 6. Concept: Structs with Lifetime Parameters

### Explanation

When a struct holds **references**, the compiler needs to know how long those references are valid. You must add a lifetime parameter to the struct definition:

```rust
struct Bookmark<'a> {
    title: &'a str,
    url: &'a str,
}
```

This says: "A `Bookmark` cannot outlive the strings that `title` and `url` borrow from." Every `impl` block for this struct also needs the lifetime parameter:

```rust
impl<'a> Bookmark<'a> {
    pub fn new(title: &'a str, url: &'a str) -> Self {
        Self { title, url }
    }
}
```

### ASCII Diagram: Struct with References

```
Stack                          Heap (or static data)
┌──────────────┐
│ Bookmark<'a> │                ┌──────────────────┐
│  title ──────┼───────────────>│ "Rust Lang"      │
│  url ────────┼───────────────>│ "https://..."    │
└──────────────┘                └──────────────────┘

  'a is the overlapping lifetime of both string slices.
  The Bookmark cannot outlive either string.
  When the String data is dropped, the Bookmark
  must already be gone.
```

### Python Comparison

```python
# Python -- no lifetime worries
class Bookmark:
    def __init__(self, title: str, url: str):
        self.title = title
        self.url = url

# The GC keeps both title and url alive as long as
# the Bookmark exists. You never need to think
# about lifetimes.
```

In Rust, the compiler enforces that `title` and `url` outlive the `Bookmark`. If you try to create a `Bookmark` with references to a local variable that goes out of scope, the compiler rejects it.

### Applying to Our Project

In `workshop/src/lib.rs`, implement `Bookmark`:

```rust
pub struct Bookmark<'a> {
    pub title: &'a str,
    pub url: &'a str,
}

impl<'a> Bookmark<'a> {
    pub fn new(title: &'a str, url: &'a str) -> Self {
        Self { title, url }
    }

    pub fn display(&self) -> String {
        format!("{} - {}", self.title, self.url)
    }
}
```

The `display` method does **not** need a lifetime parameter on its return type because the rule says `&self`'s lifetime is used for output references. Since `display` returns `String` (an owned type), there is no reference to worry about.

**Note**: String literals (like `"Rust Lang"`) have lifetime `'static` -- they live for the entire program, so they will always satisfy any lifetime constraint.

---

## 7. Concept: Move vs Copy Semantics

### Explanation

Rust has two kinds of value semantics:

- **Move semantics**: When you assign or pass a value, ownership is **moved** from the source to the destination. The source is no longer valid. This applies to types like `String`, `Vec`, and any struct that owns heap data.

- **Copy semantics**: When you assign or pass a value, the bits are **copied** and both the source and destination are valid. This applies to types that implement the `Copy` trait, like integers, booleans, floats, and chars.

```
                Move                          Copy
           ┌──────────────┐            ┌──────────────┐
Before:    │  owner: s    │            │  owner: x    │
           │  [heap data] │            │  [value: 42] │
           └──────────────┘            └──────────────┘

Pass to:   fn move_demo(s)             fn copy_demo(x)

After:     s is INVALID                x is still VALID
           ┌──────────────┐            ┌──────────────┐
           │  // s unusable│           │  owner: x    │
           │  fn owns data │            │  [value: 42] │
           └──────────────┘            └──────────────┘
                                       ┌──────────────┐
                                       │  y: 42 (copy)│
                                       └──────────────┘
```

### Python Comparison

```python
# Python: everything is a reference
a = [1, 2, 3]
b = a          # b refers to the same list
b.append(4)    # a is also affected -- mutation visible through a

# Python integers are immutable -- assignment creates a new binding
x = 42
y = x          # y now refers to 42
x = 43         # x points to a new object; y still points to 42
```

Python does not distinguish between move and copy at the language level -- everything is a reference to an object, and the GC tracks all references. In Rust:

- `String` is moved by default (like passing a unique reference).
- `i32` is copied by default (because it implements `Copy`).
- If you want to copy a `String`, you must call `.clone()` explicitly.

### Applying to Our Project

Implement `move_demo`:

```rust
pub fn move_demo(s: String) -> String {
    s  // s is moved into the function, then moved back out
}
```

Because `String` does not implement `Copy`, the value is moved. After calling `move_demo(input)`, `input` is no longer valid in the caller -- ownership has been transferred into the function and then returned.

Implement `copy_demo`:

```rust
pub fn copy_demo(x: i32) -> i32 {
    x  // x is copied, so the original is still valid
}
```

Because `i32` implements `Copy`, the value is copied into the function. The caller retains ownership of the original.

**Edge case**: The test `test_move_demo_appends_text` does not check a specific value, only that the output has `len() > 0`. This is intentional -- the function currently just returns its input unchanged. You could extend it to modify the string before returning, as long as ownership is preserved.

---

## 8. Putting It All Together

Now implement all functions in `workshop/src/lib.rs`. The complete implementation is:

```rust
/// Return the longer of two string slices (basic lifetime demo)
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

/// Return the first element of a slice (lifetime elision)
pub fn first<'a>(items: &'a [i32]) -> &'a i32 {
    &items[0]
}

/// A struct with a reference (requires lifetime annotation)
pub struct Bookmark<'a> {
    pub title: &'a str,
    pub url: &'a str,
}

impl<'a> Bookmark<'a> {
    pub fn new(title: &'a str, url: &'a str) -> Self {
        Self { title, url }
    }

    pub fn display(&self) -> String {
        format!("{} - {}", self.title, self.url)
    }
}

/// Demonstrate move semantics
pub fn move_demo(s: String) -> String {
    s
}

/// Demonstrate Copy types (i32 is Copy)
pub fn copy_demo(x: i32) -> i32 {
    x
}

/// Return a list of lifetime/ownership concepts
pub fn lifetime_concepts() -> Vec<&'static str> {
    vec![
        "Ownership: each value has exactly one owner",
        "Borrowing: &T (immutable) and &mut T (mutable)",
        "Lifetime annotations: 'a ties references together",
        "Lifetime elision: compiler infers lifetimes in common patterns",
        "Struct lifetimes: types can be parameterized with 'a",
        "Move semantics: non-Copy types transfer ownership",
        "Copy semantics: i32, bool, and other stack-only types copy by default",
    ]
}
```

**Step-by-step verification:**

1. Implement `longest` and `first` -- test `step_01_lifetime_functions` (6 tests).
2. Implement `Bookmark` struct and methods -- test `step_02_struct_lifetimes` (3 tests).
3. Implement `move_demo` and `copy_demo` -- test `step_03_move_vs_copy` (4 tests).
4. Implement `lifetime_concepts` -- test `step_04_concepts` (3 tests).

After each step, run:

```bash
cd workshop && cargo test
```

Watch the test count grow from 0 to 14.

---

## 9. Exercises

### Exercise 1: Second Element (Easy)

Add a function `second<'a>(items: &'a [i32]) -> Option<&'a i32>` that returns `Some(&items[1])` if the slice has at least two elements, and `None` otherwise. Write two tests: one with a slice containing 2+ elements, one with 1 element.

### Exercise 2: Static Bookmark (Medium)

Create a function `static_bookmark() -> Bookmark<'static>` that returns a `Bookmark` with `title: "Rust Bookmark"` and `url: "https://doc.rust-lang.org"`. Both are string literals, so they have `'static` lifetime. Write a test that verifies the returned `Bookmark` has the correct fields.

### Exercise 3: Lifetime-Safe Configuration (Medium)

Suppose you have a config struct:

```rust
struct Config<'a> {
    host: &'a str,
    port: u16,
}
```

Implement `Config::new` and a `Config::endpoint` method that returns a `String` in the form `"host:port"`. Write a test that creates a `Config` and calls `endpoint`. Also write a test that is rejected by the compiler (explain why in a comment -- this test does not need to compile). Example: a `Config` that borrows from a local `String` that goes out of scope.

---

## 10. Summary

| Concept | Description | Used In |
|---|---|---|
| Lifetime annotations (`'a`) | Tie the lifetimes of function parameters and return values together | `longest`, `first` |
| Lifetime elision | Rules that let Rust infer lifetimes automatically (1 input = output gets its lifetime) | Implicit in method calls |
| Struct lifetime params | Structs holding references must declare lifetime parameters | `Bookmark<'a>` |
| Move semantics | Non-Copy types transfer ownership on assignment/parameter passing | `move_demo` |
| Copy semantics | Stack-only types (i32, bool) implement `Copy` and duplicate bits automatically | `copy_demo` |
| `'static` lifetime | References that live for the entire program (e.g., string literals) | `lifetime_concepts` return type, Exercise 2 |
| Borrow checker | Compile-time analysis that enforces all reference validity | Enforced across all functions |

**What you have learned**: You can now write functions and structs that use references with explicit lifetime annotations. You understand why Rust requires lifetimes (no GC) and how they differ from Python's reference-counting model. You can distinguish between move and copy semantics and know when each applies.

**Next steps**: Continue to Section 3 (Collections) where you will use these same ownership and lifetime concepts with `Vec`, `HashMap`, and iterators -- all of which depend on understanding how references and lifetimes work.
