use serde::{Deserialize, Serialize};

use validator::Validate;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "1"))]
    pub password: String,
}
