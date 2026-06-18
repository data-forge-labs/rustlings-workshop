# 🦀 Argon2 Password Hashing — Python to Rust Workshop

*Subtitle: Hash and verify passwords with the Argon2id algorithm — the OWASP-recommended standard.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 10 tests pass**.

---

## What Is Argon2?

OWASP-recommended password hashing — memory-hard, parallel, and resistant to GPU attacks.

### Python equivalent

```python
import hashlib
import bcrypt

# hashlib SHA-256: too fast for passwords (GPU cracks in seconds)
h = hashlib.sha256(b"password").hexdigest()

# bcrypt: better, but 72-byte limit and not the modern standard
hashed = bcrypt.hashpw(b"password", bcrypt.gensalt())
```

```rust
use argon2::Argon2;
use password_hash::{PasswordHasher, SaltString, rand_core::OsRng};

let salt = SaltString::generate(&mut OsRng);
let hash = Argon2::default().hash_password(b"hunter2", &salt)?.to_string();
```

The output is a self-describing string: `$argon2id$v=19$m=19456,t=2,p=1$...salt...$...hash...`. You can verify it with no extra metadata.

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Argon2id | `argon2::Argon2` | `argon2-cffi` | OWASP-recommended |
| 2 | Random salt | `SaltString::generate(&mut OsRng)` | `os.urandom(16)` | One salt per password |
| 3 | Hash + encode | `hash_password(...).to_string()` | `ph.hash(...).encode()` | Self-describing format |
| 4 | Verify | `verify_password(...)` | `ph.verify(...)` | Constant-time |
| 5 | Constant-time compare | `subtle::ConstantTimeEq` | `hmac.compare_digest` | Prevents timing attacks |

---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib argon2_workshop
cd argon2_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "argon2_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
argon2 = "0.5"
password-hash = { version = "0.5", features = ["std"] }
rand = "0.8"
subtle = "2"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "08-Security/04-Argon2/workshop/src/lib.rs" src/lib.rs
cp "08-Security/04-Argon2/workshop/src/main.rs" src/main.rs


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
cargo new --lib argon2_workshop
cd argon2_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "argon2_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
argon2 = "0.5"
password-hash = { version = "0.5", features = ["std"] }
rand = "0.8"
subtle = "2"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "08-Security/04-Argon2/workshop/src/lib.rs" src/lib.rs
cp "08-Security/04-Argon2/workshop/src/main.rs" src/main.rs


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
3. [Concept: The Argon2id Algorithm](#3-concept-the-argon2id-algorithm)
4. [Concept: Hashing and Salting](#4-concept-hashing-and-salting)
5. [Concept: Verification](#5-concept-verification)
6. [Concept: Constant-Time Comparison](#6-concept-constant-time-comparison)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Complete Code Reference](#8-complete-code-reference)
9. [Summary](#9-summary)

## 1. Introduction

Argon2 won the [Password Hashing Competition](https://www.password-hashing.net/) in 2015. It comes in three variants:
- **Argon2d** — data-dependent, fastest, side-channel vulnerable
- **Argon2i** — data-independent, slower, side-channel resistant
- **Argon2id** — hybrid (recommended; first pass is Argon2i, then Argon2d)

**Python to Rust:** `argon2-cffi` is the standard Python binding. The Rust `argon2` crate wraps `RustCrypto`'s pure-Rust implementation. The two have the same hash format, so hashes are interoperable.

**Data-engineering motivation:** When you build a user-facing system, you hash passwords. Argon2id with proper cost parameters is the modern standard.

## 2. Prerequisites

- Completed [08-Security/03-RustCryptoHashes](../../03-RustCryptoHashes/README.md) — familiar with hashing.
- Comfortable with `Result` and `Box<dyn Error>`.

## 3. Concept: The Argon2id Algorithm

Argon2id takes three parameters:
- `m_cost` (memory, in KB) — how much RAM to use per hash
- `t_cost` (iterations) — how many passes
- `p_cost` (parallelism) — how many threads

OWASP's 2024 recommendation: `m_cost = 19456 (19 MB), t_cost = 2, p_cost = 1`. The `Argon2::default()` uses these.

**Why these matter:** Each parameter adds cost. `m_cost = 65536 (64 MB)` and `t_cost = 3` would make a single hash take ~300ms. A 6-character password becomes 1000x harder to brute-force than SHA-256.

## 4. Concept: Hashing and Salting

```rust
use argon2::Argon2;
use password_hash::{PasswordHasher, SaltString};
use rand::rngs::OsRng;

let salt = SaltString::generate(&mut OsRng);
let hash = Argon2::default()
    .hash_password(b"hunter2", &salt)
    .map(|h| h.to_string())?;
```

The output string contains everything needed to verify later:
```
$argon2id$v=19$m=19456,t=2,p=1$<salt-b64>$<hash-b64>
```

**No separate salt storage needed** — the salt is in the hash.

**In Python (`argon2-cffi`):**

```python
from argon2 import PasswordHasher
ph = PasswordHasher()
hash = ph.hash("hunter2")
```

Same output format, same security.

## 5. Concept: Verification

```rust
use argon2::Argon2;
use password_hash::{PasswordHash, PasswordVerifier};

let parsed = PasswordHash::new(&hash)?;
Argon2::default().verify_password(b"hunter2", &parsed)?;
```

`verify_password` returns:
- `Ok(())` — correct password
- `Err(password_hash::Error::Password)` — wrong password
- `Err(other)` — malformed hash or other error

The library's verification is **constant-time**: it always does the full Argon2 computation, so an attacker can't time the response to guess the password.

## 6. Concept: Constant-Time Comparison

Sometimes you have a pre-computed hash (e.g., a session token) and need to compare it to a candidate. Use `subtle::ConstantTimeEq` to prevent timing attacks:

```rust
use subtle::ConstantTimeEq;
let a = b"secret_token_abc";
let b = b"secret_token_xyz";
let eq = a.ct_eq(b).into(); // bool, but constant-time
```

**Never use `==` for security-sensitive comparisons.** `==` short-circuits on the first differing byte, leaking timing information.

**In Python:** `hmac.compare_digest(a, b)`.

## 7. Putting It All Together

`lib.rs` is organized in four progressive steps:

1. **Step 1 (`step_01_hash_and_verify`)** — hash_password, verify_password.
2. **Step 2 (`step_02_salt`)** — generate_salt, hash_with_salt (deterministic with fixed salt).
3. **Step 3 (`step_03_validation`)** — is_password_valid (length check).
4. **Step 4 (`step_04_constant_time`)** — constant_time_eq via `subtle`.

`main.rs` ties it together: validate, hash, verify, reject wrong.

## 8. Complete Code Reference

See [`workshop/src/lib.rs`](workshop/src/lib.rs) and [`workshop/src/main.rs`](workshop/src/main.rs).

## 9. Summary

| Concept | Used In |
|---------|---------|
| `Argon2::default()` | `hash_password`, `verify_password` |
| `SaltString::generate(&mut OsRng)` | `generate_salt` |
| `PasswordHash::new` + `verify_password` | `verify_password` |
| `subtle::ConstantTimeEq` | `constant_time_eq` |
| Length validation | `is_password_valid` |

## Further Reading

- [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheatsheet.html)
- [Argon2 RFC 9106](https://datatracker.ietf.org/doc/html/rfc9106)
- [RustCrypto Argon2 docs](https://docs.rs/argon2/)
- ssojet.com, "Argon2 in Rust for password hashing" (Medium, Oct 2025)
- compile7.org, "Implementing Argon2id" (Medium, Aug 2025)

## Exercises

1. **Easy**: Add a `hash_password_with_params(password, m_cost, t_cost)` function that uses custom parameters, and 1 test.
2. **Medium**: Add a `verify_password_with_old_params(password, hash, m_cost)` that detects when a hash uses old parameters and returns `Ok(false)` instead of erroring.
3. **Hard**: Add a `needs_rehash(hash, m_cost, t_cost)` function that returns `true` if the stored hash was made with cost parameters below the current minimum (forcing a rehash on next login).

---

**Goal**: Implement all functions in `src/lib.rs` to pass all 10 tests.

## Functions to Implement

### Step 1 — Hash and verify

#### `hash_password`
- **Signature**: `pub fn hash_password(password: &str) -> Result<String, password_hash::Error>`
- **Task**: Generate a random salt, then `Argon2::default().hash_password(password.as_bytes(), &salt)?.to_string()`.

#### `verify_password`
- **Signature**: `pub fn verify_password(password: &str, hash: &str) -> Result<bool, password_hash::Error>`
- **Task**: `let parsed = PasswordHash::new(hash)?; Argon2::default().verify_password(password.as_bytes(), &parsed).map(|_| true).or_else(|e| if matches!(e, password_hash::Error::Password) { Ok(false) } else { Err(e) })`

### Step 2 — Salt

#### `generate_salt`
- **Signature**: `pub fn generate_salt() -> SaltString`
- **Task**: `SaltString::generate(&mut OsRng)`

#### `hash_with_salt`
- **Signature**: `pub fn hash_with_salt(password: &str, salt: &SaltString) -> Result<String, password_hash::Error>`
- **Task**: `Argon2::default().hash_password(password.as_bytes(), salt).map(|h| h.to_string())`

### Step 3 — Validation

#### `is_password_valid`
- **Signature**: `pub fn is_password_valid(password: &str, min_length: usize) -> bool`
- **Task**: `password.chars().count() >= min_length && !password.is_empty()`

### Step 4 — Constant-time comparison

#### `constant_time_eq`
- **Signature**: `pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool`
- **Task**: `a.ct_eq(b).into()`

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_hash_and_verify | 3 | Hash + verify correct + reject wrong |
| step_02_salt | 2 | Salt uniqueness + determinism with same salt |
| step_03_validation | 2 | Min-length check + reject empty |
| step_04_constant_time | 3 | `subtle::ConstantTimeEq` correctness |

## How to Run Tests
```bash
cargo test
```
