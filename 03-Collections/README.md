# Section 3: Collections — Faster Than Python Lists & Dicts

*Python lists and dicts are great. Rust's Vec and HashMap remove the interpreter overhead. Plus: sets, queues, heaps, and functional iterators.*

---

## Why This Section?

### The Problem — Python's Collection Tax

Every Python data engineer has felt the pain of processing data at scale:

```python
# Python — flexible but costly
records = []
for i in range(10_000_000):
    records.append({"id": i, "value": i * 1.5})

# Memory: each dict ~ 200 bytes → 2 GB
# Time: ~5 seconds just to build
# Type check: none — could be anything inside
```

Python's collections are convenient but **pay a runtime cost for every operation**:

```
┌─────────────────────────────────────────────────────┐
│  Python list (memory layout)                         │
│                                                      │
│  [PyObject*, PyObject*, PyObject*, ...]              │
│       │          │          │                        │
│       ▼          ▼          ▼                        │
│    ┌──────┐  ┌──────┐  ┌──────┐                     │
│    │ int  │  │ int  │  │ int  │      Each element    │
│    │ 42   │  │ 99   │  │ 177  │      is a PyObject  │
│    └──────┘  └──────┘  └──────┘      on the heap     │
│                                                      │
│  Memory: 8 bytes per pointer + 28 bytes per int      │
│  = 36 bytes per element vs 4 bytes in Rust           │
└─────────────────────────────────────────────────────┘
```

**Common pain points:**

| Problem | Python | Rust |
|---------|--------|------|
| List of mixed types | Runtime `TypeError` | Compile-time enforced |
| Dict iteration order | Guaranteed in 3.7+ only | `HashMap` = unordered, `BTreeMap` = ordered |
| Missing key | `KeyError` at runtime | `.get()` returns `Option` |
| Large memory | 36+ bytes per int | 4 bytes per `i32` |
| Sorting | Sorts in-place | Iterator-based, functional |
| Queue | `collections.deque` or `queue.Queue` | `VecDeque`, `BinaryHeap` |

### The Rust Solution — Zero-Cost Abstractions

Rust's collections are **zero-cost abstractions**: they compile down to the same machine code as hand-written C, with no hidden interpreter overhead:

```rust
let records: Vec<(u32, f64)> = (0..10_000_000)
    .map(|i| (i, i as f64 * 1.5))
    .collect();

// Memory: 12 bytes per element → 120 MB
// Time: ~200ms to build
// Type: enforced at compile time — (u32, f64) only
```

Every collection in `std::collections` is designed for a specific use case with known performance characteristics. Choose the right tool, and your data pipelines run **10-50x faster** with **3-10x less memory**.

---

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | Dynamic arrays | `Vec<T>` | `list` | Type-safe, contiguous growable array |
| 2 | Fixed-size arrays | `[T; N]` | `array.array` | Stack-allocated, known-length collection |
| 3 | Key-value maps | `HashMap<K, V>` | `dict` | Hash-based key-value storage |
| 4 | Ordered maps | `BTreeMap<K, V>` | `sortedcontainers.SortedDict` | Sorted key-value pairs (by key) |
| 5 | Hash sets | `HashSet<T>` | `set` | Unique elements, fast membership |
| 6 | Ordered sets | `BTreeSet<T>` | `sortedcontainers.SortedSet` | Sorted unique elements |
| 7 | Double-ended queues | `VecDeque<T>` | `collections.deque` | Push/pop from both ends, ring buffer |
| 8 | Priority queues | `BinaryHeap<T>` | `heapq` | Max-heap, priority-based extraction |
| 9 | Linked lists | `LinkedList<T>` | `collections.deque` (diff) | Doubly-linked, O(1) splice/split |
| 10 | Entry API | `.entry().or_insert()` | `dict.setdefault()` | Upsert / insert-or-modify pattern |
| 11 | Lazy iteration | `Iterator` trait | Iterator protocol | Functional chain: `map`, `filter`, `fold` |
| 12 | Slices | `&[T]` | `list[:]` (view) | Borrowed view into a contiguous sequence |
| 13 | Array slices | `&[T; N]` → `&[T]` | N/A | Coerce array to slice for function args |

---

## Concepts at a Glance

### 1. `Vec<T>` — Dynamic Array

Rust's workhorse collection. Contiguous memory, cache-friendly, type-safe:

```rust
let mut numbers: Vec<i32> = Vec::new();
numbers.push(10);          // append
numbers.push(20);
let first = numbers[0];    // index (panics if out of bounds)
let safe = numbers.get(0); // returns Option<&T>
```

**Performance**: `push` amortized O(1), index O(1), insert/remove O(n).

```
  Vec memory layout:
  ┌──────────┬──────────┬──────────┬──────────┬──────
  │    10    │    20    │    30    │    40    │  ...   contiguous heap memory
  └──────────┴──────────┴──────────┴──────────┴──────
  ▲          ▲                    ▲
  ptr        len (4)             capacity (8)
```

### 2. `[T; N]` — Fixed-Size Array

Stack-allocated, known at compile time:

```rust
let scores: [i32; 5] = [90, 80, 85, 95, 88];
let first = scores[0];  // index
```

**Performance**: Stack allocation (fast), no heap, known size at compile time.

### 3. `HashMap<K, V>` — Hash Map

Like Python's `dict`, but unordered and type-fixed:

```rust
let mut counts: HashMap<String, u32> = HashMap::new();
counts.insert("apple".to_string(), 3);
counts.insert("banana".to_string(), 5);

// Safe access — no KeyError
let apple_count = counts.get("apple");  // Option<&u32>
```

### 4. `BTreeMap<K, V>` — Ordered Map

Iterates in key order (sorted):

```rust
let mut map = BTreeMap::new();
map.insert("z", 3);
map.insert("a", 1);
for (k, v) in &map {
    println!("{}: {}", k, v);  // a: 1, z: 3
}
```

### 5. `HashSet<T>` — Hash Set

Fast membership testing:

```rust
let mut seen: HashSet<String> = HashSet::new();
seen.insert("error_001".to_string());
if seen.contains("error_001") {
    println!("Duplicate detected!");
}
```

### 6. `VecDeque<T>` — Double-Ended Queue

Push/pop from both ends efficiently:

```rust
let mut deque: VecDeque<i32> = VecDeque::new();
deque.push_back(10);
deque.push_front(0);
deque.pop_back();   // removes 10
deque.pop_front();  // removes 0
```

### 7. `BinaryHeap<T>` — Priority Queue

Max-heap by default (largest pops first):

```rust
let mut heap = BinaryHeap::new();
heap.push(5);
heap.push(10);
heap.push(3);
assert_eq!(heap.pop(), Some(10));  // largest first
```

### 8. `.entry().or_insert()` — The Upsert Pattern

Insert if missing, modify if present:

```rust
let mut counts: HashMap<String, u32> = HashMap::new();
for word in text.split_whitespace() {
    *counts.entry(word.to_string()).or_insert(0) += 1;
}
```

In Python: `counts[word] = counts.get(word, 0) + 1`

### 9. The `Iterator` Trait — Lazy Processing

Rust's iterators are **lazy** — nothing happens until you consume:

```rust
let result: Vec<i32> = vec![1, 2, 3, 4, 5]
    .iter()
    .filter(|x| *x % 2 == 0)     // only evens
    .map(|x| x * 10)             // multiply by 10
    .collect();                  // consume → [20, 40]
```

No intermediate allocations. Each chained method is a **zero-cost abstraction**.

### 10. Slices `&[T]` — Borrowed View

Slices let you pass a view into data without copying:

```rust
fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

let vec = vec![1, 2, 3, 4, 5];
println!("{}", sum(&vec[1..4]));  // 2 + 3 + 4 = 9
```

### Collection Selection Guide

```
┌─────────────────────────────────────────────────────────┐
│  Need this?                    →  Use this               │
├─────────────────────────────────────────────────────────┤
│  Dynamic list                   →  Vec                   │
│  Fixed-size, stack performance  →  [T; N]               │
│  Key-value (unsorted)           →  HashMap               │
│  Key-value (sorted)             →  BTreeMap              │
│  Unique items                   →  HashSet               │
│  Unique, sorted                 →  BTreeSet              │
│  Queue (FIFO)                   →  VecDeque              │
│  Priority queue                 →  BinaryHeap            │
│  Frequent splice/split          →  LinkedList            │
│  Read-only view                 →  &[T] (slice)          │
└─────────────────────────────────────────────────────────┘
```

---

## Prerequisites

- Completed [Section 2: Ownership](../02-Ownership/README.md)
- Understand ownership and borrowing

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 6 | **TicketManagement** — Vec, arrays, HashMap | `Vec`, arrays `[T;N]`, iterators, lifetimes, `impl Trait`, slices, `HashMap`, `BTreeMap` | Tutorial |
| 9 | **VectorFruitSalad** — dynamic arrays with Vec | `Vec<T>`, `SliceRandom`, `rand`, iteration, `&str` | Project |
| 10 | **ArrayFruitSalad** — fixed-size vs dynamic | Arrays `[T;N]`, `Vec`/`VecDeque`/`LinkedList` comparison | Project |
| 11 | **HashMapCount** — frequency counting | `HashMap`, `entry`/`or_insert`, `BTreeMap`, sorting | Project |
| 12 | **LinkedListFruitSalad** — doubly-linked list | `LinkedList`, memory overhead | Project |
| 13 | **VecDequeFruitSalad** — double-ended queue | `VecDeque`, ring buffer, `push_front`/`push_back` | Project |
| 15 | **HashMapLanguage** — complex HashMap data | `HashMap` with complex values, `values_mut` | Project |
| 16 | **CollectionsLessonReflection** — comparison guide | Collection trade-offs, big-O, memory efficiency | Reflection |
| 17 | **RustCollectionsDoc** — reference document | All `std::collections`, `criterion` benchmarks | Reference |
| 18 | **BinaryHeapFruit** — priority queue | `BinaryHeap`, max-heap | Project |
| 19 | **BTreeSetFruit** — ordered set | `BTreeSet`, ordered iteration | Project |
| 23 | **HashSetFruit** — unique items | `HashSet`, uniqueness, membership testing | Project |
| 28 | **RustIterators** — lazy functional iteration | `Iterator` trait, lazy eval, `map`/`filter`/`fold` | Project |
| 30 | **WhenToUseRustSet** — selection guide | All collections comparison | Reference |
| 36 | **MutableFruitSalad** — Vec mutation | `push`/`pop`/`insert`/`remove`, capacity vs length | Project |

## Learning Path

1. Start with **6-TicketManagement** tutorial for collections theory
2. Build **9-VectorFruitSalad** through **13-VecDequeFruitSalad** for hands-on practice
3. Use **11-HashMapCount** and **15-HashMapLanguage** for HashMap mastery
4. Explore **18-BinaryHeapFruit**, **19-BTreeSetFruit**, **23-HashSetFruit** for specialized collections
5. Finish with **28-RustIterators** and **16-CollectionsLessonReflection**
