# 🦀 OpenDAL Storage — One API, All Backends

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 17 tests pass**.

---

## What Is OpenDAL?

Apache OpenDAL is a unified storage abstraction — one `Operator` API to read, write, list, and delete files across 50+ backends (local FS, S3, GCS, Azure Blob, HDFS, etc.). The backend changes; the code doesn't.

### Python equivalent

```python
# Python: different libraries for different backends
import boto3          # S3
import gcsfs          # GCS
from pathlib import Path  # local FS

# S3
s3 = boto3.client('s3')
s3.put_object(Bucket='my-bucket', Key='data.csv', Body=b'...')

# Local
Path('data.csv').write_bytes(b'...')

# OpenDAL unifies this: one API, swap the backend
# (no Python equivalent — Rust-only crate)
```

In this project you'll learn to build storage-agnostic data pipelines in Rust — and along the way you'll discover the **Operator abstraction**, **composable layers** (retry, logging, metrics), and **cross-backend pipelines** (Local FS → MinIO S3).

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | `Operator` abstraction | One API for all storage backends |
| 2 | Service builders (`Fs`, `Memory`, `S3`) | Swap backends via config, not code |
| 3 | Feature flags | Compile only the backends you use |
| 4 | `RetryLayer` | Resilient pipelines against transient failures |
| 5 | `LoggingLayer` | Debug every I/O operation in production |
| 6 | `MetricsLayer` | Monitor throughput, latency, error rates |
| 7 | Cross-backend pipeline | Read from FS, write to S3 — same code |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Setup: Create the Project from Scratch](#3-setup-create-the-project-from-scratch)
4. [Concept: The Operator Abstraction](#4-concept-the-operator-abstraction)
5. [Concept: Service Builders](#5-concept-service-builders)
6. [Concept: Basic CRUD Operations](#6-concept-basic-crud-operations)
7. [Concept: Composable Layers](#7-concept-composable-layers)
8. [Concept: Cross-Backend Pipelines](#8-concept-cross-backend-pipelines)
9. [Step-by-Step Implementation](#9-step-by-step-implementation)
10. [Running End-to-End](#10-running-end-to-end)
11. [Summary](#11-summary)

---

## 1. Introduction

Every data pipeline reads from *somewhere* and writes to *somewhere*. In Python, that means `boto3` for S3, `google-cloud-storage` for GCS, `pathlib` for local files — each with its own API, error types, and retry semantics.

OpenDAL solves this with a single `Operator` type. The same `op.read("file.csv")` works whether the file is on your laptop, in S3, or on HDFS. You change one line of configuration, not the entire pipeline.

In our Section 14 data infrastructure stack:

- Projects 01–08 talk to specific services (Kafka, Postgres, Redis, ClickHouse)
- Project 09 (this one) provides **storage-agnostic file I/O** — the missing piece
- `main.rs` demonstrates a **Local FS → MinIO** pipeline (the same pattern used in production ETL)

## 2. Prerequisites

- Rust 1.75+ (1.96 recommended)
- Docker Compose (for MinIO — already in `14-DataInfrastructure/docker-compose.yml`)
- Concept: `Result<T, E>` (Section 2)
- Concept: `async`/`.await` + `tokio` (Section 5)
- Concept: Traits and closures (Sections 2–3)

## 3. Setup: Create the Project from Scratch

```bash
# From the repo root
cd 14-DataInfrastructure
cargo new --lib open_dal_storage
cd open_dal_storage

# Replace Cargo.toml with the workshop version
# (see workshop/Cargo.toml for dependencies)

# Copy the test stubs
cp ../09-OpenDalStorage/workshop/src/lib.rs src/lib.rs

# Run tests — all should fail with "not yet implemented"
cargo test
# test result: FAILED. 0 passed; 17 failed
```

## 4. Concept: The Operator Abstraction

The `Operator` is OpenDAL's core type. It wraps a **service backend** (FS, S3, Memory) and exposes a uniform API.

```
┌──────────────────────────────────────────────────────────────┐
│                     Your Pipeline Code                        │
│  op.write("data/file.csv", bytes)                            │
│  op.read("data/file.csv")                                    │
│  op.list("data/")                                            │
└───────────────────────────┬──────────────────────────────────┘
                            │
                    ┌───────▼───────┐
                    │   Operator    │  ← the single entry point
                    └───────┬───────┘
                            │
              ┌─────────────┼─────────────┐
              │             │             │
        ┌─────▼─────┐ ┌────▼────┐ ┌──────▼──────┐
        │ services:: │ │services::│ │ services::  │
        │    Fs      │ │  Memory │ │     S3      │
        │ (local)    │ │ (RAM)   │ │ (MinIO/S3)  │
        └───────────┘ └─────────┘ └─────────────┘
```

### Python comparison

In Python, you'd use different libraries:

| Backend | Python | Rust (OpenDAL) |
|---------|--------|-----------------|
| Local FS | `pathlib.Path` | `services::Fs` |
| S3 | `boto3` / `smart_open` | `services::S3` |
| GCS | `gcsfs` | `services::Gcs` |
| Azure Blob | `azure-storage-blob` | `services::Azblob` |
| Memory (testing) | `io.BytesIO` | `services::Memory` |

### Creating an operator

```rust
use opendal::{Operator, services};

// Local filesystem
let op = Operator::new(services::Fs::default())?
    .finish();

// In-memory (great for tests)
let op = Operator::new(services::Memory::default())?
    .finish();

// S3 / MinIO
let op = Operator::new(
    services::S3::default()
        .bucket("my-bucket")
        .region("us-east-1")
        .endpoint("http://localhost:9000")
        .access_key_id("minioadmin")
        .secret_access_key("minioadmin")
        .allow_anonymous()
)?
.finish();
```

### Applying to our project

In `lib.rs`, implement `operator_fs` and `operator_memory`:

```rust
pub fn operator_fs(root: &str) -> Result<Operator> {
    let op = Operator::new(services::Fs::default().root(root))?
        .finish();
    Ok(op)
}
```

## 5. Concept: Service Builders

Each backend has a **builder** that configures connection details. The builder pattern lets you chain configuration:

```rust
// S3 builder — configure bucket, region, endpoint, credentials
let builder = services::S3::default()
    .bucket("dataeng")
    .region("us-east-1")
    .endpoint("http://localhost:9000")
    .access_key_id("minioadmin")
    .secret_access_key("minioadmin")
    .allow_anonymous();
```

### Feature flags

OpenDAL uses Cargo feature flags to compile only the backends you need. In `Cargo.toml`:

```toml
[dependencies]
opendal = { version = "0.57", features = [
    "services-fs",       # local filesystem
    "services-memory",   # in-memory (testing)
    "services-s3",       # S3 / MinIO / any S3-compatible
    "layers-retry",      # retry layer
    "layers-logging",    # logging layer
    "layers-metrics",    # metrics layer
] }
```

If you only need local FS, remove `services-s3` and `services-memory` — the binary gets smaller and compiles faster.

## 6. Concept: Basic CRUD Operations

OpenDAL's `Operator` provides four core operations:

| Operation | Method | Returns |
|-----------|--------|---------|
| Write | `op.write(path, data)` | `Metadata` (content_length, etc.) |
| Read | `op.read(path)` | `Buffer` (convert with `.to_vec()`) |
| List | `op.list(prefix)` | `Vec<Entry>` (paths + metadata) |
| Stat | `op.stat(path)` | `Metadata` (size, type, etag) |
| Delete | `op.delete(path)` | `()` |
| Copy | `op.copy(from, to)` | `Metadata` |
| Exists | `op.exists(path)` | `bool` |
| Create Dir | `op.create_dir(path)` | `()` |

### Python comparison

```python
# Python
from pathlib import Path
Path("data/file.csv").write_bytes(b"col1,col2\n1,2")
data = Path("data/file.csv").read_bytes()
entries = list(Path("data/").iterdir())

# Rust (OpenDAL) — same API for all backends
op.write("data/file.csv", b"col1,col2\n1,2".to_vec()).await?;
let data = op.read("data/file.csv").await?;
let entries = op.list("data/").await?;
```

## 7. Concept: Composable Layers

Layers wrap an operator to add cross-cutting concerns **without changing your pipeline code**:

```
┌──────────────────────────────────────────────┐
│  Your pipeline code (unchanged)              │
│  op.write(...) / op.read(...)                │
└──────────────────┬───────────────────────────┘
                   │
         ┌─────────▼─────────┐
         │   MetricsLayer    │  ← tracks operation counts, latencies
         └─────────┬─────────┘
                   │
         ┌─────────▼─────────┐
         │   LoggingLayer    │  ← logs every operation
         └─────────┬─────────┘
                   │
         ┌─────────▼─────────┐
         │   RetryLayer      │  ← retries transient failures (3x, 1s)
         └─────────┬─────────┘
                   │
         ┌─────────▼─────────┐
         │   S3 / FS / etc.  │  ← the actual backend
         └───────────────────┘
```

### Why layers matter for data pipelines

- **RetryLayer**: S3 returns 503 under load. Retry 3 times with 1s backoff — your pipeline doesn't crash.
- **LoggingLayer**: Every read/write is logged. At 3 AM, you know exactly which file failed.
- **MetricsLayer**: Export to Prometheus. Dashboard shows throughput, error rate, p99 latency.

### Python comparison

Python uses middleware/decorators for this:

```python
# Python: tenacity for retries
from tenacity import retry, stop_after_attempt
@retry(stop=stop_after_attempt(3))
def read_from_s3(key):
    return s3.get_object(Bucket=bucket, Key=key)

# Rust (OpenDAL): one line, all operations covered
let op = op.layer(RetryLayer::default());
```

### Layer order

Layer order matters. Outer layers wrap inner ones. The call chain is:

```
metrics → logging → retry → service
```

Apply them in this order (innermost first):

```rust
let op = layered_operator(op);
// which does:
// op.layer(RetryLayer)      ← innermost
//   .layer(LoggingLayer)    ← middle
//   .layer(MetricsLayer)    ← outermost
```

## 8. Concept: Cross-Backend Pipelines

The real power: read from one backend, write to another, with the same code.

```
┌─────────────────┐          ┌─────────────────┐
│   Local FS      │  copy    │   MinIO S3      │
│   Operator      │ ──────►  │   Operator      │
│                 │          │                 │
│  src/data.csv   │          │  raw/data.csv   │
│  src/events.csv │          │  raw/events.csv │
└─────────────────┘          └─────────────────┘
```

### Python comparison

```python
# Python: two different libraries, two APIs
import boto3
from pathlib import Path

for file in Path("src/").iterdir():
    s3.upload_file(str(file), "my-bucket", f"raw/{file.name}")

# Rust (OpenDAL): one function works for any src/dst pair
pipeline_copy(&local_op, "src/", &s3_op, "raw/").await?;
```

### Applying to our project

`pipeline_copy` lists files in the source prefix, reads each, and writes to the destination:

```rust
pub async fn pipeline_copy(
    src_op: &Operator, src_prefix: &str,
    dst_op: &Operator, dst_prefix: &str,
) -> Result<usize> {
    let entries = src_op.list(src_prefix).await?;
    let mut count = 0;
    for entry in &entries {
        let data = src_op.read(entry.path()).await?;
        // Replace source prefix with destination prefix
        let dst_key = format!("{}{}", dst_prefix, &entry.path()[src_prefix.len()..]);
        dst_op.write(&dst_key, data).await?;
        count += 1;
    }
    Ok(count)
}
```

## 9. Step-by-Step Implementation

Open `workshop/src/lib.rs` and implement each function, running `cargo test` after each step.

### Step 1: Operator builders (2 tests)

Implement `operator_fs` and `operator_memory`:

```rust
pub fn operator_fs(root: &str) -> Result<Operator> {
    let op = Operator::new(services::Fs::default().root(root))?
        .finish();
    Ok(op)
}

pub fn operator_memory() -> Result<Operator> {
    let op = Operator::new(services::Memory::default())?
        .finish();
    Ok(op)
}
```

```bash
cargo test step_01
# test step_01_operator_builders::test_operator_memory_creation ... ok
# test step_01_operator_builders::test_operator_fs_creation ... ok
```

### Step 2: Write and read (3 tests)

```rust
pub async fn write_file(op: &Operator, path: &str, data: Vec<u8>) -> Result<u64> {
    let meta = op.write(path, data).await?;
    Ok(meta.content_length())
}

pub async fn read_file(op: &Operator, path: &str) -> Result<Vec<u8>> {
    let data = op.read(path).await?;
    Ok(data.to_vec())
}
```

```bash
cargo test step_02
# test step_02_write_read::test_write_returns_byte_count ... ok
# test step_02_write_read::test_read_returns_written_data ... ok
# test step_02_write_read::test_read_nonexistent_file ... ok
```

### Step 3: List and stat (3 tests)

```rust
pub async fn list_dir(op: &Operator, prefix: &str) -> Result<Vec<String>> {
    let entries = op.list(prefix).await?;
    let mut paths: Vec<String> = entries.iter().map(|e| e.path().to_string()).collect();
    paths.sort();
    Ok(paths)
}

pub async fn stat_file(op: &Operator, path: &str) -> Result<(u64, bool, bool, Option<String>)> {
    let meta = op.stat(path).await?;
    Ok((
        meta.content_length(),
        meta.is_file(),
        meta.is_dir(),
        meta.etag().map(|s| s.to_string()),
    ))
}
```

```bash
cargo test step_03
# test step_03_list_stat::test_list_empty_directory ... ok
# test step_03_list_stat::test_list_with_files ... ok
# test step_03_list_stat::test_stat_file ... ok
```

### Step 4: Copy and delete (3 tests)

```rust
pub async fn copy_file(op: &Operator, src: &str, dst: &str) -> Result<()> {
    op.copy(src, dst).await?;
    Ok(())
}

pub async fn delete_file(op: &Operator, path: &str) -> Result<()> {
    op.delete(path).await?;
    Ok(())
}
```

```bash
cargo test step_04
# test step_04_copy_delete::test_copy_file ... ok
# test step_04_copy_delete::test_delete_file ... ok
# test step_04_copy_delete::test_delete_nonexistent_is_ok ... ok
```

### Step 5: Composable layers (4 tests)

```rust
use opendal::layers::*;

pub fn with_retry_layer(op: Operator) -> Operator {
    op.layer(RetryLayer::default())
}

pub fn with_logging_layer(op: Operator) -> Operator {
    op.layer(LoggingLayer::default())
}

pub fn with_metrics_layer(op: Operator) -> Operator {
    op.layer(MetricsLayer::default())
}

pub fn layered_operator(op: Operator) -> Operator {
    op.layer(RetryLayer::default())
        .layer(LoggingLayer::default())
        .layer(MetricsLayer::default())
}
```

```bash
cargo test step_05
# test step_05_layers::test_retry_layer_does_not_break_read_write ... ok
# test step_05_layers::test_logging_layer_does_not_break_read_write ... ok
# test step_05_layers::test_metrics_layer_does_not_break_read_write ... ok
# test step_05_layers::test_all_layers_combined ... ok
```

### Step 6: Cross-backend pipeline (3 tests)

```rust
pub async fn pipeline_copy(
    src_op: &Operator,
    src_prefix: &str,
    dst_op: &Operator,
    dst_prefix: &str,
) -> Result<usize> {
    let entries = src_op.list(src_prefix).await?;
    let mut count = 0;
    for entry in &entries {
        let src_path = entry.path();
        let data = src_op.read(src_path).await?;
        let dst_path = format!("{}{}", dst_prefix, &src_path[src_prefix.len()..]);
        dst_op.write(&dst_path, data).await?;
        count += 1;
    }
    Ok(count)
}
```

```bash
cargo test step_06
# test step_06_pipeline::test_pipeline_memory_to_memory ... ok
# test step_06_pipeline::test_pipeline_fs_to_memory ... ok
# test step_06_pipeline::test_pipeline_fs_to_minio ... ok (requires MinIO)
```

## 10. Running End-to-End

### Run all tests

```bash
cargo test
# 17 passed, 0 failed
```

### Run the Local FS → MinIO pipeline

```bash
# 1. Start MinIO (from 14-DataInfrastructure/)
docker compose up -d minio

# 2. Wait for healthcheck
docker compose ps minio

# 3. Run the pipeline
cargo run --release
# INFO opendal_storage: Starting Local FS → MinIO pipeline...
# INFO opendal_storage: files_copied=2 Pipeline complete
# INFO opendal_storage: Files in MinIO after pipeline entries=["raw/inventory.csv", "raw/orders.csv"]
```

### What the pipeline does

1. Creates sample CSV files in a local temp directory
2. Builds a **local FS operator** (reads from disk)
3. Builds a **MinIO S3 operator** (writes to `http://localhost:9000/dataeng`)
4. Calls `pipeline_copy` — reads from local FS, writes to MinIO
5. Lists and previews files from MinIO to verify

## 11. Summary

| Concept | What you learned |
|---------|-----------------|
| `Operator` | The single entry point for all storage operations |
| Service builders | `Fs`, `Memory`, `S3` — configure the backend, not the code |
| Feature flags | `services-fs`, `services-s3` — compile only what you use |
| CRUD operations | `read`, `write`, `list`, `stat`, `delete`, `copy` |
| `RetryLayer` | Automatic retries for transient failures |
| `LoggingLayer` | Structured logging for every I/O operation |
| `MetricsLayer` | Operation counts, latencies, bytes for monitoring |
| `pipeline_copy` | Cross-backend data movement with the same API |

### Python → Rust mental model

| Python concept | Rust equivalent |
|----------------|-----------------|
| `boto3.client('s3')` | `Operator::new(services::S3::default())` |
| `Path('file').read_bytes()` | `op.read("file").await?` |
| `smart_open.open('s3://...')` | Same `Operator` API, different service builder |
| `tenacity.retry(stop=3)` | `op.layer(RetryLayer::default())` |
| `logging.info(...)` | `LoggingLayer` auto-logs every operation |

## Exercises

**Easy** — Modify `pipeline_copy` to skip `.csv` files (only copy `.json` files). Add a test.

**Medium** — Add a `pipeline_copy_with_retry` function that wraps the destination operator with `RetryLayer` before copying. Verify it works with a memory-to-memory test.

**Hard** — Add a `pipeline_stats` function that returns `PipelineStats { files_copied: usize, bytes_written: u64, duration_ms: u64 }`. Use `std::time::Instant` for timing.
