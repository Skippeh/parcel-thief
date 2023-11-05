use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

use crate::db::schema::{custom_mission_collection_cargo, custom_mission_rewards, custom_missions};

#[derive(Debug, Queryable)]
pub struct CustomMission {
    pub id: i64,
    pub creator_id: String,
    #[diesel(column_name = "type")]
    pub ty: i16,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = custom_missions)]
pub struct NewCustomMission<'a> {
    pub creator_id: i64,
    #[diesel(column_name = "type_")]
    pub ty: i16,
    pub created_at: Option<&'a NaiveDateTime>,
}

#[derive(Debug, Queryable)]
pub struct CustomMissionReward {
    pub id: i64,
    pub custom_mission_id: i64,
    pub item_hash: i32,
    pub amount: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = custom_mission_rewards)]
pub struct NewCustomMissionReward {
    pub custom_mission_id: i64,
    pub item_hash: i32,
    pub amount: i32,
}

#[derive(Debug, Queryable)]
pub struct CustomMissionCollectionCargo {
    pub id: i64,
    pub custom_mission_id: i64,
    pub item_hash: i32,
    pub target_amount: i32,
    pub current_amount: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = custom_mission_collection_cargo)]
pub struct NewCustomMissionCollectionCargo {
    pub custom_mission_id: i64,
    pub item_hash: i32,
    pub target_amount: i32,
    pub current_amount: i32,
}
