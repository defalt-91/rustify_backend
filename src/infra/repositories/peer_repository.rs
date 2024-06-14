use chrono::NaiveDateTime;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::peer::PeerModel;
use crate::handlers::UpdatePeerRequest;
use crate::infra::db::schema::peers;
use crate::infra::errors::{adapt_infra_error, InfraError};
// Define a struct representing the database schema for peers
#[derive(Serialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = peers)] // Use the 'peers' table
#[diesel(check_for_backend(diesel::pg::Pg))] // Check compatibility with PostgreSQL
pub struct PeerDb {
    pub id: Uuid,
    pub name: String,
    pub enabled: bool,
    pub persistent_keepalive: i32,
    pub allowed_ips: String,
    pub preshared_key: Option<String>,
    pub private_key: String,
    pub public_key: String,
    pub if_pubkey: String,
    pub address: String,
    pub transfer_rx: i32,
    pub transfer_tx: i32,
    pub last_handshake_at: Option<NaiveDateTime>,
    pub endpoint_addr: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Define a struct for inserting new peers into the database
#[derive(Deserialize, Insertable)]
#[diesel(table_name = peers)] // Use the 'peers' table
pub struct NewPeerDb {
    pub name: String,
    pub preshared_key: Option<String>,
    pub private_key: String,
    pub public_key: String,
    pub if_pubkey: String,
    pub address: String,
}

// Define a struct for filtering peers
#[derive(Deserialize)]
pub struct PeersFilter {
    enabled: bool,
    name_contains: Option<String>,
}

// Function to insert a new peer into the database
pub async fn create(
    pool: &deadpool_diesel::postgres::Pool,
    new_post: NewPeerDb,
) -> Result<PeerModel, InfraError> {
    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    // Insert the new peer into the 'peers' table, returning the inserted peer
    let res = conn
        .interact(|conn| {
            diesel::insert_into(peers::table)
                .values(new_post)
                .returning(PeerDb::as_returning()) // Return the inserted peer
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    // Adapt the database representation to the application's domain model
    Ok(adapt_peer_db_to_peer(res))
}

// Function to retrieve a peer from the database by its ID
pub async fn read(
    pool: &deadpool_diesel::postgres::Pool,
    id: Uuid,
) -> Result<PeerModel, InfraError> {
    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    // Query the 'peers' table to retrieve the peer by its ID
    let res = conn
        .interact(move |conn| {
            peers::table
                .filter(peers::id.eq(id))
                .select(PeerDb::as_select()) // Select the peer
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    // Adapt the database representation to the application's domain model
    Ok(adapt_peer_db_to_peer(res))
}

// Function to retrieve a list of peers from the database with optional filtering
pub async fn get_all(
    pool: &deadpool_diesel::postgres::Pool,
    filter: PeersFilter,
) -> Result<Vec<PeerModel>, InfraError> {
    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    // Build a dynamic query for retrieving peers
    let res = conn
        .interact(move |conn| {
            let mut query = peers::table.into_boxed::<diesel::pg::Pg>();

            // Apply filtering conditions if provided
            if filter.enabled {
                query = query.filter(peers::enabled.eq(filter.enabled));
            }

            if let Some(name_contains) = filter.name_contains {
                query = query.filter(peers::name.ilike(format!("%{name_contains}%")));
            }

            // Select the peers matching the query
            query.select(PeerDb::as_select()).load::<PeerDb>(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    // Adapt the database representations to the application's domain models
    let peers: Vec<PeerModel> = res.into_iter().map(adapt_peer_db_to_peer).collect();

    Ok(peers)
}

pub async fn update_peer(
    pool: &deadpool_diesel::postgres::Pool,
    peer_id: Uuid,
    update_peer: UpdatePeerRequest,
) -> Result<PeerModel, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(move |conn| {
            diesel::update(peers::table.filter(peers::id.eq(peer_id)))
                .set(peers::name.eq(update_peer.name))
                .returning(PeerDb::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;
    Ok(adapt_peer_db_to_peer(res))
}

// Function to adapt a database representation of a peer to the application's domain model
fn adapt_peer_db_to_peer(post_db: PeerDb) -> PeerModel {
    PeerModel {
        id: post_db.id,
        enabled: post_db.enabled,
        name: post_db.name,
        allowed_ips: post_db.allowed_ips,
        persistent_keepalive: post_db.persistent_keepalive as usize,
        preshared_key: post_db.preshared_key,
        public_key: post_db.public_key,
        private_key: post_db.private_key,
        transfer_tx: post_db.transfer_tx as usize,
        transfer_rx: post_db.transfer_rx as usize,
        last_handshake_at: post_db.last_handshake_at,
        endpoint_addr: post_db.endpoint_addr,
        address: post_db.address,
        if_pubkey: post_db.if_pubkey,
        created_at: post_db.created_at,
        updated_at: post_db.updated_at,
    }
}
