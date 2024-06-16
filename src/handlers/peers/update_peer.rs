use crate::domain::models::peer::PeerError;
use crate::handlers::peers::{PeerResponse, UpdatePeerRequest};
use crate::infra::errors::{adapt_infra_error, InfraError};
use crate::infra::peer_repository;
use crate::utils::middlewares::mw_ctx::AppState;
use crate::utils::{JsonExtractor, PathExtractor};
use axum::extract::State;
use axum::Json;
use uuid::Uuid;
use crate::domain::ctx::Ctx;
use crate::errors::{ApiError, ApiResult, BaseError};
use crate::infra::peer_repository::UpdatePeerForm;

pub async fn update_peer(
    State(state): State<AppState>,
    ctx:Ctx,
    PathExtractor(id): PathExtractor<Uuid>,
    JsonExtractor(payload): JsonExtractor<UpdatePeerRequest>,
) -> ApiResult<Json<PeerResponse>> {
    let update_values = UpdatePeerForm{
        interface_id:None,
        name:payload.name
    };
    peer_repository::update_peer(&state.pool, id, update_values)
        .await
        .map(|updated_peer|PeerResponse::from_db(updated_peer))
        .map(Json)
        .map_err(|db_error| match db_error {
            // Map infrastructure errors to custom PeerError types
            InfraError::InternalServerError => PeerError::InternalServerError,
            InfraError::NotFound => PeerError::NotFound(id),
        }).map_err(|e|ApiError{
        req_id:ctx.req_id(),
        error:BaseError::from(e)
    })
}
