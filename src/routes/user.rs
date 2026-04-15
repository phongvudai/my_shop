use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/register", get({}))
}
