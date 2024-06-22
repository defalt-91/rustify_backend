use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum::extract::FromRef;
use deadpool_diesel::postgres::Pool;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use tower_cookies::{Cookie, Cookies};
use tracing::info;
use tracing::log::debug;
use uuid::Uuid;

use crate::core::{get_config, verify_token};
use crate::errors::ApiResult;
use crate::{domain::ctx::Ctx, errors::BaseError, errors::Result};
// use crate::utils::ExtractJwt;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub key_enc: EncodingKey,
    pub key_dec: DecodingKey,
}
impl FromRef<AppState> for EncodingKey{
    fn from_ref(input: &AppState) -> Self {
       input.key_enc.clone()
    }
}
impl FromRef<AppState> for DecodingKey{
    fn from_ref(input: &AppState) -> Self {
       input.key_dec.clone()
    }
}
pub async fn mw_require_auth(
    ctx: Ctx,
    req: Request, next: Next) -> ApiResult<Response> {
    debug!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");
    ctx.user_id()?;
    Ok(next.run(req).await)
}

pub async fn mw_ctx_constructor(
    // ExtractJwt(token):ExtractJwt,
    State(AppState { key_dec, .. }): State<AppState>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Response {
    let config = get_config().await;
    info!("->> {:<12} - mw_ctx_constructor", "MIDDLEWARE");
    let uuid = Uuid::new_v4();
    let result_user_id: Result<String> =
        extract_token(config.jwt_key(), config.jwt_algorithm(), key_dec, &cookies)
            .await
            .map_err(|error| {
                // Remove an invalid cookie
                if let BaseError::AuthFailJwtInvalid { .. } = error {
                    cookies.remove(Cookie::from(config.jwt_key()))
                }
                error
            });
    // NOTE: DB should be checked here

    // Store Ctx in the request extension, for extracting in rest handlers
    let ctx = Ctx::new(result_user_id, uuid);
    req.extensions_mut().insert(ctx);
    next.run(req).await
}

async fn extract_token(
    auth_key: &str,
    alg: Algorithm,
    dec_key: DecodingKey,
    cookies: &Cookies,
) -> Result<String> {
    cookies
        .get(auth_key)
        .ok_or(BaseError::AuthFailNoJwtCookie)
        .and_then(|cookie| verify_token(dec_key, alg, cookie.value()))
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use jsonwebtoken::{
        decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
    };

    use crate::core::security::Claims;

    const SECRET: &[u8] = b"some-secret";
    const SOMEONE: &str = "someone";
    // cspell:disable-next-line
    const TOKEN_EXPIRED: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjEsImlhdCI6MTcxODM2ODk4OSwiYXV0aCI6InNvbWVvbmUifQ.csXx-tPNZqdrDscUAy4l0dChp5FMqFdEfMexUD2Br_s";

    #[test]
    fn jwt_sign_expired() {
        let my_claims = Claims {
            exp: 1,
            iat: chrono::Utc::now().timestamp() as usize,
            name: SOMEONE.to_string(),
            sub:uuid::Uuid::new_v4()
        };
        let token_str = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(SECRET),
        )
        .unwrap();
        assert_eq!(token_str, TOKEN_EXPIRED);
    }

    #[test]
    fn jwt_verify_expired_ignore() {
        let mut validation = Validation::default();
        validation.validate_exp = false;
        let token = decode::<Claims>(
            TOKEN_EXPIRED,
            &DecodingKey::from_secret(SECRET),
            &validation,
        )
        .unwrap();
        assert_eq!(token.claims.name, SOMEONE);
    }

    #[test]
    fn jwt_verify_expired_fail() {
        let token_result = decode::<Claims>(
            TOKEN_EXPIRED,
            &DecodingKey::from_secret(SECRET),
            &Validation::default(),
        );
        assert!(token_result.is_err());
        let kind = token_result.map_err(|e| e.into_kind()).err();
        assert_eq!(kind, Some(ErrorKind::ExpiredSignature));
    }

    #[test]
    fn jwt_sign_and_verify_with_chrono() {
        let exp = Utc::now() + Duration::minutes(1);
        let my_claims = Claims {
            exp: exp.timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
            name: SOMEONE.to_string(),
            sub:uuid::Uuid::new_v4()
        };
        // Sign
        let token_str = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(SECRET),
        )
        .unwrap();
        // Verify
        let token_result = decode::<Claims>(
            &token_str,
            &DecodingKey::from_secret(SECRET),
            &Validation::default(),
        )
        .unwrap();
        assert_eq!(token_result.claims.name, SOMEONE);
    }
}
