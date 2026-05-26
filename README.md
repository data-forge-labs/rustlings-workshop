# Rust Tutorial — Learn by Doing

A hands-on workshop series that teaches Rust by porting existing Python programs. Each project starts with a working Python implementation and walks through converting it to idiomatic Rust step by step, explaining new concepts along the way.

## How It Works

```
RustTut/
├── README.md                    ← this file (also used by agents.md)
├── agents.md                    ← instructions for the AI workshop designer
├── 1-MasterMind/                ← project 1
│   ├── master-mind.py           ← original Python version
│   └── master_mind.md           ← full step-by-step workshop guide
├── 2-Advanced-MasterMind/       ← project 2
│   ├── master-mind.py           ← (same Python code, but focuses on library)
│   └── advanced_mastermind.md   ← advanced workshop guide
├── 3-... (next project)
└── ...
```

For each project:
1. **Read the Python code** — understand the problem and the logic.
2. **Open the workshop guide** (e.g. `master_mind.md` or `advanced_mastermind.md`) — the intro section gives an overview; the rest walks you through writing the Rust version step by step, explaining each new concept.
3. **Write the code yourself** as you follow each section, then compare with the reference.

## How to Design the Next Workshop (for `agents.md`)

The file `agents.md` contains the instructions for an automated agent that designs new workshops. This `README.md` provides the data the agent needs:

- **Covered concepts** are listed in the [Rust Concepts Coverage](#rust-concepts-coverage) section below.
- The agent parses that section, finds unchecked concepts, selects a cohesive subset, and designs a Python → Rust project that introduces them.
- Each new workshop must be added as a new row in the **Projects** table and the coverage checklist must be updated.

---

## Table of Contents

- [Rust Tutorial — Learn by Doing](#rust-tutorial--learn-by-doing)
  - [How It Works](#how-it-works)
  - [How to Design the Next Workshop (for `agents.md`)](#how-to-design-the-next-workshop-for-agentsmd)
  - [Table of Contents](#table-of-contents)
  - [Projects](#projects)
  - [Prerequisites](#prerequisites)
  - [Quick Start](#quick-start)
  - [Rust Concepts Coverage](#rust-concepts-coverage)
  - [License](#license)

---

## Projects

| # | Project | Rust Topics Covered |
|---|---------|-------------------|
| 1 | **MasterMind** — guess a 4-digit secret code with hints | `struct`, `impl`, `fn new() -> Self`, `Vec<T>`, `vec![]` macro, `&self` / `&mut self`, `&str`, `String`, `Option<T>`, `if let`, `loop`, `while`, `const`, `println!`, `print!`, `std::io::stdin()`, `read_line()`, `io::stdout().flush()`, iterators (`iter`, `zip`, `map`, `filter`, `count`, `enumerate`, `position`, `any`, `all`, `collect`), `chars()`, `to_digit()`, `trim()`, `to_lowercase()`, `is_ascii_digit()`, ranges (`0..=9`), `SliceRandom::shuffle` / `choose`, `rand` crate, type casting `as`, tuples, `unwrap()`, `cargo new`, `Cargo.toml`, `cargo build` / `cargo run` |
| 2 | **Advanced MasterMind** — library crate, CLI args, documentation & testing | `lib.rs` vs `main.rs` (package layout), modules (`mod`, file organization), `pub` visibility & re‑exports, `derive` macros (with `clap::Parser`), external crates (`clap`), `cargo doc`, `cargo test`, `#[test]`, `#[cfg(test)]`, documentation comments (`///`), `saturating_sub`, separation of library & binary code |
| 3 | *(coming soon)* | |

---

## Prerequisites

- Rust installed via [rustup](https://rustup.rs/)
- Basic Python knowledge
- Familiarity with fundamental programming concepts (variables, functions, conditionals, loops)

## Quick Start

```bash
# Start with project 1 (basic MasterMind)
cd 1-MasterMind

# Read the Python code
cat master-mind.py

# Read and follow the workshop
cat master_mind.md
```

Create a new Rust project and follow along:

```bash
cargo new mastermind
cd mastermind
# ... follow the workshop guide
cargo run
```

To try the advanced workshop, go to `2-Advanced-MasterMind` and follow `advanced_mastermind.md`.

---

## Rust Concepts Coverage

The table below lists all core Rust concepts a learner should eventually see. **Checked items** are already introduced in at least one existing workshop.

| Concept | Covered? | First Project |
|---------|----------|---------------|
| `cargo new`, `cargo build`, `cargo run` | ✅ | 1 |
| `Cargo.toml` dependencies | ✅ | 1 |
| Variables (`let`, `let mut`) | ✅ | 1 |
| Data types (`u32`, `i32`, `f64`, `bool`, `char`, `usize`, `u8`) | ✅ | 1 |
| `String` vs `&str` | ✅ | 1 |
| Ownership, borrowing, references (`&`, `&mut`) | ✅ | 1 |
| `Vec<T>`, `vec![]` | ✅ | 1 |
| `struct`, `impl`, methods (`&self`, `&mut self`) | ✅ | 1 |
| `Option<T>`, `Some`, `None`, `if let` | ✅ | 1 |
| `match` (basic) | ✅ | 1 |
| `loop`, `while`, `continue`, `break` | ✅ | 1 |
| `const` | ✅ | 1 |
| Iterators (`iter`, `map`, `filter`, `count`, `collect`, `zip`, `enumerate`, `any`, `all`) | ✅ | 1 |
| Closures (`|x| x * 2`) | ✅ | 1 |
| `print!`, `println!` | ✅ | 1 |
| `std::io::stdin()`, `read_line()` | ✅ | 1 |
| `io::stdout().flush()` | ✅ | 1 |
| String methods (`chars`, `trim`, `to_lowercase`, `is_ascii_digit`, `to_digit`) | ✅ | 1 |
| Ranges (`0..=9`) | ✅ | 1 |
| `rand` crate (`thread_rng`, `shuffle`, `choose`) | ✅ | 1 |
| Type casting (`as`) | ✅ | 1 |
| Tuples | ✅ | 1 |
| `unwrap()` / basic error handling | ✅ | 1 |
| `Result<T, E>`, `?` operator | ❌ | — |
| `enum` (custom enums) | ❌ | — |
| `match` with patterns (advanced) | ❌ | — |
| `impl` with generics and traits | ❌ | — |
| `HashMap` | ❌ | — |
| `HashSet` | ❌ | — |
| `BTreeMap` / `BTreeSet` | ❌ | — |
| `Box<T>`, `Rc<T>`, `Arc<T>` (smart pointers) | ❌ | — |
| Lifetimes and borrow checker annotations | ❌ | — |
| Error handling with `Result` and custom error types | ❌ | — |
| `mod`, `pub`, `use` (modules & visibility) | ✅ | 2 |
| External crates beyond `rand` | ✅ | 2 |
| File I/O (`std::fs`, `File`, `BufReader`) | ❌ | — |
| Serde (serialisation / deserialisation) | ❌ | — |
| Testing (`#[test]`, `cargo test`) | ✅ | 2 |
| Documentation (`///`, `cargo doc`) | ✅ | 2 |
| Concurrency (`std::thread`, `mpsc`, `Mutex`, `Arc`) | ❌ | — |
| `async` / `.await` basics | ❌ | — |
| `derive` macros (`Debug`, `Clone`, `Copy`, etc.) | ✅ | 2 |
| `HashMap` iteration and entry API | ❌ | — |
| Pattern matching with `@` bindings, guards, etc. | ❌ | — |
| Package layout (`lib.rs` + `main.rs`) | ✅ | 2 |
| Library re‑exports (`pub use`) | ✅ | 2 |
| CLI argument parsing (`clap` derive) | ✅ | 2 |
| `saturating_sub` | ✅ | 2 |
| Separation of concerns (lib vs bin) | ✅ | 2 |

> **For the agent (`agents.md`):** Choose 3–6 unchecked concepts that form a natural group, then design a Python → Rust project that introduces them. Update this file after adding the workshop.

---

## License

MIT
