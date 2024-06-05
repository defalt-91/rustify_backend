use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Hash)]
pub struct PeerCreate {
    name: String,
    // interface_id:u32,
    persistent_keepalive: Option<u16>,
    allowed_ips: String,
}

impl PeerCreate {
    pub fn new(name:String,allowed_ips:String,persistent_keepalive:Option<u16>)-> Self{
        PeerCreate{
           name,
            allowed_ips,
            persistent_keepalive
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
