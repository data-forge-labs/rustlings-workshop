# 🦀 Rustls TLS — Python to Rust Workshop

*Subtitle: Build a TLS server and client with `rustls` — the pure-Rust TLS library.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 5 tests pass**.

> **Compile-heavy workshop**: This project depends on `rustls` + `aws-lc-rs` (a
> C/ASM crypto backend). The first `cargo test` may take 5-10 minutes.
> Subsequent runs are cached.

---

## Why Rustls for TLS?

**Python pain:** TLS in Python is invisible — `ssl.wrap_socket()` "just works" because `requests`, `urllib3`, and `http.client` all use OpenSSL under the hood. You never think about it until a CVE drops and you realize your whole stack is vulnerable to a 25-year-old C library's memory unsafety.

**Rust fix:** `rustls` is a TLS 1.2/1.3 implementation in pure Rust, with `aws-lc-rs` providing the cryptographic primitives. No C dependencies, no memory unsafety, no CVEs from OpenSSL. The API is async-first and integrates with `tokio` via `tokio-rustls`:

```rust
use rustls::{ServerConfig, pki_types::CertificateDer};
use tokio_rustls::TlsAcceptor;

let config = ServerConfig::builder()
    .with_no_client_auth()
    .with_single_cert(cert_chain, private_key)?;
let acceptor = TlsAcceptor::from(Arc::new(config));

let tls_stream = acceptor.accept(tcp_stream).await?;
// Now you have an encrypted byte stream
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Pure-Rust TLS | `rustls` | `ssl` (OpenSSL) | Memory-safe |
| 2 | Server config | `ServerConfig::builder()` | `ssl.SSLContext` | Type-safe config |
| 3 | Client config | `ClientConfig::builder()` | `ssl.create_default_context()` | Built-in cert verification |
| 4 | Async TLS | `tokio-rustls` | n/a | `async/await` integration |
| 5 | Self-signed cert | `rcgen::generate_simple_self_signed` | `openssl req -x509` | For testing |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: `rustls` and Crypto Backends](#3-concept-rustls-and-crypto-backends)
4. [Concept: `ServerConfig` and `ClientConfig`](#4-concept-serverconfig-and-clientconfig)
5. [Concept: TLS Handshake with `tokio-rustls`](#5-concept-tls-handshake-with-tokio-rustls)
6. [Concept: Self-Signed Certificates for Testing](#6-concept-self-signed-certificates-for-testing)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Complete Code Reference](#8-complete-code-reference)
9. [Summary](#9-summary)

## 1. Introduction

`rustls` is the de-facto TLS library in the Rust ecosystem. Used by:
- **curl** (since 7.84)
- **Apache HTTP Server** (mod_tls)
- **Linkerd** (service mesh)
- **BoringSSL** alternative in many Rust services

**Python to Rust:** Python uses OpenSSL through the `ssl` module. The library is well-tested but has had multiple memory-safety CVEs (Heartbleed, etc.). `rustls` is a drop-in replacement with no C dependencies.

**Data-engineering motivation:** When you build an internal service that talks to S3, Kafka, or Postgres, you're using TLS. Knowing how to set up a TLS server is essential for any production-grade data tool.

## 2. Prerequisites

- Completed [09-ProductionSystems/02-AxumShop](../09-ProductionSystems/02-AxumShop/README.md) — comfortable with async Rust + tokio.
- Familiar with `Result` and `Box<dyn Error>`.

## 3. Concept: `rustls` and Crypto Backends

`rustls` doesn't include crypto primitives — it delegates to a backend. Two main options:

- **`ring`** — pure-Rust, mature, but no longer recommended (in maintenance mode)
- **`aws-lc-rs`** — fork of BoringSSL, more algorithms, more platform support

This project uses `aws-lc-rs`. The choice is controlled by Cargo features:

```toml
rustls = { version = "0.23", default-features = false, features = ["std", "logging", "aws_lc_rs", "tls12"] }
```

`aws-lc-rs` includes a C/ASM implementation, so the first build pulls in a C compiler.

## 4. Concept: `ServerConfig` and `ClientConfig`

The server side needs a cert + key pair. The client side needs to know which CAs to trust.

**Server:**

```rust
use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

let config = ServerConfig::builder()
    .with_no_client_auth()
    .with_single_cert(cert_chain, private_key)?;
```

**Client (with system roots):**

```rust
use rustls::ClientConfig;
use rustls_platform_verifier::ConfigVerifierExt;

let config = ClientConfig::builder()
    .with_webpki_roots()
    .with_no_client_auth();
```

For testing, the client often uses `dangerous().with_custom_certificate_verifier(Arc::new(SkipVerifier))` to skip verification. **Never do this in production.**

## 5. Concept: TLS Handshake with `tokio-rustls`

`tokio-rustls` wraps a `TcpStream` into a `TlsStream` (server) or `ClientTlsStream` (client):

**Server (accept):**

```rust
use tokio_rustls::TlsAcceptor;

let acceptor = TlsAcceptor::from(Arc::new(server_config));
let tls_stream = acceptor.accept(tcp_stream).await?;
// tls_stream: TlsStream<TcpStream>
```

**Client (connect):**

```rust
use tokio_rustls::TlsConnector;
use rustls::pki_types::ServerName;

let connector = TlsConnector::from(Arc::new(client_config));
let name = ServerName::try_from("example.com")?;
let tls_stream = connector.connect(name, tcp_stream).await?;
```

After the handshake, both sides have a `TlsStream` that implements `AsyncRead + AsyncWrite` — same interface as `TcpStream`, but encrypted.

## 6. Concept: Self-Signed Certificates for Testing

For development, generate a self-signed cert with `rcgen`:

```rust
use rcgen::{generate_simple_self_signed, CertifiedKey};

let CertifiedKey { cert, key_pair } =
    generate_simple_self_signed(vec!["localhost".into()])?;
let cert_der = cert.der().to_vec();
let key_der = key_pair.serialized_der().to_vec();
```

For production, use Let's Encrypt (via `acme-lib` or `instant-acme`) or your internal CA.

**In Python:**

```bash
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes -subj '/CN=localhost'
```

## 7. Putting It All Together

`lib.rs` is organized in three progressive steps:

1. **Step 1 (`step_01_build_configs`)** — `build_server_config`, `build_client_config`.
2. **Step 2 (`step_02_name_parsing`)** — `parse_server_name` (DNS name → `ServerName`).
3. **Step 3 (`step_03_tls_handshake`)** — `run_echo_server`, `tls_echo_roundtrip`.

`main.rs` ties it together: bind a TCP listener, accept TLS connections, echo data.

## 8. Complete Code Reference

See [`workshop/src/lib.rs`](workshop/src/lib.rs) and [`workshop/src/main.rs`](workshop/src/main.rs).

## 9. Summary

| Concept | Used In |
|---------|---------|
| `ServerConfig::builder` | `build_server_config` |
| `ClientConfig::builder` | `build_client_config` |
| `ServerName::try_from` | `parse_server_name` |
| `TlsAcceptor::accept` | `run_echo_server` |
| `TlsConnector::connect` | `tls_echo_roundtrip` |
| `aws-lc-rs` crypto backend | All TLS operations |

## Further Reading

- [rustls documentation](https://docs.rs/rustls/)
- [rustls website](https://www.rustls.org/)
- [tokio-rustls docs](https://docs.rs/tokio-rustls/)
- [aws-lc-rs docs](https://docs.rs/aws-lc-rs/)

## Exercises

1. **Easy**: Add `build_client_config_with_custom_ca(ca_der: CertificateDer) -> Arc<ClientConfig>` that trusts only the given CA, and 1 test.
2. **Medium**: Add a `client_auth_required(cert: CertificateDer, key: PrivateKeyDer) -> Arc<ServerConfig>` that requires the client to present a cert, and 1 test.
3. **Hard**: Add a `tls_proxy(server_addr, backend_addr) -> !` function that accepts TLS connections on `server_addr`, opens plain TCP to `backend_addr`, and forwards bytes between them (TLS termination proxy).
