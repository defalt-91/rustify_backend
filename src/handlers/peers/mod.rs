use axum::routing::{delete, get, patch, post};
use axum::Router;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::peer::PeerModel;

use crate::utils::middlewares::mw_ctx::AppState;

mod create_peer;
mod get_peer;
mod list_peers;
mod update_peer;
mod wg_dump;
mod remove_peer;

// Define a sub-router for handling peers-related routes
pub fn peers_router(state: AppState) -> Router {
    // Create a new Router for peers-related routes
    Router::new()
        .route("/", get(wg_dump::wg_dump))
        .route("/rxtx", get(wg_dump::wg_rxtx_lha))
        .with_state(state)
}

pub fn peer_router(state: AppState) -> Router {
    // Create a new Router for peers-related routes
    Router::new()
        // Define a route for creating a new peer using the HTTP POST method
        .route("/", post(create_peer::create_peer))
        // Define a route for listing peers using the HTTP GET method
        .route("/", get(list_peers::list_peers))
        // Define a route for retrieving a specific peer by ID using the HTTP GET method
        .route("/:id", get(get_peer::get_peer))
        .route("/:id", patch(update_peer::update_peer))
        .route("/:id", delete(remove_peer::delete_peer))
        // Provide the application state to this sub-router
        .with_state(state)
}

#[derive(Debug, Deserialize)]
pub struct CreatePeerRequest {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePeerRequest {
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerResponse {
    id: Uuid,
    name: String,
    enabled: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

impl PeerResponse {
    pub fn from_db(peer: PeerModel) -> Self {
        Self {
            id: peer.id,
            name: peer.name,
            enabled: peer.enabled,
            created_at: peer.created_at,
            updated_at: peer.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPeersResponse {
    peers: Vec<PeerResponse>,
}

impl ListPeersResponse {
    pub fn from_db(peers: Vec<PeerModel>) -> Self {
        Self {
            peers: peers.into_iter().map(PeerResponse::from_db).collect(),
        }
    }
}
