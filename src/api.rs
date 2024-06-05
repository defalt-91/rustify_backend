use axum::{middleware, Router};
use crate::{Pool, mw_ctx};
use crate::mw_ctx::CtxState;

mod routes_login;
mod routes_peer;


pub fn api_routes(ctx_state:CtxState, db:Pool) -> Router {
    Router::new()
        .nest("/peers", routes_peer::peers_routes(db.clone()))
        .route_layer(middleware::from_fn(mw_ctx::mw_require_auth))
        .nest("/auth", routes_login::routes(ctx_state.clone()))
}
