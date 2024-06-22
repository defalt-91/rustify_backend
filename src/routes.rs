use std::time::Duration;
use axum::{middleware, Router, routing::get, response::IntoResponse, http::StatusCode};
use tower_cookies::CookieManagerLayer;
use tower_http::{timeout::TimeoutLayer, cors::CorsLayer};
use crate::core::Config;
use crate::handlers::api_router;
use crate::utils::middlewares::{mw_req_logger::mw_req_logger, mw_ctx::{mw_ctx_constructor, AppState}};

pub fn v1_router(state: AppState, config: &'static Config) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/v1", api_router(state.clone()))
        .layer(middleware::map_response(mw_req_logger))
        // This is where Ctx gets created, with every new request
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_ctx_constructor,
        ))
        // Layers are executed from bottom up, so CookieManager has to be under ctx_constructor
        .layer(CookieManagerLayer::new())
        .layer(create_cors(config))
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .fallback(handler_404)
    // .fallback_service(routes_static())
}

// fn routes_static() -> Router {
//     Router::new().nest_service("/", get_service(ServeDir::new("./")))
// }
async fn root() -> &'static str {
    "Server is running!"
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

fn create_cors(config: &Config) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(config.allow_origin())
        .allow_methods(config.allow_methods())
        .allow_credentials(true)
        .allow_headers(config.allow_headers())
}