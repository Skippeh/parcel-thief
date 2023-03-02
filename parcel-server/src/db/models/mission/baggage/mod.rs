pub mod ammo_info;

use diesel::{Insertable, Queryable};

use crate::db::schema::mission_baggages;

#[derive(Debug, Queryable)]
pub struct Baggage {
    pub id: i64,
    pub mission_id: String,
    pub amount: i32,
    pub name_hash: i32,
    pub user_index: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub is_returned: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = mission_baggages, primary_key(id))]
pub struct NewBaggage<'a> {
    pub id: i64,
    pub mission_id: &'a str,
    pub amount: i32,
    pub name_hash: i32,
    pub user_index: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub is_returned: bool,
}
