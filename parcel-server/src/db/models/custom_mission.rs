use chrono::NaiveDateTime;
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::ToSql,
    sql_types::SmallInt,
    Insertable, Queryable,
};

use crate::db::schema::{custom_mission_collection_cargo, custom_mission_rewards, custom_missions};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, FromSqlRow, AsExpression)]
#[diesel(sql_type = SmallInt)]
#[repr(i16)]
pub enum CustomMissionType {
    Delivery,
    Collection,
    Recovery,
}

impl<DB> ToSql<SmallInt, DB> for CustomMissionType
where
    DB: Backend,
    i16: ToSql<SmallInt, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        match self {
            CustomMissionType::Delivery => 0i16.to_sql(out),
            CustomMissionType::Collection => 1i16.to_sql(out),
            CustomMissionType::Recovery => 2i16.to_sql(out),
        }
    }
}

impl<DB> FromSql<SmallInt, DB> for CustomMissionType
where
    DB: Backend,
    i16: FromSql<SmallInt, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match i16::from_sql(bytes)? {
            0 => Ok(CustomMissionType::Delivery),
            1 => Ok(CustomMissionType::Collection),
            2 => Ok(CustomMissionType::Recovery),
            other => Err(format!("Unknown CustomMissionType variant: {}", other).into()),
        }
    }
}

#[derive(Debug, Queryable)]
pub struct CustomMission {
    pub id: i64,
    pub creator_id: i64,
    #[diesel(column_name = "type_")]
    pub ty: CustomMissionType,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = custom_missions)]
pub struct NewCustomMission<'a> {
    pub creator_id: i64,
    #[diesel(column_name = "type_")]
    pub ty: CustomMissionType,
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
