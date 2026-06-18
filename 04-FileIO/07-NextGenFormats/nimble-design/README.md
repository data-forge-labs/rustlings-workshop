# Project 61: Nimble — Meta's Wide-Table Columnar Format (Design Study)

> **Test-driven approach**: Each function in `src/lib.rs` starts as a `todo!()` stub. **Goal: all 8 tests pass.**

## What Is Nimble?

Meta's wide-table columnar format — independent column streams with zero-parse FlatBuffer metadata.

### Python equivalent

```python
import pyarrow.parquet as pq

# Parquet: must parse entire footer (250 MB JSON) to find 2 columns
schema = pq.read_schema("features.parquet")  # parses ALL columns
table = pq.read_table("features.parquet", columns=["user_id", "country"])
```

```rust
// Hypothetical Rust API (Nimble has no Rust crate yet):
let mut writer = NimbleWriter::new("features.nimble")?;
let id_stream = writer.add_stream("user_id", Encoding::Dictionary)?;
id_stream.write_int64(&ids)?;
let country_stream = writer.add_stream("country", Encoding::Fsst)?;
country_stream.write_string(&countries)?;
writer.finish()?;
```

## At a Glance

| # | Concept | Rust (sketch) | Python | Why it matters |
|---|---------|---------------|--------|----------------|
| 1 | **Per-stream encoding** | `Encoding::Dictionary` | N/A | One codec per column chunk, not whole column |
| 2 | **Cascading encodings** | `Encoding::Rle` → child `BitPacked` | N/A | Recursive encoding tree, BtrBlocks-style |
| 3 | **FlatBuffer metadata** | `flatbuffers::root::<Manifest>(bytes)` | N/A | Zero-parse, O(1) wide-table access |
| 4 | **Wide-table native** | stream-by-stream writes | N/A | Designed for 10k+ columns |
| 5 | **SIMD-friendly** | FastLanes bit-packing | N/A | Saturate CPU/GPU during training |
| 6 | **OpenZL backend** | uses `openzl` C++ library | N/A | Compression research from Meta |

---

## Setup

```bash
cd 04-FileIO/07-NextGenFormats/nimble-design
cargo test
```

## Implementation Steps

### Step 01 — Per-stream encoding model
Each column is a **stream**. Each stream has an **encoding** that can recursively contain child streams. Parquet's whole-column codec is replaced by a per-chunk decision tree.

### Step 02 — Writer API sketch
```rust
pub struct NimbleWriter { /* ... */ }
impl NimbleWriter {
    pub fn new(path: &str) -> Result<Self>;
    pub fn add_stream(&mut self, name: &str, encoding: Encoding) -> Result<StreamBuilder<'_>>;
    pub fn finish(self) -> Result<()>;
}
pub struct StreamBuilder<'a> { /* ... */ }
impl StreamBuilder<'_> {
    pub fn write_int64(&mut self, values: &[i64]) -> Result<()>;
    pub fn write_string(&mut self, values: &[&str]) -> Result<()>;
    pub fn write_bool(&mut self, values: &[bool]) -> Result<()>;
}
```

### Step 03 — Reader API sketch
```rust
pub struct NimbleReader { /* ... */ }
impl NimbleReader {
    pub fn open(path: &str) -> Result<Self>;
    pub fn stream(&self, name: &str) -> Result<StreamRef>;
    pub fn field_names(&self) -> &[String];
}
pub struct StreamRef { /* ... */ }
impl StreamRef {
    pub fn as_int64(&self) -> Result<&[i64]>;
    pub fn as_string(&self) -> Result<Vec<String>>;
}
```

### Step 04 — Cascading vs fixed codec
| Aspect | Parquet (fixed) | Nimble (cascading) |
|--------|------------------|---------------------|
| Codec choice | One per column | One per chunk, recursive |
| Sparse data | Snappy on zeros (waste) | RunEnd → Constant in tail |
| Strings | Dictionary (slow decode) | FSST (10x faster decode) |
| Integers | Plain (no compression) | BitPacked/FrameOfReference |
| Decision | Engineer picks | Engine samples and picks |

### Step 05 — FlatBuffer metadata
Parquet's footer is **Thrift** (slow to parse). Nimble's manifest is **FlatBuffer** (zero-parse, memory-mapped). For 10k columns, this is the difference between an 8-second startup and a 50 ms startup.

### Step 06 — SIMD/GPU encoding
Nimble's `BitPacked` encoding uses the [FastLanes](https://github.com/cwida/FastLanes) library which is SIMD-friendly (decompresses 1024 values into 16 SIMD registers in one go). This is the encoding that Meta's recommendation models train against. The encoding is also GPU-friendly — the GPU can keep the compressed data in VRAM and decompress in registers.

### Step 07 — Encoding recommendation (real implementation)
```rust
use nimble_design::{Encoding, recommend_encoding_for_sample};

let v = vec![1000_i64, 1001, 1002, 1003, 1004, 1005];
let enc = recommend_encoding_for_sample(&v);
// enc == Encoding::BitPacked (range < 256)
```

### Step 08 — Stream manifest (real implementation)
```rust
use nimble_design::{Stream, Encoding, stream_to_json};

let s = Stream {
    name: "user_id".into(),
    encoding: Encoding::Dictionary,
    byte_size: 8192,
    null_count: 0,
};
let json = stream_to_json(&s);
```

---

## Complete Code Reference

`src/lib.rs` is fully self-contained. It contains:
- `Encoding` enum (real impl)
- `Stream` struct (real impl)
- `Encoding::recommend_for` (real impl)
- `stream_to_json` (real impl, mimics FlatBuffer manifest)
- 6 doc-returning functions (todo!() → your answer)

## Exercises

1. **Easy**: Add a new `Encoding::Delta` variant and update `recommend_for` to use it for monotonic deltas.
2. **Medium**: Implement a `Stream::estimated_size` method that returns a lower bound on the compressed byte size given the encoding.
3. **Hard**: Sketch a `NimbleWriter` struct with `add_stream` and `finish` methods (no real I/O, just struct design + unit tests).

---

## Further Reading

- [Nimble on GitHub](https://github.com/facebookincubator/nimble)
- [Nimble talk (YouTube)](https://www.youtube.com/watch?v=bISBNVtXZ6M)
- [Nimble and Lance: The Parquet Killers](https://criccomini.spicytakes.org/post/2024-05-13-nimble-and-lance-parquet-killers) — Chris Riccomini, May 2024
- [Replacements for Parquet? Anyone?](https://freedium-mirror.cfd/medium.com/@moshederri/replacements-for-parquet-anyone-c66c28cf300e) — Dec 2025
- [Hetz.vc: Unleashing GenAI with Nimble](https://www.hetz.vc/news/unleashing-genai-how-a-next-gen-data-format-is-revolutionizing-ai-data-storage)
