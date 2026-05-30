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

// ---------- Request / Response DTOs ----------

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub category_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
    pub category_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateCustomerRequest {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateCustomerRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub customer_id: i32,
    pub items: Vec<OrderItemRequest>,
}

#[derive(Deserialize)]
pub struct OrderItemRequest {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Serialize)]
pub struct OrderItemResponse {
    pub id: i32,
    pub product_id: i32,
    pub product_name: Option<String>,
    pub quantity: i32,
    pub unit_price: f64,
}

#[derive(Serialize)]
pub struct OrderResponse {
    pub id: i32,
    pub customer_id: i32,
    pub customer_name: Option<String>,
    pub order_date: Option<String>,
    pub total_amount: f64,
    pub status: String,
    pub items: Vec<OrderItemResponse>,
}

#[derive(Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: String,
}
