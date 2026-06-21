# 🦀 Project 07: Next-Generation Columnar Formats — Beyond Parquet

> **Test-driven approach**: This project includes 4 Cargo sub-projects with progressive unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cargo test --workspace` to watch the pass count grow. Your goal: **all benchmark tests pass** plus write a working warehouse.

## What Are Next-Gen Formats?

Four new columnar formats — F3, Lance, Vortex, Nimble — that solve Parquet's random-access and encoding limitations for AI/ML workloads.

### Python equivalent

```python
import pyarrow.parquet as pq

# Parquet: random access is slow (must decompress entire row group)
table = pq.read_table("events.parquet")
row = table.slice(50_000_000, 1)  # decompresses 128 MB row group for 1 row
```

```rust
// Lance: read 1000 random rows from a 1B-row dataset
let dataset = lance::Dataset::open("events.lance").await?;
let batch: RecordBatch = dataset
    .take(&[50_000_321, 17_888_402, /* ...998 more */], &projection)
    .await?;  // 100x faster than Parquet row-group scan
```

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Parquet's row-group trap | Random access = read full page, throw away 99% |
| 2 | Structural encoding (Lance) | Adaptive encoding per mini-block → O(1) seeks |
| 3 | Cascading compression (Vortex) | Per-chunk codec selection |
| 4 | FlatBuffer metadata | Zero-parse, O(1) column access |
| 5 | Wasm decoders (F3) | Forward compatibility: new encodings work on old engines |
| 6 | Vector search in-file (Lance) | HNSW index embedded in column chunks |
| 7 | Zero-copy versioning (Lance) | ACID, time-travel, schema evolution |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Why Parquet Hits a Wall for AI](#3-why-parquet-hits-a-wall-for-ai)
4. [The Four Contenders](#4-the-four-contenders)
5. [Project Layout](#5-project-layout)
6. [Project 59: Lance Bench](#project-59-lance-bench)
7. [Project 60: Vortex Bench](#project-60-vortex-bench)
8. [Project 61: Nimble Design](#project-61-nimble-design)
9. [Project 62: F3 Concept](#project-62-f3-concept)
10. [Project 63: Warehouse Benchmark](#project-63-warehouse-benchmark)
11. [Benchmark Results](#benchmark-results)
12. [Summary](#summary)
13. [Further Reading](#further-reading)

---

## 1. Introduction

Parquet has been the **de facto columnar format** for data lakes since 2013. It powers Databricks, Snowflake, BigQuery, and every Apache Spark cluster on earth. But it was designed for **OLAP scans** over **dense, structured, scalar data**. The workloads of 2026 are different:

| Workload | Parquet Behavior | Pain Level |
|----------|------------------|------------|
| Full-table scan (OLAP) | ✅ Fast | None |
| Random row access (ML training) | 🟡 Reads full row groups | High — 85% GPU idle time |
| Vector search (RAG, k-NN) | ❌ No native index | Critical |
| Multimodal (images, video, audio) | ❌ Blobs bloat row groups | Critical |
| Wide tables (10k+ columns) | 🟡 Fat footer, slow projection | High |
| Schema evolution | 🟡 Requires rewrite of affected files | Medium |
| S3 / object storage | 🟡 Many small I/Os expensive | Medium |
| Point lookups | 🟡 Read full page | High |

This project teaches the **four new formats** racing to fix these pain points:

- **Lance** (production-ready, Rust-native, multimodal AI focus)
- **Vortex** (incubating at LF AI & Data, "DataFusion of file formats")
- **Nimble** (Meta, wide tables, GPU/SIMD encodings)
- **F3** (research from CMU/Tsinghua, Wasm-decoded self-describing files)

**Python → Rust**: In Python you'd reach for `pyarrow`, `lancedb`, or `pyvortex`; all of these wrap the same Rust libraries you'll learn here.

## 2. Prerequisites

- Completed [Project 55: Parquet](../../03-Parquet/README.md) — comfortable with columnar concepts
- Completed [Project 56: Arrow](../../04-Arrow/README.md) — `RecordBatch` fluency
- Familiarity with async Rust from [Section 5: Concurrency](../../../05-Concurrency/README.md)
- The `tokio`, `lance`, and `vortex` crates are in the workspace `Cargo.toml`

## 3. Why Parquet Hits a Wall for AI

Let's trace what happens when PyTorch calls `dataset[idx]` on a 500 GB Parquet file:

```
Parquet file layout (5 GB row group):
┌─────────────────────────────────────────────────────────┐
│ Footer (schema, stats, min/max for every column)        │  ← read once, parse JSON
├─────────────────────────────────────────────────────────┤
│ Row Group 1 (1 GB, sorted horizontally)                  │
│  ┌──────────────┬──────────────┬──────────────┐         │
│  │ Col A chunk  │ Col B chunk  │ Col C chunk  │         │
│  │ [10k values] │ [10k values] │ [10k values] │         │
│  │ compressed   │ compressed   │ compressed   │         │
│  └──────────────┴──────────────┴──────────────┘         │
│                                                         │
│ Row Group 2 (1 GB)...                                   │
└─────────────────────────────────────────────────────────┘

Request: "give me row 50,000,321"
→ Engine reads footer, locates row group containing row 50M
→ Row group spans ~1 GB; engine must load it
→ Decompress 10k pages of column data
→ Find row 50M, return it
→ Throw away the other 999,999 rows

Wasted I/O: 99.9999% of bytes read
Wasted CPU: full decompression pipeline on garbage data
```

**The three core problems**:

### Problem 1: Read Amplification (the "fat page" problem)
Parquet's smallest unit of compression is the **row group** (typically 128 MB – 1 GB). To fetch a single row, you must read and decompress the entire page. For ML training (shuffled access), this is catastrophic.

**Lance's fix**: 8 MB "disk pages" per column, with **structural encoding** (a B-tree of mini-blocks) so seeking to a row is O(log N) page reads, not O(N).

### Problem 2: Fat Footer (the "wide table" problem)
For a 10,000-column feature store, the Parquet footer contains min/max statistics for every column chunk. Reading the footer alone can take seconds and gigabytes of RAM. PyArrow typically loads **all of it** to answer "what columns exist?"

**Vortex's fix**: FlatBuffer metadata (binary, zero-parse) and a **layout tree** that lets you jump to column 5,000 without parsing columns 1–4,999.

### Problem 3: Rigid Encoding (the "one-size-fits-all" problem)
Parquet lets you pick one codec per column (Snappy, Zstd, Gzip). For mixed data — some sparse, some dense, some strings — that's wasteful. Sparse columns over-compress (wasted CPU); dense strings under-compress (wasted I/O).

**Vortex's fix**: **Cascading compression** — each chunk of N values can pick its own encoding, recursively. A 1M-row column might be: 200k RLE + 300k bit-packed + 500k FSST-compressed.

**F3's fix**: Files carry **WebAssembly decoders** for encodings the engine doesn't natively know. The 30% Wasm overhead is a feature, not a bug — it means the format evolves without coordinating with every engine vendor.

## 4. The Four Contenders

| Format | Backing | Maturity | Best For | Rust Crate |
|--------|---------|----------|----------|------------|
| **Lance** | LanceDB / community | Production (v7, 6.6k ⭐) | ML training, vector search, multimodal | `lance = "0.20"` |
| **Vortex** | SpiralDB → LF AI & Data | Incubating (0.36 stable) | DataFusion/DuckDB acceleration, wide tables | `vortex-file = "0.1"` |
| **Nimble** | Meta | Early (C++ only) | Wide tables, GPU training, feature stores | *(no Rust crate)* |
| **F3** | CMU + Tsinghua (SIGMOD 2026) | Research prototype | Forward compatibility, archive use | `fff-poc` (prototype) |

All four are **columnar**, **Arrow-compatible** (zero-copy), and **self-describing** (schema in file). They differ in:
- **Granularity of access** (Lance/Vortex win random access)
- **Encoding extensibility** (Vortex/Nimble win encoding flexibility)
- **Forward compatibility** (F3 wins via Wasm)
- **Ecosystem** (Lance wins for AI, Vortex wins for OLAP)

## 5. Project Layout

This is a **Cargo workspace** with 5 crates:

```
07-NextGenFormats/
├── Cargo.toml              ← workspace root
├── README.md               ← this file
├── lance-bench/            ← Project 59: full Lance implementation
│   ├── Cargo.toml
│   └── src/{lib.rs, main.rs}
├── vortex-bench/           ← Project 60: full Vortex implementation
│   ├── Cargo.toml
│   └── src/{lib.rs, main.rs}
├── nimble-design/          ← Project 61: Nimble concept + API design
│   ├── Cargo.toml
│   └── src/{lib.rs, main.rs}
├── f3-concept/             ← Project 62: F3 concept + Wasm decoder
│   ├── Cargo.toml
│   └── src/{lib.rs, main.rs}
└── warehouse/              ← Project 63: unified benchmark
    ├── Cargo.toml
    ├── src/{lib.rs, main.rs, bench.rs}
    ├── data/               ← generated fake data
    └── results/            ← benchmark output JSON + Markdown
```

## Project 59: Lance Bench

See [`lance-bench/README.md`](./lance-bench/README.md). Builds a working Lance dataset end-to-end: write batches, scan columns, take random rows, create a vector index, version the dataset, compact files.

## Project 60: Vortex Bench

See [`vortex-bench/README.md`](./vortex-bench/README.md). Builds a working Vortex file with cascading compression: encode with mixed codecs, lazy column projection, layout-tree traversal.

## Project 61: Nimble Design

See [`nimble-design/README.md`](./nimble-design/README.md). No production Rust crate exists. This project teaches Nimble's design by **sketching a hypothetical Rust API** in code, comparing every concept to Parquet and Lance.

## Project 62: F3 Concept

See [`f3-concept/README.md`](./f3-concept/README.md). F3 is research-stage. This project explains the Wasm-decoder concept, builds a tiny mock Wasm module, and discusses the security/performance tradeoffs.

## Project 63: Warehouse Benchmark

See [`warehouse/README.md`](./warehouse/README.md). Generates **1M synthetic e-commerce events**, partitions them by `year=/month=/day=`, writes them with **Lance** and **Vortex** (the two production formats), and benchmarks:

| Benchmark | What it measures |
|-----------|------------------|
| **Write throughput** | Rows/sec for batch ingestion |
| **File size** | Compression ratio on identical data |
| **Sequential scan** | Full-table scan time (OLAP-style) |
| **Column projection** | Time to read 2 of 50 columns |
| **Random take** | Time to fetch 1000 random row indices |
| **Predicate pushdown** | Time to filter on `event_date = X` |
| **Compaction** | Time to merge 10 small files into 1 |
| **Schema evolution** | Time to add 5 new columns without rewriting |
| **Vector search** (Lance only) | Time for k-NN on 384-dim embeddings |

## Benchmark Results

Results from running the warehouse benchmark on an **8-core Linux** machine with **1M synthetic e-commerce events** (100k/partition, 10 partitions), using Arrow 58, Lance v7, Vortex v0.74:

**Machine:** 8 cores, linux  
**Total rows:** 1,000,000  
**Partition size:** 100,000 rows

### Write Throughput

| Format | Duration | Rows/s | File Size | Compression vs Parquet |
|--------|----------|--------|-----------|----------------------|
| Parquet (Snappy) | 7,796 ms | 128,269 | 20.39 MB | baseline |
| Lance (default) | 1,114 ms | 897,538 | 11.74 MB | **7x faster**, 1.74x smaller |
| Vortex (default) | 8,895 ms | 112,411 | 10.21 MB | slightly slower, **2x smaller** |
| Nimble *(mocked)* | — | 800,000+ | ≈1 MB (RLE on categoricals) | conceptual |
| F3 *(mocked)* | — | 600,000+ | ≈1 MB (Wasm-dict) | conceptual |

### Read Benchmarks

| Benchmark | Parquet | Lance | Vortex | Winner |
|-----------|---------|-------|--------|--------|
| **Full scan** (1M rows) | 2,117 ms | 1,052 ms | 662 ms | 🏆 Vortex (3.2x Parquet) |
| **1 partition scan** (100k rows) | 231 ms | 127 ms | 63 ms | 🏆 Vortex (3.7x Parquet) |
| **Column projection** (2 of 6 cols) | 419 ms | 200 ms | — | 🏆 Lance (2.1x Parquet) |
| **Random take** (1000 rows) | 201 ms* | 57 ms | — | 🏆 Lance (3.5x Parquet) |
| **Predicate filter** (purchase events) | 2,358 ms | 1,436 ms | — | 🏆 Lance (1.6x Parquet) |
| **Compaction** (10→1 file) | N/A | 2 ms | — | Lance native |

*\*Parquet random take reads all 100k rows then samples — no native random access.*

### Format Summary

| Property | Parquet | Lance | Vortex |
|----------|---------|-------|--------|
| **Write speed** | ⭐⭐ (128k rows/s) | ⭐⭐⭐⭐⭐ (898k rows/s) | ⭐⭐ (112k rows/s) |
| **Read speed** | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Random access** | ❌ (read row-group) | ✅ (structural encoding) | ✅ (layout tree) |
| **Predicate pushdown** | ⭐ (row-group stats) | ⭐⭐⭐ (mini-block stats) | N/A (not benchmarked) |
| **Compression** | ⭐⭐ (20.4 MB) | ⭐⭐⭐⭐ (11.7 MB) | ⭐⭐⭐⭐⭐ (10.2 MB) |
| **Schema evolution** | ⚠️ rewrite required | ✅ native (add_columns) | N/A |
| **Maturity** | ✅ industry standard (2013) | ✅ production (v7, 2026) | 🟡 incubating (v0.74) |

### Key Takeaways

1. **Lance is the fastest writer** — 7x Parquet write speed, 1.74x better compression
2. **Vortex is the fastest reader** — 3.2x Parquet scan speed, 2x better compression
3. **Lance excels at random access** — structural encoding makes `take()` 3.5x faster than Parquet
4. **Parquet remains dominant** for ecosystem support (Spark, Trino, Hive), but Lance/Vortex offer clear performance wins for AI/ML and high-throughput ETL
5. **Compaction is virtually free** on Lance (2 ms overhead for 10 files → 1)

JSON and Markdown results are written to `warehouse/results/` after running `cargo run --release --bin warehouse`.

## Summary

| Concept | Where Covered | Rust Crate |
|---------|---------------|------------|
| Parquet row-group trap | `lance-bench` step_03_take | `parquet` |
| Structural encoding | `lance-bench` step_04 | `lance` |
| Cascading compression | `vortex-bench` step_05 | `vortex-array` |
| FlatBuffer metadata | `nimble-design` step_02 | (conceptual) |
| Wasm decoders | `f3-concept` step_03 | `wasmtime` (mock) |
| Vector index | `lance-bench` step_07 | `lance::vector` |
| Zero-copy versioning | `lance-bench` step_08 | `lance` |
| Partitioned warehouse | `warehouse` step_01-03 | `lance`, `vortex` |
| Compaction | `warehouse` step_05 | `lance` |

## Further Reading

- [The Data Engineering Podcast E494: Unfreezing The Data Lake](https://www.dataengineeringpodcast.com/future-proof-file-format-evolving-data-lakes-episode-494) — Xinyu Zeng on F3
- [Replacements for Parquet? Anyone?](https://freedium-mirror.cfd/medium.com/@moshederri/replacements-for-parquet-anyone-c66c28cf300e) — Dec 2025, Moshe Derri on Lance/Vortex/Nimble
- [F3: The Open-Source Data File Format for the Future](https://freedium-mirror.cfd/medium.com/@reliabledataengineering/f3-the-future-proof-file-format-that-finally-gets-it-right-0e7f0ddd2e72) — Oct 2025
- [Why there are so many Parquet's alternative file formats?](https://freedium-mirror.cfd/vutr.substack.com/p/why-there-are-so-many-parquets-alternative) — Apr 2026
- [Lance Format v2.2 Benchmarks](https://www.lancedb.com/blog/lance-format-v2-2-benchmarks-half-the-storage-none-of-the-slowdown) — Apr 2026
- [Is Parquet becoming the bottleneck?](https://www.databend.com/blog/category-engineering/2025-09-15-storage-format) — Sep 2025
- [Column Storage for the AI Era](https://sympathetic.ink/2025/12/11/Column-Storage-for-the-AI-era.html) — Julien Le Dem (Parquet PMC), Dec 2025
- [Do GPUs Really Need New Tabular File Formats?](https://arxiv.org/pdf/2602.17335) — Feb 2026
- [F3 paper on ACM](https://dl.acm.org/doi/10.1145/3749163) — SIGMOD 2026

---

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

