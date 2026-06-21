# Project 59: Lance — Open Lakehouse Format for Multimodal AI

> **Test-driven approach**: Each function in `src/lib.rs` starts as a `todo!()` stub. Implement step by step. **Goal: all 8 tests pass.**

## What Is Lance?

An open lakehouse format optimized for ML training — 100x faster random access than Parquet for AI workloads.

### Python equivalent

```python
import lance

ds = lance.dataset("events.lance")
# Random row access: still fast due to disk-page architecture
batch = ds.take([50_000_321, 17_888_402], columns=["id", "value"])
```

```rust
// Python: pylance
// ds = lance.dataset("events.lance")
// batch = ds.take([50_000_321, 17_888_402], columns=["id","value"]).to_pandas()

// Rust:
let dataset = lance::Dataset::open("events.lance").await?;
let batch: RecordBatch = dataset
    .take(&[50_000_321, 17_888_402], &["id", "value"])
    .await?;
```

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Lance dataset | Open an on-disk dataset |
| 2 | Write & append | Initial creation, zero-copy where possible |
| 3 | Random take | 100x faster than Parquet |
| 4 | Scan + filter | Predicate pushdown |
| 5 | Vector index | In-file HNSW/IVF |
| 6 | Versioning | ACID, time-travel |

---

## Setup

This is a Cargo workspace member. From the workspace root:

```bash
cd 04-FileIO/07-NextGenFormats/lance-bench
cargo build --release
cargo test
```

Dependencies are managed by the workspace `Cargo.toml`.

## Implementation Steps

### Step 01 — Write initial dataset
Open a Lance dataset, write a `RecordBatch`.

```rust
use lance::dataset::{WriteParams, Dataset};
use lance::arrow::RecordBatchIterator;

pub async fn write_initial_dataset(path: &str, batch: RecordBatch) -> Result<String> {
    let schema = batch.schema();
    let reader = RecordBatchIterator::new(vec![Ok(batch)], schema);
    let dataset = Dataset::write(
        reader,
        path,
        WriteParams::default(),
    ).await?;
    Ok(dataset.uri().to_string())
}
```

### Step 02 — Count rows
```rust
use lance::Dataset;

pub async fn count_rows(path: &str) -> Result<usize> {
    let dataset = Dataset::open(path).await?;
    Ok(dataset.count_rows().await?)
}
```

### Step 03 — Random take (the killer feature)
```rust
use lance::Dataset;
use arrow_array::RecordBatch;
use std::sync::Arc;

pub async fn take_rows(path: &str, indices: &[u32]) -> Result<RecordBatch> {
    let dataset = Dataset::open(path).await?;
    // projection = None means "all columns"
    let batch = dataset.take(indices, &[]).await?;
    Ok(batch)
}
```

### Step 04 — Filter with predicate pushdown
Lance uses DataFusion's `Expr` for predicates. We build `value > threshold`:

```rust
use lance::Dataset;
use datafusion::prelude::{col, lit, Expr};
use datafusion::logical_expr::Operator;
use std::sync::Arc;

pub async fn filter_by_value(path: &str, threshold: f32) -> Result<RecordBatch> {
    let dataset = Dataset::open(path).await?;
    let pred = col("value").gt(lit(threshold as f64));
    let mut scanner = dataset.scan();
    scanner.filter(pred);
    let stream = scanner.try_into_stream().await?;
    // Collect stream into one batch
    use futures::StreamExt;
    let batches: Vec<RecordBatch> = stream.collect::<Vec<_>>().await
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    // Concatenate
    Ok(arrow_select::concat::concat_batches(&batches[0].schema(), &batches)?)
}
```

### Step 05 — Column projection
```rust
pub async fn project_two_columns(path: &str) -> Result<RecordBatch> {
    let dataset = Dataset::open(path).await?;
    let mut scanner = dataset.scan();
    scanner.project(&["id", "value"]);
    let stream = scanner.try_into_stream().await?;
    use futures::StreamExt;
    let batches: Vec<RecordBatch> = stream.collect::<Vec<_>>().await
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(arrow_select::concat::concat_batches(&batches[0].schema(), &batches)?)
}
```

### Step 06 — Append batch
```rust
use lance::dataset::Dataset;

pub async fn append_batch(path: &str, new_batch: RecordBatch) -> Result<usize> {
    let mut dataset = Dataset::open(path).await?;
    let schema = new_batch.schema();
    let reader = lance::arrow::RecordBatchIterator::new(vec![Ok(new_batch)], schema);
    dataset.append(reader).await?;
    Ok(dataset.count_rows().await?)
}
```

### Step 07 — Create scalar index
```rust
use lance::index::IndexType;
use lance::dataset::Dataset;

pub async fn create_index_on_id(path: &str) -> Result<()> {
    let mut dataset = Dataset::open(path).await?;
    dataset.create_index(
        &["id"],
        IndexType::Scalar,
        None,
        &lance::index::scalar::ScalarIndexParams::default(),
        false,
    ).await?;
    Ok(())
}
```

### Step 08 — Read version
```rust
use lance::Dataset;

pub async fn current_version(path: &str) -> Result<u64> {
    let dataset = Dataset::open(path).await?;
    Ok(dataset.version().version)
}
```

---

## Complete Code Reference

See `src/lib.rs` — every function has a `todo!()` stub and a corresponding test.

Run the full suite:
```bash
cargo test --release
```

Expected:
```
test tests::step_01_write_initial_dataset ... ok
test tests::step_02_count_rows ... ok
test tests::step_03_take_random_rows ... ok
test tests::step_04_filter_by_value ... ok
test tests::step_05_project_two_columns ... ok
test tests::step_06_append_batch ... ok
test tests::step_07_create_index ... ok
test tests::step_08_version ... ok

test result: ok. 8 passed; 0 failed
```

---

## Exercises

1. **Easy**: Add `pub async fn list_columns(path: &str) -> Result<Vec<String>>` that returns the field names of the dataset.
2. **Medium**: Implement `pub async fn take_with_projection(path: &str, indices: &[u32], cols: &[&str]) -> Result<RecordBatch>` that combines random access with column pruning.
3. **Hard**: Use `dataset.optimize()` to compact small files, then compare `count_rows().await?` before/after.

---

## Further Reading

- [Lance format spec](https://lance.org/format/file/)
- [Benchmarking Random Access in Lance](https://www.lancedb.com/blog/benchmarking-random-access-in-lance)
- [Lance Format v2.2 Benchmarks: Half the Storage](https://www.lancedb.com/blog/lance-format-v2-2-benchmarks-half-the-storage-none-of-the-slowdown)
- [GitHub: lance-format/lance](https://github.com/lance-format/lance) (6.6k ⭐, 72% Rust)
- [Lance Rust API on docs.rs](https://docs.rs/lance/latest/lance/)
