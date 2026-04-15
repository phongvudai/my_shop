use std::sync::Arc;

use chrono::Utc;
use humantime::parse_duration;
use r2d2::Pool;
use redis::{Client, Commands};

use crate::{
    dto::auth::{
        LoginResponse, LoginUserRequest, LogoutRequest, RefreshResponse, RefreshTokenRequest,
        RegisterUserRequest,
    },
    entities::user::User,
    handlers::error::AppError,
    helpers::{create_jwt, generate_token, hash_token, verify_jwt, verify_password},
    repositories::refresh_token::RefreshTokenRepo,
    repositories::user::UserRepo,
};

#[derive(Clone)]
pub struct AuthService {
    user_repo: Arc<dyn UserRepo>,
    refresh_token_repo: Arc<dyn RefreshTokenRepo>,
    pub jwt_secret: String,
    jwt_ttl: String,
    redis_pool: Pool<Client>,
    refresh_ttl: String,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepo>,
        jwt_secret: String,
        jwt_ttl: String,
        refresh_ttl: String,
        redis_pool: Pool<Client>,
        refresh_token_repo: Arc<dyn RefreshTokenRepo>,
    ) -> Self {
        Self {
            user_repo,
            refresh_token_repo,
            jwt_secret,
            jwt_ttl,
            redis_pool,
            refresh_ttl,
        }
    }

    pub async fn register(&self, input: RegisterUserRequest) -> Result<User, AppError> {
        let result: User = self.user_repo.create(input).await?;

        Ok(result)
    }

    pub async fn login(&self, input: LoginUserRequest) -> Result<LoginResponse, AppError> {
        let user: User = self.user_repo.find_by_email(&input.email).await?;

        if !verify_password(&input.password, &user.password)? {
            return Err(AppError::Unauthorized("Invalid password".to_string()));
        }

        let access_token = create_jwt(&user.id.to_string(), &self.jwt_secret, &self.jwt_ttl)?;

        let raw_refresh_token = generate_token()?;
        let hashed_refresh_token = hash_token(&raw_refresh_token)?;
        let duration = parse_duration(&self.refresh_ttl).unwrap();
        let expires_at = Utc::now() + chrono::Duration::from_std(duration).unwrap();

        self.refresh_token_repo
            .create(user.id, &hashed_refresh_token, expires_at)
            .await?;

        Ok(LoginResponse {
            access_token,
            refresh_token: raw_refresh_token,
        })
    }

    pub async fn refresh(&self, input: RefreshTokenRequest) -> Result<RefreshResponse, AppError> {
        let hashed = hash_token(&input.refresh_token)?;

        let stored = self
            .refresh_token_repo
            .find_by_token(&hashed)
            .await
            .map_err(|_| AppError::Unauthorized("Invalid refresh token".to_string()))?;

        tracing::debug!("{:?}", stored);

        if stored.expires_at < Utc::now() {
            self.refresh_token_repo.delete_by_token(&hashed).await?;
            return Err(AppError::Unauthorized("Refresh token expired".to_string()));
        }

        // Token rotation: delete old, issue new
        self.refresh_token_repo.delete_by_token(&hashed).await?;

        let access_token =
            create_jwt(&stored.user_id.to_string(), &self.jwt_secret, &self.jwt_ttl)?;

        let new_raw_refresh_token = generate_token()?;
        let new_hashed_refresh_token = hash_token(&new_raw_refresh_token)?;
        let duration = parse_duration(&self.refresh_ttl).unwrap();
        let expires_at = Utc::now() + chrono::Duration::from_std(duration).unwrap();

        self.refresh_token_repo
            .create(stored.user_id, &new_hashed_refresh_token, expires_at)
            .await?;

        Ok(RefreshResponse {
            access_token,
            refresh_token: new_raw_refresh_token,
        })
    }

    pub async fn logout(&self, access_token: &str, input: LogoutRequest) -> Result<(), AppError> {
        let claims = verify_jwt(access_token, &self.jwt_secret)
            .map_err(|e| AppError::Unauthorized(e.to_string()))?;

        // Blacklist the access token in Redis
        let now = Utc::now().timestamp() as usize;
        let ttl = claims.exp.saturating_sub(now);
        if ttl > 0 {
            let key = format!("blacklist:{}", claims.jti);
            let mut conn = self.redis_pool.get().unwrap();
            conn.set_ex::<_, _, ()>(key, 1, ttl as u64).unwrap();
        }

        // Delete the refresh token from DB
        let hashed = hash_token(&input.refresh_token)?;
        self.refresh_token_repo.delete_by_token(&hashed).await?;

        Ok(())
    }

    pub async fn logout_all(&self, access_token: &str, user_id: i64) -> Result<(), AppError> {
        let claims = verify_jwt(access_token, &self.jwt_secret)
            .map_err(|e| AppError::Unauthorized(e.to_string()))?;

        // Blacklist the current access token in Redis
        let now = Utc::now().timestamp() as usize;
        let ttl = claims.exp.saturating_sub(now);
        if ttl > 0 {
            let key = format!("blacklist:{}", claims.jti);
            let mut conn = self.redis_pool.get().unwrap();
            conn.set_ex::<_, _, ()>(key, 1, ttl as u64).unwrap();
        }

        // Delete all refresh tokens for this user
        self.refresh_token_repo.delete_by_user_id(user_id).await?;

        Ok(())
    }
}
