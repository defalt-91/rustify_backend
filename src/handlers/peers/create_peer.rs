// Import necessary modules and types
use axum::extract::State;
use axum::Json;
use crate::domain::ctx::Ctx;

// Import internal modules and types
use crate::domain::models::peer::PeerError;
use crate::errors::{ApiError, ApiResult};
use crate::handlers::peers::{CreatePeerRequest, PeerResponse};
use crate::infra::peer_repository;

// This is a placeholder to extract JSON data from the request body.
use crate::utils::{middlewares::mw_ctx::AppState, JsonExtractor};

// Define the handler function for creating a new peer
pub async fn create_peer(
    State(state): State<AppState>,
    ctx: Ctx,
    JsonExtractor(new_peer): JsonExtractor<CreatePeerRequest>, // Extract JSON data from the request body
) -> ApiResult<Json<PeerResponse>> {
    // Create a NewPeerDb instance with data from the JSON request
    let new_peer_db = peer_repository::NewPeerForm {
        name: new_peer.name,
        if_pubkey: "new_peer.name".to_string(),
        address: "address".to_string(),
        private_key: "private_key".to_string(),
        public_key: "public_key".to_string(),
        preshared_key: None,
        interface_id: 1,
    };

    // Insert the new peer into the database using the repository
    peer_repository::create(&state.pool, new_peer_db)
        .await
        .map(|created_peer|
                Json(PeerResponse::from_db(created_peer))
        )
        .map_err(ApiError::from(&ctx))
}
