// Import necessary modules and types
use axum::extract::{Query, State};
use axum::Json;

use crate::AppState;
use crate::domain::ctx::Ctx;
// Import internal modules and types
use crate::errors::{ApiError, ApiResult};
use crate::handlers::peers::{ListPeersResponse, PeersFilter};
use crate::infra::peer_repository;



pub async fn list_peers(
    ctx: Ctx,
    State(state): State<AppState>,
    Query(params): Query<PeersFilter>,
) -> ApiResult<Json<ListPeersResponse>> {
    peer_repository::get_all(&state.pool, params)
        .await
        .map(ListPeersResponse::from_db)
        .map(Json)
        .map_err(
            |err| {
                tracing::error!(%err, "error from list_peers");
                ApiError::from(&ctx)(err)
            }
        )
}
