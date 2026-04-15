use axum::{Json, extract::State, http::StatusCode};
use serde_json::{Value, json};

use crate::{
    AppState,
    dto::auth::{
        LoginResponse, LoginUserRequest, LogoutRequest, RefreshResponse, RefreshTokenRequest,
        RegisterUserRequest,
    },
    entities::user::User,
    handlers::{
        error::AppError,
        extractor::{AuthUser, ValidatedJson},
    },
};

pub async fn register(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<RegisterUserRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let user: User = state.auth_svc.register(payload).await?;

    Ok((StatusCode::CREATED, Json(json!(user))))
}

pub async fn login(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<LoginUserRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let result: LoginResponse = state.auth_svc.login(payload).await?;

    Ok((StatusCode::OK, Json(json!(result))))
}

pub async fn refresh(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<RefreshTokenRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let result: RefreshResponse = state.auth_svc.refresh(payload).await?;

    Ok((StatusCode::OK, Json(json!(result))))
}

pub async fn logout(
    State(state): State<AppState>,
    user: AuthUser,
    ValidatedJson(payload): ValidatedJson<LogoutRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    state.auth_svc.logout(&user.token, payload).await?;

    Ok((StatusCode::OK, Json(json!({ "message": "logged out" }))))
}

pub async fn logout_all(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let user_id: i64 = user.user_id.parse().map_err(|_| {
        AppError::Unauthorized("Invalid user id in token".to_string())
    })?;
    state.auth_svc.logout_all(&user.token, user_id).await?;

    Ok((StatusCode::OK, Json(json!({ "message": "logged out from all devices" }))))
}

pub async fn me(user: AuthUser) -> Result<(StatusCode, Json<Value>), AppError> {
    Ok((StatusCode::OK, Json(json!(user))))
}
