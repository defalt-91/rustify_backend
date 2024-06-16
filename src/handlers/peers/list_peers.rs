// Import necessary modules and types
use axum::extract::{Query, State};
use axum::Json;

use crate::AppState;
use crate::domain::ctx::Ctx;
// Import internal modules and types
use crate::domain::models::peer::PeerError;
use crate::errors::{ApiError, ApiResult};
use crate::handlers::peers::ListPeersResponse;
use crate::infra::peer_repository::{get_all, PeersFilter};

pub async fn list_peers(
    State(state): State<AppState>,
    ctx: Ctx,
    Query(params): Query<PeersFilter>, // Extract query parameters for filtering peers
) -> ApiResult<Json<ListPeersResponse>> {
    // Use the `get_all` function to retrieve a list of peers based on the provided query parameters
    let peers = get_all(&state.pool, params)
        .await
        .map_err(|err| {
            tracing::error!(%err, "error from list_peers");
            ApiError::from(&ctx)(err)
        }
        )?;

    // Convert the retrieved list of PeerModel instances to a ListPeersResponse
    Ok(Json(ListPeersResponse::from_db(peers)))
}
