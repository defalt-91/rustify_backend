// #![feature(duration_constructors)]

use std::time::Duration;
use axum::Router;
use deadpool_diesel::postgres::{Manager, Pool};
use jsonwebtoken::{DecodingKey, EncodingKey};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::core::{get_config, run_migrations};
use crate::errors::{internal_error, Result};
use crate::routes::v1_router;
use crate::utils::middlewares::mw_ctx::AppState;
use tower_http::trace::{HttpMakeClassifier, TraceLayer};
use tracing::info;
use tokio::net::TcpListener;
use tower_http::timeout::TimeoutLayer;
// Import modules
mod core;
pub mod domain;
mod errors;
mod handlers;
pub mod infra;
mod routes;
mod service;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_config().await;
    let manager = Manager::new(config.db_url(), deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();
    run_migrations(&pool).await;


    // Load secret and create secret key for JWT
    let key_enc = EncodingKey::from_secret(config.secret().as_bytes());
    let key_dec = DecodingKey::from_secret(config.secret().as_bytes());

    let state = AppState {
        pool,
        key_enc,
        key_dec,
    };

    let app = Router::new()
        .nest("/api", v1_router(state.clone(), config))
        .layer(TimeoutLayer::new(Duration::from_secs(config.timeout_secs())))
        .layer(create_trace_layer());
    init_tracing();
    let listener = TcpListener::bind(config.bind()).await.unwrap();
    // let socket_listener = UnixListener::bind("/tmp/axum.sock").unwrap();
    // let incoming = UnixListenerStream::new(socket_listener);

    info!("->> LISTENING on http://{}", config.bind());
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(internal_error)
        .unwrap();
    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_env("LOG_LEVEL").unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace,diesel_async_postgres=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn create_trace_layer() -> TraceLayer<HttpMakeClassifier> {
    TraceLayer::new_for_http()
        // .make_span_with(
        //     DefaultMakeSpan::new()
        //         .include_headers(true)
        // )
        // .on_request(
        //     DefaultOnRequest::new()
        //         .level(Level::INFO)
        // )
        // .on_response(
        //     DefaultOnResponse::new()
        //         .level(Level::INFO)
        //         .latency_unit(LatencyUnit::Micros)
        // )
}