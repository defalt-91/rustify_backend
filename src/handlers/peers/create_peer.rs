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
    State(state): State<AppState>,                  // Extract the application state from the request
    JsonExtractor(new_peer): JsonExtractor<CreatePeerRequest>,  // Extract JSON data from the request body
) -> Result<Json<PeerResponse>, PeerError> {
    // Create a NewPeerDb instance with data from the JSON request
    let new_peer_db = peer_repository::NewPeerDb {
        title: new_peer.title,
        body: new_peer.body,
        published: false, // Set the initial 'published' status to false
    };

    // Insert the new peer into the database using the repository
    let created_peer = peer_repository::create(&state.pool, new_peer_db)
        .await
        .map_err(PeerError::InfraError)?;  // Handle potential infrastructure errors

    // Create a peerResponse instance from the newly created peer
    let peer_response = PeerResponse {
        id: created_peer.id,
        title: created_peer.title,
        body: created_peer.body,
        published: created_peer.published,
    };

    // Return the response as JSON with a success status
    Ok(Json(peer_response))
}