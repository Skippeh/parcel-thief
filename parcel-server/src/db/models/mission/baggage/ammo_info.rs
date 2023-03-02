use diesel::{Insertable, Queryable};

use crate::db::schema::mission_baggage_ammo_infos;

#[derive(Debug, Queryable)]
pub struct AmmoInfo {
    pub baggage_id: i64,
    pub ammo_id: String,
    pub clip_count: i16,
    pub count: i16,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = mission_baggage_ammo_infos, primary_key(baggage_id))]
pub struct NewAmmoInfo<'a> {
    pub baggage_id: i64,
    pub ammo_id: &'a str,
    pub clip_count: i16,
    pub count: i16,
}
