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
