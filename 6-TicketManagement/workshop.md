# Rust for Python Data Engineers — TicketManagement: Collections & Iterators

*Master Rust's collections (Vec, HashMap) and the iterator pattern — the bread and butter of data processing in any language.*

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

### Creating and Using Vectors

```rust
let mut v: Vec<i32> = Vec::new();    // Empty vector
let mut v = vec![1, 2, 3];           // With initial values (macro)

v.push(4);                            // Add to end
v.push(5);
let last = v.pop();                   // Remove and return last element (Some(5))

let first = v[0];                     // Index access
v[0] = 10;                            // Modify (if mutable)

println!("Length: {}", v.len());       // 4
println!("Capacity: {}", v.capacity()); // 8 (may over-allocate)
```

### Vec Memory Layout

```
let mut v = vec![1, 2, 3];

Stack (24 bytes):          Heap (capacity × 4 bytes):
┌────────────────┐        ┌────┬────┬────┬────┬────┬────┬────┬────┐
│ ptr: ────────────────→   │  1 │  2 │  3 │  ? │  ? │  ? │  ? │  ? │
│ len: 3          │        └────┴────┴────┴────┴────┴────┴────┴────┘
│ cap: 8          │         ↑              ↑
└────────────────┘         data           unused capacity

v.push(4):          cap stays 8 (no reallocation needed)
                    ┌────┬────┬────┬────┬────┬────┬────┬────┐
                    │  1 │  2 │  3 │  4 │  ? │  ? │  ? │  ? │
                    └────┴────┴────┴────┴────┴────┴────┴────┘
                    len: 4
```

### Python List vs Rust Vec

| Operation | Python | Rust |
|---|---|---|
| Create | `items = []` | `let items: Vec<T> = Vec::new();` |
| With values | `items = [1, 2, 3]` | `let items = vec![1, 2, 3];` |
| Add | `items.append(x)` | `items.push(x);` |
| Insert at index | `items.insert(i, x)` | `items.insert(i, x);` |
| Remove last | `items.pop()` | `items.pop();` |
| Remove at index | `items.pop(i)` | `items.remove(i);` |
| Length | `len(items)` | `items.len()` |
| Sort in place | `items.sort()` | `items.sort()` |
| Iterate | `for x in items:` | `for x in items { }` (consumes) |
| Iterate (borrow) | `for x in items:` (same) | `for x in &items { }` |

### Ownership and Vectors

```rust
let v = vec![1, 2, 3];
let first = v[0];  // ✅ i32 is Copy
// v[0] still valid

let v = vec![String::from("a"), String::from("b")];
// let first = v[0];  // ❌ ERROR: cannot move out of Vec
let first = v[0].clone();  // ✅ Explicit clone
let first = &v[0];         // ✅ Borrow (most common)
```

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

### Next Project

Proceed to [7-Threads](../7-Threads/workshop.md) for **concurrency** — running data pipelines in parallel.
