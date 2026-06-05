use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use crate::auth::Auth;
use crate::errors::AppError;
use crate::models::{Category, CreateCategoryRequest, UpdateCategoryRequest};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/categories", get(list).post(create))
        .route("/api/categories/:id", get(get_one).put(update).delete(delete_one))
}

async fn list(
    State(pool): State<AppState>,
    _auth: Auth,
) -> Result<Json<Vec<Category>>, AppError> {
    let categories = sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY name")
        .fetch_all(&*pool)
        .await?;
    Ok(Json(categories))
}

async fn get_one(
    Path(id): Path<i32>,
    State(pool): State<AppState>,
    _auth: Auth,
) -> Result<Json<Category>, AppError> {
    let category = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = ?")
        .bind(id)
        .fetch_optional(&*pool)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(category))
}

async fn create(
    State(pool): State<AppState>,
    _auth: Auth,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<(axum::http::StatusCode, Json<Category>), AppError> {
    let category = sqlx::query_as::<_, Category>(
        "INSERT INTO categories (name, description) VALUES (?, ?) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .fetch_one(&*pool)
    .await?;
    Ok((axum::http::StatusCode::CREATED, Json(category)))
}

async fn update(
    Path(id): Path<i32>,
    State(pool): State<AppState>,
    _auth: Auth,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<Json<Category>, AppError> {
    let existing = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = ?")
        .bind(id)
        .fetch_optional(&*pool)
        .await?
        .ok_or(AppError::NotFound)?;

    let updated = sqlx::query_as::<_, Category>(
        "UPDATE categories SET name = ?, description = ? WHERE id = ? RETURNING *"
    )
    .bind(payload.name.unwrap_or(existing.name))
    .bind(payload.description.or(existing.description))
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
    let rows = sqlx::query("DELETE FROM categories WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await?
        .rows_affected();
    if rows == 0 {
        Err(AppError::NotFound)
    } else {
        Ok((axum::http::StatusCode::OK, Json(serde_json::json!({"message": "Category deleted"}))))
    }
}
