# Project 28: Lazy functional iteration -- Iterator trait, map, filter, fold

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 12 tests pass**.

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: The Iterator trait and next()](#3-concept-the-iterator-trait-and-next)
4. [Concept: Functional adapters -- map, filter, fold](#4-concept-functional-adapters----map-filter-fold)
5. [Concept: take, skip, rev, and flatten](#5-concept-take-skip-rev-and-flatten)
6. [Putting It All Together](#6-putting-it-all-together)
7. [Exercises](#7-exercises)
8. [Summary](#8-summary)

## 1. Introduction

Iteration is the backbone of data processing. Every time you loop over rows in a CSV file, transform a column, filter out null values, or aggregate a sum, you are iterating. Rust's iterator system is one of its most powerful features: it is **lazy** (no work is done until you consume the iterator), **composable** (chain adapters like `map`, `filter`, `fold`), and **zero-cost** (the compiler optimizes the chain into efficient machine code with no runtime overhead).

In data engineering, iterator patterns replace many explicit loops: `map` transforms each record, `filter` drops unwanted rows, `fold` (or `reduce`) aggregates, `take` limits results, and `flatten` merges nested collections.

**Python comparison**:

```python
# Python: lazy iteration with generators and itertools
data = [1, 2, 3, 4, 5]
result = sum(x * 2 for x in data if x % 2 == 0)  # 2*2 + 2*4 = 12
```

```rust
// Rust: lazy iteration with adapters
let data = vec![1, 2, 3, 4, 5];
let result: i32 = data.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .sum();  // 12
```

Both are lazy and composable. Rust's iterators are more explicit about types and ownership, but the pattern is the same.

## 2. Prerequisites

- Safe Rust basics: functions, closures, generics
- `Vec` and slices (`&[T]`)
- Project `../13-MutableFruitSalad/README.md` -- Vec mutation (optional)

## 3. Concept: The Iterator trait and next()

### Explanation

An **iterator** is any type that implements the `Iterator` trait, which requires one method:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Each call to `next()` returns `Some(item)` or `None` when the iteration is exhausted. This is the same protocol as Python's `__next__` / `StopIteration`.

```rust
let v = vec![10, 20, 30];
let mut iter = v.iter();

assert_eq!(iter.next(), Some(&10));
assert_eq!(iter.next(), Some(&20));
assert_eq!(iter.next(), Some(&30));
assert_eq!(iter.next(), None);
```

**Python comparison**:

```python
v = [10, 20, 30]
it = iter(v)
assert next(it) == 10
assert next(it) == 20
assert next(it) == 30
# next(it) raises StopIteration (Python's None equivalent)
```

Note the difference: Rust returns `Option<&T>` from `iter()` (borrowing), while Python returns values directly and uses `StopIteration` to signal exhaustion. Rust's approach is type-safe -- the compiler ensures you handle the `None` case.

**Three ways to get an iterator**:

| Method | Yields | Ownership |
|--------|--------|-----------|
| `.iter()` | `&T` | Borrows elements |
| `.iter_mut()` | `&mut T` | Mutable borrows |
| `.into_iter()` | `T` | Consumes the collection |

### Applying to Our Project

All functions in `lib.rs` accept a slice `&[T]` or `Vec<T>` and use iterator methods internally. No explicit `next()` calls are needed -- we'll use adapter methods.

## 4. Concept: Functional adapters -- map, filter, fold

### Explanation

Instead of writing explicit `for` loops, Rust lets you chain iterator adapters.

#### `map` -- transform each element

```rust
let numbers = vec![1, 2, 3];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
assert_eq!(doubled, vec![2, 4, 6]);
```

Python equivalent: `map(lambda x: x * 2, numbers)` or `[x * 2 for x in numbers]`.

#### `filter` -- keep matching elements

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];
let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).copied().collect();
assert_eq!(evens, vec![2, 4, 6]);
```

Python equivalent: `filter(lambda x: x % 2 == 0, numbers)` or `[x for x in numbers if x % 2 == 0]`.

#### `fold` -- accumulate a value (reduce)

```rust
let numbers = vec![1, 2, 3, 4, 5];
let sum = numbers.iter().fold(0, |acc, x| acc + x);
assert_eq!(sum, 15);
```

Python equivalent: `functools.reduce(lambda acc, x: acc + x, numbers, 0)`. But more commonly, Python uses `sum(numbers)` or a manual loop.

`fold` takes:
1. An initial accumulator value (`0`)
2. A closure with two arguments: the accumulator and the current element

**Why `fold` over a `for` loop?** `fold` is explicit about the initial state and the update rule, making the code easier to reason about and less error-prone (no accidentally uninitialized variables).

### Applying to Our Project

Three functions directly use these:

```rust
// sum_with_fold: use fold to sum all elements
pub fn sum_with_fold(numbers: &[i32]) -> i32 {
    numbers.iter().fold(0, |acc, x| acc + x)
}

// keep_even: use filter to keep only even numbers
pub fn keep_even(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|x| *x % 2 == 0).copied().collect()
}

// double_all: use map to double each element
pub fn double_all(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().map(|x| x * 2).collect()
}
```

Note the `.copied()` call between `filter` and `collect`. Because `.iter()` yields `&i32`, we need to dereference. `.copied()` converts `&i32` to `i32` by copying (cheap for integers).

## 5. Concept: take, skip, rev, and flatten

### Explanation

#### `take(n)` -- limit to first n elements

```rust
let v = vec![1, 2, 3, 4, 5];
let first_three: Vec<_> = v.iter().take(3).copied().collect();
assert_eq!(first_three, vec![1, 2, 3]);
```

If there are fewer than n elements, `take` returns whatever is available.

Python equivalent: `itertools.islice(v, 3)`.

#### `skip(n)` -- drop first n elements

```rust
let v = vec![1, 2, 3, 4, 5];
let after_two: Vec<_> = v.iter().skip(2).copied().collect();
assert_eq!(after_two, vec![3, 4, 5]);
```

Python equivalent: `itertools.islice(v, 2, None)`.

#### `rev()` -- reverse the iterator direction

```rust
let v = vec![1, 2, 3];
let reversed: Vec<_> = v.iter().rev().copied().collect();
assert_eq!(reversed, vec![3, 2, 1]);
```

Python equivalent: `reversed(v)` (which returns a lazy iterator).

#### `flatten()` -- merge nested iterators

```rust
let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
let flat: Vec<_> = nested.into_iter().flatten().collect();
assert_eq!(flat, vec![1, 2, 3, 4, 5]);
```

Python equivalent: `itertools.chain.from_iterable(nested)`.

### Applying to Our Project

```rust
pub fn take_first_n<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    items.iter().take(n).cloned().collect()
}

pub fn skip_first_n<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    items.iter().skip(n).cloned().collect()
}

pub fn reverse_slice<T: Clone>(items: &[T]) -> Vec<T> {
    items.iter().rev().cloned().collect()
}

pub fn flatten<T: Clone>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}
```

Note: `.cloned()` is the same as `.copied()` but more general (works with any `Clone` type). `.copied()` is slightly more specific and only works with `Copy` types.

## 6. Putting It All Together

Open `workshop/src/lib.rs` and implement all 7 functions. Run tests step by step:

```bash
# Step 1: fold
cd workshop && cargo test step_01

# Step 2: filter
cd workshop && cargo test step_02

# Step 3: map
cd workshop && cargo test step_03

# Step 4: take / skip
cd workshop && cargo test step_04

# Step 5: rev
cd workshop && cargo test step_05

# Step 6: flatten
cd workshop && cargo test step_06

# All tests
cd workshop && cargo test
```

All 12 tests should pass.

## 7. Exercises

### Easy
Write a function `squares(n: usize) -> Vec<usize>` that returns the first n square numbers (1, 4, 9, 16...) using `map` and a range `0..n`.

### Medium
Write a function `running_total(numbers: &[i32]) -> Vec<i32>` that returns a vector of cumulative sums using `scan` (look up `Iterator::scan` in the docs).

### Hard
Implement a custom iterator `Fibonacci` that yields Fibonacci numbers. It should implement `Iterator<Item = u64>`. Then use `.take(10).collect()` to get the first 10 Fibonacci numbers.

## 8. Summary

| Concept | Description | Python equivalent |
|---------|------------|-------------------|
| `Iterator` trait | A type that yields `Option<Item>` on `next()` | `__iter__` / `__next__` protocol |
| `map()` | Transform each element | `map()` or comprehension |
| `filter()` | Keep elements matching a predicate | `filter()` or comprehension `if` |
| `fold()` | Reduce to a single value | `functools.reduce()` |
| `take(n)` | Limit to first n elements | `itertools.islice(v, n)` |
| `skip(n)` | Drop first n elements | `itertools.islice(v, n, None)` |
| `rev()` | Reverse iteration | `reversed()` |
| `flatten()` | Merge nested iterables | `itertools.chain.from_iterable()` |
| Lazy evaluation | No work until `collect()` or `next()` | Generator expressions / `yield` |
| Zero-cost | Compiler inlines iterator chains | N/A (Python has runtime overhead) |

Rust iterators are lazy, composable, and zero-cost. They let you express complex data transformations in a clear, functional style while matching or exceeding hand-written loop performance.
