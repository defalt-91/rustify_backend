
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{
        header::USER_AGENT,
        request::Parts,
        HeaderValue,
    }
};
use crate::errors::BaseError;


pub struct ExtractUserAgent(pub HeaderValue);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractUserAgent
where
    S: Send + Sync,
{
    type Rejection = BaseError;

    async fn from_request_parts(parts: &mut Parts, _s: &S) -> Result<Self,Self::Rejection> {
        if let Some(user_agent) = parts.headers.get_mut(USER_AGENT) {
            user_agent.set_sensitive(true);
            Ok(ExtractUserAgent(user_agent.clone()))
        } else {
            Err(Self::Rejection::UserAgentMissing)
        }
    }
}

// pub struct ExtractAuthorization(pub HeaderValue);
//
// #[async_trait]
// impl<B> FromRequestParts<B> for ExtractAuthorization where B: Send + Sync, {
//     type Rejection = UserError;
//
//     async fn from_request_parts(parts: &mut Parts, _s: &B) -> Result<Self, Self::Rejection> {
//         if let Some(token) = parts.headers.get(AUTHORIZATION) {
//             Ok(ExtractAuthorization(token.clone()))
//         }
//         else {
//             Err(UserError::AuhtorizationHeaderMissing)
//         }
//     }
// }
//
