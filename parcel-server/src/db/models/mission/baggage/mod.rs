pub mod ammo_info;

use diesel::{Identifiable, Insertable, Queryable};
use parcel_common::api_types;

use crate::db::schema::mission_baggages;

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = mission_baggages)]
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

impl Baggage {
    /// Converts self into equivalent api type. Relational columns are set to None.
    pub fn into_api_type(self) -> api_types::mission::Baggage {
        api_types::mission::Baggage {
            amount: self.amount,
            name_hash: self.name_hash,
            user_index: self.user_index,
            x: self.x,
            y: self.y,
            z: self.z,
            is_returned: self.is_returned,
            ammo_info: None,
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = mission_baggages, primary_key(id))]
pub struct NewBaggage<'a> {
    pub id: Option<i64>,
    pub mission_id: &'a str,
    pub amount: i32,
    pub name_hash: i32,
    pub user_index: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub is_returned: bool,
}
