# Workshop: Ed25519 Digital Signatures

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
