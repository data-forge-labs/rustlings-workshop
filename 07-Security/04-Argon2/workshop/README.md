# Workshop: Argon2 Password Hashing

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
