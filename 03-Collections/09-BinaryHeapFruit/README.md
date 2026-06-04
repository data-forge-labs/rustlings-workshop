# Project 18: Priority queue with BinaryHeap -- max-heap operations

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 5 tests pass**.

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: BinaryHeap (priority queue)](#3-concept-binaryheap-priority-queue)
4. [Concept: Ord and PartialOrd for custom ordering](#4-concept-ord-and-partialord-for-custom-ordering)
5. [Putting It All Together](#5-putting-it-all-together)
6. [Exercises](#6-exercises)
7. [Summary](#7-summary)

## 1. Introduction

A **priority queue** is a data structure where elements are dequeued in order of priority -- highest priority first -- rather than insertion order. In data engineering, priority queues appear everywhere: scheduling ETL jobs (run the most urgent task first), streaming top-K queries (find the 10 hottest trending topics), merging sorted files, and implementing Dijkstra's shortest-path algorithm.

In this workshop, you will build a fruit-salad generator that uses `BinaryHeap` to always serve "Fig" first (because figs are the best fruit, obviously). You will define a custom `Fruit` enum, implement ordering so that `Fig` outranks all other fruits, and generate a random heap.

**Python comparison**: Python's `heapq` module provides a **min-heap** -- the smallest element is popped first. Rust's `BinaryHeap` is a **max-heap** -- the largest element is popped first. To get min-heap behavior in Rust, you reverse the ordering. Another difference: Python uses a list with `heapq` functions, while Rust stores elements directly in the `BinaryHeap` collection.

```python
import heapq

# Python: min-heap
heap = []
heapq.heappush(heap, "apple")
heapq.heappush(heap, "fig")
heapq.heappush(heap, "banana")
print(heapq.heappop(heap))  # "apple" (smallest first)
```

In Rust, the same code would pop `"fig"` first because strings are sorted alphabetically and `BinaryHeap` is a max-heap.

## 2. Prerequisites

- Basic Rust syntax: enums, match, function definitions
- Familiarity with `std::collections` (Vec, HashMap from previous projects)
- Project: `../12-RustIterators/README.md` -- iterator concepts (optional but helpful)

## 3. Concept: BinaryHeap (priority queue)

### Explanation

`BinaryHeap<T>` is a priority queue implemented as a binary heap. Elements must implement `Ord` (ordered). The heap property guarantees that the **maximum** element is always at the front (accessible via `.peek()` and removable via `.pop()`).

```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(10);
heap.push(5);
heap.push(20);

assert_eq!(heap.peek(), Some(&20)); // largest element
assert_eq!(heap.pop(), Some(20));   // removed
assert_eq!(heap.pop(), Some(10));
assert_eq!(heap.pop(), Some(5));
```

**Common methods**:

| Method       | Description                                      |
|-------------|--------------------------------------------------|
| `push(value)` | Insert a value into the heap (O(log n))         |
| `pop() -> Option<T>` | Remove and return the largest value (O(log n)) |
| `peek() -> Option<&T>` | Look at the largest value without removing  |
| `len() -> usize` | Number of elements                              |
| `is_empty() -> bool` | Check if the heap is empty                   |
| `into_sorted_vec() -> Vec<T>` | Drain the heap into a sorted vector      |

**Python comparison**:

```python
import heapq

heap = []
heapq.heappush(heap, 10)
heapq.heappush(heap, 5)
heapq.heappush(heap, 20)
# In Python, pop gives the smallest:
print(heapq.heappop(heap))  # 5
```

To get max-heap behavior in Python, you push negative values or use `heapq._heapify_max`. Rust gives you max-heap by default, which is more intuitive for "highest priority first" scheduling.

### Min-heap via Reverse

If you want Python-like min-heap behavior in Rust, wrap values in `std::cmp::Reverse`:

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

let mut heap = BinaryHeap::new();
heap.push(Reverse(10));
heap.push(Reverse(5));
heap.push(Reverse(20));

// Now pop gives the smallest first:
assert_eq!(heap.pop(), Some(Reverse(5)));
```

This is cleaner than Python's manual negation trick.

### Real-world use cases

Priority queues power many data-engineering systems:
- **Task scheduling**: run the highest-priority ETL job next
- **Top-K queries**: maintain a heap of K largest elements, O(n log K) instead of O(n log n) sort
- **Merging sorted streams**: pop the smallest head across K sorted files (Dijkstra's algorithm)
- **Event-driven simulation**: process events in timestamp order

In each case, the heap ensures O(log n) insertion and O(1) peek at the next item.

### Applying to Our Project

We'll store `Fruit` values in a `BinaryHeap<Fruit>` and pop them in priority order (Figs first, everything else equal).

## 4. Concept: Ord and PartialOrd for custom ordering

### Explanation

To put custom types in a `BinaryHeap`, they must implement `Ord` (total ordering) and `PartialOrd` (partial ordering -- a superset). The easiest way is to derive `Eq` and `PartialEq`, then implement `Ord` manually.

```rust
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
struct Task {
    priority: u32,
    name: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

**Python comparison**: In Python, you make objects comparable by defining `__lt__`, `__le__`, `__gt__`, `__ge__` or by using `functools.total_ordering`:

```python
from functools import total_ordering

@total_ordering
class Task:
    def __init__(self, priority, name):
        self.priority = priority
        self.name = name
    def __lt__(self, other):
        return self.priority < other.priority
    def __eq__(self, other):
        return self.priority == other.priority
```

Rust forces you to be explicit about ordering -- no runtime `__lt__` magic. The compiler checks that your `Ord` impl is consistent at compile time.

### Compare: deriving Ord vs manual impl

You can derive `Ord` when your type already has a natural ordering:

```rust
#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Priority(u32);  // derived Ord compares the inner u32
```

But when you need **custom** ordering rules (like "Fig always wins"), you must implement `Ord` manually. The compiler cannot guess your business logic.

### Applying to Our Project

Our `Fruit` enum has two variants: `Fig` and `Other(String)`. We want `Fig > Other(...)` always, and all `Other` variants equal to each other. Here's the key insight:

```rust
impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => Ordering::Equal,
        }
    }
}
```

This ensures every Fig outranks every other fruit, while all non-Fig fruits tie. This is a deliberate design choice: in a fruit salad, Figs are the VIPs, and we don't care about ordering among other fruits.

### Why implement PartialOrd too?

Rust requires `PartialOrd` whenever you implement `Ord`. The relationship is:
- `PartialOrd` defines partial comparisons (`<`, `>`, `<=`, `>=` on types where some pairs may be incomparable, like `f64` with `NaN`)
- `Ord` is a subset of `PartialOrd` that guarantees total ordering (every pair is comparable)

For our `Fruit`, comparisons always make sense, so `partial_cmp` just delegates to `cmp`:

```rust
impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

## 5. Putting It All Together

We have two items to implement in `workshop/src/lib.rs`.

### Step 1: Implement `Ord` for `Fruit`

Open `workshop/src/lib.rs`. The `Fruit` enum is already derived with `Eq`, `PartialEq`, and `Debug`. Your job is to fill in `cmp` so that `Fig > Other(...)`.

Recall the `Ordering` enum:
- `Ordering::Less` -- self is less than other
- `Ordering::Equal` -- self equals other
- `Ordering::Greater` -- self is greater than other

Implement `cmp` as shown in Section 4 above.

After implementing, run the tests for step 1:

```bash
cd workshop && cargo test step_01
```

Three tests should pass: `test_fig_greater_than_other`, `test_fig_equal_fig`, `test_other_equal_other`.

### Step 2: Implement `generate_fruit_salad`

This function must:
1. Create a new `BinaryHeap<Fruit>`
2. Push 4-8 random fruits (a mix of `Fruit::Fig` and `Fruit::Other(...)` with various fruit names)
3. Return the heap

Use `rand` to pick from a list of fruit names. The `rand` crate is already in `Cargo.toml`.

```rust
use rand::seq::SliceRandom;

pub fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let mut heap = BinaryHeap::new();
    let fruit_names = vec!["Apple", "Orange", "Banana", "Grape", "Peach"];
    let mut rng = rand::rng();
    // Always push at least one Fig
    heap.push(Fruit::Fig);
    // Push some random other fruits
    for _ in 0..5 {
        let name = fruit_names.choose(&mut rng).unwrap();
        heap.push(Fruit::Other(name.to_string()));
    }
    heap
}
```

After this, all 5 tests should pass:

```bash
cd workshop && cargo test
```

When you pop from the heap, `Fruit::Fig` will always come out first:

```rust
let mut salad = generate_fruit_salad();
assert_eq!(salad.pop(), Some(Fruit::Fig)); // always first!
```

### Iterating a BinaryHeap

You can iterate over the heap without consuming it via `.iter()`:

```rust
for fruit in salad.iter() {
    println!("{:?}", fruit);
}
```

The iteration order is **not** sorted -- it follows the heap's internal array layout. To get sorted output without consuming the heap, use `.into_sorted_vec()`:

```rust
let sorted: Vec<Fruit> = salad.into_sorted_vec();
// sorted[0] is the smallest, sorted[last] is the largest (Fig)
```

Remember that `into_sorted_vec` sorts in **ascending** order (smallest first), which is the reverse of pop order.

## 6. Exercises

### Easy
Modify the `Ord` impl so that `Fruit::Other("Apple".into())` is greater than all other `Other` variants (hint: check the string value in `cmp`).

### Medium
Write a function `salad_into_sorted_vec(heap: BinaryHeap<Fruit>) -> Vec<Fruit>` that converts a heap into a Vec sorted by priority (highest first). Use `into_sorted_vec()`.

### Hard
Create a generic `PriorityQueue<P, T>` struct where `P: Ord` is the priority and `T` is the value. Implement `push`, `pop`, and `peek`. Use this to build a simple task scheduler. Then write a function `top_k(items: &[T], k: usize) -> Vec<&T>` that returns the k largest items using a `BinaryHeap`.

## 7. Summary

| Concept | Description | Rust equivalent of Python |
|---------|------------|--------------------------|
| `BinaryHeap<T>` | Max-heap priority queue | `heapq` (min-heap) |
| `Ord` / `PartialOrd` | Custom ordering trait | `__lt__`, `functools.total_ordering` |
| `push()`, `pop()`, `peek()` | Core heap operations | `heapq.heappush`, `heapq.heappop` |
| Max-heap default | Largest element popped first | Negate values for Python max-heap |

Priority queues are essential for scheduling, streaming top-K, and graph algorithms. Rust's `BinaryHeap` gives you this with zero external dependencies and a clear ownership model.
