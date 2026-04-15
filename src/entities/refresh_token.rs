use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct RefreshToken {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}
