use crate::handlers::auth::auth_router;
use crate::handlers::peers::{peer_router, peers_router};
use crate::utils::middlewares::mw_ctx::{mw_require_auth, AppState};
use axum::{middleware, Router};

mod auth;
mod peers;
pub use peers::PeersFilter;
// api router
pub fn api_router(state: AppState) -> Router {
    Router::new()
        .nest("/peer", peer_router(state.clone()))
        .nest("/peers", peers_router(state.clone()))
        .route_layer(middleware::from_fn(mw_require_auth))
        .nest("/auth", auth_router(state.clone()))
}
