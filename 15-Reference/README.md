# Section 15: Reference Appendix — Quick Concept Lookup

*Cheatsheets, comparison guides, and decision trees for day-to-day Rust data engineering. No projects — just reference materials to keep open in a tab while you work.*

---
## Why This Section?

Learning Rust means mastering many interlocking concepts. You won't remember everything — nobody does. This appendix gives you **one-page summaries** of each major concept cluster, so you can quickly look up syntax, patterns, and trade-offs while building real projects.

Each reference is designed as a **cheatsheet**: dense, structured, and linked back to the workshop where the concept was first introduced.

---

## How to Use This Section

This is the **last section** in the course — it's an appendix, not a sequential read.

- **During a workshop**: If a concept feels fuzzy, open the relevant reference for a concise reminder with Python comparisons.
- **Before a new section**: Skim the references that cover prerequisite concepts. For example, before starting Section 5 (Concurrency), review `concurrency-reference.md` and `send-sync.md`. Section 14 (Data Infrastructure) assumes the references in `data-management-io.md` and `distributed-systems.md` are familiar.
- **As a daily cheatsheet**: Bookmark these files for day-to-day Rust data-engineering work. The decision trees (e.g., "Which collection should I use?") will save you time.

### Reference Map

```
  Need to know...                    Open this file
  ─────────────────────────────────────────────────
  Which collection?                  collections-guide.md
  Array vs slice in fn signature?    collections-guide.md (§"Arrays vs Slices")
  Thread safety?                     send-sync.md
  File I/O pattern?                  data-management-io.md
  Consistency model?                 distributed-systems.md
  Borrow checker error?              memory-safety.md
  Memory cost / off-heap?            heap-memory.md
  Security best practice?            security-model.md
  Concurrency pattern?               concurrency-reference.md
```

---

## What You'll Find

| # | File | Covers | Best When You Need |
|---|------|--------|--------------------|
| 1 | `collections-guide.md` | Vec, HashMap, HashSet, BTreeMap, VecDeque, LinkedList, BinaryHeap, arrays `[T;N]`, slices `&[T]` — when to use what, time complexity, Python equivalents | "Should I use a `Vec` or a `VecDeque`?" |
| 2 | `concurrency-reference.md` | Send/Sync, Arc, Mutex, RwLock, channels, async/await (Tokio), Rayon, atomics, deadlock prevention | "Is this type `Send`?" or "How do I share state across threads?" |
| 3 | `data-management-io.md` | File I/O, BufReader/BufWriter, csv crate, Serde, Parquet/Arrow, Result-based error handling | "How do I read this CSV/Parquet file?" |
| 4 | `distributed-systems.md` | CAP theorem, consistency models, CRDTs, leader election, quorum, Rust's advantages | "What's the difference between strong and eventual consistency?" |
| 5 | `memory-safety.md` | Ownership, borrowing, lifetimes, stack vs heap, RAII, Drop, safe vs unsafe | "Why does the borrow checker reject this?" |
| 6 | `heap-memory.md` | Stack vs heap deep dive, Rust vs Python memory layout, off-heap concept, allocators, GC comparison | "What's the memory cost of this `Vec`/`HashMap`?" or "What is off-heap memory?" |
| 7 | `security-model.md` | Compile-time guarantees, the type system, `unsafe`, RustCrypto, security checklist | "How does Rust prevent security vulnerabilities?" |
| 8 | `send-sync.md` | `Send` and `Sync` traits, auto vs manual impl, `Rc` vs `Arc`, `RefCell` vs `Mutex` | "Can I share this type between threads?" |

This section has no Cargo projects — it's all `.md` files designed for quick lookup. Return to it whenever you need a refresher.
