# Section 4: File I/O — CSV & Parquet at Scale

*Python's pandas reads CSVs. Rust's csv and parquet crates do it faster, with less memory.*

---

## Why This Section?

### The Problem — pandas' Memory Wall

Every data engineer has hit this:

```python
df = pd.read_csv("transactions_2024.csv")  # 5 GB file
```

At that moment, Python is loading the **entire file into RAM** — and that's just the start:

```
┌─────────────────────────────────────────────────────┐
│  pandas.read_csv() Timeline                          │
│                                                      │
│  ┌─────────────────────────────────────────┐         │
│  │  Step 1: Read entire file into RAM      │ 5 GB   │
│  ├─────────────────────────────────────────┤         │
│  │  Step 2: Parse strings → Python objects │ 15 GB  │
│  ├─────────────────────────────────────────┤         │
│  │  Step 3: Infer dtypes (guess!)          │ 15 GB  │
│  ├─────────────────────────────────────────┤         │
│  │  Step 4: Optional: convert to optimized │ free?  │
│  │           dtypes (if you remember)       │         │
│  └─────────────────────────────────────────┘         │
│                                                      │
│  Peak memory: 3-5x file size!                        │
│  Single-threaded CPU-bound parsing                   │
└─────────────────────────────────────────────────────┘
```

**Common File I/O Pain Points:**

| Problem | Python | Rust |
|---------|--------|------|
| Large CSV memory | Loads everything into RAM | Streaming row-by-row |
| Type inference | Guesses (expensive, wrong) | You declare types |
| Error handling | `try/except` around the whole thing | Per-row `Result<T, E>` |
| Columnar data | Parquet through PyArrow | Native parquet crate |
| Serialization | `json.dumps` (slow) | `serde` (zero-cost derive) |

### The Rust Solution — Streaming, Typed, Zero-Cost

```rust
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Deserialize)]
struct Transaction {
    id: u32,
    amount: f64,
    date: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new()
        .from_path("transactions_2024.csv")?;

    for result in rdr.deserialize() {
        let record: Transaction = result?;
        process(&record);  // stream one row at a time
    }
    Ok(())
}
```

**Memory: ~1 row at a time, not 5 GB.** Processing millions of rows with minimal memory footprint.

---

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | File operations | `std::fs::File` | `open()` | Open, read, write files |
| 2 | Buffered reading | `BufReader` | File buffering | Efficient sequential reads |
| 3 | Buffered writing | `BufWriter` | File buffering | Efficient sequential writes |
| 4 | CSV reading | `csv::Reader` | `csv.reader` | Parse CSV row by row |
| 5 | CSV writing | `csv::Writer` | `csv.writer` | Write CSV row by row |
| 6 | CSV with serde | `csv::Reader::deserialize` | N/A (dicts) | Auto-deserialize CSV into structs |
| 7 | Custom delimiters | `csv::WriterBuilder::delimiter` | `csv.Dialect` | TSV, pipe-delimited, etc. |
| 8 | Serialization | `serde::Serialize` | `json.dumps` | Convert struct → data format |
| 9 | Deserialization | `serde::Deserialize` | `json.loads`, Pydantic | Data format → typed struct |
| 10 | Parquet format | `parquet` crate | `pyarrow.parquet` | Columnar storage format |
| 11 | Arrow arrays | `arrow` crate | `pyarrow` | In-memory columnar data |
| 12 | Error handling | `Result<T, E>` | `try/except` | Per-record error recovery |

---

## Concepts at a Glance

### 1. `std::fs::File` — Basic File Operations

```rust
use std::fs::File;
use std::io::Read;

let mut file = File::open("data.csv")?;
let mut contents = String::new();
file.read_to_string(&mut contents)?;
```

In Python: `with open("data.csv") as f: contents = f.read()`

### 2. `BufReader` — Efficient Streaming

```rust
use std::io::{BufRead, BufReader};

let file = File::open("large_file.csv")?;
let reader = BufReader::new(file);
for line in reader.lines() {
    let line = line?;
    process_line(&line);
}
```

### 3. The `csv` Crate — CSV Parsing

```rust
use csv::ReaderBuilder;

let mut reader = ReaderBuilder::new()
    .has_headers(true)
    .from_path("data.csv")?;

for result in reader.records() {
    let record = result?;
    println!("{:?}", record);
}
```

### 4. `serde` — Type-Safe Deserialization

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    age: u8,
    score: f64,
}

let mut reader = csv::Reader::from_path("data.csv")?;
for result in reader.deserialize() {
    let record: Record = result?;  // typed!
    println!("{} scored {}", record.name, record.score);
}
```

This is Rust's big advantage over Python: **each row is checked at compile time** for correct types, no runtime surprises.

### 5. `csv::Writer` — Writing CSV

```rust
use csv::Writer;

let mut writer = Writer::from_path("output.csv")?;
writer.write_record(&["name", "score"])?;
writer.serialize(("Alice", 95.5))?;
writer.flush()?;
```

### 6. Parquet — Columnar Storage

Parquet stores data **by column, not by row** — critical for analytics:

```
Row-oriented (CSV):     Column-oriented (Parquet):
┌─────┬─────┬─────┐    ┌─────┬─────┬─────┬─────┐
│ id  │ val │ cat │    │ id  │  id  │  id  │ ... │
├─────┼─────┼─────┤    ├─────┼─────┼─────┼─────┤
│ 1   │ 0.5 │  A  │    │ val │ val  │ val  │ ... │
│ 2   │ 0.3 │  B  │    ├─────┼─────┼─────┼─────┤
│ 3   │ 0.8 │  A  │    │ cat │ cat  │ cat  │ ... │
└─────┴─────┴─────┘    └─────┴─────┴─────┴─────┘

Query "AVG(val)"     → read full file   → read one column
```

**Why it matters**: Columnar storage gives **10-100x compression** and **only reads the columns you query**.

### Performance Comparison

| Operation | Python (pandas) | Rust (csv + serde) | Speedup |
|-----------|----------------|-------------------|---------|
| Read 10M CSV rows | ~8 seconds | ~0.8 seconds | 10x |
| Parse types | Auto-infer (slow) | Declared (fast) | 5x |
| Memory per row | ~200 bytes (dict) | ~24 bytes (struct) | 8x |
| Write CSV | ~6 seconds | ~0.5 seconds | 12x |
| Parquet read (1 col) | Full row read | Column projection | 10-100x |

---

## Prerequisites

- Completed [Section 3: Collections](../03-Collections/README.md)
- Understand `Result` and error handling

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 53 | **CSVCookbook** — read, write, transform CSV | `csv` crate, deserialization, record iteration, error handling | Project |
| 54 | **CSVWriter** — programmatic CSV writing | `csv::Writer`, custom delimiters, `serde` (`Deserialize`/`Serialize`) | Project |
| 55 | **Parquet** — Apache Parquet columnar format | Parquet format, columnar storage, Arrow integration, projection pushdown, statistics, schema evolution | Project |
| 56 | **Arrow** — Apache Arrow in-memory columnar format | `arrow` crate, `RecordBatch`, primitive arrays, builders, schema, CSV→Arrow, IPC, `compute` kernels (sum/filter/cast/slice) | Project |
| 57 | **YAML** — pipeline config files | `serde_yaml`, `#[derive(Deserialize)]`, custom enums with `rename_all`, config merge, NDJSON-style queries | Project |
| 58 | **JsonStream** — JSON & NDJSON streaming | `serde_json`, typed `Value` walking, NDJSON read/write, pretty-print, object merge | Project |
| 59 | **DataManagementLessonReflection** — I/O reflection | File I/O, serialization, columnar vs row-oriented | Reflection |

## Learning Path

1. Start with **01-CSVCookbook** for basic CSV reading
2. Move to **02-CSVWriter** for writing CSV data
3. Explore **03-Parquet** for columnar storage
4. Try **04-Arrow** for the in-memory columnar format that Polars, DuckDB, and DataFusion all build on
5. Configure pipelines with **05-YAML**
6. Stream events with **06-JsonStream**
7. Reflect with **DataManagementLessonReflection**
