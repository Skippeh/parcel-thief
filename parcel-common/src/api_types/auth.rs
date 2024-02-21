use std::fmt::Display;

#[cfg(feature = "diesel")]
use diesel::{
    backend::Backend, deserialize::FromSql, serialize::ToSql, sql_types::Integer, AsExpression,
    FromSqlRow,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ts")]
use typescript_type_def::TypeDef;

use crate::serde_util::deserialize_i64_from_string_or_i64;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserInfo {
    pub provider: Provider,
    pub id: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SessionProperties {
    /// The epoch time in seconds of the last login (seems to always be same as the current time)
    #[serde(rename = "ll", deserialize_with = "deserialize_i64_from_string_or_i64")]
    pub last_login: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SessionInfo {
    pub token: String,
    pub gateway: String,
    pub properties: SessionProperties,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthResponse {
    pub user: UserInfo,
    pub session: SessionInfo,
}

#[repr(i32)]
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "diesel", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Integer))]
#[cfg_attr(feature = "ts", derive(TypeDef))]
pub enum Provider {
    #[serde(rename = "identity")]
    Server = -1,
    #[serde(rename = "steam")]
    Steam = 0,
    #[serde(rename = "epic")]
    Epic = 1,
}

impl Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::Server => write!(f, "Server"),
            Provider::Steam => write!(f, "Steam"),
            Provider::Epic => write!(f, "Epic"),
        }
    }
}

#[cfg(feature = "diesel")]
impl<DB> ToSql<Integer, DB> for Provider
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        match self {
            Provider::Server => (-1).to_sql(out),
            Provider::Steam => 0.to_sql(out),
            Provider::Epic => 1.to_sql(out),
        }
    }
}

#[cfg(feature = "diesel")]
impl<DB> FromSql<Integer, DB> for Provider
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            -1 => Ok(Provider::Server),
            0 => Ok(Provider::Steam),
            1 => Ok(Provider::Epic),
            other => Err(format!("Unknown Provider variant: {}", other).into()),
        }
    }
}
