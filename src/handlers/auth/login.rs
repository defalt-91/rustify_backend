use axum::{extract::State, Json};
use axum_extra::extract::Form;
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies, cookie::{SameSite, time::{OffsetDateTime, ext::NumericalDuration}}};
use tracing::debug;

use crate::errors::{ApiError, BaseError, ApiResult};
use crate::core::{get_config,create_token};
use crate::domain::ctx::Ctx;
use crate::utils::middlewares::mw_ctx::AppState;

#[derive(Debug, Serialize)]
pub struct SingInSuccess {
    token_type: String,
    access_token: String,
}

impl SingInSuccess {
    pub fn create_bearer(token_str: String) -> Self {
        Self {
            access_token: token_str,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SignInForm {
    grant_type: String,
    username: String,
    password: String,
    scope: String,
    client_id: Option<String>,
    client_secret: Option<String>,
}

pub async fn auth_login(
    State(AppState { key_enc, .. }): State<AppState>,
    cookies: Cookies,
    ctx: Ctx,
    payload: Form<SignInForm>,
) -> ApiResult<Json<SingInSuccess>> {
    debug!("->> {:<12} - api_login", "HANDLER");
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
            error: BaseError::LoginFail,
            req_id: ctx.req_id(),
        });
    };

    let token_str = create_token(mock_user.username, key_enc).await;
    cookies.add(
        Cookie::build((get_config().await.jwt_key(), token_str.clone()))
            // if not set, the path defaults to the path from which it was called - prohibiting gql on root if login is on /api
            .path("/")
            // .domain("127.0.0.1")
            .same_site(SameSite::Strict)
            .expires(OffsetDateTime::now_utc().checked_add(7.minutes()))
            .http_only(false)
            .secure(false)
            .build(),
    );

    Ok(Json(SingInSuccess::create_bearer(token_str)))
}
