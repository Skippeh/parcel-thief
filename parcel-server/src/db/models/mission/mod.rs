pub mod baggage;
pub mod dynamic_location_info;
pub mod dynamic_mission_info;
pub mod relation;
pub mod supply_info;

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use parcel_common::api_types::{
    self,
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
    pub qpid_delivered_location: i32,
    pub mission_static_id: i64,
    pub mission_type: MissionType,
    pub online_mission_type: OnlineMissionType,
    pub progress_state: ProgressState,
    pub registered_time: NaiveDateTime,
    pub expiration_time: NaiveDateTime,
}

impl Mission {
    /// Converts self into the mission api type. All relational columns will be set to None, or empty vec.
    pub fn into_api_type(self) -> api_types::mission::Mission {
        api_types::mission::Mission {
            area_hash: self.area_id,
            creator_account_id: self.creator_id,
            worker_account_id: self.worker_id,
            qpid_id: self.qpid_id,
            qpid_start_location: self.qpid_start_location,
            qpid_end_location: self.qpid_end_location,
            qpid_delivered_location: self.qpid_delivered_location,
            online_id: self.id,
            mission_static_id: self.mission_static_id,
            mission_type: self.mission_type,
            online_mission_type: self.online_mission_type,
            progress_state: self.progress_state,
            relations: Vec::default(),
            registered_time: self.registered_time.timestamp_millis(),
            expiration_time: self.expiration_time.timestamp_millis(),
            supply_info: None,
            dynamic_start_info: None,
            dynamic_end_info: None,
            dynamic_delivered_info: None,
            dynamic_mission_info: None,
            baggages: Vec::default(),
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = missions, primary_key(id))]
pub struct NewMission<'a> {
    pub id: &'a str,
    pub area_id: AreaHash,
    pub creator_id: &'a str,
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
