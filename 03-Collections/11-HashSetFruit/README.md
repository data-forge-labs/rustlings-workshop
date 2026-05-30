# Project 23: Unique items with HashSet -- membership testing and deduplication

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 4 tests pass**.

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: HashSet (hash-based set)](#3-concept-hashset-hash-based-set)
4. [Concept: Set operations (union, intersection, difference)](#4-concept-set-operations-union-intersection-difference)
5. [Putting It All Together](#5-putting-it-all-together)
6. [Exercises](#6-exercises)
7. [Summary](#7-summary)

## 1. Introduction

A **set** is a collection that stores each element at most once. The fundamental operations -- membership testing, deduplication, and set algebra (union, intersection, difference) -- appear constantly in data engineering: deduplicating event streams, checking whether a user ID exists in a cohort, finding common records across datasets, and filtering out already-processed items.

Rust's `HashSet<T>` is a hash-based set with O(1) average-time insertions, deletions, and lookups. It is the direct counterpart to Python's built-in `set`.

**Python comparison**:

```python
# Python set
fruits = {"apple", "banana", "cherry"}
fruits.add("date")
fruits.add("apple")    # no-op, already present
assert len(fruits) == 4
assert "banana" in fruits
```

```rust
// Rust HashSet
use std::collections::HashSet;

let mut fruits = HashSet::new();
fruits.insert("apple");
fruits.insert("banana");
fruits.insert("cherry");
fruits.insert("date");
fruits.insert("apple");    // no-op, already present
assert_eq!(fruits.len(), 4);
assert!(fruits.contains("banana"));
```

## 2. Prerequisites

- Basic ownership and borrowing
- `Vec`, `HashMap` from prior projects
- Project `../10-BTreeSetFruit/README.md` for comparison between set types (recommended)

## 3. Concept: HashSet (hash-based set)

### Explanation

`HashSet<T>` stores unique values using a hash function. The type `T` must implement `Hash + Eq` -- both can be derived automatically.

```rust
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct UserId(u64);

let mut users = HashSet::new();
users.insert(UserId(42));
users.insert(UserId(7));
users.insert(UserId(42));  // duplicate, ignored

assert_eq!(users.len(), 2);
assert!(users.contains(&UserId(42)));
```

**Common methods**:

| Method | Description |
|--------|-------------|
| `insert(value)` | Add a value; returns `true` if the value was not already present |
| `remove(value)` | Remove a value; returns `true` if it was present |
| `contains(value)` | Check membership |
| `len()` | Number of elements |
| `is_empty()` | Check if empty |
| `iter()` | Iterate over references to elements (order unspecified) |
| `clear()` | Remove all elements |

**Python comparison**:

```python
s = set()
s.add("apple")          # insert
s.discard("apple")      # remove (no error if missing)
s.remove("banana")      # remove (raises KeyError if missing)
"apple" in s            # contains
len(s)                  # size
```

Note: Python's `set.discard` is like Rust's `remove` (returns `true`/`false`). Python's `set.remove` raises an error if missing -- Rust has no direct equivalent, so you'd use `remove()` and check the return value.

### Why not just use a Vec?

A `Vec` can store unique items too, but checking membership requires scanning the entire vector (O(n)). A `HashSet` checks membership in O(1) average:

```rust
// Vec: O(n) contains -- slow for large collections
let vec = vec!["apple", "banana", "cherry"];
assert!(vec.contains(&"banana"));  // scans the whole vec

// HashSet: O(1) contains -- fast
let set: HashSet<&str> = vec.iter().copied().collect();
assert!(set.contains("banana"));
```

### Applying to Our Project

We will generate random fruits, collect them into a `HashSet` to deduplicate, and also maintain a `HashMap` counting how many times each fruit was generated. The `HashSet` ensures we know which unique fruits appeared, while the `HashMap` tracks frequency.

## 4. Concept: Set operations (union, intersection, difference)

### Explanation

Rust's `HashSet` supports set algebra via methods that produce iterators:

```rust
let a: HashSet<_> = [1, 2, 3].iter().copied().collect();
let b: HashSet<_> = [3, 4, 5].iter().copied().collect();

// Union: elements in a OR b
let union: Vec<_> = a.union(&b).copied().collect();   // [1, 2, 3, 4, 5]

// Intersection: elements in a AND b
let intersection: Vec<_> = a.intersection(&b).copied().collect(); // [3]

// Difference: elements in a but NOT b
let diff: Vec<_> = a.difference(&b).copied().collect(); // [1, 2]

// Symmetric difference: elements in a or b but not both
let sym_diff: Vec<_> = a.symmetric_difference(&b).copied().collect(); // [1, 2, 4, 5]
```

**Python comparison**:

```python
a = {1, 2, 3}
b = {3, 4, 5}

assert a | b == {1, 2, 3, 4, 5}         # union
assert a & b == {3}                      # intersection
assert a - b == {1, 2}                   # difference
assert a ^ b == {1, 2, 4, 5}             # symmetric difference
```

The Python operators (`|`, `&`, `-`, `^`) are syntactic sugar. Rust uses named methods, which is more verbose but clearer when reading unfamiliar code.

### Applying to Our Project

While the current tests don't explicitly test set operations, you can experiment with them. The `collect_unique_fruits` function returns a `HashSet<&'static str>` and `HashMap<&'static str, u32>` from the same data -- a pattern used often in data engineering for "unique items + counts" (like word frequency analysis).

## 5. Putting It All Together

Open `workshop/src/lib.rs`. There are two functions to implement.

### Step 1: `generate_fruit`

This function returns a random fruit name from a fixed list:

```rust
pub fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple", "Banana", "Cherry", "Date", "Elderberry",
        "Fig", "Grape", "Honeydew",
    ];
    let mut rng = rand::thread_rng();
    fruits.choose(&mut rng).unwrap()
}
```

Run the step 1 test:

```bash
cd workshop && cargo test step_01
```

The test `test_generate_fruit_returns_valid` checks that the returned fruit is one of the 8 valid options.

### Step 2: `collect_unique_fruits`

This function generates `count` random fruits, inserts them into both a `HashSet` (for uniqueness) and a `HashMap` (for frequency):

```rust
pub fn collect_unique_fruits(count: usize) -> (HashSet<&'static str>, HashMap<&'static str, u32>) {
    let mut unique = HashSet::new();
    let mut counter = HashMap::new();

    for _ in 0..count {
        let fruit = generate_fruit();
        unique.insert(fruit);
        *counter.entry(fruit).or_insert(0) += 1;
    }

    (unique, counter)
}
```

Since there are only 8 possible fruits and `count` can be larger, some fruits will repeat. The `HashSet` will contain at most 8 elements.

Run all tests:

```bash
cd workshop && cargo test
```

All 4 tests should pass. The `test_collect_unique_fruits` test calls this with `count=10` and verifies the set has at most 8 elements (less if collisions are unlucky, but never more than 8).

## 6. Exercises

### Easy
Write a function `unique_count(items: &[&str]) -> usize` that returns the number of unique items using a `HashSet`.

### Medium
Write a function `find_common(a: &[&str], b: &[&str]) -> Vec<&str>` that returns items present in both slices, using `HashSet` intersection.

### Hard
Write a function `dedup_stable<T: Clone + Eq + Hash>(items: &[T]) -> Vec<T>` that removes duplicates but preserves the original order of first occurrence. (Hint: use a `HashSet` to track seen items while iterating.)

## 7. Summary

| Concept | Description | Rust equivalent of Python |
|---------|------------|--------------------------|
| `HashSet<T>` | Hash-based set for unique items | `set` |
| `insert()` / `contains()` | O(1) add and membership test | `set.add()` / `in` |
| `union()` | Elements in self OR other | `a | b` |
| `intersection()` | Elements in self AND other | `a & b` |
| `difference()` | Elements in self but NOT other | `a - b` |
| Deduplication pattern | `HashSet` + `HashMap` for unique + count | `collections.Counter` |

`HashSet` is your go-to for fast membership testing and deduplication. When you also need ordered iteration, reach for `BTreeSet` instead.
