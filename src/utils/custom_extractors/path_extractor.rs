use axum::extract::rejection::PathRejection;
use axum_macros::FromRequestParts;

use crate::errors::BaseError;

#[derive(FromRequestParts, Debug)]
#[from_request(via(axum::extract::Path), rejection(BaseError))]
pub struct PathExtractor<T>(pub T);

impl From<PathRejection> for BaseError {
    fn from(rejection: PathRejection) -> Self {
        BaseError::BodyParsingError(rejection.to_string())
    }
}
