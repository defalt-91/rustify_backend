use crate::domain::models::peer::PeerError;
use crate::handlers::peers::{PeerResponse, UpdatePeerRequest};
use crate::infra::errors::InfraError;
use crate::infra::peer_repository;
use crate::utils::middlewares::mw_ctx::AppState;
use crate::utils::{JsonExtractor, PathExtractor};
use axum::extract::State;
use axum::Json;
use uuid::Uuid;

pub async fn update_peer(
    State(state): State<AppState>,
    PathExtractor(id): PathExtractor<Uuid>,
    JsonExtractor(update_values): JsonExtractor<UpdatePeerRequest>,
) -> Result<Json<PeerResponse>, PeerError> {
    let update_peer = peer_repository::update_peer(&state.pool, id, update_values)
        .await
        .map_err(|db_error| match db_error {
            // Map infrastructure errors to custom PeerError types
            InfraError::InternalServerError => PeerError::InternalServerError,
            InfraError::NotFound => PeerError::NotFound(id),
        })?;
    Ok(Json(PeerResponse::from_db(update_peer)))
}
