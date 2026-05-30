# Section 7: Security & Systems Programming

*Why Rust is the safe alternative to C/C++ for data pipelines, and how cryptography fits in.*

## Prerequisites

- Completed [Section 3: Collections](../03-Collections/README.md)
- Understand ownership and unsafe concepts

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 35 | **SafeAndUnsafe** — safe vs unsafe Rust | `unsafe` keyword, raw pointers, FFI, safety invariants | Project |
| 39 | **SafetyLessonReflection** — Rust vs GC languages | Memory safety, data race prevention, explicit resource management | Reflection |
| 40 | **DecoderRing** — crack Caesar cipher | Frequency analysis, statistical scoring, `rayon` parallelism | Project |
| 41 | **RustCryptoHashes** — cryptographic hashes | SHA-2/3, BLAKE2, `Digest` trait, RustCrypto | Project |
| 42 | **RustSoftwareSecurity** — Rust vs C/C++/Java | Ownership/borrowing safety, compile-time vs runtime safety | Project |
| 43 | **SecurityLessonReflection** — high-availability security | Redundancy, encryption, access control, disaster recovery | Reflection |

## Learning Path

1. Study **01-SafeAndUnsafe** to understand Rust's safety boundaries
2. Build **02-DecoderRing** for a practical crypto application
3. Explore **03-RustCryptoHashes** for hashing algorithms
4. Compare safety models with ****
5. Reflect with **** and ****

## Key Concepts Covered

| Rust Concept | Python Equivalent | Why It Matters |
|---|---|---|
| `unsafe` keyword | N/A (Python is always safe) | Low-level systems programming |
| Raw pointers `*const T` | N/A | FFI and hardware access |
| Cryptography hashes | `hashlib` | Data integrity |
| `Digest` trait | N/A | Unified hash API |
| Memory safety | N/A (GC handles) | No segfaults or buffer overflows |
| `rayon` parallelism | `concurrent.futures` | Parallel cryptanalysis |
