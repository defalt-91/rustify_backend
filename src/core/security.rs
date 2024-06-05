use std::time::Duration;
use bcrypt::{DEFAULT_COST, hash, verify};

use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::core::get_config;


use crate::errors::Result as AppResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,  // Expiry time of the token
    // pub iat: usize,  // Issued at time of the token
    pub auth: String,  // Email associated with the token
}

pub async fn create_token(auth: String, key_enc: EncodingKey) -> String {
    let jwt_duration = get_config().await.jwt_exp_hours();
    let exp = Utc::now() + Duration::from_secs(jwt_duration as u64);
    let claims = Claims {
        exp: exp.timestamp() as usize,
        // iat,
        auth,
    };
    encode(&Header::default(), &claims, &key_enc).expect("JWT encode should work")
}


pub fn verify_token(key: DecodingKey,algorithm: Algorithm, token: &str) -> AppResult<String> {
    Ok(
        decode::<Claims>(token, &key, &Validation::new(algorithm))?
            .claims
            .auth
    )
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hash = hash(password, DEFAULT_COST)?;
    Ok(hash)
}