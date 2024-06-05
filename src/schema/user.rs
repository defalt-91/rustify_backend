use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    grant_type: String,
    pub(crate) username: String,
    pub(crate) password: String,
    scope: String,
    client_id: Option<String>,
    client_secret: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginSuccess {
    access_token: String,
    token_type: String,
}

impl LoginSuccess {
    pub fn create_bearer(token_str:String)->Self{
        Self{
            access_token:token_str,
            token_type:"Bearer".to_string()
        }
    }
}