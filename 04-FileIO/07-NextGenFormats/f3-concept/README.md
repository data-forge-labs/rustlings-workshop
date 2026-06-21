# Project 62: F3 — The Future-Proof File Format (Concept Study)

> **Test-driven approach**: Each function in `src/lib.rs` starts as a `todo!()` stub. **Goal: all 8 tests pass.**

## What Is F3?

A future-proof file format that embeds WebAssembly decoders — files are self-describing AND self-decoding.

### Python equivalent

```python
# Parquet: format versions can't be read by older engines
import pyarrow.parquet as pq

# If Parquet 2.10 adds a new encoding, Spark 2.7 can't read it
# F3 solves this by embedding decoders inside the file itself
```

```
┌──────────────────────────────────────────────────────────────┐
│ F3 File Layout                                                │
│                                                               │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │ Header (file metadata, schema)                          │ │
│  ├─────────────────────────────────────────────────────────┤ │
│  │ I/O Unit 1 (data + encoding description)               │ │
│  │ I/O Unit 2 (data + encoding description)               │ │
│  │ ...                                                     │ │
│  ├─────────────────────────────────────────────────────────┤ │
│  │ Wasm decoders (4-16 KB each, embedded)                  │ │
│  │   ├─ rle_v1.wasm        (4 KB)                         │ │
│  │   ├─ fsst_v2.wasm       (8 KB)                         │ │
│  │   └─ alp_v1.wasm        (3 KB)                         │ │
│  ├─────────────────────────────────────────────────────────┤ │
│  │ Footer (index, dictionary, manifest)                    │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                               │
│  Total Wasm overhead: ~15 KB per file (negligible for TB+)    │
└──────────────────────────────────────────────────────────────┘
```

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Decoupled layout | I/O size ≠ encoding size |
| 2 | Dictionary scope | Per-IoUnit or per-file |
| 3 | Wasm decoder | Old engine decodes new encoding |
| 4 | Self-describing format | No external spec needed |
| 5 | Forward compatible | Format evolves without EOL |

---

## Setup

```bash
cd 04-FileIO/07-NextGenFormats/f3-concept
cargo test
```

## Implementation Steps

### Step 01 — File layout
ASCII art showing header, I/O units, Wasm decoders, footer.

### Step 02 — Decoupled I/O units
In Parquet, the I/O unit (row group) and encoding (column chunk) are coupled: a 128 MB row group is encoded with one codec. In F3, the I/O unit is decoupled from encoding — you can have 8 MB I/O units with different encodings per unit. The reader picks an I/O size that matches the S3 GET request size (8 MB recommended) regardless of encoding choice.

### Step 03 — Wasm decoder bundle (real)
```rust
use f3_concept::{build_decoder_bundle, total_wasm_overhead};

let bundle = build_decoder_bundle();
// 3 decoders, 4+8+3 = 15 KB total
assert_eq!(total_wasm_overhead(&bundle), 15_360);
```

### Step 04 — Wasm vs native tradeoffs

| Aspect | Native decoder | Wasm decoder |
|--------|----------------|--------------|
| Speed | 100% (baseline) | 70-90% (10-30% overhead) |
| Portability | Compile per platform | One binary, all platforms |
| Forward compat | ❌ Stuck on known encodings | ✅ New encodings work |
| Cold start | None | ~1-10 ms to instantiate |
| Security | Trusted C/Rust code | **Sandboxed Wasm** |
| File size overhead | 0 bytes | 4-16 KB per encoding |

### Step 05 — Security model
F3's Wasm decoders run in a **sandboxed linear memory**:
- No access to filesystem, network, or other system calls
- Can only read input bytes + write output bytes
- Each module has a **declared memory budget**
- **Module allowlist** (in the F3 reference implementation): only signed Wasm modules from a curated registry

This means even a malicious decoder can only read the bytes it was given, write to its sandbox, and return. It can't exfiltrate data or corrupt the host process.

### Step 06 — Why F3 is future-proof
The F3 paper's central argument:

> "A file format is a contract between writers and readers. Parquet's contract requires every reader to be updated when the format evolves. F3's contract requires only the **writer** to ship the new decoder. Readers stay backward-compatible because they can always run the Wasm decoder for any encoding they don't natively support."

This is the same argument that made Java's `.class` files work forever (forward compatibility) — the JVM ships with old class versions but the language can add new features.

### Step 07 — Build a mock footer (real)
```rust
use f3_concept::build_mock_footer;

let footer = build_mock_footer();
assert_eq!(footer.io_units.len(), 3);
assert_eq!(footer.dictionary_scope, 0);
```

### Step 08 — JSON roundtrip (real)
```rust
use f3_concept::{build_mock_footer, footer_to_json, footer_from_json};

let f = build_mock_footer();
let json = footer_to_json(&f);
let back = footer_from_json(&json);
assert_eq!(back.io_units.len(), 3);
```

In real F3, the manifest is a **FlatBuffer**, not JSON. We use JSON here for human-readability in this teaching project.

---

## Why F3 is Different from the Others

| Format | Solves… | Doesn't solve… |
|--------|---------|----------------|
| Lance | Random access, multimodal | Format evolution (still coupled to spec) |
| Vortex | Cascading compression | Forward compat (old Vortex can't read new encodings) |
| Nimble | Wide tables, GPU | Forward compat (same as Vortex) |
| **F3** | **Forward compat, format evolution** | Random access (uses 8 MB I/O units, like Vortex) |

The four formats are **complementary**, not competing:
- F3's contribution is the Wasm-decoder idea (could be bolted onto Vortex or Nimble)
- Lance's contribution is the structural-encoding random access (could be used in any format)
- Vortex's contribution is the cascading compression
- Nimble's contribution is the wide-table + GPU encoding

---

## Exercises

1. **Easy**: Add a `Encoding::Hybrid` variant to `WasmDecoder` and rebuild the bundle.
2. **Medium**: Implement `WasmDecoder::supports(&self, encoding: &str) -> bool`.
3. **Hard**: Sketch a `F3Reader` struct that uses Wasmtime (or a mock) to "decode" an I/O unit given its encoding.

---

## Further Reading

- [F3 on GitHub](https://github.com/future-file-format/F3) (436 ⭐, research prototype)
- [F3 paper on ACM](https://dl.acm.org/doi/10.1145/3749163) — SIGMOD 2026
- [F3 paper PDF](https://dl.acm.org/doi/pdf/10.1145/3749163)
- [Quick paper reading note: F3](https://freedium-mirror.cfd/medium.com/@dichenldc/quick-paper-reading-note-f3-the-open-source-data-file-format-for-the-future-a12be53f6c45)
- [Data Engineering Podcast E494: F3](https://www.dataengineeringpodcast.com/future-proof-file-format-evolving-data-lakes-episode-494) — Dec 2025
- [F3 sparks WebAssembly debate](https://biggo.com/news/202510020712_F3_File_Format_WebAssembly_Debate) — Oct 2025
- [Unfreezing The Data Lake (LinkedIn summary)](https://www.linkedin.com/posts/danthelion_is-parquet-officially-legacy-i-read-the-activity-7379214604595396608-C1f_)
