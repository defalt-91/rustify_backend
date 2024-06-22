// use axum::{async_trait, Extension, RequestPartsExt};
// use axum::extract::{FromRef, FromRequestParts, State};
// use axum::http::request::Parts;
// use axum_extra::extract::cookie::{CookieJar, Cookie, Expiration, SameSite};
// use jsonwebtoken::Algorithm::{HS256, RS256};
// use jsonwebtoken::DecodingKey;
// use tracing::{debug, error, warn};
// use crate::core::{Config, get_config, verify_token};
// use crate::errors::BaseError;
//
// pub struct ExtractJwt(pub String);
//
// #[async_trait]
// impl<S> FromRequestParts<S> for ExtractJwt
//     where
//         S: Send + Sync,
//         DecodingKey: FromRef<S>,
// {
//     type Rejection = BaseError;
//
//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let config = get_config().await;
//         let key = config.jwt_key();
//         let key = config.jwt_key();
//         let dec_key = DecodingKey::from_ref(state);
//
//         let jar = CookieJar::from_request_parts(parts, state)
//             .await
//             .map_err(|_| Self::Rejection::InternalServerError)?;
//         if let Some(value) = jar.get(key) {
//             let token = verify_token(dec_key, RS256, &value.to_string()).unwrap();
//             Ok(ExtractJwt(value.to_string()))
//         } else {
//             Err(Self::Rejection::AuthFailNoJwtCookie)
//         }
//         // jar.get("key")
//         //     .ok_or({
//         //         CookieJar::remove()
//         //         jar.remove(Cookie::from(key));
//         //         Self::Rejection::AuthFailNoJwtCookie
//         //     })
//         //     // .and_then(|jwt|verify_token(st.clone(),HS256,jwt.value()))
//         //     .map(|v|ExtractJwt(v.to_string()))
//         // st.key_dec
//         // jar.get(key)
//         //     .ok_or(BaseError::AuthFailNoJwtCookie)
//         // .and_then(|cookie| verify_token(dec_key, alg, cookie.value()))
//     }
// }