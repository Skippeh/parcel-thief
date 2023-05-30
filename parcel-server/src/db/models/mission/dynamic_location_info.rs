use diesel::{
    backend::Backend, deserialize::FromSql, serialize::ToSql, sql_types::Int2, AsChangeset,
    AsExpression, FromSqlRow, Insertable, Queryable,
};
use parcel_common::api_types;

use crate::db::schema::mission_dynamic_location_infos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsExpression, FromSqlRow)]
#[diesel(sql_type = Int2)]
#[repr(i16)]
pub enum InfoType {
    Start,
    End,
    Delivered,
}

impl<DB> ToSql<Int2, DB> for InfoType
where
    DB: Backend,
    i16: ToSql<Int2, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        match self {
            Self::Start => 0.to_sql(out),
            Self::End => 1.to_sql(out),
            Self::Delivered => 2.to_sql(out),
        }
    }
}

impl<DB> FromSql<Int2, DB> for InfoType
where
    DB: Backend,
    i16: FromSql<Int2, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match i16::from_sql(bytes)? {
            0 => Ok(Self::Start),
            1 => Ok(Self::End),
            2 => Ok(Self::Delivered),
            other => Err(format!("Unknown InfoType variant: {}", other).into()),
        }
    }
}

#[derive(Debug, Queryable)]
pub struct DynamicLocationInfo {
    pub id: i64,
    pub mission_id: String,
    pub ty: InfoType,
    pub location_id: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl DynamicLocationInfo {
    pub fn into_api_type(self) -> api_types::mission::DynamicLocationInfo {
        api_types::mission::DynamicLocationInfo {
            location_object_id: self.location_id,
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = mission_dynamic_location_infos)]
pub struct NewDynamicLocationInfo<'a> {
    pub mission_id: &'a str,
    #[diesel(column_name = type_)]
    pub ty: InfoType,
    pub location_id: &'a str,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = mission_dynamic_location_infos)]
pub struct ChangeDynamicLocationInfo<'a> {
    pub location_id: Option<&'a str>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub z: Option<i32>,
}

impl<'a> From<&'a api_types::mission::DynamicLocationInfo> for ChangeDynamicLocationInfo<'a> {
    fn from(value: &'a api_types::mission::DynamicLocationInfo) -> Self {
        Self {
            location_id: Some(&value.location_object_id),
            x: Some(value.x),
            y: Some(value.y),
            z: Some(value.z),
        }
    }
}
