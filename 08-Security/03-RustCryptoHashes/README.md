# Cryptographic Hashes — Python to Rust Workshop

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 12 tests pass**.

---

## What Is This Project?

Cryptographic hashing with RustCrypto — SHA-2, SHA-3, and BLAKE2 for data integrity.

### Python equivalent

```python
import hashlib

data = b"hello world"
h = hashlib.sha256(data).hexdigest()
print(h)  # b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
```

```rust
use sha2::{Sha256, Digest};

pub fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
// same API for Sha3_256, Blake2b512, ...
```

In this project you'll learn to build this in Rust — and along the way
you'll discover **RustCrypto**, **`Digest` trait**, and **parallel hashing**.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | `Digest` trait | Unified API for all hash algorithms |
| 2 | SHA-256, SHA-3, BLAKE2b | NIST-standard and fast crypto hashes |
| 3 | Hex encoding | Convert digest bytes to hex string |
| 4 | Simple sum & XOR hashes | Non-crypto hashes, error-detection |
| 5 | Avalanche effect | 1-bit input change affects output |

---

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: What Is a Hash Function?](#3-concept-what-is-a-hash-function)
4. [Concept: Simple String Hash](#4-concept-simple-string-hash)
5. [Concept: XOR Checksum](#5-concept-xor-checksum)
6. [Concept: Deterministic Property](#6-concept-deterministic-property)
7. [Concept: Avalanche Effect](#7-concept-avalanche-effect)
8. [Concept: Real-World Cryptographic Hash Algorithms](#8-concept-real-world-cryptographic-hash-algorithms)
9. [Putting It All Together](#9-putting-it-all-together)
10. [Complete Code Reference](#10-complete-code-reference)
11. [Summary](#11-summary)

## 1. Introduction

In this workshop you will explore cryptographic hash functions: what they are,
what properties make them useful, and how real-world algorithms like SHA-2,
SHA-3, and BLAKE2 work. You will build simple non-cryptographic hash
functions to understand the core ideas, then compare them with production
algorithms.

In Python, the `hashlib` module provides access to SHA-256, BLAKE2b, and
other hash algorithms. Python's `hash()` built-in is a non-cryptographic hash
used for dict lookups. Rust's ecosystem provides the same algorithms through
the RustCrypto project's crates (`sha2`, `sha3`, `blake2`).

**Data-engineering motivation**: Hash functions are everywhere in data
engineering: data integrity verification (checksums), partitioning (consistent
hashing), deduplication, content-addressable storage, and digital signatures
for pipeline security.

## 2. Prerequisites

- Completed [Section 3: Collections](../../../../03-Collections/README.md) -- basic
  iteration and string manipulation
- Familiarity with byte-level operations (XOR, modulo)

## 3. Concept: What Is a Hash Function?

### Explanation

A hash function takes an input (any size) and produces a fixed-size output
(the hash, digest, or checksum). In Python, `hashlib.sha256(b"hello")` returns
a 256-bit digest. In Rust, you would use:

```rust
use sha2::{Sha256, Digest};
let mut hasher = Sha256::new();
hasher.update(b"hello");
let result = hasher.finalize();
```

Cryptographic hash functions have four key properties:
1. **Deterministic** -- same input always produces the same output
2. **Preimage resistance** -- given a hash, it is infeasible to find the input
3. **Second preimage resistance** -- given an input, infeasible to find
   another input with the same hash
4. **Collision resistance** -- infeasible to find any two inputs with the
   same hash

### Applying to Our Project

Our project's `hash_properties()` function will return these properties. The
tests verify it includes at least "collision resistance".

## 4. Concept: Simple String Hash

### Explanation

A simple non-cryptographic hash converts each character to its byte value and
combines them (e.g., via addition). This demonstrates the deterministic
property but is not secure -- collisions are trivial to find.

In Python:

```python
def simple_hash(input_str: str) -> str:
    total = sum(ord(c) for c in input_str)
    return hex(total)
```

In Rust:

```rust
pub fn simple_hash(input: &str) -> String {
    let sum: u32 = input.bytes().map(|b| b as u32).sum();
    format!("{:x}", sum)
}
```

Note the difference: Python's `ord()` returns Unicode code points; Rust's
`.bytes()` returns raw byte values. For ASCII input they are the same.

### Applying to Our Project

Implement `simple_hash()` that sums byte values and returns the hex string.
Tests verify: (1) non-empty output, (2) different inputs produce different
hashes.

## 5. Concept: XOR Checksum

### Explanation

An XOR checksum combines bytes using XOR instead of addition. This is the
simplest error-detection checksum (used in early networking protocols like
TCP checksums).

```rust
pub fn xor_checksum(input: &[u8]) -> u8 {
    input.iter().fold(0, |acc, &b| acc ^ b)
}
```

In Python:

```python
def xor_checksum(data: bytes) -> int:
    result = 0
    for b in data:
        result ^= b
    return result
```

The XOR checksum has a useful property: XORing a value twice cancels out.
This makes it reversible, which is why it is not suitable for cryptographic
use.

### Applying to Our Project

Implement `xor_checksum()` using `fold`. Tests verify: (1) basic XOR of
[1,2,3,4] equals 1^2^3^4 = 4, (2) empty input returns 0, (3) single byte
returns itself.

## 6. Concept: Deterministic Property

### Explanation

A hash function must be deterministic: the same input always yields the same
output. This function demonstrates that property:

```rust
pub fn is_deterministic(input: &str) -> bool {
    simple_hash(input) == simple_hash(input)
}
```

This always returns `true`. The test verifies that calling the hash twice on
the same input produces the same result.

In Python, `hashlib.sha256(b"hello").digest()` also always produces the same
bytes. Determinism is the foundation of all hash-based data structures
(dicts, sets, caches).

### Applying to Our Project

Implement `is_deterministic()` that hashes the input twice and compares.

## 7. Concept: Avalanche Effect

### Explanation

A good hash function exhibits the avalanche effect: changing one bit in the
input should change approximately half the bits in the output. Our simple
sum-based hash does NOT have this property (changing 'h' to 'i' in "hello"
only changes the sum by 1).

This function checks whether changing a character changes the hash:

```rust
pub fn avalanche_effect(input: &str, change_at: usize) -> bool {
    if change_at >= input.len() {
        return false;  // no change possible
    }
    let mut modified = input.to_string();
    // Toggle lowest bit of the character at change_at
    let bytes = unsafe { modified.as_bytes_mut() };
    bytes[change_at] ^= 1;
    simple_hash(input) != simple_hash(&modified)
}
```

In Python:

```python
def avalanche_effect(s: str, pos: int) -> bool:
    if pos >= len(s):
        return False
    modified = s[:pos] + chr(ord(s[pos]) ^ 1) + s[pos+1:]
    return simple_hash(s) != simple_hash(modified)
```

### Applying to Our Project

Implement `avalanche_effect()` that flips one bit in the character at
`change_at` and compares hashes. Tests verify: (1) changing a valid position
changes the hash, (2) out-of-bounds index returns `false`.

## 8. Concept: Real-World Cryptographic Hash Algorithms

### Explanation

The RustCrypto project provides individual crates for each hash algorithm:

- **SHA-256** (`sha2` crate) -- 256-bit output, NIST standard, widely used
  in TLS, blockchain, code signing
- **SHA-3** (`sha3` crate) -- newer NIST standard based on Keccak, different
  internal structure than SHA-2
- **BLAKE2b** (`blake2` crate) -- faster than SHA-2/3, used in cryptographic
  libraries and file integrity tools

In Python:

```python
import hashlib
# SHA-256
h = hashlib.sha256(b"hello").hexdigest()
# BLAKE2b
h = hashlib.blake2b(b"hello", digest_size=32).hexdigest()
```

In Rust, the pattern is consistent across all algorithms via the `Digest`
trait:

```rust
use sha2::{Sha256, Digest};
let mut hasher = Sha256::new();
hasher.update(b"hello");
let hash = hasher.finalize();       // GenericArray<u8, U32>
let hex = format!("{:x}", hash);    // hex string
```

The `Digest` trait provides `new()`, `update()`, and `finalize()` -- the same
API for every hash algorithm. This is the RustCrypto project's design.

### Applying to Our Project

The `hash_algorithms()` function returns algorithm names covered:

```rust
pub fn hash_algorithms() -> Vec<&'static str> {
    vec!["SHA-256", "SHA-3", "BLAKE2b"]
}
```

## 9. Putting It All Together

Open `workshop/src/lib.rs` and replace each `todo!()`:

**Step 1 (Hashing basics):** Implement `simple_hash` (byte-sum hex string)
and `xor_checksum` (XOR fold). Tests: 5 pass.

**Step 2 (Hash properties):** Implement `is_deterministic` (compare hash of
input with itself) and `avalanche_effect` (flip a bit, compare hashes).
Tests: 3 more pass (total 8).

**Step 3 (Concepts):** Implement `hash_algorithms` and `hash_properties`
returning lists of strings. Tests: 4 more pass (total 12).

Run `cd workshop && cargo test` after each step.

## 10. Complete Code Reference

```rust
pub fn simple_hash(input: &str) -> String {
    let sum: u32 = input.bytes().map(|b| b as u32).sum();
    format!("{:x}", sum)
}

pub fn xor_checksum(input: &[u8]) -> u8 {
    input.iter().fold(0, |acc, &b| acc ^ b)
}

pub fn is_deterministic(input: &str) -> bool {
    simple_hash(input) == simple_hash(input)
}

pub fn avalanche_effect(input: &str, change_at: usize) -> bool {
    if change_at >= input.len() {
        return false;
    }
    let mut modified = input.to_string();
    let bytes = unsafe { modified.as_bytes_mut() };
    bytes[change_at] ^= 1;
    simple_hash(input) != simple_hash(&modified)
}

pub fn hash_algorithms() -> Vec<&'static str> {
    vec!["SHA-256", "SHA-3", "BLAKE2b"]
}

pub fn hash_properties() -> Vec<&'static str> {
    vec![
        "deterministic",
        "preimage resistance",
        "second preimage resistance",
        "collision resistance",
    ]
}
```

## 11. Summary

| Concept | Python Equivalent | Where Used |
|---|---|---|
| Simple sum-based hash | `sum(ord(c) for c in s)` | `simple_hash` |
| XOR checksum | `functools.reduce(int.__xor__, data)` | `xor_checksum` |
| Deterministic property | Automatic in Python | `is_deterministic` |
| Avalanche effect | Comparing two hashes | `avalanche_effect` |
| SHA-256 | `hashlib.sha256` | `hash_algorithms` |
| BLAKE2b | `hashlib.blake2b` | `hash_algorithms` |
| `Digest` trait | N/A (no unified trait in Python) | Conceptual reference |

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

