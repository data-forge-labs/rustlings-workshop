use axum::{
    extract::{Path, State},
    routing::{get, post, put},
    Json, Router,
};
use axum::http::StatusCode;
use crate::auth::Auth;
use crate::errors::AppError;
use crate::models::{
    CreateOrderRequest, Order, OrderItemResponse, OrderResponse, Product, UpdateOrderStatusRequest,
};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/orders", get(list).post(create))
        .route("/api/orders/:id", get(get_one))
        .route("/api/orders/:id/status", put(update_status))
}

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
        id, product_id: pid, product_name: pname, quantity: qty, unit_price: up,
    }).collect();

    Ok(OrderResponse {
        id: order.id,
        customer_id: order.customer_id,
        customer_name,
        order_date: order.order_date,
        total_amount: order.total_amount,
        status: order.status,
        items: order_items,
    })
}

async fn list(
    State(pool): State<AppState>,
    _auth: Auth,
) -> Result<Json<serde_json::Value>, AppError> {
    let orders = sqlx::query_as::<_, Order>("SELECT * FROM orders ORDER BY id DESC")
        .fetch_all(&*pool)
        .await?;

    let total = orders.len();
    Ok(Json(serde_json::json!({
        "total": total,
        "orders": orders,
    })))
}

async fn get_one(
    Path(id): Path<i32>,
    State(pool): State<AppState>,
    _auth: Auth,
) -> Result<Json<OrderResponse>, AppError> {
    let response = build_order_response(&pool, id).await?;
    Ok(Json(response))
}

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

async fn update_status(
    Path(id): Path<i32>,
    State(pool): State<AppState>,
    _auth: Auth,
    Json(payload): Json<UpdateOrderStatusRequest>,
) -> Result<Json<OrderResponse>, AppError> {
    let valid_statuses = ["pending", "shipped", "completed", "cancelled"];
    if !valid_statuses.contains(&payload.status.as_str()) {
        return Err(AppError::BadRequest(format!("Invalid status '{}'", payload.status)));
    }

    let _ = sqlx::query("UPDATE orders SET status = ? WHERE id = ?")
        .bind(&payload.status)
        .bind(id)
        .execute(&*pool)
        .await?;

    let response = build_order_response(&pool, id).await?;
    Ok(Json(response))
}
