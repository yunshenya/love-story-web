use crate::api::auth::auth_routes;
use crate::api::users::create_user_router;
use crate::handler::index::index;
use crate::server::app::AppState;
use axum::routing::get_service;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

mod users;
mod auth;

pub fn create_router() -> Router<AppState> {

    let service_dir = ServeDir::new("static");

    Router::new()
        .route("/", axum::routing::get(index))
        .nest("/api", Router::new().nest("/user", create_user_router()))
        .nest("/api/auth", auth_routes())
        .route_service("/favicon.ico",  get_service(ServeFile::new("static/favicon.ico")))
        .nest_service("/static", service_dir)
        .fallback(get_service(ServeFile::new("templates/404.html")))
}