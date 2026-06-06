//! Nimble Design — Meta's columnar format for wide tables
//!
//! Nimble is implemented in C++ and integrated with Velox. There is **no
//! production Rust crate** yet. This project teaches Nimble's design by
//! sketching a hypothetical Rust API and comparing it to Parquet and Lance.
//!
//! Steps:
//!   01 — Understand the per-stream encoding model
//!   02 — Sketch the Rust writer API
//!   03 — Sketch the Rust reader API
//!   04 — Compare cascading encodings to Parquet's fixed codec
//!   05 — Understand FlatBuffer metadata for wide tables
//!   06 — Discuss SIMD/GPU encoding for training workloads

use std::sync::Arc;

use arrow_array::{Float32Array, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use serde::{Deserialize, Serialize};

/// A hypothetical Nimble encoding choice for a single stream.
/// In real Nimble, this would be an enum of `Constant`, `Trivial`, `RLE`,
/// `Dictionary`, `FSST`, `FrameOfReference`, `BitPacking`, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Encoding {
    /// All values are the same
    Constant,
    /// No compression
    Trivial,
    /// Run-length encoding (good for sorted or sparse data)
    Rle,
    /// Dictionary encoding (good for low-cardinality strings)
    Dictionary,
    /// Frame-of-reference (good for clustered integers)
    FrameOfReference,
    /// Bit-packing (good for small-range integers)
    BitPacked,
    /// Fast Static Symbol Table (FSST) for strings
    Fsst,
}

impl Encoding {
    /// Pick the best encoding for a sample of values.
    /// This is what real Nimble does recursively per chunk.
    pub fn recommend_for(values: &[i64]) -> Encoding {
        if values.is_empty() {
            return Encoding::Trivial;
        }
        let first = values[0];
        if values.iter().all(|&v| v == first) {
            return Encoding::Constant;
        }
        // Detect monotonic runs (RLE candidate)
        let mut monotonic_runs = 0;
        let mut sorted = true;
        for w in values.windows(2) {
            if w[0] <= w[1] {
                monotonic_runs += 1;
            } else {
                sorted = false;
                break;
            }
        }
        if sorted && monotonic_runs > values.len() / 2 {
            return Encoding::Rle;
        }
        // Detect small range (FrameOfReference or BitPacked candidate)
        let min = *values.iter().min().unwrap();
        let max = *values.iter().max().unwrap();
        let range = (max - min) as u64;
        if range < (1 << 8) {
            return Encoding::BitPacked;
        }
        if range < (1 << 16) {
            return Encoding::FrameOfReference;
        }
        Encoding::Trivial
    }
}

/// A Nimble "stream" is a per-column chunk with its own encoding.
/// In real Nimble, the encoding is a recursive tree:
///   Stream -> [Encoding, child stream, child stream, ...]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    pub name: String,
    pub encoding: Encoding,
    pub byte_size: usize,
    pub null_count: usize,
}

pub fn build_events_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("value", DataType::Float32, true),
    ]))
}

pub fn make_int_batch(n_rows: usize) -> RecordBatch {
    let ids: Vec<i32> = (0..n_rows as i32).collect();
    let values: Vec<f32> = (0..n_rows).map(|i| (i as f32) * 0.5).collect();
    RecordBatch::try_new(
        build_events_schema(),
        vec![
            Arc::new(Int32Array::from(ids)),
            Arc::new(Float32Array::from(values)),
        ],
    )
    .unwrap()
}

// =============================================================================
// Step 01: per-stream encoding model
// =============================================================================
/// Describe the encoding model in one sentence.
pub fn encoding_model_summary() -> &'static str {
    todo!("Step 01: explain Nimble's per-stream model in 1-2 sentences")
}

// =============================================================================
// Step 02: writer API sketch
// =============================================================================
/// Sketch the Nimble writer API surface (as a doc comment in code).
pub fn writer_api_doc() -> String {
    todo!("Step 02: return a string documenting NimbleWriter / StreamBuilder")
}

// =============================================================================
// Step 03: reader API sketch
// =============================================================================
/// Sketch the Nimble reader API surface.
pub fn reader_api_doc() -> String {
    todo!("Step 03: return a string documenting NimbleReader / StreamReader")
}

// =============================================================================
// Step 04: compare cascading vs fixed codec
// =============================================================================
/// Return a comparison table (as a String) of cascading vs fixed codec.
pub fn cascading_vs_fixed_table() -> String {
    todo!("Step 04: return Markdown table comparing cascading to fixed codec")
}

// =============================================================================
// Step 05: FlatBuffer metadata for wide tables
// =============================================================================
/// Return a description of how FlatBuffer metadata helps wide tables.
pub fn flatbuffer_metadata_benefit() -> String {
    todo!("Step 05: explain O(1) column access via FlatBuffer metadata")
}

// =============================================================================
// Step 06: SIMD/GPU encoding for training
// =============================================================================
/// Return a description of how SIMD/GPU encoding accelerates training.
pub fn simd_gpu_benefit() -> String {
    todo!("Step 06: explain how SIMD-friendly encodings speed up training")
}

// =============================================================================
// Step 07: encoding recommendation (real implementation)
// =============================================================================
/// Given a sample of integer values, recommend the best encoding.
/// This is a real implementation; pairs with Encoding::recommend_for.
pub fn recommend_encoding_for_sample(values: &[i64]) -> Encoding {
    Encoding::recommend_for(values)
}

// =============================================================================
// Step 08: serialize a stream to JSON (mimics Nimble's stream manifest)
// =============================================================================
/// Serialize a `Stream` to a JSON string (mimics FlatBuffer manifest in real Nimble).
pub fn stream_to_json(stream: &Stream) -> String {
    serde_json::to_string(stream).unwrap()
}

// =============================================================================
// Tests
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_01_encoding_model() {
        let s = encoding_model_summary();
        assert!(!s.is_empty());
        assert!(s.to_lowercase().contains("stream") || s.to_lowercase().contains("column"));
    }

    #[test]
    fn step_02_writer_api() {
        let doc = writer_api_doc();
        assert!(doc.contains("NimbleWriter") || doc.contains("Writer"));
        assert!(doc.contains("Stream") || doc.contains("stream"));
    }

    #[test]
    fn step_03_reader_api() {
        let doc = reader_api_doc();
        assert!(doc.contains("Reader") || doc.contains("read"));
    }

    #[test]
    fn step_04_comparison_table() {
        let table = cascading_vs_fixed_table();
        assert!(table.contains("|"));
        assert!(table.to_lowercase().contains("parquet") || table.to_lowercase().contains("fixed"));
    }

    #[test]
    fn step_05_flatbuffer_benefit() {
        let s = flatbuffer_metadata_benefit();
        assert!(s.to_lowercase().contains("flatbuffer") || s.to_lowercase().contains("wide"));
    }

    #[test]
    fn step_06_simd_gpu() {
        let s = simd_gpu_benefit();
        assert!(s.to_lowercase().contains("simd") || s.to_lowercase().contains("gpu"));
    }

    #[test]
    fn step_07_recommend_encoding() {
        // Constant values
        let v = vec![42_i64; 100];
        assert_eq!(recommend_encoding_for_sample(&v), Encoding::Constant);

        // Sorted (RLE candidate)
        let v: Vec<i64> = (0..100).collect();
        let enc = recommend_encoding_for_sample(&v);
        assert!(matches!(enc, Encoding::Rle | Encoding::Trivial));

        // Small range
        let v = vec![1000_i64, 1001, 1002, 1003, 1004, 1005];
        assert!(matches!(recommend_encoding_for_sample(&v), Encoding::BitPacked | Encoding::FrameOfReference));
    }

    #[test]
    fn step_08_stream_json() {
        let s = Stream {
            name: "id".into(),
            encoding: Encoding::FrameOfReference,
            byte_size: 1024,
            null_count: 0,
        };
        let json = stream_to_json(&s);
        let back: Stream = serde_json::from_str(&json).unwrap();
        assert_eq!(back.name, "id");
        assert_eq!(back.encoding, Encoding::FrameOfReference);
    }
}
