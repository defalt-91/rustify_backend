use serde::{Deserialize, Serialize};
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Peer{
//     name:Option<String>,
//     friendly_name:Option<String>,
//     friendly_json:Option<String>,
//     enabled:Option<bool>,
//     public_key: String,
//     private_key: Option<String>,
//     preshared_key: Option<String>,
//     if_pubkey:Option<String>,
//     if_id:Option<String>,
//     address:Option<String>,
//     endpoint_addr: Option<String>,
//     allowed_ips: Option<String>,
//     last_handshake_at: u16,
//     transfer_rx: u16,
//     transfer_tx: u16,
//     persistent_keepalive: Option<u32>,
//     // interface_id:Option<Interface>
// }

#[derive(Debug, Serialize, Clone, Hash)]
pub struct PeerFullDump {
    name: Option<String>,
    public_key: String,
    preshared_key: Option<String>,
    endpoint_addr: Option<String>,
    allowed_ips: Option<String>,
    last_handshake_at: u16,
    transfer_rx: u16,
    transfer_tx: u16,
    persistent_keepalive: Option<u32>,
}

impl PeerFullDump {
    pub fn from_dump_str(dump: &str) -> Self {
        let mut values = dump.split_whitespace();
        Self {
            name: Some("test".to_string()),
            public_key: values.next().unwrap().to_string(),
            preshared_key: values.next().and_then(|v| {
                if v == "(none)" {
                    None
                } else {
                    Some(v.to_string())
                }
            }),
            endpoint_addr: values.next().and_then(|v| {
                if v == "(none)" {
                    None
                } else {
                    Some(v.to_string())
                }
            }),
            allowed_ips: values.next().map(|v| v.to_string()),
            last_handshake_at: values.next().unwrap().parse().unwrap(),
            transfer_rx: values.next().unwrap().parse().unwrap(),
            transfer_tx: values.next().unwrap().parse().unwrap(),
            persistent_keepalive: values.next().map(|v| v.parse().unwrap()),
        }
    }
}

#[derive(Debug, Serialize, Clone, Hash)]
pub struct PeerRxTxDump {
    public_key: String,
    last_handshake_at: u16,
    transfer_rx: u16,
    transfer_tx: u16,
}

impl PeerRxTxDump {
    pub fn from_rxtx_lha(value: (&str, &str)) -> Self {
        // t_dump.split("\n").zip( lha_dump.split("\n"))
        //     .map(
        //         |v| {
        let mut transfer = value.0.split_whitespace();
        let mut last_handshake = value.1.split_whitespace();
        last_handshake.next().unwrap();
        Self {
            public_key: transfer.next().unwrap_or("").parse().unwrap(),
            last_handshake_at: last_handshake.next().unwrap_or("").parse().unwrap(),
            transfer_rx: transfer.next().unwrap_or("").parse().unwrap(),
            transfer_tx: transfer.next().unwrap_or("").parse().unwrap(),
        }
        //     }
        // )
        //         .collect::<Vec<PeerRxTxDump>>()
    }
}

#[derive(Debug, Deserialize, Clone, Hash)]
pub struct PeerCreate {
    name: String,
    // interface_id:u32,
    persistent_keepalive: Option<u16>,
    allowed_ips: String,
    with_psk: bool,
}

// pub struct PeerService<'a> {
//     // pub db: &'a Pool,
//     pub ctx: &'a Ctx,
// }

// impl<'a> PeerService<'a> {
// pub async fn sudo_exec(&self, cmd: Vec<&str>) -> ApiResult<Output> {
//     Command::new("sudo")
//         .args(cmd.clone())
//         .output().await.map_err(|err| ApiError {
//         req_id: self.ctx.req_id(),
//         error: Error::Execution { source: "here".to_string() },
//     })
// }
// pub async fn peer_full_dump(&self) -> ApiResult<Vec<PeerFullDump>> {
//     let dump_output = self.sudo_exec(vec!["wg", "show", "wg0", "dump"]).await?;
//     let dump = String::from_utf8(dump_output.stdout).unwrap();
//     let dump_str = dump.strip_suffix("\n");
//     let dump_vec: Vec<&str> = dump_str.map_or(vec![], |v| v.split("\n").collect());
//     let mut dump_vec_skipped_if = dump_vec.iter();
//     dump_vec_skipped_if.next();
//     Ok(dump_vec_skipped_if.map(|v| PeerFullDump::from_dump_str(v)).collect())
// }

// pub async fn wg_rxtx_lha(&self) -> ApiResult<Vec<PeerRxTxDump>> {
// let t_output = self.sudo_exec(vec!["wg", "show", "wg0", "transfer"]).await?;
// let lha_output = self.sudo_exec(vec!["wg", "show", "wg0", "latest-handshakes"]).await?;
// if !(t_output.stdout.is_empty() && lha_output.stdout.is_empty()) {
//     let t_to_string = String::from_utf8(t_output.stdout).unwrap();
//     let lha_to_string = String::from_utf8(lha_output.stdout).unwrap();
//     let dump_transfer = t_to_string.strip_suffix("\n").unwrap();
//     let dump_last_handshake = lha_to_string.strip_suffix("\n").unwrap();
//     Ok(PeerRxTxDump::from_rxtx_lha_dump(dump_transfer, dump_last_handshake))
// } else { Ok(vec![]) }
// }
// }
