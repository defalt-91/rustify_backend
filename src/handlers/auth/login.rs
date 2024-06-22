use std::ops::Add;
use crate::core::{create_token, get_config, verify_password};
use crate::domain::ctx::Ctx;
use crate::domain::models::user::UserError;
use crate::errors::{ApiError, BaseError};
use crate::infra::errors::InfraError;
use crate::infra::user_repository::read_by_username;
use crate::utils::middlewares::mw_ctx::AppState;
use axum::{extract::State, Json};
use axum_extra::extract::Form;
use serde::{Deserialize, Serialize};
use tower_cookies::{
    cookie::{time::{Duration, OffsetDateTime}, SameSite,Expiration},
    Cookie, Cookies,
};
use tracing::debug;

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
    // grant_type: String,
    username: String,
    password: String,
    // scope: String,
    // client_id: Option<String>,
    // client_secret: Option<String>,
}

pub async fn auth_login(
    State(AppState { key_enc, pool, .. }): State<AppState>,
    cookies: Cookies,
    ctx: Ctx,
    payload: Form<SignInForm>,
) -> Result<Json<SingInSuccess>, ApiError> {
    debug!("->> {:<12} - api_login", "HANDLER");
    let db_user = read_by_username(&pool, payload.username.clone())
        .await
        .map_err(|db_error| match db_error {
            // Map infrastructure errors to custom PeerError types
            InfraError::InternalServerError => UserError::InternalServerError,
            // InfraError::SerializationError => UserError::InternalServerError,
            InfraError::NotFound => UserError::UsernameNotFound(payload.username.clone()),
        })
        .map_err(|_| ApiError {
            req_id: ctx.req_id(),
            error: BaseError::UserNotFound,
        })?;
    if !verify_password(ctx.req_id(), &payload.password, &db_user.hashed_password)? {
        return Err(ApiError {
            error: BaseError::from(UserError::WrongCredentials),
            req_id: ctx.req_id(),
        });
    };
    let token_str = create_token(db_user.id,db_user.username, key_enc).await;
    let config = get_config().await;
    let duration = Duration::minutes(config.jwt_exp_minutes());
    let cookie_exp = Expiration::from(OffsetDateTime::now_utc().checked_add(duration));
    cookies.add(
        Cookie::build(Cookie::new(config.jwt_key(), token_str.clone()))
            // if not set, the path defaults to the path from which it was called - prohibiting gql on root if login is on /api
            .path("/")
            // .domain("127.0.0.1")
            .same_site(SameSite::Strict)
            .expires(cookie_exp)
                // OffsetDateTime::now_utc().checked_add(Duration::hours(2)))
            .http_only(false)
            .secure(false)
            .build(),
    );

    Ok(Json(SingInSuccess::create_bearer(token_str)))
}
