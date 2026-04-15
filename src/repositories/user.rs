use sqlx::{Pool, Postgres};

use crate::{
    dto::auth::RegisterUserRequest, entities::user::User, handlers::error::AppError,
    helpers::password::hash_password,
};

use async_trait::async_trait;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn find(&self) -> Result<Vec<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<User, AppError>;
    async fn create(&self, input: RegisterUserRequest) -> Result<User, AppError>;
    async fn update(&self);
    async fn delete(&self);
}

pub struct PgUserRepo {
    db: Pool<Postgres>,
}

impl PgUserRepo {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepo for PgUserRepo {
    async fn find(&self) -> Result<Vec<User>, AppError> {
        let result: Vec<User> = sqlx::query_as("SELECT * from users;")
            .fetch_all(&self.db)
            .await?;

        Ok(result)
    }

    async fn find_by_email(&self, email: &str) -> Result<User, AppError> {
        let result: User = sqlx::query_as("SELECT * from users WHERE email = $1;")
            .bind(email)
            .fetch_one(&self.db)
            .await?;

        Ok(result)
    }

    async fn create(&self, input: RegisterUserRequest) -> Result<User, AppError> {
        let password_hash: String = hash_password(&input.password)?;

        let user: User = sqlx::query_as(
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id, email, password",
        )
        .bind(input.email)
        .bind(password_hash)
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }

    async fn update(&self) {
        todo!()
    }

    async fn delete(&self) {
        todo!()
    }
}
