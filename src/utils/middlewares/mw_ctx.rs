use axum::{extract::{Request, State}, middleware::Next, response::Response};
use deadpool_diesel::postgres::Pool;
use jsonwebtoken::{DecodingKey, EncodingKey};
use tower_cookies::{Cookie, Cookies};
use tracing::{info};
use tracing::log::debug;
use uuid::Uuid;

use crate::{domain::ctx::Ctx, errors::BaseError, errors::Result};
use crate::core::{get_config,verify_token};
use crate::errors::ApiResult;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub key_enc: EncodingKey,
    pub key_dec: DecodingKey,
}



pub async fn mw_require_auth(
    ctx: Ctx,
    req: Request,
    next: Next,
) -> ApiResult<Response> {
    debug!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");
    ctx.user_id()?;
    Ok(next.run(req).await)
}

pub async fn mw_ctx_constructor(
    State(AppState { key_dec, .. }): State<AppState>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Response {
    let config =get_config().await;
    info!("->> {:<12} - mw_ctx_constructor", "MIDDLEWARE");
    let uuid = Uuid::new_v4();
    let result_user_id: Result<String> = extract_token(key_dec, &cookies).await.map_err(|err| {
        // Remove an invalid cookie
        if let BaseError::AuthFailJwtInvalid { .. } = err {
            cookies.remove(Cookie::named(config.jwt_key()))
        }
        err
    });
    // NOTE: DB should be checked here

    // Store Ctx in the request extension, for extracting in rest handlers
    let ctx = Ctx::new(result_user_id, uuid);
    req.extensions_mut().insert(ctx);

    let res = next.run(req).await;
    res
}


async fn extract_token(key: DecodingKey, cookies: &Cookies) -> Result<String> {
    let config = get_config().await;
    cookies
        .get(config.jwt_key())
        .ok_or(BaseError::AuthFailNoJwtCookie)
        .and_then(|cookie| verify_token(key,config.jwt_algorithm(), cookie.value()))
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use jsonwebtoken::{
        decode, DecodingKey, encode, EncodingKey, errors::ErrorKind, Header, Validation,
    };

    use crate::core::security::Claims;

    const SECRET: &[u8] = b"some-secret";
    const SOMEONE: &str = "someone";
    // cspell:disable-next-line
    const TOKEN_EXPIRED: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjEsImF1dGgiOiJzb21lb25lIn0.XXHVHu2IsUPA175aQ-noWbQK4Wu-2prk3qTXjwaWBvE";

    #[test]
    fn jwt_sign_expired() {
        let my_claims = Claims {
            exp: 1,
            iat:chrono::Utc::now().timestamp() as usize,
            auth: SOMEONE.to_string(),
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
        assert_eq!(token.claims.auth, SOMEONE);
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
            iat:chrono::Utc::now().timestamp() as usize,
            auth: SOMEONE.to_string(),
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
        assert_eq!(token_result.claims.auth, SOMEONE);
    }
}