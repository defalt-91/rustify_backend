use std::fmt;

use axum::{
    http::StatusCode,
    http::{header, HeaderMap, HeaderValue},
    response::{IntoResponse, Response}, Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::domain::ctx::Ctx;
use crate::domain::models::peer::PeerError;
use crate::domain::models::user::UserError;
use crate::infra::errors::InfraError;

#[derive(Debug, PartialEq, Eq)]
pub struct ApiError {
    pub error: BaseError,
    pub req_id: Uuid,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BaseError {
    Generic { description: String },
    LoginFail,
    AuthFailNoJwtCookie,
    AuthFailJwtInvalid { source: String },
    AuthFailCtxNotInRequestExt,
    Serde { source: String },
    // SurrealDb { source: String },
    // SurrealDbNoResult { source: String, id: String },
    // SurrealDbParse { source: String, id: String },
    Execution { source: String },
    NotAuthenticated { source: String },
    UserNotFound,
    UserAgentMissing,
    // UserAgentMissing { source: String },
    // AuthorizationHeaderMissing { source: String },
    // AuthorizationHeaderEmpty { source: String },
    // AuthorizationHeaderFormatWrong { source: String },
    // WrongToken { source: String },
    // UnAuthorizedUser { source: String },
    // EmptyHeaderValue { source: String },
    InternalServerError, // Represents an internal server error
    BodyParsingError(String),
    PeerDumpError(&'static str),
    UserError { source: String },
    PeerError { source: String },
}

/// ApiError has to have the req_id to report to the client and implements IntoResponse.
pub type ApiResult<T> = core::result::Result<T, ApiError>;
/// Any error for storing before composing a response.
/// For errors that either don't affect the response, or are build before attaching the req_id.
pub type Result<T> = core::result::Result<T, BaseError>;

impl std::error::Error for BaseError {}
// We don't implement Error for ApiError, because it doesn't implement Display.
// Implementing Display for it triggers a generic impl From ApiError- and we want to implement it ourselves, to always include extensions on Errors. It would create conflicting implementations.

// for slightly less verbose error mappings
impl ApiError {
    pub fn from<T: Into<BaseError>>(ctx: &Ctx) -> impl FnOnce(T) -> ApiError + '_ {
        |err| ApiError {
            req_id: ctx.req_id(),
            error: err.into(),
        }
    }
}

impl IntoResponse for BaseError {
    // Define the conversion to an Axum response
    fn into_response(self) -> Response {
        // Define status and error message based on the error variant
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".into(),
            ),
            Self::BodyParsingError(message) => (
                StatusCode::BAD_REQUEST,
                format!("Bad request error: {}", message),
            ),
            Self::UserError { source } => (StatusCode::UNAUTHORIZED, source),
            Self::AuthFailNoJwtCookie=>(StatusCode::UNAUTHORIZED,"You are not logged in".to_string()),
            _ => (StatusCode::BAD_REQUEST, "--> check here".to_string()),
        };

        // Create a JSON response containing the error message
        (status, Json(json!({ "message": err_msg }))).into_response()
    }
}

const INTERNAL: &str = "Internal error";

impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic { description } => write!(f, "{description}"),
            Self::LoginFail => write!(f, "Login fail"),
            Self::AuthFailNoJwtCookie => write!(f, "You are not logged in"),
            Self::AuthFailJwtInvalid { .. } => {
                write!(f, "The provided JWT token is not valid")
            }
            Self::Serde { source } => write!(f, "Serde error - {source}"),
            Self::AuthFailCtxNotInRequestExt => write!(f, "{INTERNAL}"),
            // Self::SurrealDb { .. } => write!(f, "{INTERNAL}"),
            // Self::SurrealDbNoResult { id, .. } => write!(f, "No result for id {id}"),
            // Self::SurrealDbParse { id, .. } => write!(f, "Couldn't parse id {id}"),
            Self::Execution { .. } => write!(f, "Couldn't execute "),
            Self::UserNotFound => write!(f, "No user with provided credentials"),
            Self::NotAuthenticated { .. } => write!(f, "You are not authorized"),
            // Self::UserAgentMissing { .. } => write!(f, "User-Agent header is missing"),
            // Self::AuthorizationHeaderMissing { .. } => write!(f, "Authorization header is missing"),
            // Self::AuthorizationHeaderFormatWrong { .. } => {
            //     write!(f, "Authorization header format is wrong")
            // }
            // Self::AuthorizationHeaderEmpty { .. } => write!(f, "Empty header is not allowed"),
            // Self::WrongToken { .. } => write!(f, "Unable to decode token"),
            // Self::UnAuthorizedUser { .. } => write!(f, "You are not an authorized user"),
            // Self::EmptyHeaderValue { .. } => write!(f, "Please add the JWT token to the header"),
            Self::UserAgentMissing => write!(f,"User-Agent header is missing"),
            Self::InternalServerError => write!(f, "Internal Server Error"),
            Self::BodyParsingError(msg) => write!(f, "{msg}"),
            Self::PeerDumpError(msg) => write!(f, "{msg}"),
            Self::UserError { source } => write!(f, "Auth Model - {source}"),
            Self::PeerError { source } => write!(f, "Peer Model - {source}")
        }
    }
}

// REST error response
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - into_response - {self:?}", "ERROR");
        let status_code = match self.error {
            BaseError::Serde { .. }
            // | BaseError::SurrealDbNoResult { .. }
            // | BaseError::UserAgentMissing { .. }
            | BaseError::BodyParsingError(..)
            // | BaseError::SurrealDbParse { .. },
            | BaseError::PeerError { .. } => StatusCode::BAD_REQUEST,
            BaseError::Generic { .. }
            | BaseError::LoginFail
            | BaseError::AuthFailNoJwtCookie
            | BaseError::AuthFailJwtInvalid { .. }
            | BaseError::AuthFailCtxNotInRequestExt
            // | BaseError::SurrealDb { .. }
            // | BaseError::AuthorizationHeaderEmpty { .. }
            // | BaseError::EmptyHeaderValue { .. }
            | BaseError::NotAuthenticated { .. }
            // | BaseError::AuthorizationHeaderFormatWrong { .. } ,
            | BaseError::UserError { .. } => StatusCode::UNAUTHORIZED,
            BaseError::Execution { .. }
            | BaseError::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            // BaseError::AuthorizationHeaderMissing { .. }
            // | BaseError::WrongToken { .. }
            // | BaseError::UnAuthorizedUser { .. }
            BaseError::PeerDumpError(..)
            | BaseError::UserNotFound { .. } => StatusCode::NOT_FOUND,
            BaseError::UserAgentMissing => StatusCode::BAD_REQUEST,
        };
        let body = Json(json!({
            "error": {
                "error": self.error.to_string(),
                "req_id": self.req_id.to_string()
            }
        }));
        let mut response = (status_code, headers(), body).into_response();
        // Insert the real Error into the response - for the logger
        response.extensions_mut().insert(self.error);
        response
    }
}

// External Errors
impl From<serde_json::Error> for BaseError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde {
            source: value.to_string(),
        }
    }
}

impl From<InfraError> for BaseError {
    fn from(value: InfraError) -> Self {
        Self::PeerError {
            source: value.to_string(),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for BaseError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::AuthFailJwtInvalid {
            source: value.to_string(),
        }
    }
}

impl From<UserError> for BaseError {
    fn from(value: UserError) -> Self {
        Self::UserError {
            source: value.to_string(),
        }
    }
}

impl From<PeerError> for BaseError {
    fn from(value: PeerError) -> Self {
        // match value {
            // PeerError::InternalServerError => Self::PeerError {
            //     source: value.to_string(),
            // },
            // PeerError::NotFound(err) => Self::PeerError {
            //     source: err.to_string(),
            // },
            // PeerError::InfraError(err) => Self::PeerError {
            //     source: err.to_string(),
            // }
            // // }
            Self::PeerError {
                source: value.to_string(),
            }
        }
    }
// Define an enumeration for custom application errors
// #[derive(Debug)]
// pub enum AppError2 {  // Represents an error related to request body parsing
// }

    // Define a util to create an internal server error
    pub fn internal_error<E>(_err: E) -> BaseError {
        BaseError::InternalServerError
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn display_description() {
            let err = super::BaseError::Generic {
                description: "super description".to_owned(),
            };
            assert_eq!(format!("{err}"), "super description");
            assert_eq!(err.to_string(), "super description");
        }
    }

// #[derive(Debug)]
// pub enum UserError {
// }

    fn headers() -> HeaderMap {
        HeaderMap::from_iter(vec![
            // (header::ACCEPT_RANGES,HeaderValue::from_static("bytes")),
            //  (header::CONTENT_LENGTH,HeaderValue::from_str(format!("is").as_str()).unwrap()),
            // (header::CONTENT_RANGE,HeaderValue::from_str("asdf").unwrap()),
            //(header::TRANSFER_ENCODING,"trailers".to_string()),
            (
                header::WWW_AUTHENTICATE,
                HeaderValue::from_str("Bearer").unwrap(),
            ),
        ])
    }

// impl IntoResponse for UserError {
//     fn into_response(self) -> Response {
//         match self {
//
//         }
//             .into_response()
//     }
// }

// pub enum PeerErrors {
//     NameIsRequired
// }
//
// impl IntoResponse for PeerErrors {
//     fn into_response(self) -> Response {
//         match self {
//             PeerErrors::NameIsRequired => (
//                 StatusCode::BAD_REQUEST,
//                 HeaderMap::new(),
//                 "Something went wrong: ",
//             )
//         }.into_response()
//     }
// }

// pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
//     if err.is::<tower::timeout::error::Elapsed>() {
//         (
//             StatusCode::REQUEST_TIMEOUT,
//             "Request took too long".to_string(),
//         )
//     } else {
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("Unhandled internal error: {err}"),
//         )
//     }
// }
