# Workshop: Rustls TLS

**Goal**: Implement all functions in `src/lib.rs` to pass all 5 tests.

> **Compile-heavy workshop**: `rustls` pulls in `aws-lc-rs` (a C/ASM crypto
> library). First `cargo test` may take 5-10 minutes. Subsequent runs are
> cached.

## Functions to Implement

### Step 1 — Build configs

#### `build_server_config`
- **Signature**: `pub fn build_server_config(cert: Vec<CertificateDer<'static>>, key: PrivateKeyDer<'static>) -> Result<Arc<ServerConfig>, rustls::Error>`
- **Task**: `ServerConfig::builder().with_no_client_auth().with_single_cert(cert, key).map(Arc::new)`

#### `build_client_config`
- **Signature**: `pub fn build_client_config() -> Arc<ClientConfig>`
- **Task**: `ClientConfig::builder().dangerous().with_custom_certificate_verifier(Arc::new(SkipVerifier)).with_no_client_auth().unwrap()` (or with `webpki_roots`).

### Step 2 — Name parsing

#### `parse_server_name`
- **Signature**: `pub fn parse_server_name(name: &str) -> Result<ServerName<'static>, rustls::pki_types::InvalidDnsNameError>`
- **Task**: `ServerName::try_from(name.to_string())`

### Step 3 — Echo roundtrip

#### `run_echo_server`
- **Signature**: `pub async fn run_echo_server(addr: &str, config: Arc<ServerConfig>) -> std::io::Result<()>`
- **Task**: Bind a `TcpListener`, accept connections, wrap each in `TlsAcceptor::from(config).accept(stream)`, echo the first 1024 bytes back.

#### `tls_echo_roundtrip`
- **Signature**: `pub async fn tls_echo_roundtrip(server_addr, server_config, message) -> Result<Vec<u8>, Box<dyn Error>>`
- **Task**: Connect a `TcpStream`, wrap in `TlsConnector::from(client_config).connect(name, stream)`, write `message`, read response.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_build_configs | 2 | ServerConfig + ClientConfig construction |
| step_02_name_parsing | 2 | DNS name parse |
| step_03_tls_handshake | 1 | Spawn server + connect (with dummy cert) |

## How to Run Tests
```bash
cargo test
```

## Generating a Self-Signed Certificate

For real TLS testing, generate a cert with `rcgen` (or use the test certs from the
`rustls` examples):

```rust
use rcgen::{generate_simple_self_signed, CertifiedKey};

let CertifiedKey { cert, key_pair } = generate_simple_self_signed(vec!["localhost".into()])?;
let cert_der = cert.der().to_vec();
let key_der = key_pair.serialized_der().to_vec();
```

In production, get a real cert from Let's Encrypt.
