# 🔐 Axum Auth — JWT + Bearer Middleware for Axum 0.8

*Subtitle: a typed `Claims` extractor with role-based access control, plus the production wiring for Axum routes.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 15 tests pass**.

---

## Why JWT for an Axum API?

**Python pain:** A FastAPI service needs auth. Most teams reach for `fastapi-users`
or hand-rolled `Depends(get_current_user)`. The session/cookie variant is fine
for browser apps, but mobile + service-to-service callers need bearer tokens.
`python-jose` and `pyjwt` work, but the validation/role logic is always
re-implemented per project — and gets it wrong (`alg: none`, missing exp check,
role checks in the wrong layer).

**Rust fix:** `jsonwebtoken` is the canonical HS256/RS256/EdDSA signer. Pair
it with a typed `Claims` struct, derive `Serialize`/`Deserialize` for free
JSON, and the borrow checker prevents accidentally trusting an unverified
token. `axum::extract::FromRequestParts` makes the auth check itself an
extractor; the handler body never sees a token it hasn't validated.

```rust
async fn admin_only(claims: Claims) -> &'static str { "welcome, admin" }
// Axum rejects any request without a valid Bearer token before the
// handler runs.
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | JWT signing | `jsonwebtoken::encode` | `python-jose.encode` | One function, three algorithms |
| 2 | JWT verifying | `jsonwebtoken::decode` | `python-jose.decode` | Fails closed on bad signature or expiry |
| 3 | Bearer extraction | `&str` split on `"Bearer "` | `Authorization.split()` | Constant-time not needed (no secret) |
| 4 | Typed claims | `#[derive(Serialize, Deserialize)]` | `pydantic.BaseModel` | Compile-time field validation |
| 5 | Expiry checks | `Utc::now().timestamp() > exp` | `datetime.utcnow() > exp` | Same logic, no tz bugs |
| 6 | Role-based access | `has_role(&claims, &["admin"])` | `user.role in {"admin"}` | Pure function, easy to test |
| 7 | Axum extractor | `impl FromRequestParts for Claims` | `Depends(get_current_user)` | Auth runs before the handler |
| 8 | Audit logging | `tracing::info!` | `logging.info` | Structured, correlated with request |

---

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 15 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 15 tests.


---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib axum_auth_workshop
cd axum_auth_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "axum_auth_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = { version = "0.8", features = ["macros"] }
jsonwebtoken = "9"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower = "0.5"
tower-http = { version = "0.6", features = ["trace"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
thiserror = "1"
chrono = { version = "0.4", features = ["serde"] }

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "10-ProductionSystems/03-AxumAuth/workshop/src/lib.rs" src/lib.rs
cp "10-ProductionSystems/03-AxumAuth/workshop/src/main.rs" src/main.rs
```

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
