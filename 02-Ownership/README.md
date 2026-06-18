# Section 2: Ownership — Rust's Superpower over Python's GC

*The biggest mindset shift: how Rust manages memory without a garbage collector, and how it prevents whole classes of bugs at compile time.*

---

## Why This Section?

Ownership note: In Rust, values like `String` and `Vec` live on the heap, while primitive values (e.g., `i32`, `bool`) live on the stack. Ownership rules govern when heap data is cleaned up.


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

---

## Stack vs Heap — Rust's Memory Model

Complex types like `String` and `Vec<T>` use a **hybrid approach**: the *metadata* lives on the fast Stack, the *actual data* lives on the dynamic Heap.

### Memory Layout of a Rust `String`

```text
┌─────────────────────────────────────────────┐
│  STACK (Variable `s`)                       │
│  ┌──────────┬─────────┬──────────┐          │
│  │ ptr      │ len=5   │ cap=8    │          │
│  │ 0x7f9a.. │         │          │          │
│  └────┬─────┴─────────┴──────────┘          │
│       │ points to                            │
└───────┼─────────────────────────────────────┘
        ▼
┌─────────────────────────────────────────────┐
│  HEAP (Actual data)                         │
│  ┌─────┬─────┬─────┬─────┬─────┬────┐      │
│  │ 'H' │ 'e' │ 'l' │ 'l' │ 'o' │ .. │      │
│  └─────┴─────┴─────┴─────┴─────┴────┘      │
└─────────────────────────────────────────────┘
```

**Why split?** The Stack needs fixed-size items at compile time (24 bytes for `String`: 8 ptr + 8 len + 8 cap). The variable-length text lives on the Heap, managed by Rust's ownership rules.

### Key Differences

| Feature | The Stack | The Heap |
| :--- | :--- | :--- |
| **Data types** | `i32`, `f64`, `bool`, `char`, `&T` | `String`, `Vec<T>`, `Box<T>`, `HashMap` |
| **Size** | Fixed at compile time | Dynamic, grows at runtime |
| **Speed** | Fast (CPU cache friendly) | Slower (pointer chasing) |
| **Management** | Automatic (push/pop on function call) | Rust's ownership & `Drop` trait |
| **Capacity** | ~2-8 MB per thread | Limited by system RAM |

**In Python:** every value is a heap-allocated object with ref‑counting overhead. Rust gives you the **choice**: cheap stack values for plain data, explicit heap allocation when you need dynamic growth.

### How Ownership Deallocation Works

When a `String` goes out of scope, Rust calls `drop()` automatically — no GC needed.

```rust
{
    let s = String::from("hello"); // heap allocated
    // ... use s ...
} // ← s goes out of scope, heap memory freed immediately
```

Python's `__del__` is unreliable (GC timing); Rust's `Drop` is deterministic at compile time.

### Step-by-Step Execution Trace — Three-Column View

The following trace shows how Rust's ownership model manages stack and heap memory line by line for a realistic data-engineering scenario.

```rust
fn main() {
    let x: i32 = 42;                    // Step 1
    let name = String::from("Rust");    // Step 2
    let v = vec![1, 2, 3];              // Step 3
    let b = Box::new(99);               // Step 4
    let y = x;                          // Step 5: i32: Copy
    let z = name;                       // Step 6: String: Move!
    // name is now invalid
    println!("{}", z);                  // Step 7
} // Everything dropped here             // Step 8
```

| Step | Code | Stack (before end of function) | Heap |
|------|------|--------------------------------|------|
| 1 | `let x: i32 = 42;` | `x: i32 = 42` (4B) | — |
| 2 | `let name = String::from("Rust");` | `x: i32 = 42` (4B)<br>`name: String { ptr, len=4, cap=4 }` (24B) | `"Rust"` (4B) ← `name.ptr` |
| 3 | `let v = vec![1, 2, 3];` | `x: i32 = 42` (4B)<br>`name: String { ptr, len=4, cap=4 }` (24B)<br>`v: Vec { ptr, len=3, cap=3 }` (24B) | `"Rust"` (4B) ← `name.ptr`<br>`[1, 2, 3]` (12B) ← `v.ptr` |
| 4 | `let b = Box::new(99);` | `x: i32 = 42` (4B)<br>`name: String { ptr, len=4, cap=4 }` (24B)<br>`v: Vec { ptr, len=3, cap=3 }` (24B)<br>`b: Box { ptr }` (8B) | `"Rust"` (4B) ← `name.ptr`<br>`[1, 2, 3]` (12B) ← `v.ptr`<br>`99` (4B) ← `b.ptr` |
| 5 | `let y = x;` | `x: i32 = 42` (4B)<br>`name: String { ptr, len=4, cap=4 }` (24B)<br>`v: Vec { ptr, len=3, cap=3 }` (24B)<br>`b: Box { ptr }` (8B)<br>`y: i32 = 42` (4B) | (unchanged — `i32` is `Copy`) |
| 6 | `let z = name;` | `x: i32 = 42` (4B)<br>`name: INVALID (moved)` (24B)<br>`v: Vec { ptr, len=3, cap=3 }` (24B)<br>`b: Box { ptr }` (8B)<br>`y: i32 = 42` (4B)<br>`z: String { ptr, len=4, cap=4 }` (24B) | `"Rust"` (4B) ← `z.ptr`<br>`[1, 2, 3]` (12B) ← `v.ptr`<br>`99` (4B) ← `b.ptr` |
| 7 | `println!("{}", z);` | (unchanged) | (unchanged — prints "Rust") |
| 8 | `}` (scope end) | All stack vars dropped | `"Rust"` freed (via `z`)<br>`[1, 2, 3]` freed (via `v`)<br>`99` freed (via `b`) |

**Key observations:**
- **Step 5** (`y = x`): `i32` implements `Copy` — bitwise copy, both `x` and `y` valid
- **Step 6** (`z = name`): `String` does **not** implement `Copy` — **move** transfers ownership, `name` becomes invalid
- **Step 8**: RAII — `z`, `v`, `b` all implement `Drop`, heap memory freed automatically in reverse order

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
| 01 | **TicketV1** — structs, ownership, stack vs heap | `struct`, validation, `mod`/`pub`, encapsulation, ownership, borrowing, stack/heap, destructors (`Drop`) | Tutorial |
| 02 | **Traits** — trait definitions, derive, bounds | `trait`, orphan rule, operator overloading, `derive`, trait bounds, `Deref`, `Sized`, `From` | Tutorial |
| 03 | **TicketV2** — enums, match, error handling | `enum`, `match`, `if let`, `Option`, `Result`, error enums, `Error` trait, `thiserror`, `TryFrom` | Tutorial |
| 04 | **OBRM** — ownership-based resource management | Ownership rules, `Drop` trait, RAII, borrowing, resource lifecycle | Project |
| 05 | **OwnershipLifetimes** — lifetimes & borrow checker | Move semantics, borrowing (`&T`/`&mut T`), lifetimes (`'a`), `Copy`/`Clone` | Project |
| 06 | **ConversionErrorHandling** — `unwrap`, `?`, `From`, and the whole family | `Option::unwrap_or[_default]`, `Option::map_or`, `Option::ok_or`, `Result::map_err`, `Result::and_then`, `?` operator, `From<E1>` impl, `thiserror` | Workshop |

## Learning Path
1. **01-TicketV1** — the most important workshop. Master ownership with extensive diagrams
2. **02-Traits** — learn Rust's interface system (like Python protocols)
3. **03-TicketV2** — enums and Result-based error handling (essential for production)
4. **04-OBRM** — apply ownership in a resource management project
5. **05-OwnershipLifetimes** — deep dive into lifetime annotations
6. **06-ConversionErrorHandling** — the missing reference: every `Option` / `Result` method, `?` with `From`, `thiserror`

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

