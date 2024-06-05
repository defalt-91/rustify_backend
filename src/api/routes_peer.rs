use axum::{Json, Router};
use axum::extract::State;
use axum::routing::get;

use crate::schema::ctx::Ctx;
use crate::Pool;
use crate::error::ApiResult;
use crate::service::peer::{PeerFullDump, PeerRxTxDump, PeerService};

pub fn peers_routes(db: Pool) -> Router {
    Router::new()
        .route("/", get(wg_dump))
        .route("/rxtx", get(wg_rxtx_lha))
        .with_state(db)
}

async fn wg_dump(State(db): State<Pool>, ctx: Ctx) -> ApiResult<Json<Vec<PeerFullDump>>> {
    PeerService { db: &db, ctx: &ctx }
        .peer_full_dump()
        .await
        .map(Json)
}

async fn wg_rxtx_lha(State(db): State<Pool>, ctx: Ctx) -> ApiResult<Json<Vec<PeerRxTxDump>>> {
    PeerService { db: &db, ctx: &ctx }
        .wg_rxtx_lha()
        .await
        .map(Json)
}

async fn create_peer() {}