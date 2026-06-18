# Project 60: Vortex — Extensible Columnar Format with Cascading Compression

> **Test-driven approach**: Each function in `src/lib.rs` starts as a `todo!()` stub. **Goal: all 8 tests pass.**

## What Is Vortex?

An extensible columnar format with cascading compression — each chunk picks its own encoding for optimal storage.

### Python equivalent

```python
import vortex

# Parquet: one codec per column (Snappy on everything)
# Vortex: cascading compression adapts to data shape per chunk
vx.write("data.vortex", pa_table)  # automatic encoding selection
```

```rust
// Python: vortex-data
// vx.write("data.vortex", pa_table)   // automatic sample-based encoding
// vx.read("data.vortex", columns=["value"]).to_pylist()

// Rust:
let session = VortexSession::default();
let file = VortexFile::open(&session, "data.vortex").await?;
let array = file.read_array().await?;
let value_col = array.children()[2].clone();  // zero-copy column access
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | **Vortex session** | `VortexSession::default()` | `vx.connect()` | Allocator, registry, config |
| 2 | **Write file** | `VortexFile::new(writer).write(...).await` | `vx.write(...)` | Async, buffered |
| 3 | **Read file** | `VortexFile::open(session, path).await` | `vx.read(...)` | Lazy metadata |
| 4 | **Arrow ↔ Vortex** | `array.into_arrow()` | `vx.read().to_arrow()` | Zero-copy |
| 5 | **Footer (postscript)** | `file.footer()` | `vx.info()` | Last 64KB read, then 0-parse |
| 6 | **Cascading compression** | `CompactCompressor` | automatic | BtrBlocks-style adaptive |
| 7 | **Layout tree** | `array.children()` | N/A | Recursive layouts |
| 8 | **Compute kernels** | `vortex::compute::sum` | `vx.compute.sum` | Operate on encoded data |

---

## Setup

```bash
cd 04-FileIO/07-NextGenFormats/vortex-bench
cargo test
```

## Implementation Steps

### Step 01 — Write a Vortex file
Convert an Arrow `RecordBatch` to a Vortex `ArrayRef` (a `StructArray` wrapping typed children), then write it with the file API.

```rust
use vortex::array::{Array, ArrayRef, IntoArray, StructArray};
use vortex::dtype::DType;
use vortex::file::{VortexFile, WriteOptions};
use vortex::stream::ArrayStreamExt;
use vortex::ArraySession;
use std::sync::Arc;

pub async fn write_vortex_file(path: &str, batch: RecordBatch) -> Result<u64> {
    let session = ArraySession::default();
    // Convert each column
    let columns: Vec<(Arc<str>, ArrayRef)> = batch
        .columns()
        .iter()
        .zip(batch.schema().fields().iter())
        .map(|(col, field)| {
            let vx = vortex::arrow::FromArrowType::from_arrow(col.clone());
            (field.name().as_str().into(), vx.into_array())
        })
        .collect();
    let struct_array = StructArray::try_new(
        DType::Struct(batch.schema().fields().iter().map(|f| f.name().as_str().into()).collect()),
        columns,
        batch.num_rows(),
        Validity::NonNullable,
    )?;
    // Write
    let file = std::fs::File::create(path)?;
    VortexFile::new(WriteOptions::default())
        .write(&session, file, struct_array.into_array())
        .await?;
    Ok(std::fs::metadata(path)?.len())
}
```

(For real implementation, refer to Vortex 0.74 API. The exact types/methods evolve — the warehouse crate is the canonical benchmark.)

### Step 02 — Read row count
### Step 03 — Vortex → Arrow (zero-copy)
### Step 04 — Inspect file structure
### Step 05 — Cascading compression
### Step 06 — Project a column
### Step 07 — Sum on encoded data
### Step 08 — List field names

---

## Exercises

1. **Easy**: Add `pub async fn list_segment_count(path: &str) -> Result<usize>` using `VortexFile::footer()`.
2. **Medium**: Implement filter pushdown via `vortex::expr::gt` and the scan builder.
3. **Hard**: Compare file size of `write_vortex_file` vs `write_with_cascading_compression` on the same data.

---

## Further Reading

- [Vortex on docs.rs](https://docs.rs/vortex/latest/vortex/)
- [Vortex file format spec](https://docs.vortex.dev/specs/file-format)
- [GitHub: vortex-data/vortex](https://github.com/vortex-data/vortex)
- [Vortex: Efficient Columnar Storage for Hot Data](https://www.linkedin.com/posts/lukekim_datafusion-spiceai-data-activity-7417019189477126144-1TRe) — Jan 2026
- [Vortex performance benchmarks](https://bench.vortex.dev)
