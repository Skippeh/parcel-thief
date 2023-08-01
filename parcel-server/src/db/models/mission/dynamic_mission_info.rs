use diesel::{Insertable, Queryable};
use parcel_common::api_types::{self, IntoDsApiType};

use crate::db::schema::mission_dynamic_mission_infos;

#[derive(Debug, Queryable)]
pub struct DynamicMissionInfo {
    pub mission_id: String,
    pub client_name_hash: i32,
    pub reward_name_hash: i32,
}

impl IntoDsApiType for DynamicMissionInfo {
    type ApiType = api_types::mission::DynamicMissionInfo;

    fn into_ds_api_type(self) -> Self::ApiType {
        Self::ApiType {
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
