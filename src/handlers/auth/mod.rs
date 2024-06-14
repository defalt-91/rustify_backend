use crate::domain::models::user::UserModel;
use crate::handlers::auth::register::register;
use crate::utils::middlewares::mw_ctx::AppState;
use axum::routing::post;
use axum::Router;
use login::auth_login;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod login;
mod register;

pub fn auth_router(state: AppState) -> Router {
    Router::new()
        .route("/login/access-token", post(auth_login))
        .route("/register", post(register))
        .with_state(state)
}

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

// #[derive(Debug, Deserialize)]
// pub struct SignInForm {
//     grant_type: String,
//     username: String,
//     password: String,
//     scope: String,
//     client_id: Option<String>,
//     client_secret: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TokenClaims {
//     pub sub: String,
//     pub iat: usize,
//     pub exp: usize,
// }

// #[derive(Debug, Deserialize)]
// pub struct QueryCode {
//     pub code: String,
//     pub state: String,
// }

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterUserRequest {
    // pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    id: Uuid,
    username: String,
}

impl UserResponse {
    pub fn from_db(user: UserModel) -> Self {
        Self {
            id: user.id,
            username: user.username,
        }
    }
}
