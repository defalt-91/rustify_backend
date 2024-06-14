// Import necessary modules and types
use axum::extract::State;
use axum::Json;

// Import internal modules and types
use crate::domain::models::peer::PeerError;
use crate::handlers::peers::{CreatePeerRequest, PeerResponse};
use crate::infra::peer_repository;

// This is a placeholder to extract JSON data from the request body.
use crate::utils::{middlewares::mw_ctx::AppState, JsonExtractor};

// Define the handler function for creating a new peer
pub async fn create_peer(
    State(state): State<AppState>, // Extract the application state from the request
    JsonExtractor(new_peer): JsonExtractor<CreatePeerRequest>, // Extract JSON data from the request body
) -> Result<Json<PeerResponse>, PeerError> {
    // Create a NewPeerDb instance with data from the JSON request
    let new_peer_db = peer_repository::NewPeerDb {
        name: new_peer.name,
        if_pubkey: "new_peer.name".to_string(),
        address: "address".to_string(),
        private_key: "private_key".to_string(),
        public_key: "public_key".to_string(),
        preshared_key: None,
    };

    // Insert the new peer into the database using the repository
    let created_peer = peer_repository::create(&state.pool, new_peer_db)
        .await
        .map_err(PeerError::InfraError)?; // Handle potential infrastructure errors

    // Create a peerResponse instance from the newly created peer
    let peer_response = PeerResponse {
        id: created_peer.id,
        name: created_peer.name,
        enabled: created_peer.enabled,
        created_at: created_peer.created_at,
        updated_at: created_peer.updated_at,
    };

    // Return the response as JSON with a success status
    Ok(Json(peer_response))
}
