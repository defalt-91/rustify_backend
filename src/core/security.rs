use std::ops::Add;
use bcrypt::{hash, verify};

use crate::core::get_config;
use chrono::{Utc,Duration};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{ApiError, BaseError, Result as AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub name: String,
    pub exp: usize,
    pub iat: usize,
}

pub async fn create_token(sub: Uuid,name:String, key_enc: EncodingKey) -> String {
    let config = get_config().await;
    let jwt_duration = config.jwt_exp_minutes();
    let iat = Utc::now();
    let exp = iat.add(Duration::minutes(jwt_duration));
    let claims = Claims {
        sub,
        name,
        exp: exp.timestamp() as usize,
        iat: iat.timestamp() as usize,
    };
    encode(&Header::new(config.jwt_algorithm()), &claims, &key_enc).expect("JWT encode should work")
}

pub fn verify_token(key: DecodingKey, algorithm: Algorithm, token: &str) -> AppResult<String> {
    let validations = Validation::new(algorithm);
    // validations.validate_exp=true;
    Ok(
        decode::<Claims>(token, &key, &validations)?
            .claims
            .name
    )
}

pub fn verify_password(req_id: Uuid, password: &str, hash_value: &str) -> Result<bool, ApiError> {
    verify(password, hash_value).map_err(|_err| ApiError {
        req_id,
        error: BaseError::InternalServerError,
    })
}

pub async fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let config = get_config().await;
    hash(
        password,
        config.hash_cost()
        // DEFAULT_COST
    )
}
