# Rust Security Model — Reference

## Compile-Time Safety Guarantees

Rust's safety guarantees are enforced **before the program runs** — not at runtime.

```
Source Code → Borrow Checker → Type Checker → LLVM → Binary
                  │                  │
                  └─── No dangling   └─── No type confusion
                        references         No integer overflow (debug)
                        No data races       No buffer overruns
                        No double free
```

## Type System as Security

```
┌───────────────────┐      ┌──────────────────┐
│ Option<T>         │ ───→ │ No null pointers │
│ Result<T, E>      │ ───→ │ No unhandled err │
│ Send / Sync       │ ───→ │ No data races    │
│ &T / &mut T       │ ───→ │ No aliasing UB   │
│ PhantomData       │ ───→ │ Type-level invari│
│ Newtype pattern   │ ───→ │ Domain safety    │
└───────────────────┘      └──────────────────┘
```

Example — newtype prevents unit confusion:

```rust
struct Meters(f64);
struct Feet(f64);

fn build_bridge(length: Meters) { /* ... */ }
// build_bridge(Feet(100.0)); // compile error — type mismatch
```

## `unsafe` — When and Why

Use `unsafe` when you need to:
1. Call C/FFI functions (most common reason)
2. Dereference raw pointers
3. Implement `Send`/`Sync` for custom types
4. Inline assembly
5. Access mutable statics

**Safety contract**: The caller must ensure invariants that the compiler cannot check.

```rust
unsafe trait TrustedSource { }
unsafe impl TrustedSource for MyType { }

unsafe fn risky() {
    // Must document preconditions
}

// Always wrap unsafe in a safe API
pub fn safe_wrapper() {
    unsafe { risky(); }
}
```

## Cryptography with RustCrypto

```toml
[dependencies]
sha2 = "0.10"
hex = "0.4"
aes = "0.8"
```

```rust
use sha2::{Sha256, Digest};

let hash = Sha256::digest(b"hello world");
let hex_hash = hex::encode(hash);
assert_eq!(hex_hash.len(), 64);
```

Common crates:
| Crate | Purpose |
|-------|---------|
| `sha2` | SHA-256/512 |
| `aes` | AES encryption |
| `ed25519-dalek` | Ed25519 signatures |
| `rand` | Cryptographically secure RNG |
| `ring` | BoringSSL-based crypto |
| `rustls` | TLS (no OpenSSL dependency) |

## Security Comparison

| Aspect | C/C++ | Java | Python | Rust |
|--------|-------|------|--------|------|
| Memory safety | ❌ Manual | ✅ GC | ✅ GC | ✅ Borrow checker |
| Null safety | ❌ Null ptrs | ❌ Null refs | ❌ None | ✅ Option<T> |
| Thread safety | ❌ Data races | ❌ Must synchronize | ❌ GIL helps | ✅ Send/Sync |
| Buffer overflow | ❌ Common | ✅ Bounds check | ✅ Bounds check | ✅ Bounds check |
| Type confusion | ❌ Casts | ✅ Strong typing | ✅ Duck typing | ✅ Strong + algebraic |
| Side-channel resistant | Manual | Manual | Manual | `secrets` crate |

## High-Availability Security Checklist

1. **Do not use `unwrap()` in production** — handle `Result` and `Option` with proper errors.
2. **Pin secrets in memory** — use `secrecy` crate to prevent zeroing.
3. **Limit `unsafe`** — wrap in minimal safe APIs with `#[deny(unsafe_code)]` where possible.
4. **Validate input** — use typed deserialization (Serde) rather than raw parsing.
5. **Prevent timing attacks** — use constant-time comparison (`subtle` crate).
6. **Use `rustls`** over OpenSSL bindings for TLS (memory-safe TLS).
7. **Pin dependency versions** — use `Cargo.lock` and audit with `cargo audit`.
8. **Log with context** — use `tracing` for structured, auditable logs.
