use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
use crate::api::auth::auth_routes;
use crate::api::users::create_user_router;
use crate::handler::index::index;
use crate::server::app::AppState;

mod users;
mod auth;

pub fn create_router() -> Router<AppState> {
    // 使用相对路径而不是绝对路径
    let service_dir = ServeDir::new("static");

    Router::new()
        .route("/", axum::routing::get(index))
        .nest("/api", Router::new().nest("/user", create_user_router()))
        .nest("/api/auth", auth_routes())
        .nest_service("/static", service_dir)
        .fallback(axum::routing::get_service(ServeFile::new("static/404.html")))
}