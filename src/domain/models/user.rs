use crate::infra::errors::InfraError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{NaiveDateTime, Utc};
use serde_json::json;
use std::fmt;
use std::fmt::Formatter;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub enum UserError {
    InternalServerError,
    NotFound(Uuid),
    InfraError(InfraError),
    UsernameNotFound(String),
    WrongCredentials,
    UsernameExists,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            UserError::InternalServerError => write!(f, "InternalServerError"),
            UserError::NotFound(uuid) => write!(f, "user with userid : {uuid} not found"),
            UserError::InfraError(_err) => write!(f, "infra error"),
            UserError::UsernameNotFound(_err) => write!(f, "username doesn't exist"),
            UserError::WrongCredentials => write!(f, "Username or password is wrong"),
            UserError::UsernameExists => write!(f, "username is not available"),
        }
    }
}

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("User with id {} has not been found", id),
            ),
            Self::InfraError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            Self::UsernameNotFound(username) => (
                StatusCode::UNAUTHORIZED,
                format!("User with username {} has not been found", username),
            ),
            Self::WrongCredentials => (
                StatusCode::UNAUTHORIZED,
                "Username or password is wrong!".to_string(),
            ),
            Self::UsernameExists => (
                StatusCode::CONFLICT,
                "this username is not available".to_string(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };
        (
            status,
            Json(
                json!({"resource":"UserModel", "message": err_msg, "happened_at" :  Utc::now().timestamp() }),
            ),
        )
            .into_response()
    }
}
