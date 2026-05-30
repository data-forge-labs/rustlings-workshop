# Section 3: Collections — Faster Than Python Lists & Dicts

*Python lists and dicts are great. Rust's Vec and HashMap remove the interpreter overhead.*

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

## Key Concepts Covered

| Rust Concept | Python Equivalent | Why It Matters |
|---|---|---|
| `Vec<T>` | `list` | Dynamic arrays |
| `HashMap<K,V>` | `dict` | Key-value storage |
| `BTreeMap<K,V>` | `SortedDict` | Ordered key-value |
| `HashSet<T>` | `set` | Unique membership |
| `BTreeSet<T>` | `sortedcontainers` | Ordered set |
| `VecDeque<T>` | `collections.deque` | Double-ended queue |
| `BinaryHeap<T>` | `heapq` | Priority queue |
| `LinkedList<T>` | `collections.deque` (different) | Linked list |
| `Iterator` | Iterator protocol | Lazy functional processing |
| `.entry().or_insert()` | `dict.setdefault()` | Upsert pattern |
