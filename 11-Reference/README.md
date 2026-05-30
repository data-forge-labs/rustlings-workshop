# Rust Concept Reference Guides

This section contains **quick-lookup reference documents** for the key Rust concepts covered across the workshop projects. These are not hands-on projects — they are cheatsheets and comparison guides to bookmark and revisit as you progress through the course.

Each reference condenses what you learned across multiple workshops into a single page, with Python comparisons, decision trees, and memory-layout diagrams.

## Files in This Section

| # | File | Covers |
|---|------|--------|
| 1 | `collections-guide.md` | Vec, HashMap, HashSet, BTreeMap, VecDeque, LinkedList, BinaryHeap — when to use what, time complexity, Python equivalents |
| 2 | `concurrency-reference.md` | Send/Sync, Arc, Mutex, RwLock, channels, async/await (Tokio), Rayon, atomics, deadlock prevention |
| 3 | `data-management-io.md` | File I/O, BufReader/BufWriter, csv crate, Serde, Parquet/Arrow, Result-based error handling |
| 4 | `distributed-systems.md` | CAP theorem, consistency models, CRDTs, leader election, quorum, Rust's advantages |
| 5 | `memory-safety.md` | Ownership, borrowing, lifetimes, stack vs heap, RAII, Drop, safe vs unsafe |
| 6 | `security-model.md` | Compile-time guarantees, the type system, unsafe, RustCrypto, security checklist |
| 7 | `send-sync.md` | Send and Sync traits, auto vs manual impl, Rc vs Arc, RefCell vs Mutex |

## How to Use

- **During a workshop**: If a concept feels fuzzy, open the relevant reference for a concise reminder.
- **Before a new section**: Skim the references that cover prerequisite concepts.
- **As a cheatsheet**: Bookmark these files for day-to-day Rust data-engineering work.
