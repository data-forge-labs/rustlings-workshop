# Project 57: Apache Arrow In-Memory Columnar Format — Python to Rust Workshop

*Build a typed `RecordBatch` from primitive arrays, query it, and serialize it to Arrow's IPC format — the lingua franca of the modern Rust data stack.*

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 20 tests pass**.

---

## Why Learn Apache Arrow's Zero-Copy Format?

**Python pain:** Every time you call `pyarrow.Table.from_pandas(df)`, Python makes a *copy* of every column, converts the data into Arrow's memory layout, and hands the result to a C-extension. Loading 1 GB of data into pandas then converting to Arrow can take several seconds and temporarily double your memory:

```python
import pandas as pd, pyarrow as pa
df   = pd.read_parquet("events.parquet")            # 1 GB in pandas
tbl  = pa.Table.from_pandas(df)                     # 1 GB MORE in Arrow
rs   = pc.filter(tbl, pc.greater(tbl["age"], 30))   # yet another copy
```

**Rust fix:** Rust's `arrow` crate talks directly to the same zero-copy memory layout that `polars`, `datafusion`, `duckdb`, and `pyarrow` all share — no Python interpreter, no pandas intermediary, no copy across the FFI boundary. The same bytes flow through the entire pipeline:

```rust
let batch   = build_sample_batch();                 // 5 rows, 3 columns, columnar
let bytes   = write_ipc_to_bytes(&batch)?;          // serialize to Arrow IPC
let back    = read_ipc_from_bytes(&bytes)?;         // deserialize — zero-copy
let big     = filter_batch_by_value(&back, "age", 30);
let total   = sum_int32_column(&big, "age").unwrap();
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Primitive arrays | `arrow::array::Int32Array` | `pa.array([], type=pa.int32())` | Columnar memory layout, cache-friendly |
| 2 | Schema | `arrow::datatypes::Schema` | `pa.schema([...])` | Type metadata for the whole batch |
| 3 | Fields | `Field::new(name, dt, nullable)` | `pa.field(name, type)` | One typed column descriptor |
| 4 | Builders | `Int32Builder::append(&mut self, v)` | `pa.array` from Python list | Efficient construction, no Python overhead |
| 5 | `RecordBatch` | `RecordBatch::try_new(schema, columns)` | `pa.record_batch([...])` | Tabular data with schema |
| 6 | CSV → Arrow | `arrow::csv::ReaderBuilder` | `pa.csv.read_csv` | Skip pandas, go direct |
| 7 | IPC streaming | `arrow::ipc::writer::StreamWriter` | `pa.ipc.new_stream` | Zero-copy format used by polars/datafusion/duckdb |
| 8 | `sum` | `arrow::compute::sum` | `pc.sum` | Aggregate without iteration |
| 9 | `filter` | `arrow::compute::filter_record_batch` | `pc.filter` | Vectorized WHERE |
| 10 | `cast` | `arrow::compute::cast` | `pc.cast` | Type promotion (Int32 → Float64) |
| 11 | `slice` | `RecordBatch::slice(offset, length)` | `tbl.slice(offset, length)` | Zero-copy row range |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Primitive Arrow Arrays](#3-concept-primitive-arrow-arrays)
4. [Concept: Schema and Field](#4-concept-schema-and-field)
5. [Concept: Builders for Column Construction](#5-concept-builders-for-column-construction)
6. [Concept: `RecordBatch` — Tabular Data with Schema](#6-concept-recordbatch--tabular-data-with-schema)
7. [Concept: CSV → Arrow — Skip pandas, Go Direct](#7-concept-csv--arrow--skip-pandas-go-direct)
8. [Concept: Arrow IPC — The Zero-Copy Wire Format](#8-concept-arrow-ipc--the-zero-copy-wire-format)
9. [Concept: Compute Kernels — `sum`, `filter`, `cast`, `slice`](#9-concept-compute-kernels--sum-filter-cast-slice)
10. [Putting It All Together](#10-putting-it-all-together)
11. [Complete Code Reference](#11-complete-code-reference)
12. [Summary](#12-summary)
13. [Further Reading](#13-further-reading)
14. [Exercises](#14-exercises)

## 1. Introduction

Apache Arrow is **the in-memory columnar format** that every modern Rust data tool agrees on. `polars`, `datafusion`, `duckdb`, and `parquet` all speak it. When you read a Parquet file, the data is decoded into Arrow arrays. When `polars` filters a DataFrame, it operates on Arrow arrays. When `datafusion` runs a SQL query, the result is a stream of Arrow `RecordBatch`es.

Understanding Arrow is therefore **foundational** for every data-engineering project that comes after this one in the course (Polars, DuckDB, DataFusion in the next waves all consume Arrow).

**Python → Rust**: In Python, you work with `pyarrow.Table` and `pyarrow.RecordBatch`. In Rust, you work with `arrow::array::RecordBatch` and the same primitive array types. The memory layout is **identical** — the bytes Python puts down are the bytes Rust reads.

**Data-engineering motivation**: a `RecordBatch` is a *self-describing* table (it carries its schema with it) and the columnar layout is **cache-friendly** and **SIMD-friendly** — the CPU can process 4 or 8 values at a time when the data lives in contiguous arrays. That's why `polars` is 10–100x faster than `pandas` for most analytics.

## 2. Prerequisites

- Completed [Project 56: Parquet](../03-Parquet/README.md) — comfortable with columnar concepts and the `arrow` crate.
- Familiarity with `Vec`, iterators, and `Result`/`?` from [Section 3: Collections](../../03-Collections/README.md) and [Section 2: Ownership](../../02-Ownership/README.md).
- The `arrow = "53"` crate is in `workshop/Cargo.toml` — it pulls in CSV + IPC + compute features by default.

## 3. Concept: Primitive Arrow Arrays

### Explanation

An Arrow array is a **strongly-typed, contiguous, immutable column** of values. Each type lives in its own struct:

| Arrow type | Rust type | Python (`pyarrow`) | Holds |
|------------|-----------|--------------------|-------|
| `Int32Array` | `i32` | `pa.int32()` | 32-bit signed integers |
| `Float64Array` | `f64` | `pa.float64()` | 64-bit floats |
| `StringArray` (UTF-8) | `&str` / `String` | `pa.utf8()` | Variable-length strings |
| `BooleanArray` | `bool` | `pa.bool_()` | true/false |
| `Date32Array` | `i32` (days since epoch) | `pa.date32()` | Dates |

The simplest way to construct one is `ArrayType::from(values)`, which the `From<Vec<T>>` trait implements:

```rust
use arrow::array::{Int32Array, StringArray, Float64Array};

let ids    = Int32Array::from(vec![1, 2, 3, 4, 5]);
let names  = StringArray::from(vec!["Alice", "Bob", "Carol"]);
let ages   = Float64Array::from(vec![30.0, 25.0, 35.0]);
```

Each array stores:
1. A **values buffer** (e.g. `[1, 2, 3, 4, 5]` for `Int32Array`).
2. An **optional null bitmap** (one bit per value, marking nulls).
3. An **optional offsets buffer** for variable-length types like `StringArray`.

This contrasts with a `Vec<MyStruct>` (row-oriented):

```
Row-oriented Vec<Record>:       Columnar Int32Array + StringArray + Float64Array:
┌──────┬───────┬─────┐         ┌──────┐  ┌─────────────┐  ┌──────┐
│ id=1 │ name= │ age │         │ id:  │  │ names:      │  │ ages:│
│      │ Alice │ 30  │         │  1   │  │ o:Alice     │  │ 30.0 │
├──────┼───────┼─────┤         │  2   │  │ o:Bob       │  │ 25.0 │
│ id=2 │ name= │ age │         │  3   │  │ o:Carol     │  │ 35.0 │
│      │ Bob   │ 25  │         └──────┘  └─────────────┘  └──────┘
├──────┼───────┼─────┤             ▲           ▲              ▲
│ id=3 │ ...   │ ... │             │           │              │
└──────┴───────┴─────┘           contiguous   contiguous    contiguous
                                 (cache line) (cache line)  (cache line)
```

The CPU can load `Int32Array` values 8 at a time into a SIMD register and do `+1` on all of them in one instruction. The same operation on `Vec<Record>` requires N separate loads.

### Python comparison

```python
import pyarrow as pa
ids   = pa.array([1, 2, 3, 4, 5],     type=pa.int32())
names = pa.array(["Alice", "Bob", "Carol"], type=pa.utf8())
ages  = pa.array([30.0, 25.0, 35.0],   type=pa.float64())
```

The Python `pa.array(...)` call dispatches to C++ that allocates the same buffers the Rust `Int32Array::from(...)` produces. The on-the-wire format is **identical** (which is the whole point of Arrow).

### Applying to our project

The first three functions in `lib.rs` are the simplest possible `From` conversions:

```rust
pub fn build_int32_array(values: Vec<i32>)   -> Int32Array   { Int32Array::from(values) }
pub fn build_string_array(values: Vec<&str>) -> StringArray  { StringArray::from(values) }
pub fn build_float64_array(values: Vec<f64>) -> Float64Array { Float64Array::from(values) }
```

## 4. Concept: Schema and Field

### Explanation

A `Schema` is a list of `Field`s. Each `Field` says: *column name, logical type, is it allowed to be null?*. A `RecordBatch` is then defined by `(Schema, Vec<ArrayRef>)` — the schema and the columns in matching order.

```rust
use arrow::datatypes::{DataType, Field, Schema};

let schema = Schema::new(vec![
    Field::new("id",   DataType::Int32,   false), // non-nullable
    Field::new("name", DataType::Utf8,    true),  // nullable
    Field::new("age",  DataType::Int32,   true),  // nullable
]);
```

This is the **type metadata** the batch carries with it. Any consumer (Polars, DataFusion, DuckDB, pyarrow) can read the schema and know exactly what types and nullability to expect — no external IDL needed.

### Python comparison

```python
import pyarrow as pa
schema = pa.schema([
    pa.field("id",   pa.int32(),  nullable=False),
    pa.field("name", pa.utf8(),   nullable=True),
    pa.field("age",  pa.int32(),  nullable=True),
])
```

`pa.schema(...)` and `Schema::new(vec![...])` produce the *same* wire format. Tools can read each other's files.

### Applying to our project

```rust
pub fn build_schema() -> Schema {
    Schema::new(vec![
        Field::new("id",   DataType::Int32, false),
        Field::new("name", DataType::Utf8,  true),
        Field::new("age",  DataType::Int32, true),
    ])
}

pub fn nullable_field(name: &str, dt: DataType) -> Field {
    Field::new(name, dt, true)   // the `true` makes the column allow nulls
}
```

## 5. Concept: Builders for Column Construction

### Explanation

`ArrayType::from(vec)` is convenient but copies each element. For high-throughput construction (e.g. parsing millions of CSV rows), you should use a **builder** that appends one value at a time and only allocates the final array when you call `.finish()`:

```rust
use arrow::array::Int32Builder;

let mut b = Int32Builder::new();
for v in 0..1_000_000 {
    b.append_value(v);
}
let arr = b.finish();   // single allocation, all 1M values in one Int32Array
```

`StringBuilder` works the same way:

```rust
use arrow::array::StringBuilder;

let mut b = StringBuilder::new();
b.append_value("Alice");
b.append_value("Bob");
let arr = b.finish();
```

For `null` values, use `append_null()` instead of `append_value(...)`. The builder maintains a separate null bitmap alongside the values buffer.

### Python comparison

```python
import pyarrow as pa
b = pa.Int32Builder()
for v in range(1_000_000):
    b.append(v)
arr = b.finish()  # pyarrow.Int32Array
```

`pyarrow.Int32Builder` is the C++ builder; the Python wrapper calls into it. Rust calls the same builder directly, with no FFI hop.

### Applying to our project

```rust
pub fn build_int32_with_builder(values: Vec<i32>) -> Int32Array {
    let mut b = Int32Builder::new();
    for v in values { b.append_value(v); }
    b.finish()
}

pub fn build_string_with_builder(values: Vec<&str>) -> StringArray {
    let mut b = StringBuilder::new();
    for v in values { b.append_value(v); }
    b.finish()
}

pub fn build_mixed_batch(names: Vec<&str>, ages: Vec<i32>) -> RecordBatch {
    let mut nb = StringBuilder::new();
    let mut ab = Int32Builder::new();
    for n in &names { nb.append_value(n); }
    for a in &ages  { ab.append_value(a); }
    let schema = Schema::new(vec![
        Field::new("name", DataType::Utf8,  true),
        Field::new("age",  DataType::Int32, true),
    ]);
    RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(nb.finish()), Arc::new(ab.finish())],
    ).unwrap()
}
```

## 6. Concept: `RecordBatch` — Tabular Data with Schema

### Explanation

A `RecordBatch` is the most common Arrow data structure. It is:

- A **schema** (`SchemaRef = Arc<Schema>`)
- A **vector of columns** (`Vec<ArrayRef>` where `ArrayRef = Arc<dyn Array>`)
- A **row count** (every column must have the same number of rows)

You build one with `RecordBatch::try_new(schema, columns)`, which validates the schema-field count matches the column count and every column has the same length:

```rust
use std::sync::Arc;
use arrow::array::{Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;

let schema = Arc::new(Schema::new(vec![
    Field::new("id",   DataType::Int32, false),
    Field::new("name", DataType::Utf8,  true),
]));

let batch = RecordBatch::try_new(
    schema,
    vec![
        Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef,
        Arc::new(StringArray::from(vec!["a", "b", "c"])),
    ],
).unwrap();
```

Once you have a `RecordBatch`, the typical accessors are:

| Method | Returns |
|--------|---------|
| `batch.num_rows()` | `usize` — number of rows |
| `batch.num_columns()` | `usize` — number of columns |
| `batch.schema()` | `SchemaRef` — the schema |
| `batch.column(i)` | `ArrayRef` — the i-th column |
| `batch.column_by_name("age")` | `Option<ArrayRef>` |

### Python comparison

```python
import pyarrow as pa
batch = pa.record_batch(
    [pa.array([1,2,3]), pa.array(["a","b","c"])],
    names=["id", "name"],
)
batch.num_rows, batch.num_columns, batch.schema
```

### Applying to our project

```rust
pub fn build_sample_batch() -> RecordBatch {
    let schema = build_schema();
    let ids   = Int32Array::from(vec![1, 2, 3, 4, 5]);
    let names = StringArray::from(vec!["Alice", "Bob", "Carol", "Dave", "Eve"]);
    let ages  = Int32Array::from(vec![30, 25, 35, 28, 42]);
    RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(ids),
            Arc::new(names),
            Arc::new(ages),
        ],
    ).unwrap()
}

pub fn batch_num_rows(batch: &RecordBatch) -> usize {
    batch.num_rows()
}

pub fn batch_column_name(batch: &RecordBatch, idx: usize) -> String {
    batch.schema().field(idx).name().clone()
}

pub fn batch_schema_string(batch: &RecordBatch) -> String {
    format!("{}", batch.schema())
}
```

## 7. Concept: CSV → Arrow — Skip pandas, Go Direct

### Explanation

The `arrow::csv::ReaderBuilder` reads a CSV byte slice directly into a `RecordBatch` — no Python, no pandas, no DataFrame intermediate:

```rust
use std::io::Cursor;
use std::sync::Arc;
use arrow::csv::ReaderBuilder;
use arrow::datatypes::{DataType, Field, Schema};

let schema = Arc::new(Schema::new(vec![
    Field::new("id",   DataType::Int32, false),
    Field::new("name", DataType::Utf8,  true),
    Field::new("age",  DataType::Int32, true),
]));

let csv = b"id,name,age\n1,Alice,30\n2,Bob,25\n";
let mut reader = ReaderBuilder::new(schema)
    .has_header(true)
    .build(Cursor::new(csv))?;

let batch: RecordBatch = reader.next().unwrap()?;
```

You give it the **schema explicitly** (no slow type inference, no surprises) and it parses the CSV column-by-column into Arrow arrays. On a 1 GB CSV, this is **3-5x faster than `pd.read_csv`** because the parser writes directly into Arrow's buffer format.

### Python comparison

```python
import pyarrow as pa
schema = pa.schema([pa.field("id", pa.int32(), nullable=False), ...])
reader = pa.csv.open_csv("data.csv", convert_options=pa.csv.ConvertOptions(column_types=schema))
batch = reader.read_next_batch()
```

`pa.csv.open_csv` is a wrapper around the same C++ reader the Rust `ReaderBuilder` calls. The Rust version skips the Python-side bookkeeping.

### Applying to our project

```rust
pub fn csv_bytes_to_batch(csv: &[u8]) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    let schema = Arc::new(build_schema());
    let mut reader = arrow::csv::ReaderBuilder::new(schema)
        .has_header(true)
        .build(Cursor::new(csv))?;
    let batch = reader.next().unwrap()?;
    Ok(batch)
}

pub fn csv_with_nullable_schema(csv: &[u8]) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::new(vec![
        nullable_field("id",   DataType::Int32),
        nullable_field("name", DataType::Utf8),
        nullable_field("age",  DataType::Int32),
    ]));
    let mut reader = arrow::csv::ReaderBuilder::new(schema)
        .has_header(true)
        .build(Cursor::new(csv))?;
    let batch = reader.next().unwrap()?;
    Ok(batch)
}
```

## 8. Concept: Arrow IPC — The Zero-Copy Wire Format

### Explanation

Arrow IPC is a **binary format** for serializing `RecordBatch`es. It comes in two flavours:

- **Streaming** (`StreamWriter` / `StreamReader`) — back-to-back batches, no random access, ideal for pipelines.
- **File** (`FileWriter` / `FileReader`) — a footer with random-access metadata, ideal for on-disk files.

The streaming format starts with the magic bytes `ARROW1` followed by the schema, then the column data, then `ARROW1` again as a terminator:

```
┌─────────┬───────────────┬────────────────────┬─────────┐
│ ARROW1  │ schema (msg)  │ batch (RecordBatch)│ ARROW1  │
└─────────┴───────────────┴────────────────────┴─────────┘
```

Crucially, **the on-disk layout IS the in-memory layout** — when `StreamReader` reads a batch, it memory-maps or `mmap`s the buffer and the columns are immediately usable as `Int32Array`, `StringArray`, etc. **No copy. No parse.** That's why the format is called "zero-copy".

```rust
use std::io::Cursor;
use arrow::ipc::writer::StreamWriter;
use arrow::ipc::reader::StreamReader;

let mut buf = Vec::new();
{
    let mut writer = StreamWriter::try_new(&mut buf, &batch.schema())?;
    writer.write(&batch)?;
    writer.finish()?;
}

let mut reader = StreamReader::try_new(Cursor::new(&buf), None)?;
let roundtripped = reader.next().unwrap()?;
```

### Python comparison

```python
import pyarrow as pa
sink = pa.BufferOutputStream()
writer = pa.ipc.new_stream(sink, batch.schema)
writer.write_batch(batch)
writer.close()
buf = sink.getvalue()

reader = pa.ipc.open_stream(pa.BufferReader(buf))
roundtripped = reader.read_next_batch()
```

The Python version allocates `pyarrow.Buffer` and `pyarrow.RecordBatch`; the Rust version writes the same bytes into `Vec<u8>` and reads them back as `RecordBatch`. The bytes are identical.

### Applying to our project

```rust
pub fn write_ipc_to_bytes(batch: &RecordBatch) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    {
        let mut writer = StreamWriter::try_new(&mut buf, &batch.schema())?;
        writer.write(batch)?;
        writer.finish()?;
    }
    Ok(buf)
}

pub fn read_ipc_from_bytes(bytes: &[u8]) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    let mut reader = StreamReader::try_new(Cursor::new(bytes), None)?;
    let batch = reader.next().unwrap()?;
    Ok(batch)
}
```

## 9. Concept: Compute Kernels — `sum`, `filter`, `cast`, `slice`

### Explanation

The `arrow::compute` module provides **vectorized kernels** — operations that run over the whole array in one pass, often using SIMD:

| Kernel | Signature | What it does |
|--------|-----------|--------------|
| `sum` | `sum(array) -> Option<ScalarValue>` | Sums a numeric column |
| `cast` | `cast(array, &target_type) -> Result<ArrayRef, ArrowError>` | Type promotion |
| `filter_record_batch` | `filter_record_batch(batch, &mask) -> Result<RecordBatch, ArrowError>` | WHERE-clause |
| `gt` | `gt(left, right) -> Result<BooleanArray, ArrowError>` | Greater-than comparison |

`RecordBatch::slice(offset, length)` is a **zero-copy** way to take a contiguous row range — it doesn't copy data, just adjusts a pointer and length.

### `sum`

```rust
use arrow::compute;

let total_age: Option<i64> = compute::sum(batch.column_by_name("age").unwrap())
    .and_then(|s| s.try_as_i64().ok())
    .flatten();
```

Or, if you want to avoid the `ScalarValue` dance, you can iterate manually:

```rust
let arr = batch.column_by_name("age").unwrap()
    .as_any().downcast_ref::<Int32Array>().unwrap();
let total: i64 = (0..arr.len()).map(|i| arr.value(i) as i64).sum();
```

### `filter_record_batch`

```rust
use arrow::compute;
use arrow::array::Int32Array;

let col = batch.column_by_name("age").unwrap();
let arr = col.as_any().downcast_ref::<Int32Array>().unwrap();
// Build a BooleanArray mask: true where age > threshold
let mask: BooleanArray = (0..arr.len())
    .map(|i| arr.is_valid(i) && arr.value(i) > threshold)
    .collect();
let filtered = compute::filter_record_batch(batch, &mask).unwrap();
```

### `cast`

```rust
let casted_col = compute::cast(batch.column_by_name("age").unwrap(), &DataType::Float64)?;
// Replace the column in the batch (or build a new schema)
```

### `slice`

```rust
let sliced = batch.slice(offset, length);   // zero-copy
```

### Python comparison

```python
import pyarrow.compute as pc
total  = pc.sum(batch.column("age")).as_py()
filt   = pc.filter(batch, pc.greater(batch["age"], 30))
casted = pc.cast(batch["age"], pa.float64())
sliced = batch.slice(offset, length)
```

Every one of those `pc.*` calls is a thin wrapper around the same C++ kernel the Rust `arrow::compute` function calls. In Rust you avoid the GIL, the pyobject wrapping, and the type conversion.

### Applying to our project

```rust
pub fn sum_int32_column(batch: &RecordBatch, col_name: &str) -> Option<i64> {
    let col = batch.column_by_name(col_name)?;
    let arr = col.as_any().downcast_ref::<Int32Array>()?;
    let mut total: i64 = 0;
    for i in 0..arr.len() {
        if arr.is_valid(i) { total += arr.value(i) as i64; }
    }
    Some(total)
}

pub fn filter_batch_by_value(batch: &RecordBatch, col_name: &str, threshold: i32) -> RecordBatch {
    let col = batch.column_by_name(col_name).unwrap();
    let arr = col.as_any().downcast_ref::<Int32Array>().unwrap();
    let mask: BooleanArray = (0..arr.len())
        .map(|i| arr.is_valid(i) && arr.value(i) > threshold)
        .collect();
    compute::filter_record_batch(batch, &mask).unwrap()
}

pub fn slice_batch(batch: &RecordBatch, offset: usize, length: usize) -> RecordBatch {
    batch.slice(offset, length)
}

pub fn cast_int32_to_float64(batch: &RecordBatch, col_name: &str) -> RecordBatch {
    let col = batch.column_by_name(col_name).unwrap();
    let new_col = compute::cast(col, &DataType::Float64).unwrap();
    // Build a new batch with the new column replacing the old one
    let mut new_columns: Vec<ArrayRef> = batch.columns().to_vec();
    if let Some(idx) = batch.schema().index_of(col_name).ok() {
        new_columns[idx] = new_col;
    }
    RecordBatch::try_new(batch.schema(), new_columns).unwrap()
}
```

## 10. Putting It All Together

The complete `workshop/src/lib.rs` is the union of the snippets in sections 3-9. Each function in `lib.rs` starts as `todo!()`. Work through the seven steps in order:

| Step | Topic | Functions |
|------|-------|-----------|
| 1 | Primitive arrays | `build_int32_array`, `build_string_array`, `build_float64_array` |
| 2 | Schema & Field | `build_schema`, `nullable_field` |
| 3 | Builders | `build_int32_with_builder`, `build_string_with_builder`, `build_mixed_batch` |
| 4 | `RecordBatch` queries | `build_sample_batch`, `batch_num_rows`, `batch_column_name`, `batch_schema_string` |
| 5 | CSV → Arrow | `csv_bytes_to_batch`, `csv_with_nullable_schema` |
| 6 | IPC roundtrip | `write_ipc_to_bytes`, `read_ipc_from_bytes` |
| 7 | Compute kernels | `sum_int32_column`, `filter_batch_by_value`, `slice_batch`, `cast_int32_to_float64` |

After implementing each function, run:

```bash
cd workshop && cargo test
```

Expected final output:

```
test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 11. Complete Code Reference

`workshop/Cargo.toml`:

```toml
[package]
name = "arrow"
version = "0.1.0"
edition = "2024"

[dependencies]
arrow = "53"
```

The `arrow` crate at v53 re-exports `arrow-array`, `arrow-schema`, `arrow-csv`, `arrow-ipc`, `arrow-select`, and `arrow-cast` as submodules, so you have everything you need in one import.

`workshop/src/main.rs` demonstrates the whole pipeline (build a batch, cast a column, compute a sum):

```rust
use arrow::array::{Float64Array, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

fn main() {
    // 1. Build three primitive Arrow arrays.
    let ids   = Int32Array::from(vec![1, 2, 3, 4, 5]);
    let names = StringArray::from(vec!["Alice", "Bob", "Carol", "Dave", "Eve"]);
    let ages  = Int32Array::from(vec![30, 25, 35, 28, 42]);

    // 2. Define a schema and assemble the batch.
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("name", DataType::Utf8, true),
        Field::new("age", DataType::Int32, true),
    ]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(ids), Arc::new(names), Arc::new(ages)],
    ).unwrap();

    // 3. Cast the age column to Float64 and sum it.
    let age_f64 = arrow::compute::cast(batch.column_by_name("age").unwrap(), &DataType::Float64).unwrap();
    let arr = age_f64.as_any().downcast_ref::<Float64Array>().unwrap();
    let total: f64 = (0..arr.len()).map(|i| arr.value(i)).sum();
    println!("Total age: {}", total);
}
```

## 12. Summary

| Concept | Rust API | Python Equivalent | Used In |
|---------|----------|-------------------|---------|
| Primitive arrays | `Int32Array::from(vec)` | `pa.array([], type=pa.int32())` | Step 1 |
| Schema | `Schema::new(vec![Field, ...])` | `pa.schema([...])` | Step 2 |
| Field | `Field::new(name, dt, nullable)` | `pa.field(name, type)` | Step 2 |
| Builder | `Int32Builder::append_value(v)` | `pa.Int32Builder().append(v)` | Step 3 |
| `RecordBatch` | `RecordBatch::try_new(schema, columns)` | `pa.record_batch(...)` | Step 4 |
| CSV reader | `arrow::csv::ReaderBuilder` | `pa.csv.open_csv` | Step 5 |
| IPC streaming | `StreamWriter` / `StreamReader` | `pa.ipc.new_stream` | Step 6 |
| Sum | `arrow::compute::sum` | `pc.sum` | Step 7 |
| Filter | `arrow::compute::filter_record_batch` | `pc.filter` | Step 7 |
| Cast | `arrow::compute::cast` | `pc.cast` | Step 7 |
| Slice | `RecordBatch::slice(offset, length)` | `batch.slice(offset, length)` | Step 7 |

## 13. Further Reading

- **Apache Arrow Rust docs** — https://docs.rs/arrow/latest/arrow/
- **Arrow + Parquet: The Zero-Copy Data Stack** — Nikulsinh Rajput, *Medium*, Nov 9 2025. Demonstrates how Arrow (in-memory) and Parquet (on-disk) form a single workflow with no copies.
- **Rust Data Processing: Polars vs DataFusion Performance Comparison** — dasroot.net, Feb 7 2026. Compares the two main consumer frameworks that build on Arrow.
- **DataFusion with Rust and Apache Arrow** — Matthieu L., *Medium* (3-part series on building a DataFusion CLI on top of `arrow::record_batch::RecordBatch`).
- **Apache Arrow cookbook (Python, but applicable concepts)** — https://arrow.apache.org/cookbook/py/

## 14. Exercises

**Easy**: Implement `build_int32_array` and `build_string_array` and verify `cargo test` shows 2 of 20 tests passing.

**Medium**: Implement `csv_bytes_to_batch` and `csv_with_nullable_schema`. Use `data/sample.csv` (or a hand-crafted 5-row string) and verify the batch has the right row count and the right number of columns.

**Hard**: Implement `cast_int32_to_float64` and write a new function `batch_age_average(batch: &RecordBatch) -> Option<f64>` that casts the `age` column to Float64, sums it, and divides by `batch.num_rows()`. The result should be `32.0` for the sample batch (mean of `[30, 25, 35, 28, 42]`).
