use axum::{
    middleware,
    Router,
    routing::get_service,
};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use error::{ApiResult, Result};
use mw_ctx::CtxState;
use mw_req_logger::mw_req_logger;

use crate::core::config;
use crate::api::api_routes;

mod error;
mod mw_ctx;
mod mw_req_logger;
mod service;
mod api;
mod core;
mod schema;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;


#[tokio::main]
async fn main() -> Result<()> {
    let env = dotenv::from_path(config::base_dir().join(".env"));
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
    info!("-->{:?}", config::base_dir().join(".env"));
    // db
    // NOTE: For connection to an existing db
    let db_url = config::pg_url();
        // set up connection pool
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    // debug!("->> db version: {pool}");
    // Select a specific namespace / database
    // Load secret and create secret key for JWT
    let key_enc = EncodingKey::from_secret(config::secret().as_bytes());
    let key_dec = DecodingKey::from_secret(config::secret().as_bytes());
    let ctx_state = CtxState {
        _db: pool.clone(),
        key_enc,
        key_dec,
    };

    // Main router
    let routes_all = Router::new()
        .nest(
            "/api/v1",
            api_routes(ctx_state.clone(),pool.clone())
        )
        .layer(middleware::map_response(mw_req_logger))
        // This is where Ctx gets created, with every new request
        .layer(middleware::from_fn_with_state(
            ctx_state.clone(),
            mw_ctx::mw_ctx_constructor,
        ))
        // Layers are executed from bottom up, so CookieManager has to be under ctx_constructor
        .layer(CookieManagerLayer::new())
        .layer(
            CorsLayer::new()
                .allow_origin(config::allow_origin())
                .allow_methods(config::allow_methods())
                .allow_credentials(true)
                .allow_headers(config::allow_headers())
        )
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind(config::bind()).await.unwrap();
    debug!("->> LISTENING on {:?}", config::bind());
    axum::serve(
        listener,
        routes_all.into_make_service(),
    ).await.unwrap();

    // fallback fs
    fn routes_static() -> Router {
        Router::new().nest_service("/", get_service(ServeDir::new("./")))
    }

    Ok(())
}
