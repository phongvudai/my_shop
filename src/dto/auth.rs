use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterUserRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = "1", message = "password must not be blank"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginUserRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = "1"))]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize, Validate)]
pub struct RefreshTokenRequest {
    #[validate(length(min = "1"))]
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize, Validate)]
pub struct LogoutRequest {
    #[validate(length(min = "1"))]
    pub refresh_token: String,
}
