use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    #[serde(rename= "confirmPassword")]
    pub confirm_password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user_id: Uuid,
    pub email: String,
    pub name: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchMail{
    pub mail: String
}