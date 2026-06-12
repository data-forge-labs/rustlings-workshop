# Project 19: Ordered set with BTreeSet -- sorted iteration

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 4 tests pass**.

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: BTreeSet (ordered set)](#3-concept-btreeset-ordered-set)
4. [Concept: Range queries with BTreeSet](#4-concept-range-queries-with-btreeset)
5. [Putting It All Together](#5-putting-it-all-together)
6. [Exercises](#6-exercises)
7. [Summary](#7-summary)

## 1. Introduction

A **set** stores unique elements. But what if you need to iterate over those elements in sorted order, or query a range like "all fruits between 'apple' and 'cherry'"? A `HashSet` cannot do this efficiently -- it stores elements by hash with no useful ordering. A `BTreeSet` stores elements in a balanced binary search tree, keeping them ordered by value at all times.

In data engineering, ordered sets are useful for: maintaining sorted lookup tables, deduplicating time-series data while keeping temporal order, range queries on sorted keys, and merge-join operations on pre-sorted datasets.

**Python comparison**: Python's built-in `set` is unordered (like `HashSet`). The closest equivalent to `BTreeSet` is `sortedcontainers.SortedSet` from the third-party `sortedcontainers` library, or simply using `sorted(set(...))` on a standard set when you need ordered output. Rust gives you `BTreeSet` in the standard library -- no third-party packages needed.

```python
# Python: no built-in ordered set
data = ["banana", "apple", "cherry", "apple"]

# Workaround: sort a regular set
unique = sorted(set(data))  # ["apple", "banana", "cherry"]

# Third-party SortedSet (like BTreeSet)
# from sortedcontainers import SortedSet
# s = SortedSet(data)
# s[0]  # "apple" -- supports indexing!
```

In Rust:
```rust
use std::collections::BTreeSet;

let mut set = BTreeSet::new();
set.insert("banana");
set.insert("apple");
set.insert("cherry");

// Iteration is always sorted
for fruit in &set {
    println!("{}", fruit); // apple, banana, cherry
}
```

## 2. Prerequisites

- `Vec`, `HashMap`, `HashSet` from prior projects
- Basic understanding of generics and references
- Project: `../11-HashSetFruit/README.md` -- set concepts (recommended)

## 3. Concept: BTreeSet (ordered set)

### Explanation

`BTreeSet<T>` stores unique elements in sorted order using a B-tree (a generalization of a binary search tree). Insertions, deletions, and lookups are O(log n) -- slower than `HashSet`'s O(1) average, but with the benefit of ordered iteration.

```rust
use std::collections::BTreeSet;

let mut set = BTreeSet::new();
set.insert("zebra");
set.insert("apple");
set.insert("monkey");

// Always sorted
assert_eq!(
    set.iter().copied().collect::<Vec<_>>(),
    vec!["apple", "monkey", "zebra"]
);
```

**Common methods**:

| Method | Description |
|--------|-------------|
| `insert(value)` | Add a value; returns `true` if it was not already present |
| `remove(value)` | Remove a value; returns `true` if it was present |
| `contains(value)` | Check membership |
| `len()` | Number of elements |
| `iter()` | Iterate in sorted order (ascending) |
| `is_empty()` | Check if empty |

**Python comparison**:

```python
# Python set: fast membership, no ordering
s = set(["zebra", "apple", "monkey"])
assert "apple" in s  # O(1) average

# For sorted output, materialize and sort:
sorted_items = sorted(s)  # ["apple", "monkey", "zebra"]
```

Rust's `BTreeSet` keeps the sorted order for free -- no need to call `sorted()` each time. However, insertions are O(log n) vs `HashSet`'s O(1).

### Applying to Our Project

Our `generate_fruit_set` function takes a list of fruit names and an amount, randomly selecting fruits and inserting them into a `BTreeSet`. Since the set deduplicates, the final length may be less than `amount` if there are collisions. The function also returns a `HashMap<&str, u32>` counting how many times each fruit was generated.

## 4. Concept: Range queries with BTreeSet

### Explanation

One of `BTreeSet`'s superpowers is range queries: you can ask for all elements within a range without iterating the whole set.

```rust
use std::collections::BTreeSet;

let mut set = BTreeSet::new();
for i in 0..100 {
    set.insert(i);
}

// All numbers from 20 to 39
let range: Vec<_> = set.range(20..40).copied().collect();
assert_eq!(range.len(), 20);
```

You can also iterate in **reverse** direction:

```rust
for val in set.iter().rev() {
    // descending order
}
```

**Python comparison**: Python's built-in `set` does not support range queries. You'd need to filter:
```python
s = set(range(100))
result = [x for x in s if 20 <= x < 40]  # O(n) scan
```

With `BTreeSet`, range queries are O(log n + k) where k is the number of elements in the range -- much faster for large sets.

### Applying to Our Project

Our `format_set_sorted` function returns a `Vec<&str>` of the set's elements in ascending order. Our `format_set_reverse` function returns them in descending order.

```rust
pub fn format_set_sorted(set: &BTreeSet<&str>) -> Vec<&str> {
    set.iter().copied().collect()
}

pub fn format_set_reverse(set: &BTreeSet<&str>) -> Vec<&str> {
    set.iter().rev().copied().collect()
}
```

## 5. Putting It All Together

Open `workshop/src/lib.rs`. There are two main items to implement.

### Step 1: `generate_fruit_set`

```rust
pub fn generate_fruit_set(
    fruits: &[&str],
    amount: usize,
    rng: &mut impl rand::Rng,
) -> (BTreeSet<&str>, HashMap<&str, u32>) {
    let mut set = BTreeSet::new();
    let mut counter = HashMap::new();

    for _ in 0..amount {
        let fruit = fruits.choose(rng).unwrap();
        set.insert(fruit);
        *counter.entry(fruit).or_insert(0) += 1;
    }

    (set, counter)
}
```

This randomly selects `amount` fruits from the `fruits` slice, inserts each into a `BTreeSet` (deduplicating), and counts occurrences in a `HashMap`.

### Step 2: `format_set_sorted`

Returns the set's elements as a `Vec<&str>` in sorted (ascending) order:

```rust
pub fn format_set_sorted(set: &BTreeSet<&str>) -> Vec<&str> {
    set.iter().copied().collect()
}
```

### Step 3: `format_set_reverse`

Returns elements in descending order:

```rust
pub fn format_set_reverse(set: &BTreeSet<&str>) -> Vec<&str> {
    set.iter().rev().copied().collect()
}
```

Run the tests:

```bash
cd workshop && cargo test
```

All 4 tests should pass. The `test_generate_set_unique` test checks that requesting 3 unique fruits gives a set of length 3. The `test_generate_set_no_duplicates` test verifies that providing only `"apple", "apple"` results in a set of length 1 (deduplication).

## 6. Exercises

### Easy
Write a function `set_to_uppercase(set: &BTreeSet<&str>) -> BTreeSet<String>` that converts every element to uppercase while preserving sorted order.

### Medium
Write a function `fruits_between(set: &BTreeSet<&str>, lo: &str, hi: &str) -> Vec<&str>` that returns all fruits in a lexicographic range `[lo, hi)` using `set.range(...)`.

### Hard
Compare the performance of `BTreeSet` vs `HashSet` for this workload: insert 100,000 elements, then iterate in sorted order. Measure with `std::time::Instant`. Which is faster for insertion? Which for sorted iteration?

## 7. Summary

| Concept | Description | Rust equivalent of Python |
|---------|------------|--------------------------|
| `BTreeSet<T>` | Ordered set backed by B-tree | `sortedcontainers.SortedSet` or `sorted(set(...))` |
| `insert()` / `remove()` | Add/remove elements (O(log n)) | `set.add()` / `set.discard()` |
| `iter()` / `iter().rev()` | Sorted forward/reverse iteration | `sorted(s)` (materializes new list) |
| `range()` | Range queries (O(log n + k)) | List comprehension with filter (O(n)) |
| Deduplication | Automatic on insert | `set` (same behavior) |

Choose `BTreeSet` when you need ordered iteration, range queries, or consistent comparison-based ordering. Choose `HashSet` when you only need fast membership testing and don't care about order.

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

