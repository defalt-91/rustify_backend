use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use uuid::Uuid;
use crate::domain::ctx::Ctx;
use crate::errors::{ApiError, ApiResult};
use crate::handlers::peers::PeerResponse;
use crate::infra::peer_repository::{remove_peer};
use crate::utils::middlewares::mw_ctx::AppState;
use crate::utils::PathExtractor;

pub async fn delete_peer(
    ctx: Ctx,
    State(state): State<AppState>,
    PathExtractor(peer_id): PathExtractor<Uuid>,
) -> ApiResult<Json<Value>> {
    remove_peer(&state.pool, peer_id)
        .await
        .map(|val|json!({"deleted_peer":val}).into())
        .map_err(|err| {
            ApiError {
                req_id: ctx.req_id(),
                error: err.into(),
            }
        })
}