use diesel::{
    backend::Backend, deserialize::FromSql, serialize::ToSql, sql_types::Integer, AsExpression,
    FromSqlRow,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Debug,
    Serialize_repr,
    Deserialize_repr,
    Clone,
    Copy,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    FromSqlRow,
    AsExpression,
)]
#[repr(u32)]
#[diesel(sql_type = Integer)]
pub enum AreaHash {
    #[serde(rename = "5319")]
    EasternRegion = 5319,
    #[serde(rename = "22123")]
    CentralRegion = 22123,
    #[serde(rename = "21299")]
    WesternRegion = 21299,
}

impl<DB> ToSql<Integer, DB> for AreaHash
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        match self {
            AreaHash::EasternRegion => 0.to_sql(out),
            AreaHash::CentralRegion => 1.to_sql(out),
            AreaHash::WesternRegion => 2.to_sql(out),
        }
    }
}

impl<DB> FromSql<Integer, DB> for AreaHash
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(AreaHash::EasternRegion),
            1 => Ok(AreaHash::CentralRegion),
            2 => Ok(AreaHash::WesternRegion),
            other => Err(format!("Unknown AreaHash variant: {}", other).into()),
        }
    }
}
