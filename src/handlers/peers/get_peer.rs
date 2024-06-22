// Import necessary modules and types
use axum::extract::State;
use axum::Json;
use uuid::Uuid;

// Import internal modules and types
use crate::domain::models::peer::PeerError;
use crate::handlers::peers::PeerResponse;
use crate::infra::errors::InfraError;

// Import PathExtractor for extracting the peer_id from the request path
use crate::infra::peer_repository;
use crate::utils::PathExtractor;
use crate::AppState;

// Define the handler function for retrieving a specific peer by its ID
pub async fn get_peer(
    State(state): State<AppState>, // Extract the application state from the request
    PathExtractor(peer_id): PathExtractor<Uuid>, // Extract the peer_id from the request path
) -> Result<Json<PeerResponse>, PeerError> {
    // Use the peer_repository to fetch the peer based on its ID
    peer_repository::read(&state.pool, peer_id)
        .await
        .map(PeerResponse::from_db)
        .map(Json)
        .map_err(|db_error| match db_error {
            // Map infrastructure errors to custom PeerError types
            InfraError::InternalServerError => PeerError::InternalServerError,
            InfraError::NotFound => PeerError::NotFound(peer_id),
        })
}
