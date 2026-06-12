# Building a Shop Manager API with Axum — The FastAPI of Rust

*Step-by-step workshop that migrates a full-stack FastAPI project to an async Rust backend, from database setup to authentication, CRUD, and nested orders.*

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all N tests pass**.

---

## Why Migrate FastAPI to Axum?

**Python pain:** FastAPI is great for prototyping, but the GIL limits concurrency, every request allocates Python objects (10K concurrent = 500MB+ GC pressure), SQLAlchemy lazy loading causes N+1 queries, and the Docker image is 200MB+ vs Rust's ~15MB.

**Rust fix:** Axum on Tokio — async, single-binary, ~15MB Docker image, no GIL, no per-request Python allocation:

```rust
use axum::{Router, routing::get, extract::State};

async fn list_products(State(pool): State<AppState>) -> Result<Json<Vec<Product>>, AppError> {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(&pool).await?;
    Ok(Json(products))
}

let app = Router::new()
    .route("/api/products", get(list_products))
    .with_state(state);
```

Axum brings the FastAPI developer experience to Rust:
- **Type-safe extractors** — path, query, JSON, and custom dependencies validated at compile time
- **tokio-powered** — true async I/O across all CPU cores
- **Tower middleware** — composable, the same ecosystem as production services
- **Zero-cost abstractions** — compiles to native code, ~15MB binary, no runtime overhead

This workshop rebuilds a full Shop Manager backend (FastAPI → Axum) end-to-end: database, authentication, CRUD, nested orders with transactions.

---

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Web framework | `axum::Router` | `FastAPI()` | Route definition and request dispatch |
| 2 | Async runtime | `tokio` | `uvicorn` + `asyncio` | Multi-threaded async executor |
| 3 | Request extractors | `Path`, `Query`, `Json`, `State` | Pydantic + `Depends` | Type-safe request parsing |
| 4 | Async database | `sqlx` | `SQLAlchemy` + `asyncpg` | Async SQL with compile-time checking |
| 5 | JSON serialization | `serde` / `serde_json` | Pydantic | Zero-cost struct ↔ JSON conversion |
| 6 | Middleware | `tower-http`, `tower-sessions` | Starlette middleware | CORS, sessions, logging |
| 7 | Custom extractors | `FromRequestParts` trait | `Depends()` | Auth/injection via the type system |
| 8 | Password hashing | `sha2` + `hex` | `hashlib` | SHA-256 secure password storage |
| 9 | CORS | `CorsLayer` | `CORSMiddleware` | Cross-origin request handling |
| 10 | DB transactions | `pool.begin()` + `tx.commit()` | `db.commit()` | Atomic multi-table operations |
| 11 | Error handling | `IntoResponse` for custom enums | exception handlers | Type-safe error → HTTP response |
| 12 | Module organization | `mod`, routes as modules | `APIRouter` prefix | Clean codebase structure |

---

## Concepts at a Glance

### 1. `axum::Router` — Route Definition
Like `FastAPI()`, you define routes with `.route("/path", method(handler))`. Routes can be nested and merged — equivalent to FastAPI's `include_router`.

### 2. `tokio` — Async Runtime
Like Uvicorn running your FastAPI app. Tokio is a multi-threaded work-stealing runtime (unlike asyncio's single-threaded event loop). Handlers are `async fn` and `.await` database calls without blocking.

### 3. Extractors — `Path`, `Query`, `Json`, `State`
Axum uses **extractors** to pull data from requests, like FastAPI's `Depends()`:

```rust
async fn get_product(
    Path(id): Path<i32>,                    // URL path parameter
    Query(params): Query<ProductQuery>,     // Query string → struct
    State(pool): State<AppState>,           // Shared application state
    Json(body): Json<CreateProduct>,        // Request body → struct
) -> Result<..., AppError> {
    // All typed at compile time!
}
```

Python equivalent: FastAPI's `id: int`, `search: str = Query(None)`, `db: Session = Depends(get_db)`, `body: CreateProduct`.

### 4. `sqlx` — Async Database
Like SQLAlchemy but fully async. Queries are checked at compile time with the `query!` macro. Runs on SQLite, PostgreSQL, MySQL — same API for all.

```rust
let products = sqlx::query_as::<_, Product>(
    "SELECT * FROM products WHERE category_id = ?"
).bind(cat_id).fetch_all(&pool).await?;
```

### 5. `serde` — Serialization
Like Pydantic but zero-cost. Derive `Serialize`/`Deserialize` on structs for automatic JSON conversion:

```rust
#[derive(Serialize, Deserialize)]
struct Product { id: i32, name: String, price: f64 }
```

### 6. Middleware — `tower-http` / `tower-sessions`
Rust's middleware ecosystem (Tower) is composable and generic. Same middleware stack can wrap HTTP, gRPC, or custom protocols. CORS, sessions, compression, logging — all pluggable layers.

### 7. `FromRequestParts` — Custom Extractors
Like FastAPI's `Depends()` but with compile-time type checking. The `Auth` extractor in this project reads the session and returns `401` automatically if not authenticated — the handler never needs to check.

### 8-9. `sha2` + CORS
Password hashing uses the same SHA-256 as Python's `hashlib`. CORS middleware controls which origins can call your API — identical to FastAPI's `CORSMiddleware`.

### 10. Transactions
`pool.begin()` starts a transaction, `tx.commit()` finalizes it. All operations within the transaction are atomic — if anything fails, everything rolls back. Essential for order creation (insert order + items + update stock).

### 11. Error Handling
Custom error enums implementing `IntoResponse` let you return errors as HTTP responses — like FastAPI's exception handlers but checked at compile time.

### 12. Module Organization
Rust's `mod` system maps to FastAPI's `APIRouter` with prefixes. Each route file exports a `fn router() -> Router<AppState>` that gets merged into the main app.

---

## Table of Contents

- [Table of Contents](#table-of-contents)
- [1. Why Axum? The FastAPI of Rust](#1-why-axum-the-fastapi-of-rust)
- [2. Prerequisites & Project Setup](#2-prerequisites--project-setup)
- [3. Understanding the Application Architecture](#3-understanding-the-application-architecture)
- [4. Step 1: Adding Dependencies](#4-step-1-adding-dependencies)
- [5. Step 2: Database Setup with SQLx (Async)](#5-step-2-database-setup-with-sqlx-async)
- [6. Step 3: Running Migrations & Seeding](#6-step-3-running-migrations--seeding)
- [7. Step 4: Defining Models – Rust Structs as Tables](#7-step-4-defining-models--rust-structs-as-tables)
- [8. Step 5: Application State – Sharing the Database Pool](#8-step-5-application-state--sharing-the-database-pool)
- [9. Step 6: Authentication & Session Middleware](#9-step-6-authentication--session-middleware)
- [10. Step 7: Organising Routes with Modules](#10-step-7-organising-routes-with-modules)
- [11. Step 8: Auth API – Login & Logout](#11-step-8-auth-api--login--logout)
- [12. Step 9: Dashboard Endpoint](#12-step-9-dashboard-endpoint)
- [13. Step 10: Products CRUD (with Search & Pagination)](#13-step-10-products-crud-with-search--pagination)
- [14. Step 11: Categories & Customers CRUD](#14-step-11-categories--customers-crud)
- [15. Step 12: Orders & Order Items – Nested Responses & Transactions](#15-step-12-orders--order-items--nested-responses--transactions)
- [16. Step 13: Putting It All Together – The Main Router](#16-step-13-putting-it-all-together--the-main-router)
- [17. Running the Server & Testing](#17-running-the-server--testing)
- [18. Summary of Migration](#18-summary-of-migration)

---

## 1. Why Axum? The FastAPI of Rust

Before we code, let's revisit the "why". FastAPI is loved for:

- Automatic request validation via Pydantic models
- Async support out of the box
- Dependency injection with `Depends`
- Auto-generated OpenAPI docs
- A clean, declarative routing style

Axum brings the same philosophy to Rust:

- **Type-driven extractors** – path, query, JSON, and custom dependencies are validated at compile time.
- **Async/await everywhere** – built on the Tokio runtime.
- **Tower middleware** – sessions, CORS, logging, etc. plug right in.
- **Flexible routing** – just like FastAPI's `APIRouter`.
- **Performance** – Rust's zero-cost abstractions deliver C-level speed with memory safety.

In this workshop, we'll rebuild your **Shop Manager** backend exactly as it exists in FastAPI, including:

- Admin authentication with session cookies
- Dashboard with aggregated counts
- Full CRUD for products, categories, and customers
- Orders with nested items, stock validation, and transactions
- Database seeding matching your `seed.py`

---

## 2. Prerequisites & Project Setup

Make sure you have Rust installed. Open your terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version   # ≥ 1.75
cargo --version
```

Create a new Rust project:

```bash
cargo new shop_manager_rs
cd shop_manager_rs
```

Now you have a `Cargo.toml` and `src/main.rs`. We'll replace `main.rs` later.

---

## 3. Understanding the Application Architecture

Your FastAPI project follows this layout:

```
backend/
├── app/
│   ├── api/          (routers: auth, products, …)
│   ├── models/       (SQLAlchemy models)
│   ├── schemas/      (Pydantic request/response schemas)
│   ├── static/
│   ├── database.py   (engine, get_db)
│   └── main.py       (app factory, middleware, docs)
├── seed.py
├── shop.db
└── requirements.txt
```

In Rust, we'll flatten the structure into a few source files (no sub-directories for routes/models, but we use modules):

```
shop_manager_rs/
├── Cargo.toml
├── app/static/           (empty or with assets)
└── src/
    ├── main.rs           (server startup, router assembly)
    ├── db.rs             (pool, migrations, seeding)
    ├── models.rs         (structs for rows and DTOs)
    ├── auth.rs           (custom extractor for session auth)
    ├── errors.rs         (unified error type)
    └── routes/           (one file per API module)
        ├── mod.rs
        ├── auth_routes.rs
        ├── dashboard.rs
        ├── products.rs
        ├── categories.rs
        ├── customers.rs
        └── orders.rs
```

The overall request lifecycle will look exactly like FastAPI's, but with Axum's extractors and async database.

---

## 4. Step 1: Adding Dependencies

Edit `Cargo.toml` to include the following crates:

```toml
[package]
name = "shop_manager_rs"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }
tower-http = { version = "0.5", features = ["cors", "fs"] }
tower-sessions = "0.12"
tower-cookies = "0.10"
sha2 = "0.10"
hex = "0.4"
chrono = { version = "0.4", features = ["serde"] }
```

| Crate | Purpose | FastAPI Equivalent |
|-------|---------|-------------------|
| `axum` | Web framework, routing, extractors | FastAPI core |
| `tokio` | Async runtime (like Uvicorn) | Uvicorn / Starlette |
| `serde` + `serde_json` | Serialisation / deserialisation | Pydantic + `json` |
| `sqlx` | Async SQL toolkit, connection pool | SQLAlchemy + asyncpg |
| `tower-http` | CORS middleware, static file serving | CORSMiddleware, StaticFiles |
| `tower-sessions` | Cookie-based sessions | SessionMiddleware |
| `sha2` + `hex` | SHA-256 hashing (password check) | hashlib |
| `chrono` | Date/time handling | datetime |

Run `cargo build` to download and compile everything.

---

## 5. Step 2: Database Setup with SQLx (Async)

FastAPI uses SQLAlchemy with a `Session` dependency. In Rust, we use `sqlx` with a **connection pool** that is shared across all handlers.

Create `src/db.rs`. First, we'll write a function that creates a SQLite pool and enables WAL mode for concurrency.

```rust
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

/// Create the connection pool (SQLite)
pub async fn create_pool() -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:shop.db?mode=rwc")
        .await
        .expect("Failed to create database pool")
}
```

Later, we'll use `&*pool` to pass a reference to handlers (via `State`).

> **Python comparison:**
> ```python
> engine = create_engine("sqlite:///./shop.db", connect_args={"check_same_thread": False})
> SessionLocal = sessionmaker(bind=engine)
> ```

---

## 6. Step 3: Running Migrations & Seeding

We need to create the tables exactly as defined in your SQL schema. Add a `run_migrations` function to `db.rs`:

```rust
pub async fn run_migrations(pool: &SqlitePool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS admins (
            id INTEGER NOT NULL,
            username VARCHAR(50) NOT NULL,
            password_hash VARCHAR(128) NOT NULL,
            created_at DATETIME,
            PRIMARY KEY (id),
            UNIQUE (username)
        );
        -- ... (all other tables exactly as in the schema)
        CREATE TABLE IF NOT EXISTS order_items (
            id INTEGER NOT NULL,
            order_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            unit_price FLOAT NOT NULL,
            PRIMARY KEY (id),
            FOREIGN KEY(order_id) REFERENCES orders (id),
            FOREIGN KEY(product_id) REFERENCES products (id)
        );
        "#
    )
    .execute(pool)
    .await
    .unwrap();

    // Enable WAL mode for better concurrent read/write performance
    sqlx::query("PRAGMA journal_mode=WAL;")
        .execute(pool)
        .await
        .unwrap();
}
```

Now, the seeding function – identical logic to `seed.py`. Add it to `db.rs`:

```rust
use sha2::{Digest, Sha256};

pub async fn seed_database(pool: &SqlitePool) {
    // Check if admins already exist (skip seeding)
    let admin_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM admins")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));
    if admin_count.0 > 0 {
        return;
    }

    // Admin: username "admin", password "admin123"
    let admin_hash = hex::encode(Sha256::digest(b"admin123"));
    sqlx::query("INSERT INTO admins (username, password_hash, created_at) VALUES (?, ?, datetime('now'))")
        .bind("admin")
        .bind(&admin_hash)
        .execute(pool)
        .await
        .unwrap();

    // --- Customers, categories, products, orders (exactly as in seed.py) ---
    // Insert 5 customers, 5 categories, 21 products, 8 orders with items
    // ... (full code as provided in the previous full project snippet)
    println!("✅ Database seeded successfully.");
}
```

> **Python comparison:** The `seed.py` script runs the same inserts. In Rust we do it in the same binary.

---

## 7. Step 4: Defining Models – Rust Structs as Tables

In FastAPI, you have two layers:
- **SQLAlchemy models** (`app/models/`) – for database rows.
- **Pydantic schemas** (`app/schemas/`) – for request/response validation.

In Rust, we can combine them using `serde` derives. We'll create two kinds of structs:
- **Row structs** (with `sqlx::FromRow`) – map exactly to table columns.
- **DTO structs** (with `Deserialize` / `Serialize`) – for request bodies and nested responses.

Create `src/models.rs`:

```rust
use serde::{Deserialize, Serialize};

// ---------- Database row types (use sqlx::FromRow) ----------

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub category_id: i32,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Order {
    pub id: i32,
    pub customer_id: i32,
    pub order_date: Option<String>,
    pub total_amount: f64,
    pub status: String,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub unit_price: f64,
}

// ---------- Request / Response DTOs (like Pydantic schemas) ----------

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// ... (all request/update structs as shown in the earlier complete code)
```

> **Diagram – Struct mapping:**
> ```
> SQL Table "products"          Rust struct Product
> +------------------+          +--------------------+
> | id (INTEGER)     |  <--->   | id: i32            |
> | name (VARCHAR)   |          | name: String       |
> | price (FLOAT)    |          | price: f64         |
> | ...              |          | ...                |
> +------------------+          +--------------------+
> ```

---

## 8. Step 5: Application State – Sharing the Database Pool

In FastAPI, you use `Depends(get_db)` to inject a database session. In Axum, we inject a **shared reference** to the connection pool using the `State` extractor.

We define a type alias for our state:

```rust
use std::sync::Arc;
pub type AppState = Arc<sqlx::SqlitePool>;
```

Then, when we create the router, we attach the state:

```rust
let pool = db::create_pool().await;
let state: AppState = Arc::new(pool);

let app = Router::new()
    .merge(routes::auth_routes::router())
    // ... more routes
    .with_state(state);
```

Handlers access it with:

```rust
async fn list_products(State(pool): State<AppState>) -> ... {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(&*pool)
        .await?;
    // ...
}
```

**Key point:** `Arc` allows multiple handlers to hold immutable references to the pool; `SqlitePool` is already thread-safe.

> **Python comparison:**
> ```python
> def get_products(db: Session = Depends(get_db)):
>     return db.query(Product).all()
> ```

---

## 9. Step 6: Authentication & Session Middleware

FastAPI uses `SessionMiddleware` and checks `request.session.get("admin_id")`. We'll replicate this with `tower-sessions` and a **custom extractor**.

Create `src/auth.rs`:

```rust
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use tower_sessions::Session;

/// Custom extractor that requires a valid admin session.
pub struct Auth {
    pub admin_id: i32,
}

#[async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

        let admin_id: Option<i32> = session.get("admin_id").await.unwrap_or(None);
        match admin_id {
            Some(id) => Ok(Auth { admin_id: id }),
            None => Err((StatusCode::UNAUTHORIZED, "Not authenticated").into_response()),
        }
    }
}
```

Now any handler can require authentication simply by adding `_auth: Auth` as a parameter.

**Middleware setup** (in `main.rs`):

```rust
use tower_sessions::{cookie::Key, Expiry, MemoryStore, SessionManagerLayer};
use tower_cookies::CookieManagerLayer;

let session_store = MemoryStore::default();
let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(false)   // true in production
    .with_expiry(Expiry::OnInactivity(time::Duration::hours(24)));

let app = Router::new()
    .layer(CookieManagerLayer::new())
    .layer(session_layer)
    // ...
```

> **Python comparison:**
> ```python
> app.add_middleware(SessionMiddleware, secret_key="...")
> def require_auth(request: Request):
>     return request.session.get("admin_id")
> ```

---

## 10. Step 7: Organising Routes with Modules

FastAPI uses `app.include_router(auth_api.router, prefix="/api/auth")`. Axum lets us do the same with sub-routers.

Create `src/routes/mod.rs`:

```rust
pub mod auth_routes;
pub mod dashboard;
pub mod products;
pub mod categories;
pub mod customers;
pub mod orders;
```

Each module exports a public function that returns a `Router<AppState>`.

For example, `auth_routes.rs` will export:

```rust
use crate::AppState;
pub fn router() -> Router<AppState> { ... }
```

In `main.rs`, we merge them:

```rust
let app = Router::new()
    .merge(routes::auth_routes::router())
    .merge(routes::dashboard::router())
    .merge(routes::products::router())
    // ...
    .with_state(state);
```

This is exactly analogous to FastAPI's `include_router`.

---

## 11. Step 8: Auth API – Login & Logout

Now we build the first actual endpoints. The `auth_routes` module matches `auth_api.py`.

```rust
// src/routes/auth_routes.rs
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use sha2::{Digest, Sha256};
use tower_sessions::Session;

use crate::errors::AppError;
use crate::models::{Admin, LoginRequest};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
}

async fn login(
    State(pool): State<AppState>,
    session: Session,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let hash = hex::encode(Sha256::digest(payload.password.as_bytes()));

    let admin = sqlx::query_as::<_, Admin>(
        "SELECT * FROM admins WHERE username = ? AND password_hash = ?"
    )
    .bind(&payload.username)
    .bind(&hash)
    .fetch_optional(&*pool)
    .await?;

    match admin {
        Some(admin) => {
            session.insert("admin_id", admin.id).await
                .map_err(|e| AppError::Internal(e.to_string()))?;
            Ok(Json(serde_json::json!({"message": "Login successful"})))
        }
        None => Err(AppError::BadRequest("Invalid credentials".into())),
    }
}

async fn logout(session: Session) -> impl IntoResponse {
    session.flush();
    Json(serde_json::json!({"message": "Logged out"}))
}
```

**Flow:**
1. Client sends `POST /api/auth/login` with JSON `{username, password}`.
2. Server hashes the password and queries the `admins` table.
3. If match, store `admin_id` in the session cookie.
4. For logout, flush the session.

> **Python comparison:**
> ```python
> @router.post("/login")
> async def login(credentials: ..., db: Session = Depends(get_db)):
>     # check hash, set session
> ```

---

## 12. Step 9: Dashboard Endpoint

FastAPI's dashboard returns aggregated counts. In Axum, we'll create `src/routes/dashboard.rs`:

```rust
use axum::{extract::State, routing::get, Json, Router};
use crate::auth::Auth;
use crate::errors::AppError;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/api/dashboard", get(dashboard))
}

async fn dashboard(
    State(pool): State<AppState>,
    _auth: Auth,
) -> Result<Json<serde_json::Value>, AppError> {
    let total_products: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products").fetch_one(&*pool).await?;
    let total_categories: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM categories").fetch_one(&*pool).await?;
    let total_customers: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM customers").fetch_one(&*pool).await?;
    let total_orders: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM orders").fetch_one(&*pool).await?;
    let total_revenue: (Option<f64>,) = sqlx::query_as("SELECT SUM(total_amount) FROM orders WHERE status = 'completed'")
        .fetch_one(&*pool).await?;

    Ok(Json(serde_json::json!({
        "total_products": total_products.0,
        "total_categories": total_categories.0,
        "total_customers": total_customers.0,
        "total_orders": total_orders.0,
        "total_revenue": total_revenue.0.unwrap_or(0.0),
    })))
}
```

Notice the `Auth` extractor – only authenticated admins can access the dashboard.

> **Python comparison:** FastAPI uses `require_auth(request)` checks. Here the extractor does it automatically.

---

## 13. Step 10: Products CRUD (with Search & Pagination)

This replicates `products_api.py`. We'll implement five handlers: `list`, `get_one`, `create`, `update`, `delete`.

File: `src/routes/products.rs`

```rust
use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use crate::auth::Auth;
use crate::errors::AppError;
use crate::models::{Product, CreateProductRequest, UpdateProductRequest};
use crate::AppState;

#[derive(Deserialize)]
pub struct ProductQuery {
    pub search: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/products", get(list).post(create))
        .route("/api/products/:id", get(get_one).put(update).delete(delete_one))
}

async fn list(
    State(pool): State<AppState>,
    _auth: Auth,
    Query(params): Query<ProductQuery>,
) -> Result<Json<Vec<Product>>, AppError> {
    let search = params.search.unwrap_or_default();
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(10).min(100);
    let offset = (page - 1) * per_page;

    let products = if search.is_empty() {
        sqlx::query_as::<_, Product>("SELECT * FROM products ORDER BY name LIMIT ? OFFSET ?")
            .bind(per_page as i32)
            .bind(offset as i32)
            .fetch_all(&*pool)
            .await?
    } else {
        sqlx::query_as::<_, Product>(
            "SELECT * FROM products WHERE name LIKE ? ORDER BY name LIMIT ? OFFSET ?"
        )
        .bind(format!("%{}%", search))
        .bind(per_page as i32)
        .bind(offset as i32)
        .fetch_all(&*pool)
        .await?
    };
    Ok(Json(products))
}

async fn get_one(
    Path(id): Path<i32>,
    State(pool): State<AppState>,
    _auth: Auth,
) -> Result<Json<Product>, AppError> {
    let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = ?")
        .bind(id)
        .fetch_optional(&*pool)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(product))
}

async fn create(
    State(pool): State<AppState>,
    _auth: Auth,
    Json(payload): Json<CreateProductRequest>,
) -> Result<(axum::http::StatusCode, Json<Product>), AppError> {
    let product = sqlx::query_as::<_, Product>(
        "INSERT INTO products (name, description, price, stock, category_id, created_at)
         VALUES (?, ?, ?, ?, ?, datetime('now')) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.price)
    .bind(payload.stock)
    .bind(payload.category_id)
    .fetch_one(&*pool)
    .await?;
    Ok((axum::http::StatusCode::CREATED, Json(product)))
}

async fn update(
    Path(id): Path<i32>,
    State(pool): State<AppState>,
    _auth: Auth,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<Json<Product>, AppError> {
    let existing = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = ?")
        .bind(id)
        .fetch_optional(&*pool)
        .await?
        .ok_or(AppError::NotFound)?;

    let updated = sqlx::query_as::<_, Product>(
        "UPDATE products SET
            name = ?, description = ?, price = ?, stock = ?, category_id = ?
         WHERE id = ? RETURNING *"
    )
    .bind(payload.name.unwrap_or(existing.name))
    .bind(payload.description.or(existing.description))
    .bind(payload.price.unwrap_or(existing.price))
    .bind(payload.stock.unwrap_or(existing.stock))
    .bind(payload.category_id.unwrap_or(existing.category_id))
    .bind(id)
    .fetch_one(&*pool)
    .await?;
    Ok(Json(updated))
}

async fn delete_one(
    Path(id): Path<i32>,
    State(pool): State<AppState>,
    _auth: Auth,
) -> Result<(axum::http::StatusCode, Json<serde_json::Value>), AppError> {
    let rows = sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await?
        .rows_affected();
    if rows == 0 {
        Err(AppError::NotFound)
    } else {
        Ok((axum::http::StatusCode::OK, Json(serde_json::json!({"message": "Product deleted"}))))
    }
}
```

**Highlights:**
- `Path(id)` extracts `id` from the URL.
- `Query(params)` deserialises query string into a struct.
- `Json(payload)` deserialises the request body.
- Errors are returned as `AppError`, which we'll define in `errors.rs`.

> **Python comparison:**
> ```python
> @router.get("/products")
> async def list_products(search: str = None, page: int = 1, db: Session = Depends(get_db)):
>     # similar logic
> ```

---

## 14. Step 11: Categories & Customers CRUD

These follow the exact same pattern as products but with different fields. For brevity, I'll outline the differences.

**Categories** (`src/routes/categories.rs`):
- List: `SELECT * FROM categories ORDER BY name`
- Create: `INSERT INTO categories (name, description) VALUES (?, ?) RETURNING *`
- Update: `UPDATE categories SET name=?, description=? WHERE id=?`
- Delete: `DELETE FROM categories WHERE id=?`

**Customers** (`src/routes/customers.rs`):
- Supports search on `name` and `email`.
- Pagination identical to products.

Both require `Auth` extractor. The logic mirrors the products module.

---

## 15. Step 12: Orders & Order Items – Nested Responses & Transactions

This is the most complex part – replicating `orders_api.py`. The main challenges are:
- Nesting `OrderItem` data inside each order.
- Validating products and stock.
- Using a **database transaction** to atomically insert order + items and update stock.

### Helper: Building the Nested Response

We'll define a helper function that fetches an order, joins with customer and order_items, and assembles an `OrderResponse`.

```rust
// Inside orders.rs
async fn build_order_response(pool: &sqlx::SqlitePool, order_id: i32) -> Result<OrderResponse, AppError> {
    let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = ?")
        .bind(order_id)
        .fetch_optional(pool).await?
        .ok_or(AppError::NotFound)?;

    let customer_name: Option<String> = sqlx::query_scalar(
        "SELECT name FROM customers WHERE id = ?"
    ).bind(order.customer_id).fetch_optional(pool).await?.flatten();

    let items = sqlx::query_as::<_, (i32, i32, Option<String>, i32, f64)>(
        r#"
        SELECT oi.id, oi.product_id, p.name AS product_name, oi.quantity, oi.unit_price
        FROM order_items oi
        LEFT JOIN products p ON p.id = oi.product_id
        WHERE oi.order_id = ?
        "#
    ).bind(order_id).fetch_all(pool).await?;

    let order_items: Vec<OrderItemResponse> = items.into_iter().map(|(id, pid, pname, qty, up)| OrderItemResponse {
        id, product_id: pid, product_name: pname, quantity: qty, unit_price: up
    }).collect();

    Ok(OrderResponse { id: order.id, customer_id: order.customer_id, customer_name,
        order_date: order.order_date, total_amount: order.total_amount,
        status: order.status, items: order_items })
}
```

### Create Order with Transaction

The `create` handler does everything inside a transaction:

1. Begin transaction.
2. Verify customer exists.
3. For each requested item: check product exists, check stock, calculate total.
4. Insert `Order` row.
5. Insert each `OrderItem` row.
6. Decrement product stock.
7. Commit transaction.
8. Return the full order response.

```rust
async fn create(
    State(pool): State<AppState>,
    _auth: Auth,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), AppError> {
    let mut tx = pool.begin().await?;

    // Verify customer
    let _ = sqlx::query("SELECT id FROM customers WHERE id = ?")
        .bind(payload.customer_id)
        .fetch_optional(&mut *tx).await?
        .ok_or(AppError::NotFound)?;

    let mut total = 0.0;
    let mut items_data: Vec<(Product, i32)> = Vec::new();

    for item in &payload.items {
        let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = ?")
            .bind(item.product_id)
            .fetch_optional(&mut *tx).await?
            .ok_or_else(|| AppError::BadRequest(format!("Product {} not found", item.product_id)))?;
        if product.stock < item.quantity {
            return Err(AppError::BadRequest(format!("Insufficient stock for '{}'", product.name)));
        }
        total += product.price * item.quantity as f64;
        items_data.push((product, item.quantity));
    }

    let order = sqlx::query_as::<_, Order>(
        "INSERT INTO orders (customer_id, order_date, total_amount, status)
         VALUES (?, datetime('now'), ?, 'pending') RETURNING *"
    ).bind(payload.customer_id).bind(total).fetch_one(&mut *tx).await?;

    for (product, qty) in items_data {
        sqlx::query("INSERT INTO order_items (order_id, product_id, quantity, unit_price) VALUES (?, ?, ?, ?)")
            .bind(order.id).bind(product.id).bind(qty).bind(product.price)
            .execute(&mut *tx).await?;
        sqlx::query("UPDATE products SET stock = stock - ? WHERE id = ?")
            .bind(qty).bind(product.id)
            .execute(&mut *tx).await?;
    }

    tx.commit().await?;

    let response = build_order_response(&pool, order.id).await?;
    Ok((StatusCode::CREATED, Json(response)))
}
```

### List and Update Status

- `list`: pagination with count, returns an object with `total`, `page`, `per_page`, `orders`.
- `get_one`: returns a single `OrderResponse`.
- `update_status`: validates status string, updates, then returns the full response.

These are straightforward and follow the FastAPI logic exactly.

> **Python comparison:** FastAPI's `create_order` performs the same steps using `db.add` and `db.flush`. In Rust we have explicit `tx.commit()`.

---

## 16. Step 13: Putting It All Together – The Main Router

Finally, we assemble everything in `src/main.rs`. This file:

- Creates the database pool.
- Runs migrations and seeding.
- Configures session, CORS, static file middleware.
- Merges all route modules.
- Binds the server.

```rust
mod auth;
mod db;
mod errors;
mod models;
mod routes;

use std::sync::Arc;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tower_cookies::CookieManagerLayer;

pub type AppState = Arc<sqlx::SqlitePool>;

#[tokio::main]
async fn main() {
    let pool = db::create_pool().await;
    db::run_migrations(&pool).await;
    db::seed_database(&pool).await;

    let state: AppState = Arc::new(pool);

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::hours(24)));

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse().unwrap(),
            "http://127.0.0.1:5173".parse().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(true);

    let app = Router::new()
        .merge(routes::auth_routes::router())
        .merge(routes::dashboard::router())
        .merge(routes::products::router())
        .merge(routes::categories::router())
        .merge(routes::customers::router())
        .merge(routes::orders::router())
        .nest_service("/static", ServeDir::new("app/static"))
        .layer(CookieManagerLayer::new())
        .layer(session_layer)
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("🚀 Server running on http://localhost:8000");
    axum::serve(listener, app).await.unwrap();
}
```

**Note:** We also need an error type `AppError` that implements `IntoResponse`. Place it in `src/errors.rs`:

```rust
use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub enum AppError {
    NotFound,
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found").into_response(),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Internal(e.to_string())
    }
}
```

---

## 17. Running the Server & Testing

1. Create the directory structure as described.
2. Place all the code into their respective files.
3. Ensure `app/static/` directory exists (can be empty).
4. Run:

```bash
cargo run
```

The first run will create `shop.db`, migrate tables, and seed the database. The server will listen on port 8000.

Test the API exactly like your FastAPI project – the endpoints are identical:

- `POST /api/auth/login` – admin/admin123
- `GET /api/dashboard`
- `GET /api/products?search=&page=1&per_page=10`
- `POST /api/products`
- etc.

**Frontend:** Your React app can be pointed to `http://localhost:8000` instead of `8000` (or adjust port as needed).

---

## 18. Summary of Migration

| FastAPI Component | Axum Equivalent |
|-------------------|-----------------|
| `FastAPI()` app | `axum::Router` |
| `SessionMiddleware` | `tower-sessions` with `MemoryStore` |
| `CORSMiddleware` | `tower-http::cors::CorsLayer` |
| `StaticFiles` | `tower_http::services::ServeDir` |
| `Depends(get_db)` | `State(pool): State<AppState>` |
| `APIRouter(prefix="/api/orders")` | `Router::new().route(...)` merged |
| Pydantic schemas | Structs with `serde::Deserialize/Serialize` |
| SQLAlchemy queries | `sqlx::query_as` with async pool |
| `request.session.get("admin_id")` | Custom `Auth` extractor |
| `db.add(...); db.flush()` | Explicit insert + `tx.commit()` |
| `seed.py` | `db::seed_database` function called at startup |

You now have a production-ready, fully async shop management API in Rust, matching every feature of the original FastAPI backend. Enjoy the speed and safety! 🦀

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

