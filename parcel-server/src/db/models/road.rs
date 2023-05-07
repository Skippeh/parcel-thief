use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use parcel_common::api_types::area::AreaHash;

use crate::db::schema::{road_data, road_via_qpids, roads};

#[derive(Debug, Queryable)]
pub struct Road {
    pub id: String,
    pub area_hash: AreaHash,
    pub creator_id: String,
    pub qpid_start_id: i32,
    pub qpid_end_id: i32,
    pub location_start_id: i32,
    pub location_end_id: i32,
    pub max_height_difference: i32,
    pub length: i32,
    pub created_at: NaiveDateTime,
    pub data_version: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = roads)]
pub struct NewRoad<'a> {
    pub id: &'a str,
    pub area_hash: AreaHash,
    pub creator_id: &'a str,
    pub qpid_start_id: i32,
    pub qpid_end_id: i32,
    pub location_start_id: i32,
    pub location_end_id: i32,
    pub max_height_difference: i32,
    pub length: i32,
    pub created_at: &'a NaiveDateTime,
    pub data_version: i32,
}

#[derive(Debug, Queryable)]
pub struct RoadViaQpid {
    pub road_id: String,
    pub qpid_id: i32,
    pub sort_order: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = road_via_qpids)]
pub struct NewRoadViaQpid<'a> {
    pub road_id: &'a str,
    pub qpid_id: i32,
    pub sort_order: i32,
}

#[derive(Debug, Queryable)]
pub struct RoadData {
    pub road_id: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = road_data)]
pub struct NewRoadData<'a> {
    pub road_id: &'a str,
    pub data: &'a [u8],
}
