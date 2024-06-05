use axum::Router;
use axum::routing::post;
use login::auth_login;
use crate::utils::middlewares::mw_ctx::AppState;

mod login;

pub fn auth_router(state: AppState) -> Router {
    Router::new()
        .route("/login/access-token", post(auth_login))
        .with_state(state)
}
