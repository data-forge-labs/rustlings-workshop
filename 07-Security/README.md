# Section 7: Security & Systems Programming

*Why Rust is the safe alternative to C/C++ for data pipelines, and how cryptography fits in.*

---

## Why This Section?

### The Problem — Memory Bugs in Data Systems

The most critical data breaches and system failures don't come from application logic — they come from **memory unsafety**:

```
┌─────────────────────────────────────────────────────┐
│  Real-World Memory Safety Bugs                       │
│                                                      │
│  2014: Heartbleed (OpenSSL)                          │
│    └─ Buffer over-read → leaked private keys         │
│        ┌──────────────────────┐                      │
│        │  ...patient data...SHA256PRIVATEKEY...     │
│        └──────────────────────┘                      │
│        One bug, millions of servers exposed          │
│                                                      │
│  2017: Cloudbleed (Cloudflare)                       │
│    └─ Buffer over-read → leaked customer data        │
│                                                      │
│  2021: Log4Shell (Java)                              │
│    └─ Not memory, but classic "unsafe input" bug     │
│                                                      │
│  70% of all Microsoft CVEs are memory safety bugs    │
│  (Microsoft Security Response Center, 2019)          │
└─────────────────────────────────────────────────────┘
```

Python protects you from memory bugs via the **GC and runtime checks** — but at a performance cost. C/C++ give you performance but **no safety guarantees**. Rust gives you **both**.

```
                    Safety
                    ▲
                    │
          Rust ─────┤  Python
          (safe &   │  (safe &
           fast)    │   slow)
                    │
                    ├─────────► Performance
                    │
          C/C++ ───┤
          (fast &  │
           unsafe) │
                    │
```

### What This Section Covers

- **Safe vs unsafe Rust**: When and why to drop into `unsafe`
- **Cryptographic hashing**: Data integrity verification
- **Frequency analysis**: Classic cryptanalysis in parallel
- **Rust's security model**: Why Rust is chosen for safety-critical systems

---

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | Unsafe Rust | `unsafe` keyword | N/A (always safe) | Raw pointer deref, FFI, intrinsics |
| 2 | Raw pointers | `*const T`, `*mut T` | N/A | Direct memory access (no safety checks) |
| 3 | FFI | `extern "C"` | `ctypes`, `cffi` | Call C libraries from Rust |
| 4 | Safety invariants | Ownership rules | N/A | What unsafe code must uphold |
| 5 | SHA hashing | `sha2::Sha256` | `hashlib.sha256` | Cryptographic hash computation |
| 6 | BLAKE2 hashing | `blake2::Blake2b` | `hashlib.blake2b` | Fast, modern hash |
| 7 | Digest trait | `Digest` trait | N/A | Unified hashing interface |
| 8 | Caesar cipher | Character shifting | `str.translate()` | Classic cipher, cryptanalysis demo |
| 9 | Frequency analysis | Statistical scoring | `collections.Counter` | Cipher breaking technique |
| 10 | Parallel cracking | `rayon` | `concurrent.futures` | Brute-force in parallel |
| 11 | Memory safety model | Ownership + borrowing | N/A | No buffer overflows, use-after-free |
| 12 | Compile-time checks | The borrow checker | N/A | Bugs caught before runtime |

---

## Concepts at a Glance

### 1. `unsafe` — The Escape Hatch

Rust's `unsafe` keyword lets you do things the compiler can't verify:

```rust
unsafe {
    let ptr = 0x1234 as *const i32;
    // Dereference a raw pointer (DANGEROUS)
    // Must manually guarantee it's valid
}
```

`unsafe` doesn't disable the borrow checker; it adds **5 extra abilities**:
1. Dereference raw pointers
2. Call `unsafe` functions (including FFI)
3. Implement `unsafe` traits
4. Access/modify mutable statics
5. Access union fields

**Key philosophy**: `unsafe` is not "turn off safety." It's "I, the programmer, promise this is safe."

```
  ┌─────────────────────────────────────────────────┐
  │  Safe Rust: 95% of your code                     │
  │    ┌───────────────────────────────────────────┐ │
  │    │  Unsafe Rust: 5% of your code, wrapped in │ │
  │    │  safe abstractions                        │ │
  │    │    ┌─────────────────────────────────────┐│ │
  │    │    │  Raw pointer ops, FFI calls         ││ │
  │    │    └─────────────────────────────────────┘│ │
  │    └───────────────────────────────────────────┘ │
  └─────────────────────────────────────────────────┘
```

### 2. Cryptographic Hashing — SHA-2, BLAKE2

```rust
use sha2::{Sha256, Digest};

let mut hasher = Sha256::new();
hasher.update(b"data to hash");
let result = hasher.finalize();
println!("{:x}", result);  // hex string
```

In Python: `hashlib.sha256(b"data to hash").hexdigest()`

### 3. The `Digest` Trait — Unified Interface

All hash types implement the same trait, making them swappable:

```rust
fn hash_it<D: Digest>(data: &[u8]) -> String
where
    D: Digest,
{
    format!("{:x}", D::digest(data))
}

// Use any hash implementation:
hash_it::<Sha256>(b"hello");
hash_it::<Blake2b512>(b"hello");
hash_it::<Sha3_256>(b"hello");
```

### 4. Frequency Analysis — Classic Cryptanalysis

```rust
use std::collections::HashMap;

fn frequency_analysis(text: &str) -> HashMap<char, f64> {
    let total = text.chars().filter(|c| c.is_alphabetic()).count() as f64;
    let mut counts = HashMap::new();
    for c in text.chars().filter(|c| c.is_alphabetic()) {
        *counts.entry(c.to_ascii_lowercase()).or_insert(0.0) += 1.0;
    }
    for count in counts.values_mut() {
        *count /= total;  // normalize to frequency
    }
    counts
}
```

In Python: `collections.Counter(text) / len(text)`

### 5. Parallel Cracking with Rayon

Brute-force a Caesar cipher using all CPU cores:

```rust
use rayon::prelude::*;

let best_shift = (0..26).into_par_iter()
    .map(|shift| {
        let decoded = decode_caesar(ciphertext, shift);
        let score = score_english(&decoded);
        (shift, score)
    })
    .max_by_key(|&(_, score)| score);  // best match in parallel
```

### 6. Memory Safety — Rust's Core Promise

```
┌─────────────────────────────────────────────────────┐
│  Bug Type          Python      C/C++       Rust     │
├─────────────────────────────────────────────────────┤
│  Buffer overflow   Impossible  Common      │  │
│  Use-after-free    Impossible  Common         │
│  Null dereference  Impossible  (segfault)        │
│  Double free       Impossible  Common            │
│  Data race         Possible    Common              │
│  Memory leak       Possible    Possible           │
│  Type confusion    Possible    Common                │
└─────────────────────────────────────────────────────┘
```

Rust's **ownership model** eliminates 70% of the bugs that plague C/C++ — the exact bugs that cause security vulnerabilities.

---

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
| 60 | **Argon2** — password hashing | `argon2` crate, `SaltString`, `PasswordHasher`/`PasswordVerifier`, `subtle::ConstantTimeEq` | Project |
| 61 | **Ed25519** — digital signatures | `ed25519-dalek`, `SigningKey`/`VerifyingKey`, hex serialization, tamper detection | Project |
| 62 | **RustlsTLS** — TLS server & client | `rustls` + `aws-lc-rs`, `ServerConfig`/`ClientConfig`, `tokio-rustls` handshake | Project |

## Learning Path

1. Study **01-SafeAndUnsafe** to understand Rust's safety boundaries
2. Build **02-DecoderRing** for a practical crypto application
3. Explore **03-RustCryptoHashes** for hashing algorithms
4. Compare safety models with **RustSoftwareSecurity**
5. **04-Argon2** for production password hashing (OWASP-recommended)
6. **05-Ed25519** for digital signatures (JWT, software updates, blockchain)
7. **06-RustlsTLS** for memory-safe TLS (no OpenSSL CVEs)
8. Reflect with **SafetyLessonReflection** and **SecurityLessonReflection**
