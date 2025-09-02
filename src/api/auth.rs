use axum::{routing::post, Router};
use crate::handler::auth::{register, login};
use crate::server::app::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}