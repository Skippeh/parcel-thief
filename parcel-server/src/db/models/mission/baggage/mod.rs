pub mod ammo_info;

use diesel::{Identifiable, Insertable, Queryable};
use parcel_common::api_types::{self, IntoDsApiType};

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

impl IntoDsApiType for Baggage {
    type ApiType = api_types::mission::Baggage;

    /// Converts self into equivalent api type. Relational columns are set to None.
    fn into_ds_api_type(self) -> Self::ApiType {
        Self::ApiType {
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
#[diesel(table_name = mission_baggages)]
pub struct NewBaggage<'a> {
    pub mission_id: &'a str,
    pub amount: i32,
    pub name_hash: i32,
    pub user_index: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub is_returned: bool,
}
