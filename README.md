# Rust Tutorial — Learn by Doing

A comprehensive, progressive Rust course built from two high-quality sources. It combines **tutorial-based exercises** (projects 0–8) with **hands-on Cargo projects** (projects 9–58), walking you from absolute beginner to advanced Rust.

## How It Works

```
RustTut/
├── README.md                        ← this file
├── AGENTS.md                        ← AI workshop designer instructions
├── .devcontainer/                   ← preconfigured Rust dev environment
├── 0-Intro/                         ← syntax primer (reference only)
├── 1-MasterMind/ … 8-Futures/       ← tutorial exercises (read .md, write Rust)
├── 9-VectorFruitSalad/ … 58-RustJupyterNotebook/  ← hands-on Cargo projects
```

The course has **two tracks** that work together:

**Track 1 — Tutorial Exercises (projects 0–8):**  
Each folder contains `.md` files that teach concepts step by step with exercises. You read the markdown and write Rust code as you go. Start here to build a strong foundation.

**Track 2 — Hands-on Projects (projects 9–58):**  
Each folder is a complete Cargo project (`Cargo.toml` + `src/`). You build, run, and experiment with working Rust programs that apply concepts from Track 1 in realistic scenarios.

**Progression:** Go through projects in numerical order. Track 1 (0–8) introduces each concept; Track 2 (9–58) deepens and applies it. Concepts once taught are assumed in later projects.

## Course Progression

The course is carefully sequenced so each concept is introduced by one source and deepened by the other, never repeated as a first-time lesson.

| Concept Cluster | Introduced In | Deepened / Applied In |
|----------------|---------------|----------------------|
| Variables, types, arithmetic, control flow | 0–2 (Intro, MasterMind, BasicCalculator) | 9–13 (collections fruit-salad projects) |
| Structs, ownership, references, borrowing | 1, 3 (MasterMind, TicketV1) | 33, 36–38 (CLI, MutableFruitSalad, OBRM, OwnershipLifetimes) |
| Traits, derive, Clone/Copy/Drop | 4 (Traits) | 14, 28, 33, 35, 37 (CLISalad, RustIterators, CustomCLI, SafeAndUnsafe, OBRM) |
| Enums, match, error handling, thiserror | 5 (TicketV2) | 14, 40, 51 (CLISalad, DecoderRing, SendSync) |
| Collections (Vec, arrays, HashMap, BTreeMap, iterators) | 6 (TicketManagement) | 9–13, 15–19, 23, 28, 30 (fruit-salad series, HashSet, BTreeSet, iterators) |
| Lifetimes, impl Trait, slices | 6 (TicketManagement) | 38 (OwnershipLifetimes) |
| Threads, channels, locks, Send/Sync | 7 (Threads) | 34, 46–52 (concurrency projects: DataRace, ConcurrencyParallelism, etc.) |
| Atomics, memory ordering, Rayon | 7, 8 (Threads, Futures) | 44, 50 (Atomics, RayonChallenge) |
| Concurrency design patterns | 7 (Threads) | 41, 45, 48 (DiningPhilosophers, DistributedChallenges, etc.) |
| Async/await, Future trait, tokio | 8 (Futures) | — (capstone concept) |
| File I/O, CSV, serde, Parquet | — | 53–55 (CSVCookbook, CSVWriter, Parquet) |
| Graph algorithms (petgraph, Dijkstra, PageRank) | — | 20–22, 24–27, 29, 31 (graph projects) |
| Crypto, security, unsafe Rust | — | 35, 39–43 (safe/unsafe, decoder ring, crypto hashes, software security) |
| Data Analysis (Pandas, Jupyter, evcxr) | — | 57, 58 (ExploringPandas, RustJupyterNotebook) |

Projects 9–58 are **Cargo projects** — each is a real, runnable Rust program that applies one or more previously-introduced concepts. Projects 2–8 are **tutorial exercises** (`.md` files with embedded Rust exercises). Use the tutorial projects to learn, then the Cargo projects to practice.

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

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 0 | **Intro** — Rust syntax primer (not a workshop) | `fn main()`, `let`, `mut`, macros (`println!`), basic types, `&str`, arithmetic, closures basics |
| 1 | **MasterMind** — guess a 4-digit secret code with hints | `struct`, `impl`, `Vec<T>`, `Option<T>`, `if let`, loops, `String`/`&str`, `rand` crate, iterators, `cargo` basics |
| 2 | **BasicCalculator** — integers, branching, loops, overflow | i32/u32, `if`/`else`, `while`/`for`, panics, overflow, saturating arithmetic, `as` casting |
| 3 | **TicketV1** — structs, ownership, stack vs heap | `struct`, validation, `mod`/`pub`, encapsulation, ownership, setters, stack/heap, destructors (`Drop`) |
| 4 | **Traits** — trait definitions, derive, bounds, deref | `trait`, orphan rule, operator overloading, `derive`, trait bounds, `Deref`, `Sized`, `From`, `Clone`/`Copy` |
| 5 | **TicketV2** — enums, match, error handling, thiserror | `enum`, `match`, `if let`, `Option`, `Result`, error enums, `Error` trait, `thiserror`, `TryFrom`, packages/deps |
| 6 | **TicketManagement** — arrays, vectors, lifetimes, HashMap | Arrays `[T;N]`, `Vec`, resizing, iterators, lifetimes (`'a`), combinators, `impl Trait`, slices, `Index`, `HashMap`, `BTreeMap` |
| 7 | **Threads** — threads, channels, locks, concurrency | `std::thread`, `'static`, scoped threads, `mpsc` channels, interior mutability, `Mutex`/`Arc`, `RwLock`, `Sync` |
| 8 | **Futures** — async/await, tasks, runtimes | `async fn`, `.await`, `tokio`, spawning tasks, `Future` trait, blocking, cancellation |
| 9 | **VectorFruitSalad** — random fruit salad with dynamic arrays | `Vec<T>`, `SliceRandom`, `rand`, iteration, `&str`, mutable references |
| 10 | **ArrayFruitSalad** — fixed-size arrays vs dynamic vectors | Arrays `[T;N]`, `Vec`/`VecDeque`/`LinkedList` comparison, array-vec conversion |
| 11 | **HashMapCount** — frequency counting with hash maps | `HashMap`, `entry`/`or_insert`, `BTreeMap`, sorting by value |
| 12 | **LinkedListFruitSalad** — doubly-linked list fruit salad | `LinkedList`, memory overhead, conversion between collections |
| 13 | **VecDequeFruitSalad** — double-ended queue fruit salad | `VecDeque`, ring buffer, `push_front`/`push_back`, performance |
| 14 | **CLISalad** — CLI fruit salad with clap argument parsing | `clap` derive, `std::env`, pattern matching, `std::io`, error handling |
| 15 | **HashMapLanguage** — programming language stats with HashMap | `HashMap` with complex data, `values_mut`, normalization, type casting |
| 16 | **CollectionsLessonReflection** — comparing Rust collections | Collection trade-offs, big-O, memory efficiency, selection guide |
| 17 | **RustCollectionsDoc** — comprehensive collections reference | All `std::collections`, performance characteristics, `criterion` benchmarks |
| 18 | **BinaryHeapFruit** — max-heap priority queue with fruit | `BinaryHeap`, priority queue, max-heap behavior |
| 19 | **BTreeSetFruit** — ordered set with fruit | `BTreeSet`, ordered iteration, `HashSet` vs `BTreeSet` complexity |
| 20 | **CommunityDetection** — Kosaraju's algorithm on Twitter graph | `petgraph`, directed graphs, Kosaraju SCC, DFS, graph transposition |
| 21 | **UFCGraphCentrality** — fighter centrality on UFC graph | `UnGraph`, degree/closeness centrality, `NodeIndex`, adjacency |
| 22 | **GraphVisualize** — ASCII bar charts from data | `rasciigraph`, ASCII visualization, data scaling |
| 23 | **HashSetFruit** — unique random fruits with HashSet | `HashSet`, uniqueness, membership testing |
| 24 | **LisbonShortestPath** — Dijkstra on Lisbon landmarks | Dijkstra's algorithm, weighted graphs, `BinaryHeap` as priority queue, `petgraph` |
| 25 | **Neo4jDataScience** — Neo4j graph DB centrality algorithms | Neo4j integration, centrality algorithms (degree, closeness, betweenness, eigenvector) |
| 26 | **PageRank** — PageRank algorithm on graphs | PageRank, iterative ranking, damping factor, link analysis |
| 27 | **RussianTrollTweets** — Neo4j analysis of troll tweet data | Graph DB analysis, influence detection, misinformation, social graph modeling |
| 28 | **RustIterators** — lazy functional iteration | `Iterator` trait, lazy evaluation, `map`/`filter`/`fold`, zero-cost abstraction |
| 29 | **DataStructuresLessonReflection** — when to use which structure | Graph vs other DS, centrality metrics, community detection insights |
| 30 | **WhenToUseRustSet** — collection selection guide | `Vec`/`VecDeque`/`LinkedList`/`HashMap`/`BTreeMap`/`HashSet`/`BTreeSet` comparison |
| 31 | **FullyConnectedGraph** — check if undirected graph is fully connected | Graph connectivity, `HashMap` memoization, adjacency checks |
| 32 | **Week1FinalReflection** — data structures in data engineering | Memory safety, concurrency, zero-cost abstractions, generics/traits, Rust ecosystem |
| 33 | **CustomCLIFruitSalad** — extended CLI with CSV input & lib separation | `clap` derive, CSV reading (`csv` crate), `lib.rs`/`main.rs` separation, modules |
| 34 | **DataRace** — preventing data races with Mutex and Arc | `Mutex`, `Arc`, data race prevention, `MutexGuard`, shared-state concurrency |
| 35 | **SafeAndUnsafe** — safe vs unsafe Rust | Safe Rust, `unsafe` keyword, raw pointers, FFI, safety invariants |
| 36 | **MutableFruitSalad** — Vec mutation methods | `Vec` mutation (`push`/`pop`/`insert`/`remove`), capacity vs length, reallocation |
| 37 | **OBRM** — ownership-based resource management | Ownership rules, `Drop` trait, RAII, borrowing, resource lifecycle |
| 38 | **OwnershipLifetimes** — ownership, borrowing, and lifetime annotations | Ownership (move semantics), borrowing (`&T`/`&mut T`), lifetimes (`'a`), `Copy`/`Clone` |
| 39 | **SafetyLessonReflection** — Rust safety vs garbage-collected languages | Memory safety, data race prevention, no undefined behavior, explicit resource management |
| 40 | **DecoderRing** — crack Caesar cipher with frequency analysis | Caesar cipher, frequency analysis, statistical scoring, `clap` CLI, `rayon` parallelism |
| 41 | **RustCryptoHashes** — cryptographic hash ecosystem | SHA-2/3, BLAKE2, `Digest` trait, RustCrypto project |
| 42 | **RustSoftwareSecurity** — Rust security model vs C/C++/Java | Ownership/borrowing safety, unsafe limitations, compile-time vs runtime safety |
| 43 | **SecurityLessonReflection** — high-availability security | Redundancy, encryption, access control, intrusion detection, disaster recovery |
| 44 | **Atomics** — atomic operations and memory ordering | Atomic types, memory ordering (`Relaxed`, `Acquire`, `Release`, `AcqRel`, `SeqCst`), lock-free programming |
| 45 | **DistributedChallenges** — consistency in distributed systems | Eventual vs strong consistency, CAP theorem, conflict resolution |
| 46 | **ConcurrencyParallelism** — concurrency vs parallelism in Rust | `Send`/`Sync` traits, `Mutex`, `RwLock`, `Arc`, compile-time safety |
| 47 | **DataRacesRaceConditions** — data races vs race conditions | Data races, race conditions, `Send`/`Sync`, interior mutability (`Cell`/`RefCell`) |
| 48 | **DiningPhilosophers** — classic concurrency problem | `Mutex`, deadlock prevention, ordered lock acquisition, thread synchronization |
| 49 | **DistributedComputing** — Rust for distributed computing | GC overhead, dynamic typing costs, compiled vs interpreted, distributed system challenges |
| 50 | **RayonChallenge** — parallel computation with Rayon | `rayon` parallel iterators, benchmarking (speedup), work stealing, hyperparameter tuning |
| 51 | **SendSync** — Send and Sync marker traits | `Send` trait, `Sync` trait, thread safety markers, compiler auto-impl, unsafe `unsafe impl` |
| 52 | **ConcurrencyLessonReflection** — Rust's concurrency guarantees | Ownership + concurrency, lifetime annotations, zero-cost abstractions, data-race freedom, `mpsc` channels |
| 53 | **CSVCookbook** — CSV reading, writing, and transformation | `csv` crate, deserialization, record iteration, error handling, data transformation |
| 54 | **CSVWriter** — programmatic CSV writing | `csv::Writer`, custom delimiters, headers, `serde` (`Deserialize`/`Serialize`), file I/O |
| 55 | **Parquet** — Apache Parquet columnar storage with Rust | Parquet format, columnar storage, Arrow integration, schema handling |
| 56 | **DataManagementLessonReflection** — data/file management in Rust | File I/O, data serialization, columnar vs row-oriented storage, performance |
| 57 | **ExploringPandas** — bridging Rust and Python data analysis | DataFrame operations, Python/Rust interop, filtering/grouping/joining, pandas/matplotlib/sklearn |
| 58 | **RustJupyterNotebook** — interactive Rust with Jupyter & evcxr | `evcxr` Jupyter kernel, interactive programming, notebook integration, `plotters`/`ndarray`/`rayon` |

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
