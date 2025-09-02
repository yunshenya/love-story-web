use crate::dto::auth::SearchMail;
use crate::entities::users;
use crate::server::app::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;


pub async fn users(State(app_state): State<AppState>, Json(mail): Json<SearchMail>) -> Result<Json<Option<users::Model>>, StatusCode>{
    match app_state.get_user(mail).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::UNAUTHORIZED)
    }
}