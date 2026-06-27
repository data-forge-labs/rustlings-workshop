//! F3 Concept — Future-proof File Format with embedded Wasm decoders
//!
//! F3 is a research prototype from CMU + Tsinghua, published at SIGMOD 2026.
//! The Rust implementation is `fff-poc` and is explicitly marked "not for
//! production". This project teaches the *concepts*:
//!
//! Steps:
//!   01 — Understand the F3 file layout
//!   02 — Understand decoupled I/O units vs encodings
//!   03 — Sketch the Wasm decoder bundle (manifest)
//!   04 — Compare Wasm-vs-native decoding tradeoffs
//!   05 — Discuss the security model (sandboxed linear memory)
//!   06 — Understand why F3 is "future-proof"

use serde::{Deserialize, Serialize};

/// Mock representation of an F3 I/O unit (a chunk of column data).
/// In real F3, this is a region of the file with a known offset + length.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoUnit {
    pub id: u32,
    pub offset: u64,
    pub length: u64,
    pub encoding: String,
}

/// Mock representation of a Wasm decoder bundle embedded in the F3 file.
/// In real F3, this is a flat list of `Module { name, version, bytes }` entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmDecoder {
    pub name: String,
    pub version: u32,
    pub bytes_len: usize,
    pub supported_encodings: Vec<String>,
}

/// Mock representation of the F3 file footer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct F3Footer {
    pub magic: [u8; 4],            // "F3\0\0"
    pub version: u32,
    pub io_units: Vec<IoUnit>,
    pub decoders: Vec<WasmDecoder>,
    pub dictionary_scope: u32,     // 0 = per-IoUnit, 1 = per-file
}

// =============================================================================
// Step 01: F3 file layout
// =============================================================================
/// Describe the F3 file layout as a multi-line string (ASCII diagram).
pub fn file_layout_description() -> String {
    "F3 File Layout:\n\
     +------------------+\n\
     | Header (magic)   |  4 bytes: 'F3\\0\\0'\n\
     +------------------+\n\
     | I/O Unit 0       |  Column chunk with encoding tag\n\
     | I/O Unit 1       |  Each unit has offset + length\n\
     | ...              |  Independent read granularity\n\
     +------------------+\n\
     | Footer (FlatBuf) |  Schema, I/O unit index, decoder manifest\n\
     +------------------+\n\
     | Wasm Decoders    |  Embedded decoder modules (RLE, FSST, etc.)\n\
     +------------------+"
        .to_string()
}

pub fn decoupled_io_explanation() -> String {
    "F3 decouples I/O granularity from encoding: each I/O unit is a self-contained chunk of column data with its own encoding tag. A single column can span multiple I/O units with different encodings, allowing the file to adapt to local data patterns without changing the I/O read size."
}

pub fn wasm_vs_native_table() -> String {
    "| Feature | Wasm Decoder | Native Decoder |\n\
     |---|---|---|\n\
     | Portability | Any Wasm runtime | Platform-specific binary |\n\
     | Performance | ~1.5-3x slower (sandbox overhead) | Full native speed |\n\
     | Security | Sandboxed linear memory | Full process access |\n\
     | Extensibility | Add decoders without recompiling | Requires recompilation |\n\
     | Deployment | Embedded in file | Must be installed |\n"
        .to_string()
}

pub fn security_model_description() -> String {
    "F3 uses Wasm sandboxed linear memory: each decoder module runs in its own memory space with no access to host memory or system calls. An allowlist in the file footer specifies which decoder modules are permitted, preventing execution of arbitrary code. The host only passes the encoded bytes into the sandbox and reads decoded output."
}

pub fn future_proof_argument() -> String {
    "F3 is future-proof because new encodings can be embedded as Wasm decoder modules directly in the file. A reader written today can decode data encoded with future algorithms by loading the embedded Wasm modules, eliminating the need for reader upgrades when new encoding formats are invented."
}

// =============================================================================
// Step 02: decoupled I/O units vs encodings
// =============================================================================
/// Explain decoupled I/O units in 1-2 sentences.
pub fn decoupled_io_explanation() -> String {
    todo!("Step 02: explain how F3 separates I/O granularity from encoding choice")
}

// =============================================================================
// Step 03: Wasm decoder bundle (real implementation)
// =============================================================================
/// Build a mock Wasm decoder bundle for a small set of encodings.
pub fn build_decoder_bundle() -> Vec<WasmDecoder> {
    vec![
        WasmDecoder {
            name: "rle_v1.wasm".into(),
            version: 1,
            bytes_len: 4_096,           // 4 KB
            supported_encodings: vec!["rle".into()],
        },
        WasmDecoder {
            name: "fsst_v2.wasm".into(),
            version: 2,
            bytes_len: 8_192,           // 8 KB
            supported_encodings: vec!["fsst".into(), "fsst-v2".into()],
        },
        WasmDecoder {
            name: "alp_v1.wasm".into(),
            version: 1,
            bytes_len: 3_072,           // 3 KB
            supported_encodings: vec!["alp".into()],
        },
    ]
}

/// Total Wasm overhead in the file (sum of decoder bytes).
pub fn total_wasm_overhead(decoders: &[WasmDecoder]) -> usize {
    decoders.iter().map(|d| d.bytes_len).sum()
}

// =============================================================================
// Step 04: Wasm-vs-native decoding tradeoffs
// =============================================================================
/// Return a Markdown table comparing Wasm vs native decoding.
pub fn wasm_vs_native_table() -> String {
    todo!("Step 04: return Markdown table of Wasm vs native tradeoffs")
}

// =============================================================================
// Step 05: security model
// =============================================================================
/// Return a description of the F3 sandboxing approach.
pub fn security_model_description() -> String {
    todo!("Step 05: explain sandboxed linear memory + module allowlist")
}

// =============================================================================
// Step 06: why F3 is future-proof
// =============================================================================
/// Return a description of why F3 is "future-proof".
pub fn future_proof_argument() -> String {
    todo!("Step 06: explain how embedded decoders make the format forward-compatible")
}

// =============================================================================
// Step 07: build a mock F3 footer
// =============================================================================
/// Build a mock F3Footer for a small file (e.g., 3 I/O units, 1 decoder).
pub fn build_mock_footer() -> F3Footer {
    F3Footer {
        magic: *b"F3\x00\x00",
        version: 1,
        io_units: vec![
            IoUnit { id: 0, offset: 64, length: 4096, encoding: "rle".into() },
            IoUnit { id: 1, offset: 4160, length: 8192, encoding: "fsst-v2".into() },
            IoUnit { id: 2, offset: 12352, length: 3072, encoding: "alp".into() },
        ],
        decoders: vec![WasmDecoder {
            name: "multi_decoder.wasm".into(),
            version: 1,
            bytes_len: 16_384,           // 16 KB
            supported_encodings: vec!["rle".into(), "fsst-v2".into(), "alp".into()],
        }],
        dictionary_scope: 0,             // per-I/O unit
    }
}

// =============================================================================
// Step 08: serialize footer to JSON (mimics F3's FlatBuffer manifest)
// =============================================================================
/// Serialize a footer to JSON.
pub fn footer_to_json(footer: &F3Footer) -> String {
    serde_json::to_string_pretty(footer).unwrap()
}

/// Parse a footer back from JSON.
pub fn footer_from_json(s: &str) -> F3Footer {
    serde_json::from_str(s).unwrap()
}

// =============================================================================
// Tests
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_01_layout() {
        let s = file_layout_description();
        assert!(!s.is_empty());
        assert!(s.contains("F3") || s.contains("footer") || s.contains("Wasm"));
    }

    #[test]
    fn step_02_decoupled() {
        let s = decoupled_io_explanation();
        assert!(!s.is_empty());
        assert!(s.to_lowercase().contains("i/o") || s.to_lowercase().contains("io"));
    }

    #[test]
    fn step_03_wasm_bundle() {
        let bundle = build_decoder_bundle();
        assert_eq!(bundle.len(), 3);
        let total = total_wasm_overhead(&bundle);
        // 4 KB + 8 KB + 3 KB = 15 KB
        assert_eq!(total, 15_360);
    }

    #[test]
    fn step_04_tradeoff_table() {
        let s = wasm_vs_native_table();
        assert!(s.contains("|"));
        assert!(s.to_lowercase().contains("wasm"));
        assert!(s.to_lowercase().contains("native"));
    }

    #[test]
    fn step_05_security() {
        let s = security_model_description();
        assert!(s.to_lowercase().contains("sandbox") || s.to_lowercase().contains("memory") || s.to_lowercase().contains("allowlist"));
    }

    #[test]
    fn step_06_future_proof() {
        let s = future_proof_argument();
        assert!(s.to_lowercase().contains("forward") || s.to_lowercase().contains("decoder") || s.to_lowercase().contains("encoding"));
    }

    #[test]
    fn step_07_mock_footer() {
        let f = build_mock_footer();
        assert_eq!(f.io_units.len(), 3);
        assert_eq!(f.decoders.len(), 1);
        assert_eq!(f.dictionary_scope, 0);
        // Sum of I/O unit lengths = 15,360 bytes of data
        let total: u64 = f.io_units.iter().map(|u| u.length).sum();
        assert_eq!(total, 15_360);
    }

    #[test]
    fn step_08_json_roundtrip() {
        let f = build_mock_footer();
        let json = footer_to_json(&f);
        let back = footer_from_json(&json);
        assert_eq!(back.io_units.len(), 3);
        assert_eq!(back.io_units[0].encoding, "rle");
        assert_eq!(back.decoders[0].supported_encodings, vec!["rle", "fsst-v2", "alp"]);
    }
}
