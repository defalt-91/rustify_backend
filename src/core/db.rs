use deadpool_diesel::postgres::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");
// Function to run database migrations
pub async fn run_migrations(pool: &Pool) {
    let conn = pool.get().await.unwrap();
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
}
// use axum::async_trait;
// use axum::extract::{FromRef, FromRequestParts};
// use axum::http::request::Parts;
// use axum::http::StatusCode;
// use diesel::{PgConnection, r2d2::{self,PooledConnection,ConnectionManager}};
//
// pub type Pool = bb8::Pool<PooledConnection<PgConnection>>;
//
// pub struct DatabaseConnection(
//     PooledConnection<ConnectionManager<PgConnection>>,
// );
//
// #[async_trait]
// impl<S> FromRequestParts<S> for DatabaseConnection
//     where
//         S: Send + Sync,
//         Pool: FromRef<S>,
// {
//     type Rejection = (StatusCode, String);
//
//     async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let pool = Pool::from_ref(state);
//         let conn = pool.get_owned().await.map_err(internal_error)?;
//         Ok(Self(conn))
//     }
// }
//
// /// Utility function for mapping any error into a `500 Internal Server Error`
// /// response.
// fn internal_error<E>(err: E) -> (StatusCode, String)
//     where
//         E: std::error::Error,
// {
//     (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
// }
