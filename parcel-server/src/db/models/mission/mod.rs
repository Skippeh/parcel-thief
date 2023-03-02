pub mod baggage;
pub mod dynamic_location_info;
pub mod dynamic_mission_info;
pub mod relation;
pub mod supply_info;

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use parcel_common::api_types::{
    area::AreaHash,
    mission::{MissionType, OnlineMissionType, ProgressState},
};

use crate::db::schema::missions;

#[derive(Debug, Queryable)]
pub struct Mission {
    pub id: String,
    pub area_id: AreaHash,
    pub creator_id: String,
    pub worker_id: Option<String>,
    pub qpid_id: i32,
    pub qpid_start_location: i32,
    pub qpid_end_location: i32,
    pub mission_static_id: i64,
    pub mission_type: MissionType,
    pub online_mission_type: OnlineMissionType,
    pub progress_state: ProgressState,
    pub registered_time: NaiveDateTime,
    pub expiration_time: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = missions, primary_key(id))]
pub struct NewMission<'a> {
    pub id: &'a str,
    pub area_id: AreaHash,
    pub creator_id: String,
    pub worker_id: Option<&'a str>,
    pub qpid_id: i32,
    pub qpid_start_location: i32,
    pub qpid_end_location: i32,
    pub mission_static_id: i64,
    pub mission_type: MissionType,
    pub online_mission_type: OnlineMissionType,
    pub progress_state: ProgressState,
    pub registered_time: &'a NaiveDateTime,
    pub expiration_time: &'a NaiveDateTime,
}
