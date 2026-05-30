# Section 2: Ownership — Rust's Superpower over Python's GC

*The biggest mindset shift: how Rust manages memory without a garbage collector, and how it prevents whole classes of bugs at compile time.*

---

## Why This Section?

### The Problem — Python's Garbage Collector Tax

Every Python data engineer has seen this:

```python
import pandas as pd

def transform_large_dataset():
    df = pd.read_parquet("massive_file.parquet")  # 10 GB
    df = df[df["value"] > 0]                      # filter
    df = df.groupby("key").sum()                   # aggregate
    return df
```

Looks clean. But under the hood:

```
┌─────────────────────────────────────────────────────┐
│  Python Memory Timeline                              │
│                                                      │
│  Time ───────────────────────────────────────►       │
│                                                      │
│  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐           │
│  │ 10GB │  │ 10GB │  │ 10GB │  │ 10GB │  GC kicks  │
│  │  #1  │  │  #2  │  │  #3  │  │  #4  │  in...      │
│  └──────┘  └──────┘  └──────┘  └──────┘           │
│       │         │         │         │               │
│       └─────────┴─────────┴─────────┘               │
│              Memory spikes × 3!                      │
└─────────────────────────────────────────────────────┘
```

Each intermediate `df = ...` creates a new copy. The **garbage collector** frees old copies — but **when?** You can't predict it. In production, this causes:

- **Unpredictable pauses**: GC runs at the worst time (mid-ETL, mid-API-call)
- **Memory spikes**: Two copies of your 10 GB dataset coexist
- **Cache misses**: GC-managed objects are scattered across the heap
- **Reference cycles**: Circular references that leak memory

### The Rust Solution — Ownership

Rust eliminates the garbage collector entirely. Instead, it uses a **compile-time ownership model**:

```
┌─────────────────────────────────────────────────────┐
│  Rust Memory Timeline                                │
│                                                      │
│  Time ───────────────────────────────────────►       │
│                                                      │
│  ┌──────┐                                            │
│  │ 10GB │  → drop (when owner goes out of scope)    │
│  └──────┘                                            │
│       │                                              │
│       │  No copies. No GC. No memory spikes.         │
│       │  Memory freed at known points (} braces)     │
│       ▼                                              │
└─────────────────────────────────────────────────────┘
```

**Every value in Rust has exactly one owner.** When the owner goes out of scope, the value is dropped — predictably, immediately.

```rust
fn process_records() {
    let data = read_parquet("massive_file.parquet");  // owner created
    // ... use data ...
    // data goes out of scope → memory freed HERE
    // No GC needed. No pause. No surprise.
}
```

This is the **single most important concept** in Rust. It affects every line of code you write. Master it here, and the rest of the course becomes straightforward.

---

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | Ownership rules | Ownership model | N/A (GC) | Every value has exactly one owner — no GC needed |
| 2 | Move semantics | Move (`=`, function args) | N/A (all refs) | Ownership transfers, old binding invalidated |
| 3 | Borrowing | `&T` (shared ref) | Pass-by-reference | Read data without taking ownership |
| 4 | Mutable borrowing | `&mut T` (mutable ref) | N/A | Exclusive write access, prevents data races |
| 5 | Lifetimes | `'a`, `'static` | N/A | Compiler tracks how long references are valid |
| 6 | Structs | `struct`, `impl` | `class`, `dataclass` | Custom data types with methods |
| 7 | Traits | `trait` | Protocol / ABC / interface | Define shared behavior across types |
| 8 | Enums | `enum` | Enum / Union | Type-safe variants (Result, Option, custom) |
| 9 | Error handling | `Result<T, E>` | Exceptions (`try`/`except`) | Recoverable errors as values |
| 10 | Error propagation | `?` operator | `raise` / `try` | Short-circuit on error |
| 11 | Trait derivation | `#[derive(...)]` | `@dataclass`, `@property` | Auto-implement common traits |
| 12 | Copy & Clone | `Copy`, `Clone` traits | Assignment (always copies) | Explicit vs implicit duplication |
| 13 | Drop trait | `Drop` | `__del__` (unreliable) | Cleanup on scope exit |
| 14 | Stack vs heap | Stack / Heap memory | All on heap | Performance-critical distinction |

---

## Concepts at a Glance

### 1. Ownership — One Owner, One Lifetime

```
┌─────────────────────────────────────────────┐
│  Rule 1: Each value has exactly ONE owner   │
│  Rule 2: When owner goes out of scope,      │
│           the value is dropped              │
│  Rule 3: Ownership can be TRANSFERRED (move)│
│           or BORROWED (&)                   │
└─────────────────────────────────────────────┘
```

```python
# Python — everything is reference-counted
x = [1, 2, 3]
y = x           # two references to same list
x.append(4)     # y is also affected!
```

```rust
// Rust — ownership is exclusive
let x = vec![1, 2, 3];
let y = x;      // MOVED: x is no longer valid
// x.push(4);   // compile error! x was moved
```

### 2. Move Semantics

When you assign a value or pass it to a function, ownership **moves**:

```rust
let s1 = String::from("hello");
let s2 = s1;           // s1 MOVED to s2
// println!("{}", s1); // compile error — s1 is gone
```

Move semantics mean **no hidden copies**. In Python, every assignment creates another reference; in Rust, every move is explicit and zero-cost.

### 3. Borrowing — `&T`

Borrowing lets you **read data without taking ownership**:

```rust
fn print_length(s: &String) {    // borrow (read-only)
    println!("{}", s.len());
}

let s = String::from("hello");
print_length(&s);                 // pass a reference
println!("{}", s);                // s is still valid!
```

### 4. Mutable Borrowing — `&mut T`

You need **exclusive** access to write:

```rust
fn add_world(s: &mut String) {   // mutable borrow
    s.push_str(", world");
}

let mut s = String::from("hello");
add_world(&mut s);                // only one &mut at a time
```

The compiler enforces: **at any moment, you have either one `&mut` OR unlimited `&`, but never both.** This eliminates data races at compile time.

### 5. Lifetimes — `'a`

Lifetimes are the **compiler's way of tracking how long references are valid**:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The `'a` says: "the returned reference is valid as long as both inputs are valid." In practice, the compiler infers lifetimes 90% of the time — you only annotate when needed.

### 6. `struct` — Custom Data Types

```rust
struct DataPipeline {
    name: String,
    batch_size: u32,
    active: bool,
}

impl DataPipeline {
    fn run(&self) {
        println!("Pipeline {} running...", self.name);
    }
}
```

### 7. `trait` — Shared Behavior

Traits are like Python protocols:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for DataPipeline {
    fn summarize(&self) -> String {
        format!("{} (batch: {})", self.name, self.batch_size)
    }
}
```

### 8. `enum` — Type-Safe Variants

```rust
enum ParseResult {
    Success(f64),
    InvalidInput(String),
    Overflow,
}
```

The compiler forces you to handle **every variant**:

```rust
match result {
    ParseResult::Success(v) => process(v),
    ParseResult::InvalidInput(msg) => log_error(msg),
    ParseResult::Overflow => clamp_value(),
}
```

### 9. `Result<T, E>` — Errors as Values

Instead of throwing exceptions, Rust returns errors:

```rust
fn parse_csv_line(line: &str) -> Result<Vec<f64>, String> {
    let values: Vec<f64> = line.split(',')
        .map(|s| s.parse().map_err(|_| format!("Bad number: {}", s)))
        .collect::<Result<_, _>>()?;
    Ok(values)
}
```

### 10. The `?` Operator

`?` is `try!` for data engineers — unwrap success or return error:

```rust
fn load_and_process(path: &str) -> Result<(), io::Error> {
    let data = std::fs::read_to_string(path)?;  // early return on error
    process(&data)?;
    Ok(())
}
```

### 11. `#[derive(...)]` — Auto Traits

```rust
#[derive(Debug, Clone, PartialEq)]
struct Record {
    id: u32,
    value: f64,
}
```

Equivalent to Python's `@dataclass(frozen=True)` — but with zero runtime overhead.

### 12. Stack vs Heap

```
┌─────────────────────┐     ┌──────────────────────┐
│       Stack         │     │        Heap          │
├─────────────────────┤     ├──────────────────────┤
│  Fixed size         │     │  Variable size       │
│  Fast (L1 cache)    │     │  Slower              │
│  LIFO               │     │  Arbitrary order     │
│  i32, f64, bool...  │     │  String, Vec, Box... │
│  Function calls     │     │  Dynamic allocations │
└─────────────────────┘     └──────────────────────┘
```

---

## Prerequisites

- Completed [Section 1: Foundations](../01-Foundations/README.md)
- Understand basic Rust syntax and types

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 3 | **TicketV1** — structs, ownership, stack vs heap | `struct`, validation, `mod`/`pub`, encapsulation, ownership, borrowing, stack/heap, destructors (`Drop`) | Tutorial |
| 4 | **Traits** — trait definitions, derive, bounds | `trait`, orphan rule, operator overloading, `derive`, trait bounds, `Deref`, `Sized`, `From` | Tutorial |
| 5 | **TicketV2** — enums, match, error handling | `enum`, `match`, `if let`, `Option`, `Result`, error enums, `Error` trait, `thiserror`, `TryFrom` | Tutorial |
| 37 | **OBRM** — ownership-based resource management | Ownership rules, `Drop` trait, RAII, borrowing, resource lifecycle | Project |
| 38 | **OwnershipLifetimes** — lifetimes & borrow checker | Move semantics, borrowing (`&T`/`&mut T`), lifetimes (`'a`), `Copy`/`Clone` | Project |

## Learning Path

1. **3-TicketV1** — the most important workshop. Master ownership with extensive diagrams
2. **4-Traits** — learn Rust's interface system (like Python protocols)
3. **5-TicketV2** — enums and Result-based error handling (essential for production)
4. **04-OBRM** — apply ownership in a resource management project
5. **05-OwnershipLifetimes** — deep dive into lifetime annotations
