use crate::handler::index::index;
use crate::server::app::AppState;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
mod index;

pub fn create_router() -> Router<AppState>{
    let service_dir = ServeDir::new("/static")
        .not_found_service(ServeFile::new("static/404.html"));
    
    Router::new()
        .route("/", axum::routing::get(index))
        .nest_service("/static", service_dir)
}