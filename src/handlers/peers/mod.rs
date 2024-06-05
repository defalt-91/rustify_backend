use axum::Router;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use create_peer::create_peer;
use get_peer::get_peer;
use wg_dump::{wg_dump, wg_rxtx_lha};
use crate::domain::models::peer::PeerModel;

use crate::utils::middlewares::mw_ctx::AppState;

mod get_peer;
mod create_peer;
mod list_peers;
mod wg_dump;

// Define a sub-router for handling peers-related routes
pub fn peers_router(state: AppState) -> Router {
    // Create a new Router for peers-related routes
    Router::new()
        .route("/", get(wg_dump))
        .route("/rxtx", get(wg_rxtx_lha))
        .with_state(state)
}
pub fn peer_router(state: AppState) -> Router {
    // Create a new Router for peers-related routes
    Router::new()
        // Define a route for creating a new peer using the HTTP POST method
        .route("/", post(create_peer))
        // Define a route for listing peers using the HTTP GET method
        // .route("/", get(list_peers))
        // Define a route for retrieving a specific peer by ID using the HTTP GET method
        .route("/:id", get(get_peer))
        // Provide the application state to this sub-router
        .with_state(state)
}

#[derive(Debug, Deserialize)]
pub struct CreatePeerRequest {
    title: String,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerResponse {
    id: Uuid,
    title: String,
    body: String,
    published: bool,
}

impl PeerResponse {
    pub fn from_db(peer: PeerModel) -> Self {
        Self {
            id: peer.id,
            title: peer.title,
            body: peer.body,
            published: peer.published,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPeersResponse {
    peers: Vec<PeerResponse>,
}
impl ListPeersResponse{
    pub fn from_db(peers:Vec<PeerModel>)->Self{
        Self{
            peers:peers.into_iter().map(PeerResponse::from_db).collect()
        }
    }
}

