use axum::{extract::State, response::Json, http::StatusCode};
use crate::dto::auth::*;
use crate::server::app::AppState;

pub async fn register(
    State(auth_service): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    match auth_service.register(req).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    match app_state.login(req).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}