# Rust Collections — When to Use What

## Python → Rust Mapping

| Python | Rust | Crate |
|--------|------|-------|
| `list` | `Vec<T>` | std |
| `collections.deque` | `VecDeque<T>` | std |
| `dict` | `HashMap<K, V>` | std |
| `OrderedDict` | `BTreeMap<K, V>` | std |
| `set` | `HashSet<T>` | std |
| `heapq` (priority queue) | `BinaryHeap<T>` | std |
| `collections.defaultdict` | `entry().or_insert()` pattern | std |
| pandas Index | `BTreeSet<T>` | std |
| `collections.deque` (linked) | `LinkedList<T>` | std |

## Time Complexity

### Sequences

| Operation | `Vec<T>` | `VecDeque<T>` | `LinkedList<T>` |
|-----------|----------|---------------|-----------------|
| Push back | O(1)* | O(1)* | O(1) |
| Pop back | O(1) | O(1) | O(1) |
| Push front | O(n) | O(1)* | O(1) |
| Pop front | O(n) | O(1)* | O(1) |
| Index | O(1) | O(1) | O(n) |
| Insert at index | O(n) | O(n) | O(1) at known node |
| Iterate | O(n) | O(n) | O(n) |
| `len()` | O(1) | O(1) | O(1) |

\* Amortized O(1) — may reallocate

### Maps & Sets

| Operation | `HashMap<K,V>` | `BTreeMap<K,V>` | `HashSet<T>` | `BTreeSet<T>` |
|-----------|----------------|-----------------|--------------|---------------|
| Insert | O(1)* | O(log n) | O(1)* | O(log n) |
| Get | O(1)* | O(log n) | O(1)* | O(log n) |
| Remove | O(1)* | O(log n) | O(1)* | O(log n) |
| Iterate (sorted) | — | O(n) | — | O(n) |
| Range query | — | O(log n + k) | — | O(log n + k) |
| Contains | O(1)* | O(log n) | O(1)* | O(log n) |
| Union/Intersection | O(n + m) | O(n + m) | O(min(n,m)) | O(n + m) |

\* With good hash function — worst-case O(n) if many collisions

### Heap

| Operation | `BinaryHeap<T>` (max-heap) |
|-----------|---------------------------|
| Push | O(1)* |
| Pop (max) | O(log n) |
| Peek | O(1) |

## Memory Overhead

```
        Vec<T>:     [ptr | len | cap] = 24 bytes on 64-bit
        HashMap<K,V>: [ptr | len | cap | hash_builder] ≈ 56 bytes
        BTreeMap<K,V>: internal nodes, ~16 bytes per entry overhead
        LinkedList<T>: each element is a separate allocation — worst cache locality
```

## When to Choose

- **Vec<T>**: Default sequence. Use it unless you need something else. Best cache locality.
- **VecDeque<T>**: Need O(1) push/pop at **both** ends. Ring buffer internally.
- **LinkedList<T>**: Almost never. Only when you need to split/merge lists without copying elements.
- **HashMap<K, V>**: Default map. Fast lookups. Keys must implement `Hash + Eq`.
- **BTreeMap<K, V>**: Need ordered iteration or range queries. Keys must implement `Ord`.
- **HashSet<T>**: Track unique items, set operations. Like `HashMap<T, ()>`.
- **BTreeSet<T>**: Ordered unique items. Like `BTreeMap<T, ()>`.
- **BinaryHeap<T>**: Priority queue (max-heap). Use `.iter().rev()` for min-heap.

## Common Patterns

```rust
use std::collections::*;

// HashMap entry API (Python defaultdict equivalent)
let mut counts: HashMap<String, u32> = HashMap::new();
*counts.entry("key".to_string()).or_insert(0) += 1;

// BTreeMap range query
let mut map = BTreeMap::new();
map.insert("a", 1);
map.insert("c", 3);
for (k, v) in map.range("a"..="c") { }

// BinaryHeap as min-heap
use std::cmp::Reverse;
let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
min_heap.push(Reverse(5));

// VecDeque as queue
let mut deque = VecDeque::new();
deque.push_back(1);
deque.push_front(2);
assert_eq!(deque.pop_front(), Some(2));
```
