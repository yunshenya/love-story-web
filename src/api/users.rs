use crate::handler::users::users;
use crate::server::app::AppState;
use axum::Router;

pub fn create_user_router() -> Router<AppState> {
    Router::new()
    .route("/", axum::routing::get(users))
}

