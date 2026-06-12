# 🦀 Ed25519 Digital Signatures — Python to Rust Workshop

*Subtitle: Sign and verify messages with the Ed25519 elliptic-curve algorithm.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 9 tests pass**.

---

## Why Ed25519 for Digital Signatures?

**Python pain:** You need to sign a JWT, an API request, or a software update. The Python `cryptography` library exposes Ed25519, but the API is stateful and the key serialization is awkward. Worse, key formats are inconsistent between libraries.

**Rust fix:** Ed25519 is **the** modern signature algorithm. It's deterministic (the same message + key always produces the same signature), fast (~70,000 sigs/sec on a laptop), and produces short signatures (64 bytes). The `ed25519-dalek` crate gives you a clean, hard-to-misuse API:

```rust
use ed25519_dalek::{SigningKey, Signer, Verifier};
use rand::rngs::OsRng;

let key = SigningKey::generate(&mut OsRng);
let sig = key.sign(b"message");
// To verify:
key.verifying_key().verify(b"message", &sig).is_ok();
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Ed25519 signatures | `ed25519-dalek` | `cryptography` | Modern, fast, deterministic |
| 2 | Keypair | `SigningKey::generate` | `Ed25519PrivateKey.generate` | 32-byte private key |
| 3 | Sign | `key.sign(msg)` | `key.sign(msg)` | Returns 64-byte signature |
| 4 | Verify | `key.verify(msg, sig)` | `pub.verify(msg, sig)` | Returns `Result<(), Error>` |
| 5 | Hex serialization | `hex::encode` | `binascii.hexlify` | URL-safe transport |

---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib ed25519_workshop
cd ed25519_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "ed25519_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
ed25519-dalek = { version = "2", features = ["rand_core", "pem"] }
rand = "0.8"
rand_core = { version = "0.6", features = ["std"] }
hex = "0.4"
base64 = "0.22"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "08-Security/05-Ed25519/workshop/src/lib.rs" src/lib.rs
cp "08-Security/05-Ed25519/workshop/src/main.rs" src/main.rs


### 4. Run the tests to see them fail (this is expected!)

```bash
cargo test
```

You should see all tests fail with the message "not yet implemented". That's the starting point — you are about to make them pass.

### 5. Follow the step-by-step sections below

Each section below corresponds to a step module in the test file. Implement the function(s) described, then run:

```bash
cargo test step_XX_name
```

to watch the pass count grow. The test module names match the section headings.

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib ed25519_workshop
cd ed25519_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "ed25519_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
ed25519-dalek = { version = "2", features = ["rand_core", "pem"] }
rand = "0.8"
rand_core = { version = "0.6", features = ["std"] }
hex = "0.4"
base64 = "0.22"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "08-Security/05-Ed25519/workshop/src/lib.rs" src/lib.rs
cp "08-Security/05-Ed25519/workshop/src/main.rs" src/main.rs


### 4. Run the tests to see them fail (this is expected!)

```bash
cargo test
```

You should see all tests fail with the message "not yet implemented". That's the starting point — you are about to make them pass.

### 5. Follow the step-by-step sections below

Each section below corresponds to a step module in the test file. Implement the function(s) described, then run:

```bash
cargo test step_XX_name
```

to watch the pass count grow. The test module names match the section headings.

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: The Ed25519 Algorithm](#3-concept-the-ed25519-algorithm)
4. [Concept: Keypair Generation and Sign/Verify](#4-concept-keypair-generation-and-signverify)
5. [Concept: Hex Serialization](#5-concept-hex-serialization)
6. [Putting It All Together](#6-putting-it-all-together)
7. [Complete Code Reference](#7-complete-code-reference)
8. [Summary](#8-summary)

## 1. Introduction

Ed25519 is used everywhere:
- **SSH** (default key type since OpenSSH 6.5)
- **TLS 1.3** (signature algorithm)
- **JWT** (ES256 = EdDSA with Ed25519)
- **Bitcoin** (uses secp256k1, but Ed25519 is in Solana, Stellar, Cardano)
- **Software updates** (signed binaries)
- **Git commits** (with GPG)

**Python to Rust:** `cryptography` has `Ed25519PrivateKey` and `Ed25519PublicKey`, but the API requires you to call `private_key.private_bytes(...)` for serialization. The Rust `ed25519-dalek` API is more direct.

**Data-engineering motivation:** When you sign data warehouse artifacts (Parquet files, ML models), you need a fast, deterministic signature. Ed25519 is the answer.

## 2. Prerequisites

- Completed [08-Security/03-RustCryptoHashes](../../03-RustCryptoHashes/README.md) — familiar with hashing.
- Comfortable with `Result`.

## 3. Concept: The Ed25519 Algorithm

Ed25519 is a [Schnorr signature](https://en.wikipedia.org/wiki/Schnorr_signature) using the [Curve25519](https://en.wikipedia.org/wiki/Curve25519) elliptic curve. Key properties:

- **Deterministic** — same message + same key always produces the same signature (no random nonce required, no failure mode from bad randomness).
- **Fast** — ~70,000 signatures/sec on a modern CPU.
- **Small** — 32-byte private key, 32-byte public key, 64-byte signature.
- **Side-channel resistant** — implemented in constant time.

The private key is a random 32-byte seed. The public key is derived from the seed by scalar multiplication on the curve. Signing combines the private key with a hash of the message.

## 4. Concept: Keypair Generation and Sign/Verify

```rust
use ed25519_dalek::{Signer, SigningKey, Verifier};
use rand::rngs::OsRng;

let key = SigningKey::generate(&mut OsRng);
let sig: Signature = key.sign(b"hello");

// Verify
match key.verifying_key().verify(b"hello", &sig) {
    Ok(()) => println!("Signature is valid"),
    Err(_) => println!("Signature is INVALID"),
}
```

The `SigningKey` is the private key. The `VerifyingKey` is derived from it via `key.verifying_key()`. In production, you store the private key encrypted and share only the public key.

**In Python (`cryptography`):**

```python
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
priv = Ed25519PrivateKey.generate()
sig = priv.sign(b"hello")
priv.public_key().verify(sig, b"hello")
```

Functionally identical. The Rust version returns `Signature` directly; the Python version returns `bytes`.

## 5. Concept: Hex Serialization

Public keys are 32 bytes — small enough to embed in a URL, a QR code, or a config file. Use hex encoding:

```rust
let hex = hex::encode(key.verifying_key().to_bytes());
// "a1b2c3d4..." (64 chars)
```

To parse back:

```rust
let bytes = hex::decode(&hex).map_err(|_| SignatureError::new())?;
let parsed = VerifyingKey::from_bytes(&bytes)?;
```

For URL-safe transport, use **base64url** (no `+`, `/`, `=`):

```rust
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
let b64 = URL_SAFE_NO_PAD.encode(key.verifying_key().to_bytes());
```

**In Python:** `binascii.hexlify`, `base64.urlsafe_b64encode`.

## 6. Putting It All Together

`lib.rs` is organized in three progressive steps:

1. **Step 1 (`step_01_keypair`)** — `generate_keypair` returns a `SigningKey`.
2. **Step 2 (`step_02_sign_and_verify`)** — `sign_message`, `verify_signature`, helpers for roundtrip and tamper detection.
3. **Step 3 (`step_03_serialization`)** — `public_key_to_hex`, `public_key_from_hex`.

`main.rs` ties it together: generate a key, sign a message, verify, serialize to hex, roundtrip.

## 7. Complete Code Reference

See [`workshop/src/lib.rs`](workshop/src/lib.rs) and [`workshop/src/main.rs`](workshop/src/main.rs).

## 8. Summary

| Concept | Used In |
|---------|---------|
| `SigningKey::generate` | `generate_keypair` |
| `Signer::sign` | `sign_message` |
| `Verifier::verify` | `verify_signature` |
| `hex::encode` / `hex::decode` | `public_key_to_hex` / `public_key_from_hex` |
| `VerifyingKey::from_bytes` | `public_key_from_hex` |
| Determinism check | `sign_then_verify` |
| Tamper detection | `tampered_signature_fails` |

## Further Reading

- [ed25519-dalek docs](https://docs.rs/ed25519-dalek/)
- [RFC 8032: Edwards-Curve Digital Signature Algorithm (EdDSA)](https://datatracker.ietf.org/doc/html/rfc8032)
- [NaCl crypto library](https://nacl.cr.yp.to/) (where Ed25519 was first specified)
- [Monocypher documentation](https://monocypher.org/)

## Exercises

1. **Easy**: Add a `keypair_to_hex(key: &SigningKey) -> String` that encodes the 32-byte private key as hex, and 1 test.
2. **Medium**: Add a `sign_with_nonce(key, message, nonce: &[u8; 32]) -> Signature` that uses a deterministic nonce (Ed25519ph / Ed25519ctx variant).
3. **Hard**: Add a `verify_batch(keys: &[VerifyingKey], messages: &[&[u8]], signatures: &[Signature]) -> Vec<bool>` that verifies many signatures in batched form (constant-time, vectorized).

---

**Goal**: Implement all functions in `src/lib.rs` to pass all 9 tests.

## Functions to Implement

### Step 1 — Keypair

#### `generate_keypair`
- **Signature**: `pub fn generate_keypair() -> SigningKey`
- **Task**: `SigningKey::generate(&mut OsRng)`

### Step 2 — Sign and verify

#### `sign_message`
- **Signature**: `pub fn sign_message(key: &SigningKey, message: &[u8]) -> Signature`
- **Task**: `key.sign(message)`

#### `verify_signature`
- **Signature**: `pub fn verify_signature(key: &VerifyingKey, message: &[u8], signature: &Signature) -> bool`
- **Task**: `key.verify(message, signature).is_ok()`

#### `sign_then_verify`
- **Signature**: `pub fn sign_then_verify(message: &[u8]) -> bool`
- **Task**: Generate a key, sign `message`, verify, return the bool.

#### `tampered_signature_fails`
- **Signature**: `pub fn tampered_signature_fails(message: &[u8]) -> bool`
- **Task**: Sign a different message, try to verify `message` with that signature, return the (false) result.

### Step 3 — Serialization

#### `public_key_to_hex`
- **Signature**: `pub fn public_key_to_hex(key: &VerifyingKey) -> String`
- **Task**: `hex::encode(key.to_bytes())`

#### `public_key_from_hex`
- **Signature**: `pub fn public_key_from_hex(s: &str) -> Result<VerifyingKey, ed25519_dalek::SignatureError>`
- **Task**: `let bytes = hex::decode(s).map_err(|_| ed25519_dalek::SignatureError::new())?; VerifyingKey::from_bytes(&bytes)`

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_keypair | 2 | Generate, distinct keypairs |
| step_02_sign_and_verify | 4 | Roundtrip + tampering detection |
| step_03_serialization | 3 | Hex roundtrip + invalid input + length check |

## How to Run Tests
```bash
cargo test
```
