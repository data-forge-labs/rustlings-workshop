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
