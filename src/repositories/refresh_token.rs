use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

use crate::{entities::refresh_token::RefreshToken, handlers::error::AppError};

#[async_trait]
pub trait RefreshTokenRepo: Send + Sync {
    async fn create(
        &self,
        user_id: i64,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<RefreshToken, AppError>;
    async fn find_by_token(&self, token: &str) -> Result<RefreshToken, AppError>;
    async fn delete_by_token(&self, token: &str) -> Result<(), AppError>;
    async fn delete_by_user_id(&self, user_id: i64) -> Result<(), AppError>;
}

pub struct PgRefreshTokenRepo {
    db: Pool<Postgres>,
}

impl PgRefreshTokenRepo {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RefreshTokenRepo for PgRefreshTokenRepo {
    async fn create(
        &self,
        user_id: i64,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<RefreshToken, AppError> {
        let result = sqlx::query_as(
            "INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES ($1, $2, $3) \
             RETURNING id, user_id, token, expires_at",
        )
        .bind(user_id)
        .bind(token)
        .bind(expires_at)
        .fetch_one(&self.db)
        .await?;

        Ok(result)
    }

    async fn find_by_token(&self, token: &str) -> Result<RefreshToken, AppError> {
        tracing::debug!("{:?}", token);
        let result = sqlx::query_as(
            "SELECT id, user_id, token, expires_at FROM refresh_tokens WHERE token = $1",
        )
        .bind(token)
        .fetch_one(&self.db)
        .await?;

        Ok(result)
    }

    async fn delete_by_token(&self, token: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM refresh_tokens WHERE token = $1")
            .bind(token)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: i64) -> Result<(), AppError> {
        sqlx::query("DELETE FROM refresh_tokens WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
