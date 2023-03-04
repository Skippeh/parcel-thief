use diesel::{Insertable, Queryable};
use parcel_common::api_types;

use crate::db::schema::mission_baggage_ammo_infos;

#[derive(Debug, Queryable)]
pub struct AmmoInfo {
    pub baggage_id: i64,
    pub ammo_id: String,
    pub clip_count: i16,
    pub count: i16,
}

impl AmmoInfo {
    pub fn into_api_type(self) -> api_types::mission::AmmoInfo {
        api_types::mission::AmmoInfo {
            ammo_id: self.ammo_id,
            clip_count: self.clip_count,
            count: self.count,
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = mission_baggage_ammo_infos, primary_key(baggage_id))]
pub struct NewAmmoInfo<'a> {
    pub baggage_id: i64,
    pub ammo_id: &'a str,
    pub clip_count: i16,
    pub count: i16,
}
