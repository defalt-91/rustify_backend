use axum::extract::Query;
use axum::Json;
use serde::Deserialize;
use tracing::debug;

use crate::domain::ctx::Ctx;
use crate::errors::{ApiError, ApiResult, BaseError};
use crate::service::peer::{PeerFullDump, PeerRxTxDump};
use crate::utils::sudo_exec;

#[derive(Deserialize)]
pub struct IfId {
    interface_id: Option<usize>,
}

pub async fn wg_dump(
    Query(payload): Query<IfId>,
    ctx: Ctx,
) -> ApiResult<Json<Vec<PeerFullDump>>> {
    debug!("{:?}",payload.interface_id);
    let dump_output = sudo_exec(vec!["wg", "show", "wg0", "dump"])
        .await
        .map_err(ApiError::from(&ctx))
        ?;
    let dump = String::from_utf8(dump_output.stdout).unwrap();
    let dump_vec: Vec<&str> = dump
        .strip_suffix("\n")
        .map(|v| v.split("\n").collect())
        .ok_or(ApiError {
            req_id: ctx.req_id(),
            error: BaseError::PeerDumpError("wg show dump error"),
        })?;
    let mut dump_vec_skipped_if = dump_vec.iter();
    dump_vec_skipped_if.next();
    Ok(
        Json(
            dump_vec_skipped_if
                .map(|v| PeerFullDump::from_dump_str(v))
                .collect()
        )
    )
}

pub async fn wg_rxtx_lha(ctx: Ctx) -> ApiResult<Json<Vec<PeerRxTxDump>>> {
    let t_output = sudo_exec(vec!["wg", "show", "wg0", "transfer"])
        .await
        .map_err(ApiError::from(&ctx))?;
    let t_string = String::from_utf8(t_output.stdout).unwrap();
    let dump_transfer = t_string
        .strip_suffix("\n")
        .map(|v| v.split("\n"))
        .ok_or(BaseError::PeerDumpError("wg show transfer error"),
        ).map_err(ApiError::from(&ctx))?;
    let lha_output = sudo_exec(vec!["wg", "show", "wg0", "latest-handshakes"])
        .await
        .map_err(ApiError::from(&ctx))?;
    let lha_string = String::from_utf8(lha_output.stdout).unwrap();
    let dump_last_handshake =
        lha_string
            .strip_suffix("\n")
            .map(|v| v.split("\n"))
            .ok_or(ApiError {
                req_id: ctx.req_id(),
                error: BaseError::PeerDumpError("latest-handshakes"),
            })?;
    Ok(Json(dump_transfer
        .zip(dump_last_handshake)
        .map(PeerRxTxDump::from_rxtx_lha)
        .collect()))
}
