use axum::extract::State;
use axum::Json;

use crate::core::hash_password;
use crate::domain::models::user::UserError;
use crate::handlers::auth::{RegisterUserRequest, UserResponse};
use crate::infra::user_repository;
use crate::infra::user_repository::read_by_username;
use crate::utils::middlewares::mw_ctx::AppState;
use crate::utils::JsonExtractor;
use tracing::debug;

pub async fn register(
    State(AppState { pool, .. }): State<AppState>,
    JsonExtractor(payload): JsonExtractor<RegisterUserRequest>,
) -> Result<Json<UserResponse>, UserError> {
    debug!("{:?}", payload.clone());
    if let Some(_value) = read_by_username(&pool, payload.username.clone()).await.ok() {
        return Err(UserError::UsernameExists);
    }
    let new_user = user_repository::NewUserDb {
        username: payload.username,
        hashed_password: hash_password(&payload.password).unwrap().to_string(),
    };
    let db_user = user_repository::create(&pool, new_user)
        .await
        .map_err(UserError::InfraError)?;
    let user_response = UserResponse::from_db(db_user);
    Ok(Json(user_response))
}
