// Import necessary modules and types
use axum::extract::rejection::JsonRejection;
use axum::response::IntoResponse;
use axum_macros::FromRequest;

// Import internal AppError type
use crate::errors::BaseError;

// Define a custom extractor for JSON data
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(BaseError))]  // Derive the FromRequest trait with specific configuration
pub struct JsonExtractor<T>(pub T);

// Implement the conversion from JsonRejection to AppError
impl From<JsonRejection> for BaseError {
    fn from(rejection: JsonRejection) -> Self {
        // Convert the JsonRejection into a BodyParsingError with the rejection message
        BaseError::BodyParsingError(rejection.to_string())
    }
}
