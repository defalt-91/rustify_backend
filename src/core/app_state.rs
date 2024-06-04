use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use uuid::Uuid;
use surrealdb::{
    engine::local::{Db as LocalDb, Mem},
    Surreal,
};
use crate::errors::base::{ApiError, ApiResult, Result};
type Db = Surreal<LocalDb>;
static DB: Lazy<Db> = Lazy::new(Surreal::init);
#[derive(Clone, Debug)]
pub struct Ctx {
    result_user_id: Result<String>,
    req_id: Uuid,
}

impl Ctx {
    pub fn new(result_user_id: Result<String>, uuid: Uuid) -> Self {
        Self {
            result_user_id,
            req_id: uuid,
        }
    }

    pub fn user_id(&self) -> ApiResult<String> {
        self.result_user_id.clone().map_err(|error| ApiError {
            error,
            req_id: self.req_id,
        })
    }

    pub fn req_id(&self) -> Uuid {
        self.req_id
    }
}
#[derive(Clone)]
pub struct CtxState {
    // NOTE: with DB, because a real login would check the DB
    _db: Db,
    key_enc: EncodingKey,
    key_dec: DecodingKey,
}
impl CtxState {
    pub fn new(key_enc:EncodingKey,key_dec:DecodingKey) -> Self {
        CtxState {
            _db: DB.clone(),
            key_enc,
            key_dec
        }
    }
}


