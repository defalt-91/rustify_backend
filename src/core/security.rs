use std::time::Duration;

use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::core::config;
use crate::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub auth: String,
}

pub fn create_token(auth: String, key_enc: EncodingKey) -> String {
    let jwt_duration = config::jwt_exp_secs();
    let exp = Utc::now() + Duration::from_secs(jwt_duration);
    let claims = Claims {
        exp: exp.timestamp() as usize,
        auth,
    };
    encode(&Header::default(), &claims, &key_enc).expect("JWT encode should work")
}


pub(crate) fn verify_token(key: DecodingKey, token: &str) -> Result<String> {
    Ok(
        decode::<Claims>(token, &key, &Validation::new(config::jwt_algorithm()))?
            .claims
            .auth
    )
}

