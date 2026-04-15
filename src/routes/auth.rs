use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    handlers::refresh,
    handlers::{login, logout, logout_all, me, register},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/logout-all", post(logout_all))
        .route("/refresh", post(refresh))
        .route("/me", get(me))
}
