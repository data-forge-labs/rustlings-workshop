# Rust Tutorial — Learn by Doing

A comprehensive, progressive Rust course for Python data engineers. It combines **tutorial-based exercises** (Section 1 projects) with **hands-on Cargo projects** (Sections 2–10), walking you from absolute beginner to productive Rust developer.

## How It Works

```
RustTut/
├── README.md                        ← this file
├── AGENTS.md                        ← AI workshop designer instructions
├── .devcontainer/                   ← preconfigured Rust dev environment
├── 01-Foundations/                  ← Section 1 (projects 01-04)
├── 02-Ownership/                    ← Section 2 (projects 01-06)
├── 03-Collections/                  ← Section 3 (projects 01-13)
├── 04-FileIO/                       ← Section 4 (projects 01-03)
├── 05-Concurrency/                  ← Section 5 (projects 01-11)
├── 06-CLIAndTools/                  ← Section 6 (projects 01-10)
├── 07-Security/                     ← Section 7 (projects 01-03)
├── 08-Interop/                      ← Section 8 (projects 01-02)
├── 09-ProductionSystems/           ← Section 9 (projects 01-02)
├── 10-ToolsAndFrameworks/          ← Section 10 (projects 01-03)
├── 11-Reference/                    ← Section 11 (reference material)
└── 12-DataEngAnalytics/             ← Section 12 (projects 01-03) — Polars, DuckDB, DataFusion
└── 13-ActorModel/                   ← Section 13 (projects 01-03) — DIY actor, ractor, ETL pipeline
```

The course is organized into **12 sections** designed for a Python data engineer moving to Rust. Each section starts with tutorial-style projects (read `.md` files, write code alongside) and progresses to hands-on Cargo projects (build and run complete programs).

**Progression:** Go through sections in order. Within each section, start with lower-numbered projects (introduce concepts) then move to higher-numbered ones (apply and deepen). Concepts from earlier sections are assumed in later ones.

## Course Progression

The course is carefully sequenced so each concept is introduced by one source and deepened by the other, never repeated as a first-time lesson.

> **Test-driven learning**: Every project has a `Cargo.toml` + `src/lib.rs` with progressive unit tests. Each function starts as a `todo!()` stub. As you follow the project's README, replace `todo!()` with real code. Run `cargo test` after each section to see your pass count grow. When all tests pass, you've completed the project.

| Section | Concept Cluster | Projects |
|---------|----------------|----------|
| 1 — Foundations | Syntax, types, control flow, basic I/O, console games | 01 (Intro), 02 (GuessGame), 03 (BasicCalculator), 04 (MasterMind) |
| 2 — Ownership | Structs, ownership, borrowing, lifetimes, traits, enums, error handling | 01 (TicketV1), 02 (Traits), 03 (TicketV2), 04 (OBRM), 05 (OwnershipLifetimes), 06 (ConversionErrorHandling) |
| 3 — Collections | Vec, arrays, HashMap, HashSet, BTreeMap, iterators, LinkedList, VecDeque, BinaryHeap | 01 (TicketManagement), 02–13 (Fruit Salad series, HashMap, iterators) |
| 4 — File I/O | CSV reading/writing, Parquet, Arrow, YAML, JSON/NDJSON, serde, file I/O | 01 (CSVCookbook), 02 (CSVWriter), 03 (Parquet), 04 (Arrow), 05 (YAML), 06 (JsonStream) |
| 5 — Concurrency | Threads, async/await, Mutex, Arc, Send/Sync, Rayon, atomics, channels | 01 (Threads), 02 (Futures), 03–11 (DataRace, Atomics, DiningPhilosophers, Rayon, etc.), 12 (AdvancedSync), 13 (AsyncPatterns) |
| 6 — CLI & Tools | clap, petgraph, Dijkstra, PageRank, Neo4j | 01–10 (CLISalad, CommunityDetection, PageRank, GraphVisualize, etc.) |
| 7 — Security | Safe vs unsafe, crypto, security model | 01 (SafeAndUnsafe), 02 (DecoderRing), 03 (RustCryptoHashes) |
| 8 — Interop | evcxr, Jupyter, pandas bridge | 01 (ExploringPandas), 02 (RustJupyterNotebook), 03 (PyO3Bindings), 04 (GILRelease) |
| 9 — Production Systems | Tokio, async, TCP, RESP protocol, Axum web API | 01 (Radish), 02 (AxumShop), 03 (AxumAuth), 04 (OpenTelemetry) |
| 10 — Tools & Frameworks | Logging, configuration management, testing frameworks | 01 (Logging), 02 (Configuration), 03 (Testing), 04 (Proptest), 05 (Mockall), 06 (Insta) |
| 11 — Reference | Quick concept lookup, cheatsheets, memory architecture | (no cargo projects — reference materials only) |
| 12 — DataEng Analytics | Polars DataFrame, DuckDB in-process OLAP, Apache DataFusion query engine | 01 (Polars), 02 (DuckDB), 03 (DataFusion) |
| 13 — Actor Model | DIY actor with `mpsc` + `oneshot`, `ractor` production crate, ETL pipeline composition | 01 (DIY-Actor), 02 (Ractor), 03 (ETLPipeline) |

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
| 01 | **Intro** — Rust syntax primer (reference) | `fn main()`, `let`, `mut`, macros (`println!`), basic types, `&str`, tuples, arrays `[T;N]`, `if`/`else`, loops |
| 02 | **GuessGame** — interactive "guess the number" game | `String` vs `&str`, custom `enum`, `derive`, `read_line`, `Result<T, E>`, `.parse()`, `.expect()`, basic `match`, `?` operator, external crates (`rand`) |
| 03 | **BasicCalculator** — integers, arithmetic, loops, overflow | `i32`/`u32`/`u64`/`usize`, `while`/`for`, `panic!`, integer overflow, saturating/wrapping arithmetic, `as` casting, `#[test]`, `#[should_panic]` |
| 04 | **MasterMind** — guess a 4-digit secret code with hints | `struct`, `impl`, `Vec<T>`, `Option<T>`, exhaustive `match`, `if let`, `String`/`&str` (deeper), `rand`, iterators, console I/O |

### Section 2: Ownership — Rust's Superpower over Python's GC

*The biggest mindset shift: how Rust manages memory without a garbage collector, and how it prevents whole classes of bugs at compile time.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 01 | **TicketV1** — structs, ownership, stack vs heap | `struct`, validation, `mod`/`pub`, encapsulation, ownership, setters, stack/heap, `Drop` recap (see OBRM) |
| 02 | **Traits** — trait definitions, derive, bounds | `trait`, orphan rule, operator overloading, `derive`, trait bounds, `Deref`, `Sized`, `From`, `Drop` recap (see OBRM) |
| 03 | **TicketV2** — enums, match, error handling | `enum`, `match`, `if let`, `Option`, `Result`, error enums, `Error` trait, `thiserror`, `TryFrom` |
| 04 | **OBRM** — ownership-based resource management | **Canonical** `Drop` trait, RAII, resource lifecycle; ownership/borrowing recap (see TicketV1) |
| 05 | **OwnershipLifetimes** — lifetimes & borrow checker | Lifetimes (`'a`), lifetime elision, struct lifetimes; move/copy recap (see TicketV1) |
| 06 | **ConversionErrorHandling** — `unwrap`, `?`, `From`, and the whole family | `Option::unwrap_or[_default]`, `Option::map_or`, `Option::ok_or`, `Result::map_err`, `Result::and_then`, `?` operator, `From<E1>` impl, `thiserror` |

### Section 3: Collections — Faster Than Python Lists & Dicts

*Python lists and dicts are great. Rust's Vec and HashMap remove the interpreter overhead. Plus: sets, queues, heaps, and functional iterators.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 01 | **TicketManagement** — Vec, arrays, HashMap, BTreeMap | `Vec` recap, arrays `[T;N]`, iterators, lifetimes, `impl Trait`, slices recap, `HashMap`, `BTreeMap`, `Index` |
| 02 | **VectorFruitSalad** — dynamic arrays with Vec | `Vec<T>`, `SliceRandom`, `rand`, iteration, `&str`, mutable refs |
| 03 | **ArrayFruitSalad** — fixed-size vs dynamic arrays | Arrays `[T;N]`, `Vec`/`VecDeque`/`LinkedList` comparison |
| 04 | **HashMapCount** — frequency counting | `HashMap`, `entry`/`or_insert`, `BTreeMap`, sorting by value |
| 05 | **LinkedListFruitSalad** — doubly-linked list | `LinkedList`, memory overhead, collection conversion |
| 06 | **VecDequeFruitSalad** — double-ended queue | `VecDeque`, ring buffer, `push_front`/`push_back` |
| 07 | **HashMapLanguage** — complex HashMap data | `HashMap` with complex values, `values_mut`, normalization |
| 08 | **RustCollectionsDoc** — reference document | All `std::collections`, `criterion` benchmarks |
| 09 | **BinaryHeapFruit** — priority queue | `BinaryHeap`, max-heap, priority queue behavior |
| 10 | **BTreeSetFruit** — ordered set | `BTreeSet`, ordered iteration, `HashSet` vs `BTreeSet` |
| 11 | **HashSetFruit** — unique items with HashSet | `HashSet`, uniqueness, membership testing |
| 12 | **RustIterators** — lazy functional iteration | `Iterator` trait, lazy eval, `map`/`filter`/`fold` |
| 13 | **MutableFruitSalad** — Vec mutation | `push`/`pop`/`insert`/`remove`, capacity vs length |

### Section 4: File I/O — CSV & Parquet at Scale

*Python's pandas reads CSVs. Rust's csv and parquet crates do it faster, with less memory.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 01 | **CSVCookbook** — read, write, transform CSV | `csv` crate, deserialization, record iteration, error handling |
| 02 | **CSVWriter** — programmatic CSV writing | `csv::Writer`, custom delimiters, `serde` (`Deserialize`/`Serialize`) |
| 03 | **Parquet** — Apache Parquet columnar format | Parquet format, columnar storage, Arrow integration, projection pushdown, statistics, schema evolution |
| 04 | **Arrow** — Apache Arrow in-memory columnar format | `arrow` crate, `RecordBatch`, primitive arrays, builders, schema, CSV→Arrow, IPC, `compute` kernels |
| 05 | **YAML** — pipeline configuration files | `serde_yaml`, `#[derive(Deserialize)]`, custom enums, `rename_all`, config merge, NDJSON-style queries |
| 06 | **JsonStream** — JSON & NDJSON streaming | `serde_json`, typed `Value` walking, NDJSON read/write, pretty-print, object merge |

### Section 5: Concurrency — Beyond Python's GIL

*Python threads are limited by the GIL. Rust gives you true parallelism with compile-time safety guarantees.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 01 | **Threads** — threads, channels, locks | `std::thread`, `'static`, scoped threads, `mpsc`, interior mutability, `Mutex`/`Arc`, `RwLock`, `Sync` |
| 02 | **Futures** — async/await, tasks, runtimes | `async fn`, `.await`, `tokio`, `Future` trait, spawning, cancellation |
| 03 | **DataRace** — preventing data races | `Mutex`, `Arc`, `MutexGuard`, shared-state concurrency |
| 04 | **Atomics** — lock-free atomics | Atomic types, memory ordering (`Relaxed`, `Acquire`, `Release`, `SeqCst`) |
| 05 | **DistributedChallenges** — consistency in distributed systems | Eventual vs strong consistency, CAP theorem |
| 06 | **ConcurrencyParallelism** — Send/Sync, RwLock | `Send`/`Sync` traits, `Mutex`, `RwLock`, `Arc` |
| 07 | **DataRacesRaceConditions** — data races vs race conditions | Data races, race conditions, `Cell`/`RefCell` |
| 08 | **DiningPhilosophers** — deadlock prevention | `Mutex`, ordered lock acquisition, thread synchronization |
| 09 | **DistributedComputing** — Rust for distributed systems | GC overhead, compiled vs interpreted, distributed challenges |
| 10 | **RayonChallenge** — data parallelism with Rayon | `rayon` parallel iterators, speedup benchmarking |
| 11 | **SendSync** — Send and Sync marker traits | `Send`, `Sync`, thread safety markers, `unsafe impl` |
| 12 | **AdvancedSync** — High-performance concurrency primitives | `parking_lot::Mutex`/`RwLock`, `crossbeam_channel`, `arc_swap` (lock-free), `triomphe::Arc` |
| 13 | **AsyncPatterns** — Real-world Tokio patterns | `tokio::select!`, `tokio::time::timeout`, `Semaphore`, `Notify`, `JoinSet`, bounded `mpsc`, `CancellationToken` (tokio-util) |

### Section 6: CLI & Data Engineering Tools

*Building production-ready CLI tools, graph analytics, and connecting Rust to Python data tools.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 01 | **CLISalad** — CLI with clap arg parsing | `clap` derive, `std::env`, pattern matching, `std::io` |
| 02 | **CommunityDetection** — Kosaraju's SCC algorithm | `petgraph`, directed graphs, SCC, DFS, graph transposition |
| 03 | **UFCGraphCentrality** — centrality on UFC data | `UnGraph`, degree/closeness centrality, `NodeIndex` |
| 04 | **GraphVisualize** — ASCII bar charts | `rasciigraph`, ASCII visualization, data scaling |
| 05 | **LisbonShortestPath** — Dijkstra's algorithm | Dijkstra, weighted graphs, `BinaryHeap` as priority queue |
| 06 | **Neo4jDataScience** — Neo4j graph DB | Neo4j integration, centrality algorithms (degree, closeness, betweenness, eigenvector) |
| 07 | **PageRank** — PageRank algorithm | PageRank, iterative ranking, damping factor, link analysis |
| 08 | **RussianTrollTweets** — Neo4j analysis | Graph DB analysis, influence detection, social graph modeling |
| 09 | **FullyConnectedGraph** — graph connectivity | Graph connectivity, `HashMap` memoization |
| 10 | **CustomCLIFruitSalad** — advanced CLI + CSV | `clap` derive, CSV reading, `lib.rs`/`main.rs` separation, modules |
| 11 | **RatatuiTUI** — terminal dashboard | `ratatui`, `crossterm`, `TestBackend`, immediate-mode UI, layouts, widgets, event loop |
| 12 | **AsyncClap** — async CLI with subcommands | `clap` derive, `#[tokio::main]`, `ExitCode`, subcommand trees, JSON config |

### Section 7: Security & Systems Programming

*Why Rust is the safe alternative to C/C++ for data pipelines, and how cryptography fits in.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 01 | **SafeAndUnsafe** — safe vs unsafe Rust | `unsafe` keyword, raw pointers, FFI, safety invariants |
| 02 | **DecoderRing** — crack Caesar cipher | Frequency analysis, statistical scoring, `rayon` parallelism |
| 03 | **RustCryptoHashes** — cryptographic hashes | SHA-2/3, BLAKE2, `Digest` trait, RustCrypto |
| 04 | **Argon2** — password hashing | `argon2`, `SaltString`, `PasswordHasher`/`PasswordVerifier`, `subtle::ConstantTimeEq` |
| 05 | **Ed25519** — digital signatures | `ed25519-dalek`, `SigningKey`/`VerifyingKey`, hex serialization, tamper detection |
| 06 | **RustlsTLS** — TLS server & client | `rustls` + `aws-lc-rs`, `ServerConfig`/`ClientConfig`, `tokio-rustls` handshake |

### Section 8: Rust + Python Interop

*Bridge the two worlds: use Rust from Jupyter notebooks, and call pandas from Rust.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 01 | **ExploringPandas** — Rust meets pandas | DataFrame operations, Python/Rust interop, filtering/grouping, matplotlib |
| 02 | **RustJupyterNotebook** — interactive Rust with evcxr | `evcxr` Jupyter kernel, interactive Rust, `plotters`/`ndarray`/`rayon` |
| 03 | **PyO3Bindings** — Call Rust from Python | `pyo3 0.23`, `#[pyfunction]` / `#[pymodule]`, `cdylib`, `maturin develop`, feature-gated FFI |
| 04 | **GILRelease** — Free the GIL, free the CPU | `pyo3::Python::allow_threads`, GIL contention factor, multi-threaded CPU work |

### Section 9: Production Systems — Building Real-World Services

*Production-grade Rust: building networked services, async I/O, wire protocols, and in-memory data stores.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 01 | **Radish** — Redis-compatible KV store | `tokio` async, RESP protocol, TCP networking, `Rc<RefCell>`, `BytesMut`, TTL expiry |
| 02 | **AxumShop** — Shop Manager API with Axum | `axum::Router`, `tokio` async, `sqlx` async DB, `serde` JSON, `tower-http` CORS, `tower-sessions`, `FromRequestParts` auth, SHA-256 hashing, DB transactions |
| 03 | **AxumAuth** — JWT + Bearer middleware for Axum 0.8 | `jsonwebtoken` 9, HS256 sign/verify, typed `Claims`, role-based access, refresh tokens, `kid` header inspection |
| 04 | **OpenTelemetry** — Traces, spans, and correlation IDs | `tracing` 0.1, `tracing-subscriber` JSON output, OTel attribute model, `AtomicU64` pipeline metrics, `Uuid` correlation ids |

### Section 10: Tools & Frameworks

*Logging, configuration management, and testing frameworks — the tools you need for production Rust applications.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 01 | **Logging** — structured logging with the `log` crate | `log` crate, `env_logger`, log levels, structured logging |
| 02 | **Configuration** — manage app configuration | `config` crate, environment variables, TOML/YAML config files |
| 03 | **Testing** — testing strategies and frameworks | `#[test]`, test organization, integration tests, mocking, property-based testing |
| 04 | **Proptest** — property-based testing | `proptest` 1, strategies, random sampling, shrinking, invariants |
| 05 | **Mockall** — mocking traits for testable pipelines | `mockall` 0.13, `#[automock]`, `&dyn Trait`, predicate matchers, error simulation |
| 06 | **Insta** — snapshot testing | `insta` 1, inline snapshots, `cargo insta review`, struct Debug snapshots |

### Section 11: Reference

*Quick reference materials for concept lookup — no cargo projects, just cheatsheets and reference documents.*

This section contains reference documents for quick lookup of Rust syntax, idioms, and patterns covered across all 10 prior sections.

### Section 12: Data Engineering Analytics — Polars, DuckDB, DataFusion

*The three high-performance Rust OLAP engines, all built on Apache Arrow — production data engineering at scale.*

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 01 | **Polars** — single-node DataFrame library | `polars` DataFrame, `LazyFrame` query plans, `group_by`, Parquet I/O, predicate pushdown |
| 02 | **DuckDB** — in-process OLAP database | `duckdb` crate, DDL/DML, prepared statements, `read_csv_auto`, raw SQL execution |
| 03 | **DataFusion** — Apache query engine | `SessionContext`, async SQL, CSV registration, Arrow zero-copy, Parquet write |

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
cd <section>/<project-number-ProjectName>
cargo run
```

### Option 2 — Local Rust Installation

```bash
# Build and run any project with cargo
cd 03-Collections/02-VectorFruitSalad
cargo build
cargo run

# Or use the project's Makefile (if present)
make run
```

### Option 3 — Compile directly with rustc

```bash
# Some projects can be compiled without cargo
cd 01-Foundations/02-GuessGame/workshop
cargo run
```

Browse the full list of projects in the [Projects](#projects) table above.

---

## Rust Concepts Coverage

The table below lists all core Rust concepts a learner should eventually see. **Checked items** are already introduced in at least one existing workshop.

| Concept | Covered? | First Project |
|---------|----------|---------------|
| `cargo new`, `cargo build`, `cargo run` | ✅ | 01-02 |
| `Cargo.toml` dependencies | ✅ | 01-02, 02-03 |
| Variables (`let`, `let mut`) | ✅ | 01-01, 01-02 |
| Data types (`u32`, `i32`, `f64`, `bool`, `char`, `usize`, `u8`) | ✅ | 01-01, 01-02, 01-03 |
| `String` vs `&str` | ✅ | 01-02, 02-02 |
| Ownership, borrowing, references (`&`, `&mut`) | ✅ | 01-02, 02-01, 02-05 |
| `Vec<T>`, `vec![]` | ✅ | 01-02, 03-01, 03-02 |
| `struct`, `impl`, methods (`&self`, `&mut self`) | ✅ | 01-02, 02-01 |
| `Option<T>`, `Some`, `None`, `if let` | ✅ | 01-03, 02-03 |
| `match` (basic) | ✅ | 01-03, 02-03 |
| `match` with patterns (advanced) | ✅ | 02-03, 06-01, 05-11 |
| `loop`, `while`, `continue`, `break` | ✅ | 01-02, 01-03 |
| `const` | ✅ | 01-02 |
| `if` / `else` branching | ✅ | 01-03 |
| Integer overflow & saturating arithmetic | ✅ | 01-02 |
| Iterators (`iter`, `map`, `filter`, `count`, `collect`, `zip`, `enumerate`, `any`, `all`) | ✅ | 01-02, 03-01, 03-12 |
| Closures (`\|x\| x * 2`) | ✅ | 01-02, 03-12 |
| `print!`, `println!` | ✅ | 01-02 |
| `std::io::stdin()`, `read_line()` | ✅ | 01-02 |
| `io::stdout().flush()` | ✅ | 01-02 |
| String methods (`chars`, `trim`, `to_lowercase`, `is_ascii_digit`, `to_digit`) | ✅ | 01-02 |
| Ranges (`0..=9`) | ✅ | 01-02, 01-03 |
| `rand` crate (`rng`, `shuffle`, `choose`) | ✅ | 01-02, 03-02 |
| Type casting (`as`) | ✅ | 01-02, 01-03 |
| Tuples (creation, destructuring, return values) | ✅ | 01-01, 01-02 |
| `unwrap()` / basic error handling | ✅ | 01-02, 02-03 |
| `Result<T, E>`, `?` operator | ✅ | 02-03, 06-01, 06-10, 04-01 |
| `enum` (custom enums) | ✅ | 02-03, 06-01, 07-02, 05-11 |
| `impl` with generics and traits | ✅ | 02-02, 03-01, 03-12, 02-04, 05-11 |
| Arrays `[T; N]` | ✅ | 01-01, 03-01, 03-03 |
| Slices `&[T]`, `&str` (borrowed views) | ✅ | 01-01, 03-01 |
| `HashMap` | ✅ | 03-01, 03-04, 03-07 |
| `HashSet` | ✅ | 03-11 |
| `BTreeMap` / `BTreeSet` | ✅ | 03-01, 03-04, 03-10 |
| `LinkedList` | ✅ | 03-05 |
| `VecDeque` | ✅ | 03-06 |
| `BinaryHeap` | ✅ | 03-09, 06-05 |
| `Box<T>`, `Rc<T>`, `Arc<T>` (smart pointers) | ✅ | 02-01, 05-03, 02-04, 05-11 |
| Lifetimes and borrow checker annotations | ✅ | 03-01, 02-05 |
| Stack vs heap memory | ✅ | 02-01 |
| Error handling with `Result` and custom error types | ✅ | 02-03, 06-01, 06-10, 04-01 |
| `thiserror` crate | ✅ | 02-03 |
| `TryFrom` / `TryInto` traits | ✅ | 02-03 |
| `Option` combinators (`unwrap_or[_default]`, `map_or`, `ok_or`, `filter`, `or_else`, `transpose`) | ✅ | 02-06 |
| `Result` combinators (`map_err`, `and_then`, `or_else`, `?` operator) | ✅ | 02-06 |
| `From<E1>` for error conversion + `thiserror` | ✅ | 02-06 |
| `mod`, `pub`, `use` (modules & visibility) | ✅ | 02-01, 06-10 |
| External crates beyond `rand` | ✅ | 02-03, 05-01, 06-01, 06-02, 06-10, 04-01 |
| File I/O (`std::fs`, `File`, `BufReader`) | ✅ | 04-01, 04-02 |
| CSV parsing / writing (`csv` crate) | ✅ | 04-01, 04-02 |
| Serde (serialisation / deserialisation) | ✅ | 04-02, 04-03, 04-05, 04-06 |
| Parquet / Arrow columnar format | ✅ | 04-03, 04-04 |
| Apache Arrow `RecordBatch`, primitive arrays, builders, schema, CSV→Arrow, IPC streaming | ✅ | 04-04 |
| Parquet write→read round-trip, projection pushdown, statistics, schema evolution | ✅ | 04-03 |
| YAML config (`serde_yaml`, `rename_all` enums, config merge) | ✅ | 04-05 |
| JSON & NDJSON streaming (`serde_json`, `Value` walking, NDJSON read/write) | ✅ | 04-06 |
| Polars DataFrame (eager + lazy, `group_by`, Parquet I/O, predicate pushdown) | ✅ | 12-01 |
| DuckDB in-process OLAP (`read_csv_auto`, prepared statements, raw SQL) | ✅ | 12-02 |
| Apache DataFusion (`SessionContext`, async SQL, Arrow zero-copy, Parquet write) | ✅ | 12-03 |
| Ratatui terminal UIs (`ratatui`, `crossterm`, `TestBackend`, layouts, widgets) | ✅ | 06-11 |
| Async CLIs with `clap` derive (subcommands, `#[tokio::main]`, `ExitCode`) | ✅ | 06-12 |
| Testing (`#[test]`, `cargo test`) | ✅ | 01-02 |
| Documentation (`///`, `cargo doc`) | ✅ | 01-02 |
| `derive` macros (`Debug`, `Clone`, `Copy`, `PartialEq`, etc.) | ✅ | 02-02, 06-01, 06-10, 04-02 |
| Trait definitions, bounds, and orphan rule | ✅ | 02-02 |
| `Deref` / `Sized` / `From` / `Clone` / `Copy` / `Drop` traits | ✅ | 02-02 |
| Concurrency (`std::thread`, `mpsc`, `Mutex`, `Arc`) | ✅ | 05-01, 05-03, 05-06, 05-08, 05-10 |
| Scoped threads | ✅ | 05-01 |
| `mpsc` channels | ✅ | 05-01 |
| `RwLock` | ✅ | 05-01 |
| Interior mutability (`Cell`, `RefCell`) | ✅ | 05-01, 05-07 |
| `Send` / `Sync` marker traits | ✅ | 05-01, 05-06, 05-11 |
| `rayon` parallel iterators | ✅ | 07-02, 05-10 |
| Atomics & memory ordering | ✅ | 05-04 |
| `async` / `.await` basics | ✅ | 05-02 |
| `Future` trait & `tokio` runtime | ✅ | 05-02 |
| Spawning async tasks & cancellation | ✅ | 05-02 |
| Graph algorithms (`petgraph`, Dijkstra, PageRank, SCC) | ✅ | 06-02, 06-03, 06-05, 06-07 |
| `HashMap` iteration and entry API | ✅ | 03-04 |
| Pattern matching with `@` bindings, guards, etc. | ✅ | 01-01 |
| Package layout (`lib.rs` + `main.rs`) | ✅ | 06-10 |
| Library re‑exports (`pub use`) | ✅ | 06-10 |
| CLI argument parsing (`clap` derive) | ✅ | 06-01, 06-10, 07-02 |
| Safe vs unsafe Rust | ✅ | 07-01 |
| RAII / `Drop` trait / OBRM | ✅ | 02-04 (canonical), 02-01, 02-02 |
| Cryptographic hashes (`Digest` trait) | ✅ | 07-03 |
| Caesar cipher / frequency analysis | ✅ | 07-02 |
| Argon2 password hashing (`argon2` crate, salt, `PasswordHasher`/`PasswordVerifier`, `subtle::ConstantTimeEq`) | ✅ | 07-04 |
| Ed25519 digital signatures (`ed25519-dalek`, `SigningKey`/`VerifyingKey`, hex serialization) | ✅ | 07-05 |
| Rustls TLS (`rustls` + `aws-lc-rs`, `ServerConfig`/`ClientConfig`, `tokio-rustls` handshake) | ✅ | 07-06 |
| PyO3 bindings (`#[pyfunction]`, `#[pymodule]`, `cdylib`, `maturin develop`, feature-gated FFI) | ✅ | 08-03 |
| GIL release (`pyo3::Python::allow_threads`, GIL contention factor) | ✅ | 08-04 |
| JWT auth (`jsonwebtoken` HS256 sign/verify, `Claims`, role checks, refresh tokens) | ✅ | 09-03 |
| Tracing + OTel data model (`tracing`, `tracing-subscriber` JSON, spans, correlation ids, atomic metrics) | ✅ | 09-04 |
| Property-based testing (`proptest` strategies, shrinking, invariants) | ✅ | 10-04 |
| Trait mocking (`mockall` `#[automock]`, `&dyn Trait`, predicate matchers) | ✅ | 10-05 |
| Snapshot testing (`insta` inline + external snapshots, `cargo insta review`) | ✅ | 10-06 |
| `parking_lot` Mutex/RwLock, `crossbeam_channel` (MPMC), `arc_swap` (lock-free) | ✅ | 05-12 |
| `tokio::select!`, `Semaphore`, `Notify`, `JoinSet`, bounded `mpsc`, `CancellationToken` | ✅ | 05-13 |
| DIY actor (`mpsc` mailbox + `oneshot` reply, `tokio::spawn` loop) | ✅ | 13-01 |
| `ractor` framework (Actor trait, `cast` / `call` / `CallResult`, supervision) | ✅ | 13-02 |
| Actor pipeline (source → transform → sink with bounded channels + atomic metrics) | ✅ | 13-03 |
| Jupyter notebook / `evcxr` | ✅ | 08-02 |
| Pandas / DataFrame operations | ✅ | 08-01 |

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

- **[data-engineering-rust](https://github.com/jolisper/data-engineering-rust)** by [Jorge López](https://github.com/jolisper) — the hands-on Cargo projects teaching Rust for data engineering through practical examples (collections, graphs, concurrency, file I/O, etc.).
- **[100-exercises-to-learn-rust](https://github.com/mainmatter/100-exercises-to-learn-rust)** by [Mainmatter](https://mainmatter.com) — the tutorial exercises teaching Rust fundamentals through structured, progressive exercises.

The original content, structure, and teaching design belong to their respective authors. This repository reorganizes and sequences the material into a single progressive curriculum. Huge thanks to both projects for their excellent work.

---

## License

MIT
