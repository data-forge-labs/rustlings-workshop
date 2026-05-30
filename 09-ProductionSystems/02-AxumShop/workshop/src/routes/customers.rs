use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use crate::auth::Auth;
use crate::errors::AppError;
use crate::models::{Customer, CreateCustomerRequest, UpdateCustomerRequest};
use crate::AppState;

#[derive(Deserialize)]
pub struct CustomerQuery {
    pub search: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/customers", get(list).post(create))
        .route("/api/customers/:id", get(get_one).put(update).delete(delete_one))
}

async fn list(
    State(pool): State<AppState>,
    _auth: Auth,
    Query(params): Query<CustomerQuery>,
) -> Result<Json<Vec<Customer>>, AppError> {
    let search = params.search.unwrap_or_default();
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(10).min(100);
    let offset = (page - 1) * per_page;

    let customers = if search.is_empty() {
        sqlx::query_as::<_, Customer>("SELECT * FROM customers ORDER BY name LIMIT ? OFFSET ?")
            .bind(per_page as i32)
            .bind(offset as i32)
            .fetch_all(&*pool)
            .await?
    } else {
        sqlx::query_as::<_, Customer>(
            "SELECT * FROM customers WHERE name LIKE ? OR email LIKE ? ORDER BY name LIMIT ? OFFSET ?"
        )
        .bind(format!("%{}%", search))
        .bind(format!("%{}%", search))
        .bind(per_page as i32)
        .bind(offset as i32)
        .fetch_all(&*pool)
        .await?
    };
    Ok(Json(customers))
}

async fn get_one(
    Path(id): Path<i32>,
    State(pool): State<AppState>,
    _auth: Auth,
) -> Result<Json<Customer>, AppError> {
    let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = ?")
        .bind(id)
        .fetch_optional(&*pool)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(customer))
}

async fn create(
    State(pool): State<AppState>,
    _auth: Auth,
    Json(payload): Json<CreateCustomerRequest>,
) -> Result<(axum::http::StatusCode, Json<Customer>), AppError> {
    let customer = sqlx::query_as::<_, Customer>(
        "INSERT INTO customers (name, email, phone, created_at) VALUES (?, ?, ?, datetime('now')) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&payload.phone)
    .fetch_one(&*pool)
    .await?;
    Ok((axum::http::StatusCode::CREATED, Json(customer)))
}

async fn update(
    Path(id): Path<i32>,
    State(pool): State<AppState>,
    _auth: Auth,
    Json(payload): Json<UpdateCustomerRequest>,
) -> Result<Json<Customer>, AppError> {
    let existing = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = ?")
        .bind(id)
        .fetch_optional(&*pool)
        .await?
        .ok_or(AppError::NotFound)?;

    let updated = sqlx::query_as::<_, Customer>(
        "UPDATE customers SET name = ?, email = ?, phone = ? WHERE id = ? RETURNING *"
    )
    .bind(payload.name.unwrap_or(existing.name))
    .bind(payload.email.unwrap_or(existing.email))
    .bind(payload.phone.or(existing.phone))
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
    let rows = sqlx::query("DELETE FROM customers WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await?
        .rows_affected();
    if rows == 0 {
        Err(AppError::NotFound)
    } else {
        Ok((axum::http::StatusCode::OK, Json(serde_json::json!({"message": "Customer deleted"}))))
    }
}
