# 🦀 TicketManagement — Python to Rust Workshop

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 15 tests pass**.

---

## What Is This Project?

A ticket management system that demonstrates `Vec`, `HashMap`, `BTreeMap`, and iterators — the core data structures for grouping and querying data.

### Python equivalent

```python
from collections import defaultdict

tickets = [{"status": "Open", "title": "Bug"}, {"status": "Closed", "title": "Feature"}]

# Group by status — no type safety on keys
by_status = defaultdict(list)
for t in tickets:
    by_status[t["status"]].append(t)  # typo in key = silent new bucket
```

In this project you'll learn to build this in Rust — and along the way
you'll discover **`Vec<T>`**, **fixed-size arrays**, **`HashMap`**, **`BTreeMap`**, and **iterators**.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | `Vec<T>` | Type-safe, contiguous growable array |
| 2 | `[T; N]` arrays | Stack-allocated, known-length collection |
| 3 | Slices `&[T]` | Borrowed view into a contiguous sequence |
| 4 | Iterators | Lazy, composable functional iteration |
| 5 | Iterator combinators | `.map()`, `.filter()`, `.fold()` — chain transformations |
| 6 | `HashMap<K, V>` | Hash-based key-value storage |
| 7 | `BTreeMap<K, V>` | Sorted key-value pairs |
| 8 | Lifetimes | Compiler tracks how long references are valid |

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Concept: Arrays — Fixed Size](#2-concept-arrays--fixed-size)
3. [Concept: Vec — Dynamic Arrays](#3-concept-vec--dynamic-arrays)
4. [Concept: Slices — Views into Data](#4-concept-slices--views-into-data)
5. [Concept: Iterators — Lazy Processing](#5-concept-iterators--lazy-processing)
6. [Concept: Iterator Combinators — map, filter, fold](#6-concept-iterator-combinators--map-filter-fold)
7. [Concept: HashMap — Key-Value Store](#7-concept-hashmap--key-value-store)
8. [Concept: BTreeMap — Sorted Map](#8-concept-btreemap--sorted-map)
9. [Concept: Lifetimes — Connecting Data](#9-concept-lifetimes--connecting-data)
10. [Putting It All Together](#10-putting-it-all-together)
11. [Summary](#11-summary)
12. [Appendix: Original Step-by-Step Tutorial](#12-appendix-original-step-by-step-tutorial)

---

## 1. Project Overview

We'll build a ticket **management system** that stores, indexes, and queries tickets using:

- `Vec<Ticket>` — store all tickets
- `HashMap<Status, Vec<Ticket>>` — index by status for fast lookup
- Iterators — filter, sort, and transform tickets
- Lifetimes — ensure data references are valid

### Python Comparison

```python
# Python — lists and dicts
tickets = [ticket1, ticket2, ticket3]
by_status = {s: [] for s in Status}
for t in tickets:
    by_status[t.status].append(t)
open_tickets = [t for t in tickets if t.status == "Open"]
```

```rust
// Rust — Vec and HashMap
let mut tickets: Vec<Ticket> = vec![ticket1, ticket2, ticket3];
let mut by_status: HashMap<Status, Vec<Ticket>> = HashMap::new();
for t in tickets {
    by_status.entry(t.status()).or_insert(Vec::new()).push(t);
}
let open: Vec<&Ticket> = tickets.iter().filter(|t| t.is_open()).collect();
```

---

## 2. Concept: Arrays — Fixed Size

### Fixed vs Dynamic

```python
# Python — everything is dynamic
coords = [1, 2, 3]  # Can add or remove elements
```

```rust
// Rust — two options
let coords: [f64; 3] = [1.0, 2.0, 3.0];  // Fixed size = 3
let coords: Vec<f64> = vec![1.0, 2.0, 3.0];  // Dynamic
```

### Array Memory

```
let arr: [u8; 5] = [10, 20, 30, 40, 50];

Stack (5 × 1 byte = 5 bytes):
┌────┬────┬────┬────┬────┐
│ 10 │ 20 │ 30 │ 40 │ 50 │
└────┴────┴────┴────┴────┘
```

### When to Use Arrays

```rust
// Configuration with fixed number of items
const VALID_STATUSES: [&str; 4] = ["Open", "In Progress", "Resolved", "Closed"];

// Small, fixed-size data
struct Point([f64; 3]);  // 3D coordinates

// RGB color
struct Color([u8; 3]);  // [R, G, B]
```

---

## 3. Concept: Vec — Dynamic Arrays

> **Recap**: `Vec<T>` is taught in depth in [02-VectorFruitSalad](../../02-VectorFruitSalad/README.md) (creation, push/pop, indexing, iteration, Python-list parallels) and [13-MutableFruitSalad](../13-MutableFruitSalad/README.md) covers the mutation patterns (insert, remove, sort, dedup, capacity). Read those first if you have not.

The only Vec-specific point that matters for *this* project: `Vec<T>` is the right type when you need an *owned, growable sequence* — which is exactly what we need as the **value type** in our `HashMap<Status, Vec<Ticket>>` index below.

```rust
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Status { ToDo, InProgress, Done }

struct TicketStore {
    by_status: HashMap<Status, Vec<Ticket>>,
}

impl TicketStore {
    fn add(&mut self, ticket: Ticket) {
        // entry().or_insert_with(Vec::new) gives us a Vec to push into
        self.by_status
            .entry(ticket.status.clone())
            .or_insert_with(Vec::new)
            .push(ticket);
    }
}
```

> **Python comparison**: A `dict[Status, list[Ticket]]` in Python would work the same way. The Rust version catches key errors at compile time (the `Status` enum is exhaustive) and gives O(1) `entry` access.

See [§7 — HashMap](#7-concept-hashmap--key-value-store) below for the full `HashMap` / `Vec` interplay in this project.

---

## 4. Concept: Slices — Views into Data

> **Recap**: Slices `&[T]` and array-vs-slice trade-offs were taught in [01-Intro §9 — Arrays and Slices](../../../../01-Foundations/01-Intro/README.md#9-arrays-and-slices--fixed-and-dynamic-sequences) and in [14-Reference/collections-guide.md](../../14-Reference/collections-guide.md#arrays-tn-vs-slices-t). Read those first if you have not.

The only slice-specific point that matters for *this* project: the function `get_tickets_by_status(&self, status: Status) -> &[Ticket]` returns a `&[Ticket]` slice borrowed from the `HashMap`'s `Vec` — no allocation, no copy, and the caller can iterate, sort, or filter without us having to clone:

```rust
impl TicketStore {
    fn get_tickets_by_status(&self, status: Status) -> &[Ticket] {
        self.by_status.get(&status).map(Vec::as_slice).unwrap_or(&[])
    }
}
```

The `&[]` literal is an empty slice — a zero-length borrow that always works, used as the "no such key" return value.

---

## 4. Concept: Slices — Views into Data

### What Is a Slice?

A slice (`&[T]`) is a **view** into a contiguous sequence of elements — it borrows data without owning it.

```rust
let v = vec![10, 20, 30, 40, 50];

let slice: &[i32] = &v[1..4];   // [20, 30, 40]
let slice: &[i32] = &v[..3];    // [10, 20, 30]
let slice: &[i32] = &v[2..];    // [30, 40, 50]
let slice: &[i32] = &v[..];     // All elements
```

### Slice Memory

```
let v = vec![10, 20, 30, 40, 50];
let s = &v[1..4];

Stack:
┌────────────────────┐
│ v:                  │
│   ptr: ──────┐     │
│   len: 5     │     │
│   cap: 5     │     │
├────────────────────┤
│ s (&[i32]):  │     │
│   ptr: ──────┤     │
│   len: 3     │     │
└────────────────────┘
         │           │
         ▼           ▼
Heap:  [10, 20, 30, 40, 50]
         ▲─── s points here
```

### Slices as Function Parameters

```rust
// GOOD: accepts both &Vec<T> and &[T]
fn sum(values: &[i32]) -> i32 {
    let mut total = 0;
    for v in values {
        total += v;
    }
    total
}

let v = vec![1, 2, 3, 4, 5];
let arr = [1, 2, 3, 4, 5];

sum(&v);    // ✅ Vec auto-derefs to slice
sum(&arr);  // ✅ Array auto-derefs to slice
sum(&v[1..3]);  // ✅ Sub-slice
```

### Why Slices Matter for Data Engineers

```rust
fn process_batch(data: &[DataRow]) -> Vec<Result> {
    // Accepts any contiguous data: Vec, array, or sub-slice
    data.iter().map(process_row).collect()
}

// Can work with:
process_batch(&all_data);        // Entire dataset
process_batch(&all_data[..100]);  // First 100 rows (no copy!)
```

---

## 5. Concept: Iterators — Lazy Processing

### Python vs Rust Iterators

```python
# Python — iterators everywhere
numbers = [1, 2, 3, 4, 5]
doubled = (x * 2 for x in numbers)     # Lazy generator
result = [x for x in doubled if x > 5] # Lazy comprehension
```

```rust
// Rust — iterators everywhere
let numbers = vec![1, 2, 3, 4, 5];
let doubled = numbers.iter().map(|x| x * 2);      // Lazy
let result: Vec<_> = doubled.filter(|x| x > 5).collect(); // Eager
```

### Three Iteration Modes

```rust
let v = vec![1, 2, 3];

// 1. Immutable borrow (most common)
for x in &v {           // for x in v.iter()
    println!("{}", x);  // x: &i32
}
// v is still usable

// 2. Mutable borrow
for x in &mut v {       // for x in v.iter_mut()
    *x *= 2;            // x: &mut i32
}

// 3. Consume (move)
for x in v {            // for x in v.into_iter()
    println!("{}", x);  // x: i32 (owned)
}
// v is no longer usable
```

### Implementing Your Own Iterator

```rust
struct TicketIter {
    tickets: Vec<Ticket>,
    index: usize,
}

impl Iterator for TicketIter {
    type Item = Ticket;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tickets.len() {
            let ticket = self.tickets[self.index].clone();
            self.index += 1;
            Some(ticket)
        } else {
            None
        }
    }
}
```

But usually you just use `.iter()` — the standard library does this for you.

---

## 6. Concept: Iterator Combinators — map, filter, fold

### The Combinator Toolbox

```rust
let tickets = vec![t1, t2, t3, t4, t5];

// map — transform each element
let titles: Vec<&str> = tickets.iter()
    .map(|t| t.title())
    .collect();

// filter — keep elements matching a condition
let open: Vec<&Ticket> = tickets.iter()
    .filter(|t| t.is_open())
    .collect();

// chain multiple operations
let result: Vec<String> = tickets.iter()
    .filter(|t| !t.is_closed())
    .map(|t| format!("{}: {}", t.status(), t.title()))
    .collect();

// fold — reduce to a single value (like Python's functools.reduce)
let total_chars: usize = tickets.iter()
    .map(|t| t.title().len())
    .fold(0, |acc, len| acc + len);

// any / all — check conditions
let has_open = tickets.iter().any(|t| t.is_open());
let all_done = tickets.iter().all(|t| t.is_closed());

// find — first matching element
let first_urgent = tickets.iter().find(|t| t.is_urgent());

// count — number of matching elements
let open_count = tickets.iter().filter(|t| t.is_open()).count();
```

### Python vs Rust Combinators

| Operation | Python | Rust |
|---|---|---|
| Transform | `map(f, iter)` or `(f(x) for x in iter)` | `.map(\|x\| f(x))` |
| Filter | `(x for x in iter if cond)` | `.filter(\|x\| cond)` |
| Reduce | `functools.reduce(f, iter)` | `.fold(init, \|acc, x\| f(acc, x))` |
| Check any | `any(cond for x in iter)` | `.any(\|x\| cond)` |
| Check all | `all(cond for x in iter)` | `.all(\|x\| cond)` |
| Take first N | `itertools.islice(iter, n)` | `.take(n)` |
| Skip first N | `itertools.islice(iter, n, None)` | `.skip(n)` |
| Unique | `set(iter)` | `.unique()` (itertools) |
| Sort | `sorted(iter)` | `.sorted()` (collect + sort) |
| Group by | `itertools.groupby(iter)` | (manual or itertools) |
| Enumerate | `enumerate(iter)` | `.enumerate()` |
| Zip | `zip(iter1, iter2)` | `.zip(other_iter)` |

### Lazy Evaluation Chain

```
tickets.iter()          // Nothing computed yet
    .filter(|t| ...)    // Still lazy
    .map(|t| ...)       // Still lazy
    .take(10)           // Still lazy
    .collect()          // NOW it runs!

Visual:
    Source ──→ filter ──→ map ──→ take(10) ──→ collect
    (Vec)      │         │        │            │
               │         │        │            ▼
               │         │        │       Output Vec
               │         │        │
               v         v        v
          ✓ only passes✓ transforms✓ stops after 10
```

### Data Engineering Example

```rust
#[derive(Debug)]
struct DataRow {
    id: u32,
    value: f64,
    label: String,
}

fn analyze(rows: &[DataRow]) {
    // Compute statistics using iterators
    let valid: Vec<_> = rows.iter()
        .filter(|r| r.value.is_finite())
        .collect();

    let avg = valid.iter()
        .map(|r| r.value)
        .sum::<f64>() / valid.len() as f64;

    let max_val = valid.iter()
        .map(|r| r.value)
        .fold(f64::NEG_INFINITY, |a, b| a.max(b));

    let labels: Vec<&str> = rows.iter()
        .filter(|r| r.value > avg)
        .map(|r| r.label.as_str())
        .collect();

    println!("Avg: {avg:.2}, Max: {max_val:.2}");
    println!("Above avg labels: {:?}", labels);
}
```

---

## 6.5 Concept: HashMap Primer — Python `dict` vs Rust `HashMap`

> **Why this primer?** Sections 1–2 (Foundations, Ownership) never introduce `HashMap`. Python developers know `dict` deeply, but Rust's `HashMap` is its own type with subtle differences. This 30-second preview gives you enough to understand §7. The full teaching (`.entry().or_insert()`, `BTreeMap`, trade-offs) follows in §7–8.

### Python's `dict` — no type safety

```python
counts = {}                          # empty dict — any keys, any values
counts["apple"] = 1                  # insert
counts["banana"] = "two"             # mixed types silently allowed!
val = counts.get("x", 0)             # 0 (default)
val = counts["x"]                    # KeyError at runtime
```

### Rust's `HashMap` — typed and explicit

```rust
use std::collections::HashMap;

let mut counts: HashMap<String, u32> = HashMap::new();  // K, V both fixed
counts.insert("apple".to_string(), 1);                  // explicit insert
let val: Option<&u32> = counts.get("apple");            // Option, never a crash
// let val = counts["apple"];                          // panics if missing
```

| Python | Rust |
|--------|------|
| `{}` (untyped) | `HashMap::new()` with explicit `K, V` |
| `counts["x"]` (KeyError) | `counts["x"]` (panics) or `counts.get("x")` (Option) |
| `counts.get(x, 0)` | `counts.get(x).copied().unwrap_or(0)` |
| `counts[x] = counts.get(x, 0) + 1` | `*counts.entry(x).or_insert(0) += 1` (see §7) |

**Three things to remember:**
1. The type signature `HashMap<K, V>` fixes both key and value types at compile time
2. `.get()` returns `Option<&V>` — no panics, no crashes
3. Use `.entry()` for the "insert or modify" pattern (full syntax in §7)

See §7 for the full `HashMap` teaching in this project.

---

## 7. Concept: HashMap — Key-Value Store

### Creating and Using HashMap

```rust
use std::collections::HashMap;

let mut scores: HashMap<String, u32> = HashMap::new();

scores.insert(String::from("Alice"), 100);
scores.insert(String::from("Bob"), 90);

// Access
let alice = scores.get("Alice");  // Option<&u32>
let bob = scores["Bob"];          // Panics if missing!

// Remove
scores.remove("Alice");

// Length
println!("{} entries", scores.len());
```

### HashMap Memory

```
let map: HashMap<&str, u32>
┌────────────────────────────┐
│ Bucket 0: empty             │
│ Bucket 1: ("Bob" → 90)     │
│ Bucket 2: empty             │
│ Bucket 3: ("Alice" → 100)  │
│ Bucket 4: empty             │
│ ...                         │
└────────────────────────────┘

Hash function maps keys to buckets for O(1) lookup
```

### Entry API — Idiomatic Rust

```python
# Python
counts = {}
for word in words:
    if word not in counts:
        counts[word] = 0
    counts[word] += 1
```

```rust
// Rust — entry API
let mut counts: HashMap<&str, u32> = HashMap::new();
for word in &words {
    *counts.entry(word).or_insert(0) += 1;
}
```

### Python Dict vs Rust HashMap

| Operation | Python | Rust |
|---|---|---|
| Create | `d = {}` | `let d: HashMap<K,V> = HashMap::new();` |
| Insert | `d[k] = v` | `d.insert(k, v);` |
| Get | `d.get(k)` | `d.get(&k)` → `Option<&V>` |
| Get with default | `d.get(k, default)` | `d.get(&k).unwrap_or(&default)` |
| Check exists | `k in d` | `d.contains_key(&k)` |
| Delete | `del d[k]` | `d.remove(&k);` |
| Length | `len(d)` | `d.len()` |
| Iterate | `for k, v in d.items()` | `for (k, v) in &d` |
| Insert if missing | `d.setdefault(k, v)` | `d.entry(k).or_insert(v)` |

### Data Engineering Example: Index by Status

```rust
use std::collections::HashMap;

fn index_by_status(tickets: &[Ticket]) -> HashMap<&str, Vec<&Ticket>> {
    let mut map: HashMap<&str, Vec<&Ticket>> = HashMap::new();

    for ticket in tickets {
        map.entry(ticket.status())
            .or_insert(Vec::new())
            .push(ticket);
    }

    map
}

fn main() {
    let tickets = vec![/* ... */];
    let by_status = index_by_status(&tickets);

    // Fast lookup by status
    let open = by_status.get("Open").unwrap_or(&vec![]);
    println!("Open tickets: {}", open.len());
}
```

---

## 8. Concept: BTreeMap — Sorted Map

### HashMap vs BTreeMap

```rust
use std::collections::{HashMap, BTreeMap};

// HashMap: fastest O(1), no ordering
let mut map: HashMap<u32, &str> = HashMap::new();

// BTreeMap: O(log n), sorted by key
let mut map: BTreeMap<u32, &str> = BTreeMap::new();
```

| | HashMap | BTreeMap |
|---|---|---|
| Ordering | No guaranteed order | Sorted by key |
| Lookup | O(1) average | O(log n) |
| Insert | O(1) average | O(log n) |
| Range queries | Not supported | `range(start..=end)` |
| Memory | Higher overhead | Lower overhead |
| Key requirement | `Hash + Eq` | `Ord` |

### Range Queries with BTreeMap

```rust
let mut tickets = BTreeMap::new();
tickets.insert(1001, "Bug fix");
tickets.insert(1005, "Feature request");
tickets.insert(1010, "Documentation");

// Get all tickets with ID between 1000 and 1006
for (id, title) in tickets.range(1000..=1006) {
    println!("Ticket #{id}: {title}");
}
// Output:
// Ticket #1001: Bug fix
// Ticket #1005: Feature request
```

### Python Counterpart

```python
# Python — no sorted dict in stdlib until 3.7 (insertion-order only)
from sortedcontainers import SortedDict  # Third-party
from collections import OrderedDict      # Insertion order only
```

```rust
// Rust — BTreeMap in stdlib!
use std::collections::BTreeMap;
```

---

## 9. Concept: Lifetimes — Connecting Data

### What Is a Lifetime?

A lifetime is the period during which a reference is valid. Rust uses lifetimes to ensure references never outlive the data they point to.

```rust
fn first_word(s: &str) -> &str {
    // The returned &str borrows from s
    // Rust needs to know: how long does the return live?
    // Answer: as long as the input &str lives
    s.split_whitespace().next().unwrap_or("")
}
```

### Lifetime Annotations

```rust
// 'a is a lifetime parameter — "at least as long as 'a"
fn first_word<'a>(s: &'a str) -> &'a str {
    // Return lives as long as input
    s.split_whitespace().next().unwrap_or("")
}
```

### Why Lifetimes Matter

```python
# Python — dangling reference at runtime
def get_name():
    user = User("Alice")
    return user.name  # Returns reference to attribute
# user is GC'd... but Python keeps reference alive anyway
```

```rust
// Rust — caught at compile time
fn get_name(user: &User) -> &str {
    &user.name  // Lifetime tied to the input
}

fn main() {
    let name;
    {
        let user = User::new("Alice");
        name = get_name(&user);  // name borrows from user
    }  // user is dropped here
    // println!("{}", name);  // ❌ user was dropped! name is dangling
}
```

### Lifetime Elision (You Don't Always Need to Write Them)

```rust
// These are equivalent:
fn first_word(s: &str) -> &str { ... }
fn first_word<'a>(&'a str) -> &'a str { ... }  // Implicit 'a

struct Ticket {
    title: String,
}

impl Ticket {
    // &self has lifetime, return borrows from it
    fn title(&self) -> &str {
        &self.title
    }
}
```

### Structs with References Need Lifetime Annotations

```rust
struct TicketRef<'a> {
    title: &'a str,   // Must live at least as long as the struct
    status: &'a str,
}
```

### Lifetime Visual

```
fn main() {
    let title = String::from("Bug");  // ┐ title's lifetime
    let ticket = TicketRef {           // │
        title: &title,                 // │
        status: &"Open",               // │
    };                                 // │
    println!("{}", ticket.title);      // │
}                                      // ┘ both drop here together
```

---

## 10. Putting It All Together

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Ticket {
    id: u32,
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(id: u32, title: String, description: String, status: String) -> Ticket {
        Ticket { id, title, description, status }
    }

    pub fn status(&self) -> &str { &self.status }
    pub fn title(&self) -> &str { &self.title }
    pub fn id(&self) -> u32 { self.id }
    pub fn is_open(&self) -> bool { self.status == "Open" }
    pub fn is_closed(&self) -> bool { self.status == "Closed" }
}

/// Index tickets by status using HashMap
fn index_by_status(tickets: &[Ticket]) -> HashMap<&str, Vec<&Ticket>> {
    let mut map: HashMap<&str, Vec<&Ticket>> = HashMap::new();
    for t in tickets {
        map.entry(t.status()).or_insert(Vec::new()).push(t);
    }
    map
}

/// Find the most common status
fn most_common_status(tickets: &[Ticket]) -> Option<(&str, usize)> {
    let by_status = index_by_status(tickets);
    by_status.into_iter()
        .max_by_key(|(_, group)| group.len())
        .map(|(status, group)| (status, group.len()))
}

fn main() {
    let tickets = vec![
        Ticket::new(1, "Login bug".into(), "Cannot log in".into(), "Open".into()),
        Ticket::new(2, "Slow query".into(), "Query takes 10s".into(), "Open".into()),
        Ticket::new(3, "Add export".into(), "CSV export".into(), "In Progress".into()),
        Ticket::new(4, "Fix typo".into(), "Typo in docs".into(), "Closed".into()),
        Ticket::new(5, "Security patch".into(), "Update deps".into(), "Open".into()),
    ];

    // --- Iterators ---
    println!("=== Open Tickets ===");
    let open: Vec<&Ticket> = tickets.iter()
        .filter(|t| t.is_open())
        .collect();
    for t in &open {
        println!("  #{}: {}", t.id(), t.title());
    }

    // --- Iterator chains ---
    println!("\n=== Summary ===");
    tickets.iter()
        .map(|t| format!("#{} [{}] {}", t.id(), t.status(), t.title()))
        .for_each(|line| println!("  {line}"));

    // --- HashMap index ---
    println!("\n=== Tickets by Status ===");
    let by_status = index_by_status(&tickets);
    for (status, group) in &by_status {
        println!("  {status} ({}):", group.len());
        for t in group {
            println!("    #{}: {}", t.id(), t.title());
        }
    }

    // --- Stats ---
    println!("\n=== Stats ===");
    println!("Total: {}", tickets.len());
    println!("Open: {}", tickets.iter().filter(|t| t.is_open()).count());
    println!("Closed: {}", tickets.iter().filter(|t| t.is_closed()).count());

    if let Some((status, count)) = most_common_status(&tickets) {
        println!("Most common status: {status} ({count})");
    }
}
```

---

## 11. Summary

| Concept | Description | Python Equivalent |
|---|---|---|
| `[T; N]` | Fixed-size array | `tuple` (sort of) |
| `Vec<T>` | Dynamic array | `list` |
| `&[T]` | Slice (view) | `list[:]` (slicing) |
| `.iter()` | Create iterator | `iter()` |
| `.map()` | Transform each element | `map()` / comprehension |
| `.filter()` | Keep matching elements | `filter()` / comprehension |
| `.fold()` | Reduce to single value | `functools.reduce()` |
| `.collect()` | Eagerly produce collection | `list()` / `set()` |
| `HashMap<K,V>` | Hash table | `dict` |
| `BTreeMap<K,V>` | Sorted map | `SortedDict` (3rd party) |
| Lifetimes `'a` | Valid duration of a reference | N/A (GC handles) |
| `.entry().or_insert()` | Insert if missing | `dict.setdefault()` |

### Key Takeaways for Data Engineers

1. **`Vec<T>`** is your go-to collection — like Python's `list`
2. **Iterator chains** (`filter` → `map` → `collect`) are the Rust equivalent of pandas/SQL pipelines
3. **`HashMap`** is your `dict` — use `.entry().or_insert()` for clean "upsert" logic
4. **Slices `&[T]`** let you pass sub-sections of data without copying
5. **Lifetimes** ensure your references are always valid — no dangling data

### Further Reading

The supplementary lesson files have been merged into the [Appendix](#12-appendix-original-step-by-step-tutorial) below.

### Next Project

Proceed to [7-Threads](../../05-Concurrency/01-Threads/README.md) for **concurrency** — running data pipelines in parallel.

---

## 12. Appendix: Original Step-by-Step Tutorial

### 12.1 Intro

In the previous chapter we modelled `Ticket` in a vacuum: we defined its fields and their constraints, we learned
how to best represent them in Rust, but we didn't consider how `Ticket` fits into a larger system.
We'll use this chapter to build a simple workflow around `Ticket`, introducing a (rudimentary) management system to
store and retrieve tickets.

The task will give us an opportunity to explore new Rust concepts, such as:

- Stack-allocated arrays
- `Vec`, a growable array type
- `Iterator` and `IntoIterator`, for iterating over collections
- Slices (`&[T]`), to work with parts of a collection
- Lifetimes, to describe how long references are valid
- `HashMap` and `BTreeMap`, two key-value data structures
- `Eq` and `Hash`, to compare keys in a `HashMap`
- `Ord` and `PartialOrd`, to work with a `BTreeMap`
- `Index` and `IndexMut`, to access elements in a collection

### 12.2 Arrays (01_arrays.md)

As soon as we start talking about "ticket management" we need to think about a way to store _multiple_ tickets.
In turn, this means we need to think about collections. In particular, homogeneous collections:
we want to store multiple instances of the same type.

What does Rust have to offer in this regard?

#### Arrays

A first attempt could be to use an **array**.\\
Arrays in Rust are fixed-size collections of elements of the same type.

Here's how you can define an array:

```rust
// Array type syntax: [ <type> ; <number of elements> ]
let numbers: [u32; 3] = [1, 2, 3];
```

This creates an array of 3 integers, initialized with the values `1`, `2`, and `3`.\\
The type of the array is `[u32; 3]`, which reads as "an array of `u32`s with a length of 3".

If all array elements are the same, you can use a shorter syntax to initialize it:

```rust
// [ <value> ; <number of elements> ]
let numbers: [u32; 3] = [1; 3];
```

`[1; 3]` creates an array of three elements, all equal to `1`.

#### Accessing elements

You can access elements of an array using square brackets:

```rust
let first = numbers[0];
let second = numbers[1];
let third = numbers[2];
```

The index must be of type `usize`.\\
Arrays are **zero-indexed**, like everything in Rust. You've seen this before with string slices and field indexing in
tuples/tuple-like variants.

#### Out-of-bounds access

If you try to access an element that's out of bounds, Rust will panic:

```rust
let numbers: [u32; 3] = [1, 2, 3];
let fourth = numbers[3]; // This will panic
```

This is enforced at runtime using **bounds checking**. It comes with a small performance overhead, but it's how
Rust prevents buffer overflows.\\
In some scenarios the Rust compiler can optimize away bounds checks, especially if iterators are involved—we'll speak
more about this later on.

If you don't want to panic, you can use the `get` method, which returns an `Option<&T>`:

```rust
let numbers: [u32; 3] = [1, 2, 3];
assert_eq!(numbers.get(0), Some(&1));
// You get a `None` if you try to access an out-of-bounds index
// rather than a panic.
assert_eq!(numbers.get(3), None);
```

#### Performance

Since the size of an array is known at compile-time, the compiler can allocate the array on the stack.
If you run the following code:

```rust
let numbers: [u32; 3] = [1, 2, 3];
```

You'll get the following memory layout:

```text
        +---+---+---+
Stack:  | 1 | 2 | 3 |
        +---+---+---+
```

In other words, the size of an array is `std::mem::size_of::<T>() * N`, where `T` is the type of the elements and `N` is
the number of elements.\\
You can access and replace each element in `O(1)` time.

### 12.3 Vec (02_vec.md)

Arrays' strength is also their weakness: their size must be known upfront, at compile-time.
If you try to create an array with a size that's only known at runtime, you'll get a compilation error:

```rust
let n = 10;
let numbers: [u32; n];
```

```text
error[E0435]: attempt to use a non-constant value in a constant
 --> src/main.rs:3:20
  |
2 | let n = 10;
3 | let numbers: [u32; n];
  |                    ^ non-constant value
```

Arrays wouldn't work for our ticket management system—we don't know how many tickets we'll need to store at compile-time.
This is where `Vec` comes in.

#### `Vec`

`Vec` is a growable array type, provided by the standard library.\\
You can create an empty array using the `Vec::new` function:

```rust
let mut numbers: Vec<u32> = Vec::new();
```

You would then push elements into the vector using the `push` method:

```rust
numbers.push(1);
numbers.push(2);
numbers.push(3);
```

New values are added to the end of the vector.\\
You can also create an initialized vector using the `vec!` macro, if you know the values at creation time:

```rust
let numbers = vec![1, 2, 3];
```

#### Accessing elements

The syntax for accessing elements is the same as with arrays:

```rust
let numbers = vec![1, 2, 3];
let first = numbers[0];
let second = numbers[1];
let third = numbers[2];
```

The index must be of type `usize`.\\
You can also use the `get` method, which returns an `Option<&T>`:

```rust
let numbers = vec![1, 2, 3];
assert_eq!(numbers.get(0), Some(&1));
// You get a `None` if you try to access an out-of-bounds index
// rather than a panic.
assert_eq!(numbers.get(3), None);
```

Access is bounds-checked, just like element access with arrays. It has O(1) complexity.

#### Memory layout

`Vec` is a heap-allocated data structure.\\
When you create a `Vec`, it allocates memory on the heap to store the elements.

If you run the following code:

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
```

you'll get the following memory layout:

```text
      +---------+--------+----------+
Stack | pointer | length | capacity |
      |  |      |   2    |    3     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+
Heap:  | 1 | 2 | ? |
       +---+---+---+
```

`Vec` keeps track of three things:

- The **pointer** to the heap region you reserved.
- The **length** of the vector, i.e. how many elements are in the vector.
- The **capacity** of the vector, i.e. the number of elements that can fit in the space reserved on the heap.

This layout should look familiar: it's exactly the same as `String`!\\
That's not a coincidence: `String` is defined as a vector of bytes, `Vec<u8>`, under the hood:

```rust
pub struct String {
    vec: Vec<u8>,
}
```

### 12.4 Resizing (03_resizing.md)

We said that `Vec` is a "growable" vector type, but what does that mean?
What happens if you try to insert an element into a `Vec` that's already at maximum capacity?

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
numbers.push(3); // Max capacity reached
numbers.push(4); // What happens here?
```

The `Vec` will **resize** itself.\\
It will ask the allocator for a new (larger) chunk of heap memory, copy the elements over, and deallocate the old memory.

This operation can be expensive, as it involves a new memory allocation and copying all existing elements.

#### `Vec::with_capacity`

If you have a rough idea of how many elements you'll store in a `Vec`, you can use the `Vec::with_capacity`
method to pre-allocate enough memory upfront.\\
This can avoid a new allocation when the `Vec` grows, but it may waste memory if you overestimate actual usage.

Evaluate on a case-by-case basis.

### 12.5 Iteration — Iterator and IntoIterator (04_iterators.md)

During the very first exercises, you learned that Rust lets you iterate over collections using `for` loops.
We were looking at ranges at that point (e.g. `0..5`), but the same holds true for collections like arrays and vectors.

```rust
// It works for `Vec`s
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}

// It also works for arrays
let a: [u32; 3] = [1, 2, 3];
for n in a {
    println!("{}", n);
}
```

It's time to understand how this works under the hood.

#### `for` desugaring

Every time you write a `for` loop in Rust, the compiler _desugars_ it into the following code:

```rust
let mut iter = IntoIterator::into_iter(v);
loop {
    match iter.next() {
        Some(n) => {
            println!("{}", n);
        }
        None => break,
    }
}
```

`loop` is another looping construct, on top of `for` and `while`.\\
A `loop` block will run forever, unless you explicitly `break` out of it.

#### `Iterator` trait

The `next` method in the previous code snippet comes from the `Iterator` trait.
The `Iterator` trait is defined in Rust's standard library and provides a shared interface for
types that can produce a sequence of values:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

The `Item` associated type specifies the type of the values produced by the iterator.

`next` returns the next value in the sequence.\\
It returns `Some(value)` if there's a value to return, and `None` when there isn't.

Be careful: there is no guarantee that an iterator is exhausted when it returns `None`. That's only
guaranteed if the iterator implements the (more restrictive)
[`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) trait.

#### `IntoIterator` trait

Not all types implement `Iterator`, but many can be converted into a type that does.\\
That's where the `IntoIterator` trait comes in:

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

The `into_iter` method consumes the original value and returns an iterator over its elements.\\
A type can only have one implementation of `IntoIterator`: there can be no ambiguity as to what `for` should desugar to.

One detail: every type that implements `Iterator` automatically implements `IntoIterator` as well.
They just return themselves from `into_iter`!

#### Bounds checks

Iterating over iterators has a nice side effect: you can't go out of bounds, by design.\\
This allows Rust to remove bounds checks from the generated machine code, making iteration faster.

In other words,

```rust
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}
```

is usually faster than

```rust
let v = vec![1, 2, 3];
for i in 0..v.len() {
    println!("{}", v[i]);
}
```

There are exceptions to this rule: the compiler can sometimes prove that you're not going out of bounds even
with manual indexing, thus removing the bounds checks anyway. But in general, prefer iteration to indexing
where possible.

### 12.6 `.iter()` — Iteration Modes (05_iter.md)

`IntoIterator` **consumes** `self` to create an iterator.

This has its benefits: you get **owned** values from the iterator.
For example: if you call `.into_iter()` on a `Vec<Ticket>` you'll get an iterator that returns `Ticket` values.

That's also its downside: you can no longer use the original collection after calling `.into_iter()` on it.
Quite often you want to iterate over a collection without consuming it, looking at **references** to the values instead.
In the case of `Vec<Ticket>`, you'd want to iterate over `&Ticket` values.

Most collections expose a method called `.iter()` that returns an iterator over references to the collection's elements.
For example:

```rust
let numbers: Vec<u32> = vec![1, 2];
// `n` has type `&u32` here
for n in numbers.iter() {
    // [...]
}
```

This pattern can be simplified by implementing `IntoIterator` for a **reference to the collection**.
In our example above, that would be `&Vec<Ticket>`.\\
The standard library does this, that's why the following code works:

```rust
let numbers: Vec<u32> = vec![1, 2];
// `n` has type `&u32` here
// We didn't have to call `.iter()` explicitly
// It was enough to use `&numbers` in the `for` loop
for n in &numbers {
    // [...]
}
```

It's idiomatic to provide both options:

- An implementation of `IntoIterator` for a reference to the collection.
- An `.iter()` method that returns an iterator over references to the collection's elements.

The former is convenient in `for` loops, the latter is more explicit and can be used in other contexts.

### 12.7 Lifetimes (06_lifetimes.md)

Let's try to complete the previous exercise by adding an implementation of `IntoIterator` for `&TicketStore`, for
maximum convenience in `for` loops.

Let's start by filling in the most "obvious" parts of the implementation:

```rust
impl IntoIterator for &TicketStore {
    type Item = &Ticket;
    type IntoIter = // What goes here?

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}
```

What should `type IntoIter` be set to?\\
Intuitively, it should be the type returned by `self.tickets.iter()`, i.e. the type returned by `Vec::iter()`.\\
If you check the standard library documentation, you'll find that `Vec::iter()` returns an `std::slice::Iter`.
The definition of `Iter` is:

```rust
pub struct Iter<'a, T> { /* fields omitted */ }
```

`'a` is a **lifetime parameter**.

#### Lifetime parameters

Lifetimes are **labels** used by the Rust compiler to keep track of how long a reference (either mutable or
immutable) is valid.\\
The lifetime of a reference is constrained by the scope of the value it refers to. Rust always makes sure, at compile-time,
that references are not used after the value they refer to has been dropped, to avoid dangling pointers and use-after-free bugs.

This should sound familiar: we've already seen these concepts in action when we discussed ownership and borrowing.
Lifetimes are just a way to **name** how long a specific reference is valid.

Naming becomes important when you have multiple references and you need to clarify how they **relate to each other**.
Let's look at the signature of `Vec::iter()`:

```rust
impl <T> Vec<T> {
    // Slightly simplified
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // [...]
    }
}
```

`Vec::iter()` is generic over a lifetime parameter, named `'a`.\\
`'a` is used to **tie together** the lifetime of the `Vec` and the lifetime of the `Iter` returned by `iter()`.
In plain English: the `Iter` returned by `iter()` cannot outlive the `Vec` reference (`&self`) it was created from.

This is important because `Vec::iter`, as we discussed, returns an iterator over **references** to the `Vec`'s elements.
If the `Vec` is dropped, the references returned by the iterator would be invalid. Rust must make sure this doesn't happen,
and lifetimes are the tool it uses to enforce this rule.

#### Lifetime elision

Rust has a set of rules, called **lifetime elision rules**, that allow you to omit explicit lifetime annotations in many cases.
For example, `Vec::iter`'s definition looks like this in `std`'s source code:

```rust
impl <T> Vec<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        // [...]
    }
}
```

No explicit lifetime parameter is present in the signature of `Vec::iter()`.
Elision rules imply that the lifetime of the `Iter` returned by `iter()` is tied to the lifetime of the `&self` reference.
You can think of `'_` as a **placeholder** for the lifetime of the `&self` reference.

See the [References](#references) section for a link to the official documentation on lifetime elision.\\
In most cases, you can rely on the compiler telling you when you need to add explicit lifetime annotations.

#### References

- [std::vec::Vec::iter](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter)
- [std::slice::Iter](https://doc.rust-lang.org/std/slice/struct.Iter.html)
- [Lifetime elision rules](https://doc.rust-lang.org/reference/lifetime-elision.html)

### 12.8 Iterator Combinators (07_combinators.md)

Iterators can do so much more than `for` loops!\\
If you look at the documentation for the `Iterator` trait, you'll find a **vast** collection of
methods that you can leverage to transform, filter, and combine iterators in various ways.

Let's mention the most common ones:

- `map` applies a function to each element of the iterator.
- `filter` keeps only the elements that satisfy a predicate.
- `filter_map` combines `filter` and `map` in one step.
- `cloned` converts an iterator of references into an iterator of values, cloning each element.
- `enumerate` returns a new iterator that yields `(index, value)` pairs.
- `skip` skips the first `n` elements of the iterator.
- `take` stops the iterator after `n` elements.
- `chain` combines two iterators into one.

These methods are called **combinators**.\\
They are usually **chained** together to create complex transformations in a concise and readable way:

```rust
let numbers = vec![1, 2, 3, 4, 5];
// The sum of the squares of the even numbers
let outcome: u32 = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .sum();
```

#### Closures

What's going on with the `filter` and `map` methods above?\\
They take **closures** as arguments.

Closures are **anonymous functions**, i.e. functions that are not defined using the `fn` syntax we are used to.\\
They are defined using the `|args| body` syntax, where `args` are the arguments and `body` is the function body.
`body` can be a block of code or a single expression.
For example:

```rust
// An anonymous function that adds 1 to its argument
let add_one = |x| x + 1;
// Could be written with a block too:
let add_one = |x| { x + 1 };
```

Closures can take more than one argument:

```rust
let add = |x, y| x + y;
let sum = add(1, 2);
```

They can also capture variables from their environment:

```rust
let x = 42;
let add_x = |y| x + y;
let sum = add_x(1);
```

If necessary, you can specify the types of the arguments and/or the return type:

```rust
// Just the input type
let add_one = |x: i32| x + 1;
// Or both input and output types, using the `fn` syntax
let add_one: fn(i32) -> i32 = |x| x + 1;
```

#### `collect`

What happens when you're done transforming an iterator using combinators?\\
You either iterate over the transformed values using a `for` loop, or you collect them into a collection.

The latter is done using the `collect` method.\\
`collect` consumes the iterator and collects its elements into a collection of your choice.

For example, you can collect the squares of the even numbers into a `Vec`:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let squares_of_evens: Vec<u32> = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .collect();
```

`collect` is generic over its **return type**.\\
Therefore you usually need to provide a type hint to help the compiler infer the correct type.
In the example above, we annotated the type of `squares_of_evens` to be `Vec<u32>`.
Alternatively, you can use the **turbofish syntax** to specify the type:

```rust
let squares_of_evens = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    // Turbofish syntax: `<method_name>::<type>()`
    // It's called turbofish because `::<>` looks like a fish
    .collect::<Vec<u32>>();
```

#### Further reading

- [`Iterator`'s documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html) gives you an
  overview of the methods available for iterators in `std`.
- [The `itertools` crate](https://docs.rs/itertools/) defines even **more** combinators for iterators.

### 12.9 `impl Trait` in Return Position (08_impl_trait.md)

`TicketStore::to_dos` returns a `Vec<&Ticket>`.\\
That signature introduces a new heap allocation every time `to_dos` is called, which may be unnecessary depending
on what the caller needs to do with the result.
It'd be better if `to_dos` returned an iterator instead of a `Vec`, thus empowering the caller to decide whether to
collect the results into a `Vec` or just iterate over them.

That's tricky though!
What's the return type of `to_dos`, as implemented below?

```rust
impl TicketStore {
    pub fn to_dos(&self) -> ??? {
        self.tickets.iter().filter(|t| t.status == Status::ToDo)
    }
}
```

#### Unnameable types

The `filter` method returns an instance of `std::iter::Filter`, which has the following definition:

```rust
pub struct Filter<I, P> { /* fields omitted */ }
```

where `I` is the type of the iterator being filtered on and `P` is the predicate used to filter the elements.\\
We know that `I` is `std::slice::Iter<'_, Ticket>` in this case, but what about `P`?\\
`P` is a closure, an **anonymous function**. As the name suggests, closures don't have a name,
so we can't write them down in our code.

Rust has a solution for this: **impl Trait**.

#### `impl Trait`

`impl Trait` is a feature that allows you to return a type without specifying its name.
You just declare what trait(s) the type implements, and Rust figures out the rest.

In this case, we want to return an iterator of references to `Ticket`s:

```rust
impl TicketStore {
    pub fn to_dos(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets.iter().filter(|t| t.status == Status::ToDo)
    }
}
```

That's it!

#### Generic?

`impl Trait` in return position is **not** a generic parameter.

Generics are placeholders for types that are filled in by the caller of the function.
A function with a generic parameter is **polymorphic**: it can be called with different types, and the compiler will generate
a different implementation for each type.

That's not the case with `impl Trait`.
The return type of a function with `impl Trait` is **fixed** at compile time, and the compiler will generate
a single implementation for it.
This is why `impl Trait` is also called **opaque return type**: the caller doesn't know the exact type of the return value,
only that it implements the specified trait(s). But the compiler knows the exact type, there is no polymorphism involved.

#### RPIT

If you read RFCs or deep-dives about Rust, you might come across the acronym **RPIT**.\\
It stands for **"Return Position Impl Trait"** and refers to the use of `impl Trait` in return position.

### 12.10 `impl Trait` in Argument Position (09_impl_trait_2.md)

In the previous section, we saw how `impl Trait` can be used to return a type without specifying its name.\\
The same syntax can also be used in **argument position**:

```rust
fn print_iter(iter: impl Iterator<Item = i32>) {
    for i in iter {
        println!("{}", i);
    }
}
```

`print_iter` takes an iterator of `i32`s and prints each element.\\
When used in **argument position**, `impl Trait` is equivalent to a generic parameter with a trait bound:

```rust
fn print_iter<T>(iter: T)
where
    T: Iterator<Item = i32>
{
    for i in iter {
        println!("{}", i);
    }
}
```

#### Downsides

As a rule of thumb, prefer generics over `impl Trait` in argument position.\\
Generics allow the caller to explicitly specify the type of the argument, using the turbofish syntax (`::<>`),
which can be useful for disambiguation. That's not the case with `impl Trait`.

### 12.11 Slices — `&[T]` (10_slices.md)

Let's go back to the memory layout of a `Vec`:

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
```

```text
      +---------+--------+----------+
Stack | pointer | length | capacity |
      |  |      |   2    |    3     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+
Heap:  | 1 | 2 | ? |
       +---+---+---+
```

We already remarked how `String` is just a `Vec<u8>` in disguise.\\
The similarity should prompt you to ask: "What's the equivalent of `&str` for `Vec`?"

#### `&[T]`

`[T]` is a **slice** of a contiguous sequence of elements of type `T`.\\
It's most commonly used in its borrowed form, `&[T]`.

There are various ways to create a slice reference from a `Vec`:

```rust
let numbers = vec![1, 2, 3];
// Via index syntax
let slice: &[i32] = &numbers[..];
// Via a method
let slice: &[i32] = numbers.as_slice();
// Or for a subset of the elements
let slice: &[i32] = &numbers[1..];
```

`Vec` implements the `Deref` trait using `[T]` as the target type, so you can use slice methods on a `Vec` directly
thanks to deref coercion:

```rust
let numbers = vec![1, 2, 3];
// Surprise, surprise: `iter` is not a method on `Vec`!
// It's a method on `&[T]`, but you can call it on a `Vec`
// thanks to deref coercion.
let sum: i32 = numbers.iter().sum();
```

##### Memory layout

A `&[T]` is a **fat pointer**, just like `&str`.\\
It consists of a pointer to the first element of the slice and the length of the slice.

If you have a `Vec` with three elements:

```rust
let numbers = vec![1, 2, 3];
```

and then create a slice reference:

```rust
let slice: &[i32] = &numbers[1..];
```

you'll get this memory layout:

```text
                  numbers                          slice
      +---------+--------+----------+      +---------+--------+
Stack | pointer | length | capacity |      | pointer | length |
      |    |    |   3    |    4     |      |    |    |   2    |
      +----|----+--------+----------+      +----|----+--------+
           |                                    |
           |                                    |
           v                                    |
         +---+---+---+---+                      |
Heap:    | 1 | 2 | 3 | ? |                      |
         +---+---+---+---+                      |
               ^                                |
               |                                |
               +--------------------------------+
```

#### `&Vec<T>` vs `&[T]`

When you need to pass an immutable reference to a `Vec` to a function, prefer `&[T]` over `&Vec<T>`.\\
This allows the function to accept any kind of slice, not necessarily one backed by a `Vec`.

For example, you can then pass a subset of the elements in a `Vec`.
But it goes further than that—you could also pass a **slice of an array**:

```rust
let array = [1, 2, 3];
let slice: &[i32] = &array;
```

Array slices and `Vec` slices are the same type: they're fat pointers to a contiguous sequence of elements.
In the case of arrays, the pointer points to the stack rather than the heap, but that doesn't matter
when it comes to using the slice.

### 12.12 Mutable Slices — `&mut [T]` (11_mutable_slices.md)

Every time we've talked about slice types (like `str` and `[T]`), we've used their immutable borrow form (`&str` and `&[T]`).\\
But slices can also be mutable!

Here's how you create a mutable slice:

```rust
let mut numbers = vec![1, 2, 3];
let slice: &mut [i32] = &mut numbers;
```

You can then modify the elements in the slice:

```rust
slice[0] = 42;
```

This will change the first element of the `Vec` to `42`.

#### Limitations

When working with immutable borrows, the recommendation was clear: prefer slice references over references to
the owned type (e.g. `&[T]` over `&Vec<T>`).\\
That's **not** the case with mutable borrows.

Consider this scenario:

```rust
let mut numbers = Vec::with_capacity(2);
let mut slice: &mut [i32] = &mut numbers;
slice.push(1);
```

It won't compile!\\
`push` is a method on `Vec`, not on slices. This is the manifestation of a more general principle: Rust won't
allow you to add or remove elements from a slice. You will only be able to modify/replace the elements that are
already there.

In this regard, a `&mut Vec` or a `&mut String` are strictly more powerful than a `&mut [T]` or a `&mut str`.\\
Choose the type that best fits based on the operations you need to perform.

### 12.13 Two States — TicketDraft and Ticket (12_two_states.md)

Let's think again about our ticket management system.\\
Our ticket model right now looks like this:

```rust
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

One thing is missing here: an **identifier** to uniquely identify a ticket.\\
That identifier should be unique for each ticket. That can be guaranteed by generating it automatically when
a new ticket is created.

#### Refining the model

Where should the id be stored?\\
We could add a new field to the `Ticket` struct:

```rust
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

But we don't know the id before creating the ticket. So it can't be there from the get-go.\\
It'd have to be optional:

```rust
pub struct Ticket {
    pub id: Option<TicketId>,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

That's also not ideal—we'd have to handle the `None` case every single time we retrieve a ticket from the store,
even though we know that the id should always be there once the ticket has been created.

The best solution is to have two different ticket **states**, represented by two separate types:
a `TicketDraft` and a `Ticket`:

```rust
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

A `TicketDraft` is a ticket that hasn't been created yet. It doesn't have an id, and it doesn't have a status.\\
A `Ticket` is a ticket that has been created. It has an id and a status.\\
Since each field in `TicketDraft` and `Ticket` embeds its own constraints, we don't have to duplicate logic
across the two types.

### 12.14 Index Trait (13_index.md)

`TicketStore::get` returns an `Option<&Ticket>` for a given `TicketId`.\\
We've seen before how to access elements of arrays and vectors using Rust's
indexing syntax:

```rust
let v = vec![0, 1, 2];
assert_eq!(v[0], 0);
```

How can we provide the same experience for `TicketStore`?\\
You guessed right: we need to implement a trait, `Index`!

#### `Index`

The `Index` trait is defined in Rust's standard library:

```rust
// Slightly simplified
pub trait Index<Idx>
{
    type Output;

    // Required method
    fn index(&self, index: Idx) -> &Self::Output;
}
```

It has:

- One generic parameter, `Idx`, to represent the index type
- One associated type, `Output`, to represent the type we retrieved using the index

Notice how the `index` method doesn't return an `Option`. The assumption is that
`index` will panic if you try to access an element that's not there, as it happens
for array and vec indexing.

### 12.15 IndexMut Trait (14_index_mut.md)

`Index` allows read-only access. It doesn't let you mutate the value you
retrieved.

#### `IndexMut`

If you want to allow mutability, you need to implement the `IndexMut` trait.

```rust
// Slightly simplified
pub trait IndexMut<Idx>: Index<Idx>
{
    // Required method
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```

`IndexMut` can only be implemented if the type already implements `Index`,
since it unlocks an _additional_ capability.

### 12.16 HashMap (15_hashmap.md)

Our implementation of `Index`/`IndexMut` is not ideal: we need to iterate over the entire
`Vec` to retrieve a ticket by id; the algorithmic complexity is `O(n)`, where
`n` is the number of tickets in the store.

We can do better by using a different data structure for storing tickets: a `HashMap<K, V>`.

```rust
use std::collections::HashMap;

// Type inference lets us omit an explicit type signature (which
// would be `HashMap<String, String>` in this example).
let mut book_reviews = HashMap::new();

book_reviews.insert(
    "Adventures of Huckleberry Finn".to_string(),
    "My favorite book.".to_string(),
);
```

`HashMap` works with key-value pairs. It's generic over both: `K` is the generic
parameter for the key type, while `V` is the one for the value type.

The expected cost of insertions, retrievals and removals is **constant**, `O(1)`.
That sounds perfect for our usecase, doesn't it?

#### Key requirements

There are no trait bounds on `HashMap`'s struct definition, but you'll find some
on its methods. Let's look at `insert`, for example:

```rust
// Slightly simplified
impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        // [...]
    }
}
```

The key type must implement the `Eq` and `Hash` traits.\\
Let's dig into those two.

#### `Hash`

A hashing function (or hasher) maps a potentially infinite set of a values (e.g.
all possible strings) to a bounded range (e.g. a `u64` value).\\
There are many different hashing functions around, each with different properties
(speed, collision risk, reversibility, etc.).

A `HashMap`, as the name suggests, uses a hashing function behind the scene.
It hashes your key and then uses that hash to store/retrieve the associated value.
This strategy requires the key type must be hashable, hence the `Hash` trait bound on `K`.

You can find the `Hash` trait in the `std::hash` module:

```rust
pub trait Hash {
    // Required method
    fn hash<H>(&self, state: &mut H)
       where H: Hasher;
}
```

You will rarely implement `Hash` manually. Most of the times you'll derive it:

```rust
#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
}
```

#### `Eq`

`HashMap` must be able to compare keys for equality. This is particularly important
when dealing with hash collisions—i.e. when two different keys hash to the same value.

You may wonder: isn't that what the `PartialEq` trait is for? Almost!\\
`PartialEq` is not enough for `HashMap` because it doesn't guarantee reflexivity, i.e. `a == a` is always `true`.\\
For example, floating point numbers (`f32` and `f64`) implement `PartialEq`,
but they don't satisfy the reflexivity property: `f32::NAN == f32::NAN` is `false`.\\
Reflexivity is crucial for `HashMap` to work correctly: without it, you wouldn't be able to retrieve a value
from the map using the same key you used to insert it.

The `Eq` trait extends `PartialEq` with the reflexivity property:

```rust
pub trait Eq: PartialEq {
    // No additional methods
}
```

It's a marker trait: it doesn't add any new methods, it's just a way for you to say to the compiler
that the equality logic implemented in `PartialEq` is reflexive.

You can derive `Eq` automatically when you derive `PartialEq`:

```rust
#[derive(PartialEq, Eq)]
struct Person {
    id: u32,
    name: String,
}
```

#### `Eq` and `Hash` are linked

There is an implicit contract between `Eq` and `Hash`: if two keys are equal, their hashes must be equal too.
This is crucial for `HashMap` to work correctly. If you break this contract, you'll get nonsensical results
when using `HashMap`.

### 12.17 BTreeMap — Sorted Map (16_btreemap.md)

By moving from a `Vec` to a `HashMap` we have improved the performance of our ticket management system,
and simplified our code in the process.\\
It's not all roses, though. When iterating over a `Vec`-backed store, we could be sure that the tickets
would be returned in the order they were added.\\
That's not the case with a `HashMap`: you can iterate over the tickets, but the order is random.

We can recover a consistent ordering by switching from a `HashMap` to a `BTreeMap`.

#### `BTreeMap`

A `BTreeMap` guarantees that entries are sorted by their keys.\\
This is useful when you need to iterate over the entries in a specific order, or if you need to
perform range queries (e.g. "give me all tickets with an id between 10 and 20").

Just like `HashMap`, you won't find trait bounds on the definition of `BTreeMap`.
But you'll find trait bounds on its methods. Let's look at `insert`:

```rust
// `K` and `V` stand for the key and value types, respectively,
// just like in `HashMap`.
impl<K, V> BTreeMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        // implementation
    }
}
```

`Hash` is no longer required. Instead, the key type must implement the `Ord` trait.

#### `Ord`

The `Ord` trait is used to compare values.\\
While `PartialEq` is used to compare for equality, `Ord` is used to compare for ordering.

It's defined in `std::cmp`:

```rust
pub trait Ord: Eq + PartialOrd {
    fn cmp(&self, other: &Self) -> Ordering;
}
```

The `cmp` method returns an `Ordering` enum, which can be one
of `Less`, `Equal`, or `Greater`.\\
`Ord` requires that two other traits are implemented: `Eq` and `PartialOrd`.

#### `PartialOrd`

`PartialOrd` is a weaker version of `Ord`, just like `PartialEq` is a weaker version of `Eq`.
You can see why by looking at its definition:

```rust
pub trait PartialOrd: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}
```

`PartialOrd::partial_cmp` returns an `Option`—it is not guaranteed that two values can
be compared.\\
For example, `f32` doesn't implement `Ord` because `NaN` values are not comparable,
the same reason why `f32` doesn't implement `Eq`.

#### Implementing `Ord` and `PartialOrd`

Both `Ord` and `PartialOrd` can be derived for your types:

```rust
// You need to add `Eq` and `PartialEq` too,
// since `Ord` requires them.
#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct TicketId(u64);
```

If you choose (or need) to implement them manually, be careful:

- `Ord` and `PartialOrd` must be consistent with `Eq` and `PartialEq`.
- `Ord` and `PartialOrd` must be consistent with each other.

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

