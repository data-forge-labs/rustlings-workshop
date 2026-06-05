# Workshop: Axum Auth — JWT + Bearer Middleware

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 15 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 15 tests.

## Functions to Implement

### Step 1 — Sign / verify

#### `sign_token`
- **Signature**: `pub fn sign_token(claims: &Claims, secret: &[u8]) -> Result<String, AuthError>`
- **Task**: `encode(&Header::new(Algorithm::HS256), claims, &EncodingKey::from_secret(secret))` → `Ok(jwt_string)`. Use `?` to convert the `jsonwebtoken::errors::Error` into `AuthError::InvalidToken(format!("{e}"))`.

#### `verify_token`
- **Signature**: `pub fn verify_token(token: &str, secret: &[u8]) -> Result<Claims, AuthError>`
- **Task**: `decode::<Claims>(token, &DecodingKey::from_secret(secret), &Validation::new(Algorithm::HS256))`. Return `Ok(data.claims)`, or map errors to `AuthError::InvalidToken(...)`.

### Step 2 — Extract Bearer

#### `extract_bearer`
- **Signature**: `pub fn extract_bearer(authorization_header: &str) -> Result<&str, AuthError>`
- **Task**: Empty → `MissingHeader`. Split once on `' '`; expect exactly 2 parts and the first to be `"Bearer"`, otherwise `InvalidScheme`. Return the second part.

### Step 3 — Expiration

#### `is_expired`
- **Signature**: `pub fn is_expired(claims: &Claims) -> bool`
- **Task**: `Utc::now().timestamp() > claims.exp`.

### Step 4 — Token factories

#### `create_access_token`
- **Signature**: `pub fn create_access_token(subject: &str, role: &str, secret: &[u8], ttl_seconds: i64) -> Result<String, AuthError>`
- **Task**: Build `Claims { sub, role, iat: now, exp: now + ttl_seconds }`, then `sign_token`.

#### `create_refresh_token`
- **Signature**: `pub fn create_refresh_token(subject: &str, secret: &[u8]) -> Result<String, AuthError>`
- **Task**: Same as access token but with `ttl_seconds = 30 * 86_400` (30 days) and `role = "refresh"`.

### Step 5 — Roles

#### `has_role`
- **Signature**: `pub fn has_role(claims: &Claims, allowed: &[&str]) -> bool`
- **Task**: `allowed.iter().any(|r| *r == claims.role)`.

#### `require_role`
- **Signature**: `pub fn require_role(claims: &Claims, allowed: &[&str]) -> Result<(), AuthError>`
- **Task**: If `has_role` is true → `Ok(())`. Else `Err(AuthError::UnauthorizedRole(claims.role.clone()))`.

### Step 6 — Header inspection

#### `key_id_from_header`
- **Signature**: `pub fn key_id_from_header(token: &str) -> Option<&str>`
- **Task**: Decode the header (the part before the first `.`) as JSON; return the `kid` field. Use `jsonwebtoken::decode_header(token).ok().and_then(|h| h.kid)`.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_sign_verify | 3 | sign/verify roundtrip + wrong secret + garbage token |
| step_02_extract_bearer | 4 | bearer, lowercase rejected, missing, basic scheme rejected |
| step_03_expiration | 2 | fresh + past |
| step_04_create_tokens | 2 | access + refresh |
| step_05_roles | 4 | has_role ok/denied + require_role ok/denied |
| step_06_key_id | 1 | no kid by default |

## How to Run Tests
```bash
cargo test
```
