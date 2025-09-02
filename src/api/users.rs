use crate::entities::users;
use crate::server::app::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{debug_handler, Json, Router};

pub fn create_user_router() -> Router<AppState> {
    Router::new()
    .route("/", axum::routing::get(users))
}


#[debug_handler]
async fn users(State(app): State<AppState>) -> Result<Json<Option<users::Model>>, StatusCode>{
    match app.get_user("test@gmail.com".to_string()).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::UNAUTHORIZED)
    }
}