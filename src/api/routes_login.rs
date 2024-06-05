use axum::{extract::State, Form, Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};
use tower_cookies::cookie::SameSite;
use tower_cookies::cookie::time::ext::NumericalDuration;
use tower_cookies::cookie::time::OffsetDateTime;

use crate::{ApiResult, schema::ctx::Ctx, error::ApiError, error::Error};
use crate::core::config;
use crate::core::security::create_token;
use crate::mw_ctx::CtxState;
use crate::schema::user::{LoginInput, LoginSuccess};

pub fn routes(state: CtxState) -> Router {
    Router::new()
        .route("/login/access-token", post(api_login))
        .with_state(state)
}



async fn api_login(
    State(CtxState { _db, key_enc, .. }): State<CtxState>,
    cookies: Cookies,
    ctx: Ctx,
    payload: Form<LoginInput>,
) -> ApiResult<Json<LoginSuccess>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // NOTE: DB should be checked here
    // Mock user
    struct User {
        username: String,
        password: String,
    }
    let mock_user = User {
        username: "defalt".to_string(),
        password: "6367411".to_string(),
    };

    if payload.username != mock_user.username || payload.password != mock_user.password {
        return Err(ApiError {
            error: Error::LoginFail,
            req_id: ctx.req_id(),
        });
    };

    let token_str = create_token(mock_user.username, key_enc);
    cookies.add(
        Cookie::build((config::jwt_key(), token_str.clone()))
            // if not set, the path defaults to the path from which it was called - prohibiting gql on root if login is on /api
            .path("/")
            // .domain("127.0.0.1")
            .same_site(SameSite::Strict)
            .expires(OffsetDateTime::now_utc().checked_add(7.minutes()))
            .http_only(false)
            .secure(false)
            .build(),
    );

    Ok(Json(LoginSuccess::create_bearer(token_str)))
}
