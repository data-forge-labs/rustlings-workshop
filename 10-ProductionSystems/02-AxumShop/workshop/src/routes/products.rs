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
