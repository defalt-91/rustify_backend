use diesel::{
    ExpressionMethods, Insertable, PgTextExpressionMethods, QueryDsl, Queryable, RunQueryDsl,
    Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


use crate::domain::models::peer::PeerModel;
use crate::infra::db::schema::peers;
use crate::infra::errors::{adapt_infra_error, InfraError};

// Define a struct representing the database schema for peers
#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = peers)] // Use the 'peers' table
#[diesel(check_for_backend(diesel::pg::Pg))] // Check compatibility with PostgreSQL
pub struct PeerDb {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}

// Define a struct for inserting new peers into the database
#[derive(Deserialize, Insertable)]
#[diesel(table_name = peers)] // Use the 'peers' table
pub struct NewPeerDb {
    pub title: String,
    pub body: String,
    pub published: bool,
}

// Define a struct for filtering peers
#[derive(Deserialize)]
pub struct PeersFilter {
    published: Option<bool>,
    title_contains: Option<String>,
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
            if let Some(published) = filter.published {
                query = query.filter(peers::published.eq(published));
            }

            if let Some(title_contains) = filter.title_contains {
                query = query.filter(peers::title.ilike(format!("%{}%", title_contains)));
            }

            // Select the peers matching the query
            query.select(PeerDb::as_select()).load::<PeerDb>(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    // Adapt the database representations to the application's domain models
    let peers: Vec<PeerModel> = res
        .into_iter()
        .map(|post_db| adapt_peer_db_to_peer(post_db))
        .collect();

    Ok(peers)
}

// Function to adapt a database representation of a peer to the application's domain model
fn adapt_peer_db_to_peer(post_db: PeerDb) -> PeerModel {
    PeerModel {
        id: post_db.id,
        title: post_db.title,
        body: post_db.body,
        published: post_db.published,
    }
}