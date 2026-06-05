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
