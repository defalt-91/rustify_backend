use crate::domain::models::user::UserModel;
use crate::infra::db::schema::users;
use crate::infra::errors::{adapt_infra_error, InfraError};
use deadpool_diesel::postgres::Pool;
use diesel::{
    ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDb {
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUserDb {
    pub username: String,
    pub hashed_password: String,
}

pub async fn create(pool: &Pool, new_user: NewUserDb) -> Result<UserModel, InfraError> {
    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(UserDb::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_user_db_to_user(res))
}
pub async fn read(pool: &Pool, id: Uuid) -> Result<UserModel, InfraError> {
    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(move |conn| {
            users::table
                .filter(users::id.eq(id))
                .select(UserDb::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    // Adapt the database representation to the application's domain model
    Ok(adapt_user_db_to_user(res))
}

pub async fn read_by_username(pool: &Pool, username: String) -> Result<UserModel, InfraError> {
    // Get a database connection from the pool and handle any potential errors
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(move |conn| {
            users::table
                .filter(users::username.eq(&username))
                .select(UserDb::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    // Adapt the database representation to the application's domain model
    Ok(adapt_user_db_to_user(res))
}

// Function to adapt a database representation of a user to the application's domain model
fn adapt_user_db_to_user(db_user: UserDb) -> UserModel {
    UserModel {
        id: db_user.id,
        username: db_user.username,
        hashed_password: db_user.hashed_password,
    }
}
