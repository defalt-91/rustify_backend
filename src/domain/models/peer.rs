use std::fmt;
use std::fmt::Formatter;
use crate::infra::errors::InfraError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
#[derive(Clone, Debug, PartialEq)]
pub struct PeerModel {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
    pub persistent_keepalive: usize,
    pub allowed_ips: String,
    pub preshared_key: Option<String>,
    pub private_key: String,
    pub public_key: String,
    pub if_pubkey: String,
    pub address: String,
    pub transfer_rx: usize,
    pub transfer_tx: usize,
    pub last_handshake_at: Option<NaiveDateTime>,
    pub endpoint_addr: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub interface_id:i32,
}

#[derive(Debug)]
pub enum PeerError {
    InternalServerError,
    NotFound(Uuid),
    InfraError(InfraError),
}
impl fmt::Display for PeerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PeerError::InternalServerError => write!(f, "InternalServerError"),
            PeerError::NotFound(uuid) => write!(f, "user with userid : {uuid} not found"),
            PeerError::InfraError(_err) => write!(f, "infra error"),
        }
    }
}
impl IntoResponse for PeerError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("PeerModel with id {} has not been found", id),
            ),
            Self::InfraError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };
        (
            status,
            Json(
                json!({"resource":"PeerModel", "message": err_msg, "happened_at" :  Utc::now().timestamp() }),
            ),
        )
            .into_response()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Hash)]
pub struct PeerCreate {
    name: String,
    // interface_id:u32,
    persistent_keepalive: Option<u16>,
    allowed_ips: String,
}

impl PeerCreate {
    pub fn new(name: String, allowed_ips: String, persistent_keepalive: Option<u16>) -> Self {
        PeerCreate {
            name,
            allowed_ips,
            persistent_keepalive,
        }
    }
}

// #[derive(Debug, Deserialize, Serialize, Clone, Hash)]
// pub struct PeerFullDump {
//     name:Option<String>,
//     public_key: String,
//     preshared_key: Option<String>,
//     endpoint_addr: Option<String>,
//     allowed_ips: Option<String>,
//     last_handshake_at: u16,
//     transfer_rx: u16,
//     transfer_tx: u16,
//     persistent_keepalive: Option<u32>,
// }

// impl PeerFullDump {
//     pub fn from_dump_str(dump: &str) -> Self {
//         let mut values = dump.split_whitespace();
//         Self {
//             name:Some("test".to_string()),
//             public_key: values.next().unwrap().to_string(),
//             preshared_key: values.next().map_or(None, |v| if v == "(none)" {
//                 None
//             } else { Some(v.to_string()) }),
//             endpoint_addr: values.next().map_or(None, |v| if v == "(none)" {
//                 None
//             } else { Some(v.to_string()) },
//             ),
//             allowed_ips: values.next().map_or(None, |v| Some(v.to_string())),
//             last_handshake_at: values.next().unwrap().parse().unwrap(),
//             transfer_rx: values.next().unwrap().parse().unwrap(),
//             transfer_tx: values.next().unwrap().parse().unwrap(),
//             persistent_keepalive: values.next().map_or(None, |v| Some(v.parse().unwrap())),
//         }
//     }
// }

// #[derive(Debug, Deserialize, Serialize, Clone, Hash)]
// pub struct PeerRxTxDump {
//     public_key: String,
//     last_handshake_at: u16,
//     transfer_rx: u16,
//     transfer_tx: u16,
// }

// impl PeerRxTxDump {
//     pub fn from_dump_str(values: (&str, &str)) -> Self {
//         let mut transfer = values.0.split_whitespace();
//         let mut last_handshake = values.1.split_whitespace();
//         // skipping public key
//         last_handshake.next().unwrap();
//         Self {
//             public_key: transfer.next().unwrap().parse().unwrap(),
//             last_handshake_at: last_handshake.next().unwrap().parse().unwrap(),
//             transfer_rx: transfer.next().unwrap().parse().unwrap(),
//             transfer_tx: transfer.next().unwrap().parse().unwrap(),
//         }
//     }
// }
