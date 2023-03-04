use diesel::{Insertable, Queryable};
use parcel_common::api_types;

use crate::db::schema::mission_dynamic_mission_infos;

#[derive(Debug, Queryable)]
pub struct DynamicMissionInfo {
    pub mission_id: String,
    pub client_name_hash: i32,
    pub reward_name_hash: i32,
}

impl DynamicMissionInfo {
    pub fn into_api_type(self) -> api_types::mission::DynamicMissionInfo {
        api_types::mission::DynamicMissionInfo {
            client_name_hash: self.client_name_hash,
            reward_name_hash: self.reward_name_hash,
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = mission_dynamic_mission_infos, primary_key(mission_id))]
pub struct NewDynamicMissionInfo<'a> {
    pub mission_id: &'a str,
    pub client_name_hash: i32,
    pub reward_name_hash: i32,
}
