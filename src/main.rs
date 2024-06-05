use axum::{Router, routing::get_service};
use deadpool_diesel::postgres::{Manager, Pool};
use jsonwebtoken::{DecodingKey, EncodingKey};
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::core::{get_config, run_migrations};
use crate::errors::{internal_error, Result};
use crate::routes::v1_router;
use crate::utils::middlewares::mw_ctx::AppState;

// Import modules
mod errors;
mod service;
mod core;
pub mod domain;
pub mod infra;
mod handlers;
mod utils;
mod routes;


#[tokio::main]
async fn main() -> Result<()> {
    // Create a connection pool to the PostgresSQL database
    let config = get_config().await;
    let manager = Manager::new(
        config.db_url(),
        deadpool_diesel::Runtime::Tokio1,
    );
    let pool = Pool::builder(manager).build().unwrap();
    run_migrations(&pool).await;
    init_tracing();

    // Load secret and create secret key for JWT
    let key_enc = EncodingKey::from_secret(config.secret().as_bytes());
    let key_dec = DecodingKey::from_secret(config.secret().as_bytes());

    // Create an instance of the application state
    let state = AppState { pool, key_enc, key_dec };

    let app = Router::new().nest(
        "/api", v1_router(state.clone(), config),
    );
    let listener = tokio::net::TcpListener::bind(config.bind()).await.unwrap();
    debug!("->> LISTENING on http://{}", config.bind());
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(internal_error).unwrap();
    Ok(())
}


// Function to initialize tracing for logging
fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_env("LOG_LEVEL").unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace,example_diesel_async_postgres=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
