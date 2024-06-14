use bcrypt::{hash, verify, DEFAULT_COST};
use std::time::Duration;

use crate::core::get_config;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{ApiError, BaseError, Result as AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub auth: String,
}

pub async fn create_token(auth: String, key_enc: EncodingKey) -> String {
    let jwt_duration = get_config().await.jwt_exp_hours();
    let iat = Utc::now();
    let exp = iat + Duration::from_secs(jwt_duration as u64);
    let claims = Claims {
        exp: exp.timestamp() as usize,
        iat: iat.timestamp() as usize,
        auth,
    };
    encode(&Header::default(), &claims, &key_enc).expect("JWT encode should work")
}

pub fn verify_token(key: DecodingKey, algorithm: Algorithm, token: &str) -> AppResult<String> {
    let validations = Validation::new(algorithm);
    // validations.validate_exp=true;
    Ok(
        decode::<Claims>(token, &key, &validations)?
            .claims
            .auth
    )
}

pub fn verify_password(req_id: Uuid, password: &str, hash_value: &str) -> Result<bool, ApiError> {
    verify(password, hash_value).map_err(|_err| ApiError {
        req_id,
        error: BaseError::InternalServerError,
    })
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}
