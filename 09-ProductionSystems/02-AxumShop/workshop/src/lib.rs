pub mod auth;
pub mod db;
pub mod errors;
pub mod models;
pub mod routes;

use std::sync::Arc;

pub type AppState = Arc<sqlx::SqlitePool>;
