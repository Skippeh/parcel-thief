use diesel::{Insertable, Queryable};

use crate::db::schema::mission_dynamic_mission_infos;

#[derive(Debug, Queryable)]
pub struct DynamicMissionInfo {
    pub mission_id: String,
    pub client_name_hash: i32,
    pub reward_name_hash: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = mission_dynamic_mission_infos, primary_key(mission_id))]
pub struct NewDynamicMissionInfo<'a> {
    pub mission_id: &'a str,
    pub client_name_hash: i32,
    pub reward_name_hash: i32,
}
