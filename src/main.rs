mod dto;
mod entities;
mod handlers;
mod helpers;
mod repositories;
mod routes;
mod services;

use std::{sync::Arc, time::Duration};

use crate::repositories::refresh_token::PgRefreshTokenRepo;
use crate::{repositories::user::PgUserRepo, routes::create_router, services::auth::AuthService};
use axum::Router;
use dotenv::dotenv;
use redis::Client;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    auth_svc: AuthService,
    redis_pool: r2d2::Pool<Client>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL is not present");

    let db_pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to db");

    let client: redis::Client = redis::Client::open("redis://localhost:6379").unwrap();
    let redis_pool: r2d2::Pool<Client> = r2d2::Pool::builder().build(client).unwrap();

    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET is not present");
    let jwt_ttl: String = std::env::var("JWT_TTL").expect("JWT_TTL is not present");
    let refresh_ttl: String =
        std::env::var("REFRESH_TTL").expect("REFRESH_TTL is not present");
    let user_repo: Arc<PgUserRepo> = Arc::new(PgUserRepo::new(db_pool.clone()));
    let refresh_token_repo: Arc<PgRefreshTokenRepo> = Arc::new(PgRefreshTokenRepo::new(db_pool));
    let auth_svc: AuthService = AuthService::new(
        user_repo,
        jwt_secret,
        jwt_ttl,
        refresh_ttl,
        redis_pool.clone(),
        refresh_token_repo,
    );

    let state: AppState = AppState {
        auth_svc,
        redis_pool,
    };

    let app: Router = create_router(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}
