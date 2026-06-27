# Data Cleansing Engine — Cargo Workspace with Streaming Polars

> **Test-driven approach**: This project includes a Cargo workspace with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 19 tests pass**.

---

## What Is This Project?

A high-performance data cleansing engine built as a Cargo workspace — a core library, a CLI binary, and streaming Polars pipelines that process multi-GB files without loading them into RAM.

### Python equivalent

```python
import polars as pl

# Python: load the whole file, then clean
df = pl.read_csv("dirty.csv")           # loads entire file into RAM
df = df.drop_nulls(subset=["amount"])   # eager — already in memory
df = df.filter(pl.col("amount").is_between(q1 - 1.5*iqr, q3 + 1.5*iqr))
df.write_parquet("clean.parquet")
```

In this project you'll build the same pipeline in Rust — and along the way
you'll discover **Cargo workspaces**, **streaming execution**, **adversarial input safety**, and the **eager-for-metadata, lazy-for-data** pattern.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Cargo workspace | Multi-crate projects: core library + CLI + tests |
| 2 | Streaming execution | Process GB-scale files without OOM |
| 3 | Eager vs Lazy mixing | Compute metadata eagerly, apply filters lazily |
| 4 | Resource limits | Defend against gzip bombs and schema crashes |
| 5 | `thiserror` enums | Library-grade typed errors (review from Section 2) |
| 6 | Polars expressions | Vectorized data cleaning without row-by-row loops |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Cargo Workspaces](#3-concept-cargo-workspaces)
4. [Concept: Streaming Execution](#4-concept-streaming-execution)
5. [Concept: Eager-for-Metadata, Lazy-for-Data](#5-concept-eager-for-metadata-lazy-for-data)
6. [Concept: Resource Limits & Adversarial Safety](#6-concept-resource-limits--adversarial-safety)
7. [Building the Engine](#7-building-the-engine)
8. [Running the CLI](#8-running-the-cli)
9. [Common Pitfalls](#9-common-pitfalls)
10. [Exercises](#10-exercises)
11. [Summary](#11-summary)

---

## 1. Introduction

Data engineers frequently build cleaning pipelines: read dirty CSVs, apply rules (drop nulls, filter outliers, trim whitespace), write clean output. In Python this is 10 lines with pandas/Polars — but it loads the entire file into RAM and provides no defense against adversarial inputs (gzip bombs, million-column schemas).

In Rust, we can build a cleansing engine that:
- Processes files as a **stream** (constant memory regardless of file size)
- Enforces **resource limits** (reject gzip bombs, column-count bombs)
- Mixes **eager metadata computation** with **lazy data filtering**
- Exposes a clean CLI (and optionally PyO3 bindings — see [Section 11](../../11-Interop/README.md))

This project is a **capstone** that synthesizes Polars (Section 12), error handling (Section 2), and CLI patterns (Section 6) into a production-grade tool.

---

## 2. Prerequisites

- Completed [Section 12: DataEngAnalytics/01-Polars](../01-Polars/README.md) — comfortable with `DataFrame`, `LazyFrame`, `group_by`
- Familiar with `Result`, `?` operator, `thiserror` (see [Section 2: TicketV2](../../02-Ownership/03-TicketV2/README.md))
- Basic CLI parsing with `clap` (see [Section 6: CLISalad](../../06-TerminalApps/01-CLISalad/README.md))

---

## 3. Concept: Cargo Workspaces

### Python comparison

In Python, you might structure a project as:
```
mypackage/
├── __init__.py
├── core.py
└── cli.py
```
Python packages are flat — one `pyproject.toml`, importable modules.

In Rust, a **Cargo workspace** lets you group multiple crates (packages) in one repository with a shared `Cargo.lock` and unified build:

```
project/
├── Cargo.toml          ← workspace root (no [package])
├── crates/
│   ├── core/           ← library crate (the engine)
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── cli/            ← binary crate (the CLI)
│       ├── Cargo.toml
│       └── src/main.rs
```

### Explanation

The workspace root `Cargo.toml` declares members:

```toml
[workspace]
members = ["crates/core", "crates/cli"]
```

Each member has its own `Cargo.toml` with `[package]`. The CLI crate depends on the core crate via `path = "../core"`.

**Why workspaces matter for data engineering:** You separate the library (testable, reusable, potentially wrapped with PyO3) from the binary (CLI entry point). This is the same pattern Python data engineers use when they separate `mypackage/` from `cli.py` — but Rust enforces it at the crate level.

### Applying to our project

Our workspace has:
- `crates/core` — the cleansing engine (all logic lives here)
- `crates/cli` — a thin Clap wrapper that calls into core

---

## 4. Concept: Streaming Execution

### Python comparison

```python
# Python Polars — lazy but still materializes on collect()
df = pl.scan_csv("huge.csv")        # lazy scan
df = df.filter(pl.col("x") > 10)    # builds query plan
df = df.collect()                    # materializes entire result into RAM
```

Polars in Python supports lazy evaluation, but `collect()` still materializes the full result. For a 50GB file, this crashes.

### Explanation

Polars in Rust supports **streaming execution** via `.collect_async()` or `.with_streaming(true).sink_parquet()`. The key difference:

```
Lazy (no streaming):  scan → filter → group_by → collect  →  full result in RAM
Lazy + Streaming:     scan → filter → group_by → sink     →  constant memory, writes directly to disk
```

With streaming, Polars processes the file in batches (default ~50K rows per batch). Memory usage stays constant regardless of file size.

### The critical pattern

```rust
use polars::prelude::*;

// WRONG — loads everything into RAM
let df = LazyCsvReader::new("huge.csv").finish()?.collect()?;

// RIGHT — streams to disk, constant memory
LazyCsvReader::new("huge.csv")?
    .filter(col("amount").gt(lit(0.0)))?
    .with_streaming(true)
    .sink_parquet("clean.parquet", None)?;
```

**Rule of thumb:** If your output fits in RAM, use `.collect()`. If it doesn't, use `.sink_*()`.

---

## 5. Concept: Eager-for-Metadata, Lazy-for-Data

### Python comparison

```python
# Python: compute quantiles eagerly, then filter lazily
q1 = df["amount"].quantile(0.25)
q3 = df["amount"].quantile(0.75)
iqr = q3 - q1
df = df.filter(
    (pl.col("amount") >= q1 - 1.5 * iqr) &
    (pl.col("amount") <= q3 + 1.5 * iqr)
)
```

### Explanation

Some operations **require** knowing the full data before they can proceed — like computing quantiles for IQR filtering. You can't do this in a purely lazy pipeline because the filter bounds don't exist until the data is scanned.

The pattern: **compute metadata eagerly (cheap, returns scalar values), apply filters lazily (cheap per-batch)**.

### Applying to our project

```rust
// STEP 1: Eagerly compute bounds (scans the file once, returns 2 scalars)
let bounds = lf.clone().select([
    col("amount").quantile(lit(0.25), QuantileInterpolOptions::Linear).alias("q1"),
    col("amount").quantile(lit(0.75), QuantileInterpolOptions::Linear).alias("q3"),
]).collect()?;  // eager — but only 2 rows!

let q1 = bounds.column("q1")?.f64()?.get(0).unwrap_or(0.0);
let q3 = bounds.column("q3")?.f64()?.get(0).unwrap_or(0.0);
let iqr = q3 - q1;

// STEP 2: Apply filter lazily (streams through the file, constant memory)
lf = lf.filter(
    col("amount").is_between(lit(q1 - 1.5 * iqr), lit(q3 + 1.5 * iqr))
);
```

**Key insight:** The eager step is cheap (returns exactly 2 rows). The lazy step streams through the entire file. Total cost: one full scan for metadata + one full scan for filtering.

---

## 6. Concept: Resource Limits & Adversarial Safety

### Python comparison

Python has no built-in defense against adversarial inputs. A gzip bomb (small file that decompresses to terabytes) will silently OOM your process:

```python
import gzip
# A 1KB gzip file that decompresses to 10GB → Python happily reads it all
with gzip.open("bomb.csv.gz", "rt") as f:
    for line in f:  # OOM after ~10GB
        process(line)
```

### Explanation

In Rust, we wrap the gzip reader in a `SafeGzipReader` that tracks bytes decompressed and refuses to exceed a configurable limit:

```
┌──────────────────────────────────────────────┐
│  SafeGzipReader<R: Read>                      │
│                                               │
│  decoder: GzDecoder<R>   ← actual decompress  │
│  bytes_read: u64         ← running total      │
│  max_bytes: u64          ← compressed × ratio  │
│                                               │
│  read() → if bytes_read > max_bytes           │
│           return Err("gzip bomb detected")    │
└──────────────────────────────────────────────┘
```

This implements the `std::io::Read` trait, so it's a drop-in replacement for any reader. It's also an example of **RAII-style resource management** — the limit is enforced for the entire lifetime of the reader.

### Applying to our project

```rust
pub struct SafeGzipReader<R: Read> {
    decoder: GzDecoder<R>,
    bytes_read: u64,
    max_bytes: u64,
}

impl<R: Read> Read for SafeGzipReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.decoder.read(buf)?;
        self.bytes_read += n as u64;
        if self.bytes_read > self.max_bytes {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Gzip bomb: decompressed {} bytes, limit {}", self.bytes_read, self.max_bytes),
            ));
        }
        Ok(n)
    }
}
```

---

## 7. Building the Engine

### Step 1: Create the workspace

```bash
cargo new --lib turboclean-core
cd turboclean-core
# (We'll use a flat structure for this workshop — see Cargo.toml below)
```

### Step 2: Cargo.toml

```toml
[package]
name = "turboclean_core"
version = "0.1.0"
edition = "2021"

[dependencies]
polars = { version = "0.46", features = ["lazy", "csv", "parquet", "streaming", "dtype-full"] }
flate2 = "1.0"
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
log = "0.4"
```

### Step 3: Implement the modules

Follow the README steps to implement each `todo!()` stub in `src/lib.rs`. Run `cargo test` after each step to verify progress.

---

## 8. Running the CLI

After implementing the engine:

```bash
cargo run -- clean --input data/dirty.csv --output data/clean.parquet --auto
cargo run -- profile --input data/dirty.csv
```

---

## 9. Common Pitfalls

| Anti-pattern | Why it's wrong | Fix |
|---|---|---|
| `CsvReader::new(file).finish()` | Loads entire file into RAM | Use `LazyCsvReader` |
| `.unwrap()` in library code | Panics on adversarial input | Return `Result<T, TurboError>` |
| `for row in df.iter()` | Row-by-row = 100x slower | Use Polars vectorized expressions |
| `.map_elements(\|s\| s.trim().to_string())` | Forces row-by-row, allocates per row | Use `col("x").str().strip_chars()` |
| `.fetch(10)` for preview | Returns 10 rows *per partition*, not 10 total | Use `.slice(0, 10).collect()` |
| Default `LazyCsvReader` schema | Infers from first 100 rows, panics on row 101 | Use `.with_infer_schema_length(None)` |

---

## 10. Exercises

**Easy** — Add a `TrimWhitespace` rule that uses `col("x").str().strip_chars()` instead of `map_elements`.

**Medium** — Add a `NormalizeNumeric` rule that rescales a column to [0, 1] using min-max normalization, computed eagerly (like IQR) then applied lazily.

**Hard** — Add a `SafeGzipWriter` that tracks bytes written and refuses to compress beyond a ratio limit (the write-side counterpart to `SafeGzipReader`).

---

## 11. Summary

| Concept | Where used |
|---|---|
| Cargo workspace | Multi-crate structure: core library + CLI binary |
| Streaming execution | `.with_streaming(true)` for constant-memory processing |
| Eager-for-metadata | Compute quantiles/bounds before lazy filter |
| Resource limits | `SafeGzipReader` defends against gzip bombs |
| `thiserror` enums | `TurboError` with `#[from]` auto-conversion (review from [Section 2](../../02-Ownership/03-TicketV2/README.md)) |
| Polars expressions | Vectorized cleaning without row-by-row loops (review from [Section 12](../01-Polars/README.md)) |
