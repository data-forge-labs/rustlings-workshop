# Rust Tutorial — Learn by Doing

A comprehensive, progressive Rust course built from two high-quality sources. It combines **tutorial-based exercises** (projects 0–8) with **hands-on Cargo projects** (projects 9–58), walking you from absolute beginner to advanced Rust.

## How It Works

```
RustTut/
├── README.md                        ← this file
├── AGENTS.md                        ← AI workshop designer instructions
├── .devcontainer/                   ← preconfigured Rust dev environment
├── 0-Intro/  … 2-BasicCalculator/   ← Section 1: Foundations
├── 3-TicketV1/ … 38-OwnershipLifetimes/  ← Section 2: Ownership
├── 6-TicketManagement/ … 36-MutableFruitSalad/  ← Section 3: Collections
├── 53-CSVCookbook/ … 56-DataManagementLessonReflection/  ← Section 4: File I/O
├── 7-Threads/ … 52-ConcurrencyLessonReflection/  ← Section 5: Concurrency
├── 14-CLISalad/ … 33-CustomCLIFruitSalad/  ← Section 6: CLI & Tools
├── 35-SafeAndUnsafe/ … 43-SecurityLessonReflection/  ← Section 7: Security
└── 57-ExploringPandas/ … 58-RustJupyterNotebook/  ← Section 8: Interop
```

The course is organized into **8 sections** designed for a Python data engineer moving to Rust. Each section starts with tutorial-style projects (read `.md` files, write code alongside) and progresses to hands-on Cargo projects (build and run complete programs).

**Progression:** Go through sections in order. Within each section, start with lower-numbered projects (introduce concepts) then move to higher-numbered ones (apply and deepen). Concepts from earlier sections are assumed in later ones.

## Course Progression

The course is carefully sequenced so each concept is introduced by one source and deepened by the other, never repeated as a first-time lesson.

| Section | Concept Cluster | Projects |
|---------|----------------|----------|
| 1 — Foundations | Syntax, types, control flow, basic I/O | 0 (Intro), 1 (MasterMind), 2 (BasicCalculator) |
| 2 — Ownership | Structs, ownership, borrowing, lifetimes, traits, enums, error handling | 3 (TicketV1), 4 (Traits), 5 (TicketV2), 37 (OBRM), 38 (OwnershipLifetimes) |
| 3 — Collections | Vec, arrays, HashMap, HashSet, BTreeMap, iterators, LinkedList, VecDeque, BinaryHeap | 6 (TicketManagement), 9–13, 15–19, 23, 28, 30, 36 |
| 4 — File I/O | CSV reading/writing, Parquet, serde, file I/O | 53 (CSVCookbook), 54 (CSVWriter), 55 (Parquet), 56 (Reflection) |
| 5 — Concurrency | Threads, async/await, Mutex, Arc, Send/Sync, Rayon, atomics, channels | 7 (Threads), 8 (Futures), 34, 44–52 |
| 6 — CLI & Tools | CLAP, petgraph, Dijkstra, PageRank, Neo4j | 14, 20–22, 24–27, 29, 31, 33 |
| 7 — Security | Safe vs unsafe, crypto, security model | 35, 39–43 |
| 8 — Interop | evcxr, Jupyter, pandas bridge | 57, 58 |

**How to use this table:** Start from Section 1 and work forward. Within each section, start with the lower-numbered projects (they introduce the concepts) and then move to the higher-numbered ones (they apply and deepen them).

---

## Table of Contents

- [How It Works](#how-it-works)
- [Course Progression](#course-progression)
- [Projects](#projects)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Rust Concepts Coverage](#rust-concepts-coverage)
- [Pattern Matching: @ Bindings and Guards](#pattern-matching--bindings-and-guards)
- [Credits](#credits)
- [License](#license)

---

## Projects

Projects are grouped into **sections** that map concepts a Python data engineer already knows to their Rust equivalents.

### Section 1: Foundations — From Python Loops to Rust Safety

*Getting started: syntax, types, control flow, and your first Rust programs.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 0 | **Intro** — Rust syntax primer (reference) | `fn main()`, `let`, `mut`, macros (`println!`), basic types, `&str`, arithmetic |
| 1 | **MasterMind** — guess a 4-digit secret code with hints | `struct`, `impl`, `Vec<T>`, `Option<T>`, `if let`, loops, `String`/`&str`, `rand`, iterators |
| 2 | **BasicCalculator** — integers, branching, loops, overflow | `i32`/`u32`, `if`/`else`, `while`/`for`, panics, overflow, saturating arithmetic, `as` casting |
| 32 | **Week1FinalReflection** — data structures mindset | Memory safety, zero-cost abstractions, Rust vs Python data engineering |

### Section 2: Ownership — Rust's Superpower over Python's GC

*The biggest mindset shift: how Rust manages memory without a garbage collector, and how it prevents whole classes of bugs at compile time.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 3 | **TicketV1** — structs, ownership, stack vs heap | `struct`, validation, `mod`/`pub`, encapsulation, ownership, setters, stack/heap, destructors (`Drop`) |
| 4 | **Traits** — trait definitions, derive, bounds | `trait`, orphan rule, operator overloading, `derive`, trait bounds, `Deref`, `Sized`, `From` |
| 5 | **TicketV2** — enums, match, error handling | `enum`, `match`, `if let`, `Option`, `Result`, error enums, `Error` trait, `thiserror`, `TryFrom` |
| 37 | **OBRM** — ownership-based resource management | Ownership rules, `Drop` trait, RAII, borrowing, resource lifecycle |
| 38 | **OwnershipLifetimes** — lifetimes & borrow checker | Ownership (move semantics), borrowing (`&T`/`&mut T`), lifetimes (`'a`), `Copy`/`Clone` |

### Section 3: Collections — Faster Than Python Lists & Dicts

*Python lists and dicts are great. Rust's Vec and HashMap remove the interpreter overhead. Plus: sets, queues, heaps, and functional iterators.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 6 | **TicketManagement** — Vec, arrays, HashMap, BTreeMap | `Vec`, arrays `[T;N]`, iterators, lifetimes, `impl Trait`, slices, `HashMap`, `BTreeMap`, `Index` |
| 9 | **VectorFruitSalad** — dynamic arrays with Vec | `Vec<T>`, `SliceRandom`, `rand`, iteration, `&str`, mutable refs |
| 10 | **ArrayFruitSalad** — fixed-size vs dynamic arrays | Arrays `[T;N]`, `Vec`/`VecDeque`/`LinkedList` comparison |
| 11 | **HashMapCount** — frequency counting | `HashMap`, `entry`/`or_insert`, `BTreeMap`, sorting by value |
| 12 | **LinkedListFruitSalad** — doubly-linked list | `LinkedList`, memory overhead, collection conversion |
| 13 | **VecDequeFruitSalad** — double-ended queue | `VecDeque`, ring buffer, `push_front`/`push_back` |
| 15 | **HashMapLanguage** — complex HashMap data | `HashMap` with complex values, `values_mut`, normalization |
| 16 | **CollectionsLessonReflection** — comparison guide | Collection trade-offs, big-O, memory efficiency |
| 17 | **RustCollectionsDoc** — reference document | All `std::collections`, `criterion` benchmarks |
| 18 | **BinaryHeapFruit** — priority queue | `BinaryHeap`, max-heap, priority queue behavior |
| 19 | **BTreeSetFruit** — ordered set | `BTreeSet`, ordered iteration, `HashSet` vs `BTreeSet` |
| 23 | **HashSetFruit** — unique items with HashSet | `HashSet`, uniqueness, membership testing |
| 28 | **RustIterators** — lazy functional iteration | `Iterator` trait, lazy eval, `map`/`filter`/`fold` |
| 30 | **WhenToUseRustSet** — selection guide | All collections comparison, complexity trade-offs |
| 36 | **MutableFruitSalad** — Vec mutation | `push`/`pop`/`insert`/`remove`, capacity vs length |

### Section 4: File I/O — CSV & Parquet at Scale

*Python's pandas reads CSVs. Rust's csv and parquet crates do it faster, with less memory.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 53 | **CSVCookbook** — read, write, transform CSV | `csv` crate, deserialization, record iteration, error handling |
| 54 | **CSVWriter** — programmatic CSV writing | `csv::Writer`, custom delimiters, `serde` (`Deserialize`/`Serialize`) |
| 55 | **Parquet** — Apache Parquet columnar format | Parquet format, columnar storage, Arrow integration |
| 56 | **DataManagementLessonReflection** — I/O reflection | File I/O, serialization, columnar vs row-oriented |

### Section 5: Concurrency — Beyond Python's GIL

*Python threads are limited by the GIL. Rust gives you true parallelism with compile-time safety guarantees.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 7 | **Threads** — threads, channels, locks | `std::thread`, `'static`, scoped threads, `mpsc`, interior mutability, `Mutex`/`Arc`, `RwLock`, `Sync` |
| 8 | **Futures** — async/await, tasks, runtimes | `async fn`, `.await`, `tokio`, `Future` trait, spawning, cancellation |
| 34 | **DataRace** — preventing data races | `Mutex`, `Arc`, `MutexGuard`, shared-state concurrency |
| 44 | **Atomics** — lock-free atomics | Atomic types, memory ordering (`Relaxed`, `Acquire`, `Release`, `SeqCst`) |
| 45 | **DistributedChallenges** — consistency in distributed systems | Eventual vs strong consistency, CAP theorem |
| 46 | **ConcurrencyParallelism** — Send/Sync, RwLock | `Send`/`Sync` traits, `Mutex`, `RwLock`, `Arc` |
| 47 | **DataRacesRaceConditions** — data races vs race conditions | Data races, race conditions, `Cell`/`RefCell` |
| 48 | **DiningPhilosophers** — deadlock prevention | `Mutex`, ordered lock acquisition, thread synchronization |
| 49 | **DistributedComputing** — Rust for distributed systems | GC overhead, compiled vs interpreted, distributed challenges |
| 50 | **RayonChallenge** — data parallelism with Rayon | `rayon` parallel iterators, speedup benchmarking |
| 51 | **SendSync** — Send and Sync marker traits | `Send`, `Sync`, thread safety markers, `unsafe impl` |
| 52 | **ConcurrencyLessonReflection** — concurrency review | Ownership + concurrency, data-race freedom, `mpsc` |

### Section 6: CLI & Data Engineering Tools

*Building production-ready CLI tools, graph analytics, and connecting Rust to Python data tools.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 14 | **CLISalad** — CLI with clap arg parsing | `clap` derive, `std::env`, pattern matching, `std::io` |
| 20 | **CommunityDetection** — Kosaraju's SCC algorithm | `petgraph`, directed graphs, SCC, DFS, graph transposition |
| 21 | **UFCGraphCentrality** — centrality on UFC data | `UnGraph`, degree/closeness centrality, `NodeIndex` |
| 22 | **GraphVisualize** — ASCII bar charts | `rasciigraph`, ASCII visualization, data scaling |
| 24 | **LisbonShortestPath** — Dijkstra's algorithm | Dijkstra, weighted graphs, `BinaryHeap` as priority queue |
| 25 | **Neo4jDataScience** — Neo4j graph DB | Neo4j integration, centrality algorithms (degree, closeness, betweenness, eigenvector) |
| 26 | **PageRank** — PageRank algorithm | PageRank, iterative ranking, damping factor, link analysis |
| 27 | **RussianTrollTweets** — Neo4j analysis | Graph DB analysis, influence detection, social graph modeling |
| 29 | **DataStructuresLessonReflection** — graph DS reflection | Graph vs other DS, centrality metrics, community detection |
| 31 | **FullyConnectedGraph** — graph connectivity | Graph connectivity, `HashMap` memoization |
| 33 | **CustomCLIFruitSalad** — advanced CLI + CSV | `clap` derive, CSV reading, `lib.rs`/`main.rs` separation, modules |

### Section 7: Security & Systems Programming

*Why Rust is the safe alternative to C/C++ for data pipelines, and how cryptography fits in.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 35 | **SafeAndUnsafe** — safe vs unsafe Rust | `unsafe` keyword, raw pointers, FFI, safety invariants |
| 39 | **SafetyLessonReflection** — Rust vs GC languages | Memory safety, data race prevention, explicit resource management |
| 40 | **DecoderRing** — crack Caesar cipher | Frequency analysis, statistical scoring, `rayon` parallelism |
| 41 | **RustCryptoHashes** — cryptographic hashes | SHA-2/3, BLAKE2, `Digest` trait, RustCrypto |
| 42 | **RustSoftwareSecurity** — Rust vs C/C++/Java | Ownership/borrowing safety, compile-time vs runtime safety |
| 43 | **SecurityLessonReflection** — high-availability security | Redundancy, encryption, access control, disaster recovery |

### Section 8: Rust + Python Interop

*Bridge the two worlds: use Rust from Jupyter notebooks, and call pandas from Rust.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 57 | **ExploringPandas** — Rust meets pandas | DataFrame operations, Python/Rust interop, filtering/grouping, matplotlib |
| 58 | **RustJupyterNotebook** — interactive Rust with evcxr | `evcxr` Jupyter kernel, interactive Rust, `plotters`/`ndarray`/`rayon` |

---

## Prerequisites

- Rust installed via [rustup](https://rustup.rs/) **or** use the included Dev Container
- Basic Python knowledge
- Familiarity with fundamental programming concepts (variables, functions, conditionals, loops)

## Quick Start

Each numbered folder is a standalone Rust project with a `Cargo.toml` and `src/`. You can run it in two ways:

### Option 1 — Dev Container (VS Code / GitHub Codespaces)

The `.devcontainer/` folder at the root provides a preconfigured Rust environment with all dependencies and the `evcxr_jupyter` kernel (for notebooks). Open the repo root in VS Code and click **"Reopen in Container"** when prompted. Then:

```bash
cd <project-number>
cargo run
```

### Option 2 — Local Rust Installation

```bash
# Build and run any project with cargo
cd 9-VectorFruitSalad
cargo build
cargo run

# Or use the project's Makefile (if present)
make run
```

### Option 3 — Compile directly with rustc

```bash
# Some projects can be compiled without cargo
cd 1-MasterMind
rustc master_mind.rs && ./master_mind
```

Browse the full list of 59 projects (0–58) in the [Projects](#projects) table above.

---

## Rust Concepts Coverage

The table below lists all core Rust concepts a learner should eventually see. **Checked items** are already introduced in at least one existing workshop.

| Concept | Covered? | First Project |
|---------|----------|---------------|
| `cargo new`, `cargo build`, `cargo run` | ✅ | 1 |
| `Cargo.toml` dependencies | ✅ | 1, 5 |
| Variables (`let`, `let mut`) | ✅ | 0, 1 |
| Data types (`u32`, `i32`, `f64`, `bool`, `char`, `usize`, `u8`) | ✅ | 0, 1, 2 |
| `String` vs `&str` | ✅ | 1, 4 |
| Ownership, borrowing, references (`&`, `&mut`) | ✅ | 1, 3, 38 |
| `Vec<T>`, `vec![]` | ✅ | 1, 6, 9 |
| `struct`, `impl`, methods (`&self`, `&mut self`) | ✅ | 1, 3 |
| `Option<T>`, `Some`, `None`, `if let` | ✅ | 1, 5 |
| `match` (basic) | ✅ | 1, 5 |
| `match` with patterns (advanced) | ✅ | 5, 14, 51 |
| `loop`, `while`, `continue`, `break` | ✅ | 1, 2 |
| `const` | ✅ | 1 |
| `if` / `else` branching | ✅ | 2 |
| Integer overflow & saturating arithmetic | ✅ | 2 |
| Iterators (`iter`, `map`, `filter`, `count`, `collect`, `zip`, `enumerate`, `any`, `all`) | ✅ | 1, 6, 28 |
| Closures (`\|x\| x * 2`) | ✅ | 1, 28 |
| `print!`, `println!` | ✅ | 1 |
| `std::io::stdin()`, `read_line()` | ✅ | 1 |
| `io::stdout().flush()` | ✅ | 1 |
| String methods (`chars`, `trim`, `to_lowercase`, `is_ascii_digit`, `to_digit`) | ✅ | 1 |
| Ranges (`0..=9`) | ✅ | 1, 2 |
| `rand` crate (`thread_rng`, `shuffle`, `choose`) | ✅ | 1, 9 |
| Type casting (`as`) | ✅ | 1, 2 |
| Tuples | ✅ | 1 |
| `unwrap()` / basic error handling | ✅ | 1, 5 |
| `Result<T, E>`, `?` operator | ✅ | 5, 14, 33, 53 |
| `enum` (custom enums) | ✅ | 5, 14, 40, 51 |
| `impl` with generics and traits | ✅ | 4, 6, 28, 37, 51 |
| Arrays `[T; N]` | ✅ | 6, 10 |
| `HashMap` | ✅ | 6, 11, 15 |
| `HashSet` | ✅ | 23 |
| `BTreeMap` / `BTreeSet` | ✅ | 6, 11, 19 |
| `LinkedList` | ✅ | 12 |
| `VecDeque` | ✅ | 13 |
| `BinaryHeap` | ✅ | 18, 24 |
| `Box<T>`, `Rc<T>`, `Arc<T>` (smart pointers) | ✅ | 3, 34, 37, 51 |
| Lifetimes and borrow checker annotations | ✅ | 6, 38 |
| Stack vs heap memory | ✅ | 3 |
| Error handling with `Result` and custom error types | ✅ | 5, 14, 33, 53 |
| `thiserror` crate | ✅ | 5 |
| `TryFrom` / `TryInto` traits | ✅ | 5 |
| `mod`, `pub`, `use` (modules & visibility) | ✅ | 3, 33 |
| External crates beyond `rand` | ✅ | 5, 7, 14, 20, 33, 53 |
| File I/O (`std::fs`, `File`, `BufReader`) | ✅ | 53, 54 |
| CSV parsing / writing (`csv` crate) | ✅ | 53, 54 |
| Serde (serialisation / deserialisation) | ✅ | 54, 55 |
| Parquet / Arrow columnar format | ✅ | 55 |
| Testing (`#[test]`, `cargo test`) | ✅ | 1 |
| Documentation (`///`, `cargo doc`) | ✅ | 1 |
| `derive` macros (`Debug`, `Clone`, `Copy`, `PartialEq`, etc.) | ✅ | 4, 14, 33, 54 |
| Trait definitions, bounds, and orphan rule | ✅ | 4 |
| `Deref` / `Sized` / `From` / `Clone` / `Copy` / `Drop` traits | ✅ | 4 |
| Concurrency (`std::thread`, `mpsc`, `Mutex`, `Arc`) | ✅ | 7, 34, 46, 48, 50 |
| Scoped threads | ✅ | 7 |
| `mpsc` channels | ✅ | 7 |
| `RwLock` | ✅ | 7 |
| Interior mutability (`Cell`, `RefCell`) | ✅ | 7, 47 |
| `Send` / `Sync` marker traits | ✅ | 7, 46, 51 |
| `rayon` parallel iterators | ✅ | 40, 50 |
| Atomics & memory ordering | ✅ | 44 |
| `async` / `.await` basics | ✅ | 8 |
| `Future` trait & `tokio` runtime | ✅ | 8 |
| Spawning async tasks & cancellation | ✅ | 8 |
| Graph algorithms (`petgraph`, Dijkstra, PageRank, SCC) | ✅ | 20, 21, 24, 26 |
| `HashMap` iteration and entry API | ✅ | 11 |
| Pattern matching with `@` bindings, guards, etc. | ✅ | 0 |
| Package layout (`lib.rs` + `main.rs`) | ✅ | 33 |
| Library re‑exports (`pub use`) | ✅ | 33 |
| CLI argument parsing (`clap` derive) | ✅ | 14, 33, 40 |
| Safe vs unsafe Rust | ✅ | 35 |
| RAII / `Drop` trait / OBRM | ✅ | 3, 37 |
| Cryptographic hashes (`Digest` trait) | ✅ | 41 |
| Caesar cipher / frequency analysis | ✅ | 40 |
| Jupyter notebook / `evcxr` | ✅ | 58 |
| Pandas / DataFrame operations | ✅ | 57 |

---

## Pattern Matching: @ Bindings and Guards

This section covers the two missing pattern-matching features not yet introduced by any project: **`@` bindings** (binding a value to a name while destructuring) and **match guards** (additional `if` conditions on match arms).

### `@` bindings

The `@` syntax lets you test a value against a pattern *and* bind it to a variable at the same time:

```rust
let number = 42;
match number {
    n @ 0..=9     => println!("{n} is a single digit"),
    n @ 10..=99   => println!("{n} is two digits"),
    n @ 100..=999 => println!("{n} is three digits"),
    _             => println!("{number} is big"),
}
```

Without `@`, you'd need a separate `if` check after matching:

```rust
// Without @ — more verbose
match number {
    0..=9 => {
        let n = number; // re-bind manually
        println!("{n} is a single digit");
    }
    // ...
}
```

`@` is especially useful with nested enums and structs:

```rust
enum Message {
    Hello { name: String },
}

let msg = Message::Hello { name: "Alice".into() };
match msg {
    Message::Hello { name: n @ "Alice" } => println!("Hi Alice!"),
    Message::Hello { name } => println!("Hello, {name}"),
}
```

### Match guards

A **guard** is an `if` condition attached to a match arm. The arm matches only if the condition is true:

```rust
let pair = (10, 5);
match pair {
    (x, y) if x > y => println!("{x} wins (>{y})"),
    (x, y) if x < y => println!("{y} wins (>{x})"),
    (x, y)          => println!("tie at {x}"),
}
```

Guards can reference the variables bound in the pattern. They can also be combined with `@`:

```rust
let num = 42;
match num {
    n @ 0..=50 if n % 2 == 0 => println!("{n} is small and even"),
    n @ 0..=50               => println!("{n} is small and odd"),
    n                        => println!("{n} is large"),
}
```

### Exercise

Run the following in the Rust Playground or your local environment:

```rust
fn describe_point(point: (i32, i32)) -> &'static str {
    match point {
        (0, 0) => "origin",
        (x, y) if x == y => "on diagonal",
        (x, _) if x > 0 => "right half",
        (_, y) if y > 0 => "top half",
        _ => "somewhere else",
    }
}

fn main() {
    println!("{}", describe_point((3, 3))); // on diagonal
    println!("{}", describe_point((5, -2))); // right half
}
```

Both features are now ✅ covered in this course. See the [Concepts Coverage](#rust-concepts-coverage) table for the full list.

---

## Credits

This course is built from two excellent open-source Rust resources:

- **[data-engineering-rust](https://github.com/jolisper/data-engineering-rust)** by [Jorge López](https://github.com/jolisper) — the hands-on Cargo projects (9–58) teaching Rust for data engineering through practical examples (collections, graphs, concurrency, file I/O, etc.).
- **[100-exercises-to-learn-rust](https://github.com/mainmatter/100-exercises-to-learn-rust)** by [Mainmatter](https://mainmatter.com) — the tutorial exercises (0–8) teaching Rust fundamentals through structured, progressive exercises.

The original content, structure, and teaching design belong to their respective authors. This repository reorganizes and sequences the material into a single progressive curriculum. Huge thanks to both projects for their excellent work.

---

## License

MIT
