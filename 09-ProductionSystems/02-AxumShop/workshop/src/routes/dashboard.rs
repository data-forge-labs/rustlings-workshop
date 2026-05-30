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
