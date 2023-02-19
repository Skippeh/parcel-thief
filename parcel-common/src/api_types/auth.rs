use diesel::{
    backend::Backend, deserialize::FromSql, serialize::ToSql, sql_types::Integer, AsExpression,
    FromSqlRow,
};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserInfo {
    pub provider: Provider,
    pub id: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SessionProperties {
    /// The epoch time in seconds of the last login (seems to always be same as the current time)
    #[serde(rename = "ll", deserialize_with = "deserialize_i64_from_string")]
    pub last_login: i64,
}

fn deserialize_i64_from_string<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    str.parse::<i64>()
        .map_err(|_| serde::de::Error::custom("unexpected value, not i64"))
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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, FromSqlRow, AsExpression)]
#[diesel(sql_type = Integer)]
pub enum Provider {
    #[serde(rename = "steam")]
    Steam = 0,
    #[serde(rename = "epic")]
    Epic = 1,
}

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
            Provider::Steam => 0.to_sql(out),
            Provider::Epic => 1.to_sql(out),
        }
    }
}

impl<DB> FromSql<Integer, DB> for Provider
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(Provider::Steam),
            1 => Ok(Provider::Epic),
            other => Err(format!("Unknown Provider variant: {}", other).into()),
        }
    }
}
