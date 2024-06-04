use axum::BoxError;
use axum::http::StatusCode;

pub enum PeerErrors {
    NameIsRequired
}

// impl IntoResponse for PeerErrors {
//     fn into_response(self) -> Response {
//         match self {
//             PeerErrors::NameIsRequired => (
//                 StatusCode::BAD_REQUEST,
//                  HeaderMap::new(),
//                 "Something went wrong: ",
//             )
//         }
//     }
// }
pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {err}"),
        )
    }
}