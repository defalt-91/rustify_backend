use axum::{Json, Router};
use axum::routing::get;
use crate::mw_ctx::CtxState;

use crate::peer::schema::{PeerFullDump, PeerRxTxDump};
use crate::utils::sudo_exec;

pub fn peers_routes(state:CtxState) -> Router {
    Router::new()
        .route("/peers", get(wg_dump))
        .route("/peers/rxtx", get(wg_rxtx_lha))
        .with_state(state)
}

async fn wg_dump() -> Json<Vec<PeerFullDump>> {
    let dump_output = sudo_exec(vec!["wg", "show", "wg0", "dump"]);
    let dump = String::from_utf8(dump_output.stdout).unwrap();
    let dump_str = dump.strip_suffix("\n");
    let dump_vec: Vec<&str> = dump_str.map_or(vec![], |v| v.split("\n").collect());
    let mut dump_vec_skipped_if = dump_vec.iter();
    dump_vec_skipped_if.next();
    Json(
        dump_vec_skipped_if.map(|v| PeerFullDump::from_dump_str(v)).collect()
    )
}

async fn wg_rxtx_lha() -> Json<Vec<PeerRxTxDump>> {
    let t_output = sudo_exec(vec!["wg", "show", "wg0", "transfer"]);
    let t_dump = String::from_utf8(t_output.stdout).unwrap();
    let t_dump_str = t_dump.strip_suffix("\n");
    let lha_output = sudo_exec(vec!["wg", "show", "wg0", "latest-handshakes"]);
    let lha_dump = String::from_utf8(lha_output.stdout).unwrap();
    let lha_dump_str = lha_dump.strip_suffix("\n");
    Json(
        match t_dump_str.zip(lha_dump_str) {
            Some(v) => v.0.split("\n").zip(v.1.split("\n")).map(|v| PeerRxTxDump::from_str(v)).collect(),
            None => vec![],
        }
    )
}

async fn create_peer() {}