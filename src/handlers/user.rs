use axum::{Json, extract::State, http::StatusCode};
use serde_json::{Value, json};
use validator::Validate;

use crate::{
    AppState,
    entities::user::{CreateUserRequest, User},
};

// pub async fn user_list(State(state): State<AppState>) -> Result<Json<Value>, (StatusCode, String)> {
//     let users: Vec<User> = state
//         .user_repo
//         .find()
//         .await
//         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

//     Ok(Json(json!(users)))
// }

// pub async fn create_user(
//     State(state): State<AppState>,
//     Json(payload): Json<CreateUserRequest>,
// ) -> Result<Json<Value>, (StatusCode, String)> {
//     payload.validate().map_err(|error| error.to_string());

//     let user = state
//         .user_repo
//         .create(payload)
//         .await
//         .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
//     Ok(Json(json!(user)))
// }
