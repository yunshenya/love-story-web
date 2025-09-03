use axum::{routing::post, Router};
use crate::server::app::AppState;
use axum::{extract::State, response::Json, http::StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::delete;
use crate::dto::auth::*;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", delete(logout))
}


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

pub async fn logout() -> Response {
    Redirect::to("/").into_response()
}