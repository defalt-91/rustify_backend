use diesel::prelude::*;
use crate::infra::db::schema::wg_if;
#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = wg_if)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Interface{
    pub id:i32,
    pub name :String,
    pub pubkey :String,
    pub privkey :String,
    pub address :String,
    pub port :i32,
    pub mtu :Option<i32>,
    pub fwmark: Option<i32>
}



