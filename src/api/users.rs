use crate::server::app::AppState;
use axum::response::IntoResponse;
use axum::{debug_handler, Router};
use axum::extract::State;
use sea_orm::EntityTrait;
use crate::entities::prelude::*;

pub fn create_user_router() -> Router<AppState> {
    Router::new()
    .route("/", axum::routing::get(users))
}


#[debug_handler]
async fn users(State(AppState { db }): State<AppState>) -> impl IntoResponse {
    let users = Users::find().all(&db).await.unwrap();
    axum::Json(users)
}