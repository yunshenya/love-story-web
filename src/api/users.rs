use crate::server::app::AppState;
use axum::Router;
use crate::dto::auth::SearchMail;
use crate::entities::users;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;


pub fn create_user_router() -> Router<AppState> {
    Router::new()
    .route("/", axum::routing::get(users))
}



pub async fn users(State(app_state): State<AppState>, Json(mail): Json<SearchMail>) -> Result<Json<Option<users::Model>>, StatusCode>{
    match app_state.get_user(mail).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::UNAUTHORIZED)
    }
}