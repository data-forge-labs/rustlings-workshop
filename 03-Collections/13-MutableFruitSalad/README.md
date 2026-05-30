# Project 36: Vec mutation -- push, pop, insert, remove, capacity vs length

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 6 tests pass**.

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Vec ownership and mutation](#3-concept-vec-ownership-and-mutation)
4. [Concept: push, pop, insert, remove](#4-concept-push-pop-insert-remove)
5. [Concept: sort and random selection](#5-concept-sort-and-random-selection)
6. [Concept: Capacity vs length](#6-concept-capacity-vs-length)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Exercises](#8-exercises)
9. [Summary](#9-summary)

## 1. Introduction

A `Vec<T>` is Rust's growable array -- the closest equivalent to Python's list. It's the most commonly used collection in Rust, and understanding how to mutate it efficiently is essential for every data engineer. In this workshop, you will build functions to add, remove, sort, and randomly pick elements from a fruit salad stored in a `Vec<&str>`.

You will learn the fundamental mutation patterns: `push` (append), `pop` (remove last), `insert` (add at index), `remove` (delete at index), `sort` (in-place ordering), and random selection. Each function demonstrates a different aspect of Rust's ownership system -- particularly the need for `&mut` access to modify a vector.

**Python comparison**:

```python
# Python list mutations
salad = ["apple", "banana"]
salad.append("cherry")     # push
last = salad.pop()         # pop from end
salad.insert(0, "date")    # insert at index
removed = salad.pop(1)     # remove at index (no dedicated method)
salad.sort()               # in-place sort
```

```rust
// Rust Vec mutations
let mut salad = vec!["apple", "banana"];
salad.push("cherry");      // push
let last = salad.pop();    // pop from end (returns Option)
salad.insert(0, "date");   // insert at index
let removed = salad.remove(1); // remove at index
salad.sort();              // in-place sort
```

## 2. Prerequisites

- Ownership and borrowing basics (`&mut` references)
- `Vec` creation and indexing
- Project `../02-VectorFruitSalad/README.md` -- Vec fundamentals

## 3. Concept: Vec ownership and mutation

### Explanation

To modify a `Vec`, you need a **mutable reference** (`&mut Vec<T>`). This is a key difference from Python, where any reference to a list can modify it. Rust's borrow checker ensures that while you hold a mutable reference, no other reference can read or write the same data -- preventing the kind of bugs that plague multi-threaded Python code.

```rust
let mut fruits = vec!["apple", "banana"];

let mut_ref = &mut fruits;
mut_ref.push("cherry");   // OK: we have exclusive access

// let other_ref = &fruits;  // ERROR: cannot borrow as immutable
//                               because it is also borrowed as mutable
```

**Python comparison**: Python has no such restriction:

```python
fruits = ["apple", "banana"]
ref1 = fruits
ref2 = fruits
ref1.append("cherry")   # ref2 also sees the change -- was that intended?
```

Rust's rule is: either one mutable reference XOR any number of immutable references. This eliminates an entire class of bugs.

### Applying to Our Project

Every function in `lib.rs` that modifies the `Vec` takes `&mut Vec<&str>`:

```rust
pub fn add_fruit(fruit_salad: &mut Vec<&str>, fruit: &str) {
    fruit_salad.push(fruit);
}
```

## 4. Concept: push, pop, insert, remove

### Explanation

**`push(value)`** -- appends to the end. O(1) amortized.

```rust
let mut v = vec![1, 2, 3];
v.push(4);
assert_eq!(v, vec![1, 2, 3, 4]);
```

Python equivalent: `list.append(4)`.

**`pop() -> Option<T>`** -- removes and returns the last element. Returns `None` if empty.

```rust
let mut v = vec![1, 2, 3];
assert_eq!(v.pop(), Some(3));
assert_eq!(v.pop(), Some(2));
assert_eq!(v.pop(), Some(1));
assert_eq!(v.pop(), None);
```

Python equivalent: `list.pop()` (raises `IndexError` on empty -- Rust uses `Option` instead).

**`insert(index, value)`** -- inserts at a position, shifting all elements after it right. O(n).

```rust
let mut v = vec![1, 2, 3];
v.insert(1, 99);
assert_eq!(v, vec![1, 99, 2, 3]);
```

Python equivalent: `list.insert(1, 99)`.

**`remove(index) -> T`** -- removes and returns the element at index, shifting all elements after it left. O(n).

```rust
let mut v = vec![1, 2, 3];
let removed = v.remove(1);
assert_eq!(removed, 2);
assert_eq!(v, vec![1, 3]);
```

Python equivalent: `list.pop(1)`. Note: Python uses the same `pop()` for both last and indexed removal. Rust separates them: `pop()` for last, `remove(index)` for arbitrary position.

### Applying to Our Project

Two functions map directly:

```rust
pub fn add_fruit(fruit_salad: &mut Vec<&str>, fruit: &str) {
    fruit_salad.push(fruit);
}

pub fn remove_fruit(fruit_salad: &mut Vec<&str>, fruit_to_remove: &str) -> bool {
    if let Some(pos) = fruit_salad.iter().position(|&f| f == fruit_to_remove) {
        fruit_salad.remove(pos);
        true
    } else {
        false
    }
}
```

`remove_fruit` finds the **first** occurrence using `position()`, removes it, and returns `true`. If the fruit is not found, it returns `false`.

## 5. Concept: sort and random selection

### Explanation

**`sort()`** -- sorts the vector in place. Requires elements to implement `Ord`.

```rust
let mut v = vec![3, 1, 2];
v.sort();
assert_eq!(v, vec![1, 2, 3]);
```

Python equivalent: `list.sort()`. Both sort in-place.

**Random selection** -- using `rand::seq::SliceRandom::choose`:

```rust
use rand::seq::SliceRandom;

let v = vec!["apple", "banana", "cherry"];
let mut rng = rand::thread_rng();

if let Some(picked) = v.choose(&mut rng) {
    println!("Picked: {}", picked);
}
```

Python equivalent: `random.choice(list)`.

Note that `choose` returns `Option<&T>` -- if the slice is empty, it returns `None`. Python's `random.choice` raises `IndexError` instead.

### Applying to Our Project

```rust
pub fn sort_fruits(fruit_salad: &mut Vec<&str>) {
    fruit_salad.sort();
}

pub fn pick_random_fruit<'a>(
    fruit_salad: &[&'a str],
    rng: &mut impl rand::Rng,
) -> Option<&'a str> {
    fruit_salad.choose(rng).copied()
}
```

Note that `pick_random_fruit` takes `&[&str]` (an immutable slice) rather than `&mut Vec<&str>` -- it only reads, not writes. The return type is `Option<&str>`: `None` for an empty salad, `Some(fruit)` otherwise.

## 6. Concept: Capacity vs length

### Explanation

A `Vec` has two internal numbers:

- **Length** (`len()`): how many elements are currently stored.
- **Capacity** (`capacity()`): how many elements the `Vec` can hold before it needs to reallocate.

```rust
let mut v: Vec<i32> = Vec::with_capacity(10);
assert_eq!(v.len(), 0);
assert_eq!(v.capacity(), 10);

v.push(1);
v.push(2);
v.push(3);
assert_eq!(v.len(), 3);
assert_eq!(v.capacity(), 10);  // still has room for 7 more
```

When `len()` would exceed `capacity()`, the `Vec` doubles its capacity (reallocating all elements to new memory). This is why `push` is O(1) amortized.

**Python comparison**: Python's list also uses dynamic arrays with over-allocation, but you cannot inspect capacity directly. The overhead management is hidden.

```python
import sys
v = []
prev = 0
for i in range(100):
    v.append(i)
    size = sys.getsizeof(v)
    if size != prev:
        print(f"Len {len(v):3d}: {size} bytes")  # capacity resizes implicitly
        prev = size
```

### Applying to Our Project

While the current tests don't explicitly test capacity, understanding it helps you write efficient code. If you know the final size of a `Vec`, use `Vec::with_capacity(n)` to avoid unnecessary reallocations:

```rust
let mut salad = Vec::with_capacity(100);  // we know we'll push ~100 fruits
```

## 7. Putting It All Together

Open `workshop/src/lib.rs` and implement all 4 functions:

```rust
use rand::seq::SliceRandom;

pub fn remove_fruit(fruit_salad: &mut Vec<&str>, fruit_to_remove: &str) -> bool {
    if let Some(pos) = fruit_salad.iter().position(|&f| f == fruit_to_remove) {
        fruit_salad.remove(pos);
        true
    } else {
        false
    }
}

pub fn add_fruit(fruit_salad: &mut Vec<&str>, fruit: &str) {
    fruit_salad.push(fruit);
}

pub fn sort_fruits(fruit_salad: &mut Vec<&str>) {
    fruit_salad.sort();
}

pub fn pick_random_fruit<'a>(
    fruit_salad: &[&'a str],
    rng: &mut impl rand::Rng,
) -> Option<&'a str> {
    fruit_salad.choose(rng).copied()
}
```

Run the tests:

```bash
cd workshop && cargo test
```

All 6 tests should pass.

## 8. Exercises

### Easy
Write a function `double_pop(v: &mut Vec<i32>) -> Option<(i32, i32)>` that pops two elements and returns them as a tuple. Return `None` if there are fewer than 2 elements.

### Medium
Write a function `rotate_left(v: &mut Vec<i32>, k: usize)` that rotates the vector left by `k` positions. Use `remove(0)` and `push()`.

### Hard
Write a function `compress<T: Clone + Eq>(v: &[T]) -> Vec<T>` that removes consecutive duplicates (like `Iterator::dedup`). For example, `[1, 1, 2, 2, 2, 3]` becomes `[1, 2, 3]`. Use a `Vec` to build the result.

## 9. Summary

| Concept | Description | Rust equivalent of Python |
|---------|------------|--------------------------|
| `push(value)` | Append to end | `list.append(value)` |
| `pop() -> Option<T>` | Remove and return last | `list.pop()` (raises on empty) |
| `insert(i, v)` | Insert at index (O(n)) | `list.insert(i, v)` |
| `remove(i) -> T` | Remove at index (O(n)) | `list.pop(i)` |
| `sort()` | In-place sort | `list.sort()` |
| `choose(rng) -> Option<&T>` | Random element | `random.choice(list)` (raises on empty) |
| `capacity()` vs `len()` | Allocated vs used space | Hidden (sys.getsizeof) |
| `&mut Vec<T>` | Mutable reference for mutation | Any reference can mutate |

Vec mutation patterns in Rust match Python's list operations closely, with the key difference that every mutation requires explicit `&mut` access. This makes data-flow clear and prevents aliasing bugs, especially in concurrent or complex code.
