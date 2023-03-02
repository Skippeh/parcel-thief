use diesel::{
    backend::Backend, deserialize::FromSql, serialize::ToSql, sql_types::Int2, AsExpression,
    FromSqlRow, Insertable, Queryable,
};

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
    fn from_sql(bytes: diesel::backend::RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
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

#[derive(Debug, Insertable)]
#[diesel(table_name = mission_dynamic_location_infos, primary_key(id))]
pub struct NewDynamicLocationInfo<'a> {
    pub id: i64,
    pub mission_id: &'a str,
    #[diesel(column_name = type_)]
    pub ty: InfoType,
    pub location_id: &'a str,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
