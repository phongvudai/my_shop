use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Request, rejection::JsonRejection},
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use redis::Commands;
use serde::{Serialize, de::DeserializeOwned};
use validator::Validate;

use crate::{
    AppState,
    helpers::{Claims, verify_jwt},
};

use super::error::AppError;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        value
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        Ok(ValidatedJson(value))
    }
}

#[derive(Serialize)]
pub struct AuthUser {
    pub user_id: String,
    #[serde(skip)]
    pub token: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, AppError);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header: &str = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                AppError::Unauthorized("Missing Authorization header".to_string()),
            ))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((
                StatusCode::UNAUTHORIZED,
                AppError::Unauthorized("Invalid token format".to_string()),
            ));
        }

        let token: &str = &auth_header[7..];

        let claims: Claims = verify_jwt(token, &state.auth_svc.jwt_secret).map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                AppError::Unauthorized(e.to_string()),
            )
        })?;

        let blacklist_key: String = format!("blacklist:{}", claims.jti);
        let is_blacklisted: bool = state
            .redis_pool
            .get()
            .ok()
            .and_then(|mut conn| conn.exists::<_, bool>(&blacklist_key).ok())
            .unwrap_or(false);
        if is_blacklisted {
            return Err((
                StatusCode::UNAUTHORIZED,
                AppError::Unauthorized("Token has been revoked".to_string()),
            ));
        }

        Ok(AuthUser {
            user_id: claims.sub,
            token: token.to_string(),
        })
    }
}
