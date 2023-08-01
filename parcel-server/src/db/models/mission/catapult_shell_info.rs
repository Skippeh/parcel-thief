use diesel::{AsChangeset, Insertable, Queryable};
use parcel_common::api_types::{self, IntoDsApiType};

use crate::db::schema::mission_catapult_shell_infos;

#[derive(Debug, Queryable)]
pub struct CatapultShellInfo {
    pub mission_id: String,
    pub local_id: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl IntoDsApiType for CatapultShellInfo {
    type ApiType = api_types::mission::CatapultShellInfo;

    fn into_ds_api_type(self) -> Self::ApiType {
        Self::ApiType {
            mission_id: self.mission_id,
            local_id: self.local_id,
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = mission_catapult_shell_infos, primary_key(mission_id))]
pub struct NewCatapultShellInfo<'a> {
    pub mission_id: &'a str,
    pub local_id: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = mission_catapult_shell_infos)]
pub struct ChangeCatapultShellInfo {
    pub local_id: Option<i32>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub z: Option<i32>,
}

impl From<&api_types::mission::CatapultShellInfo> for ChangeCatapultShellInfo {
    fn from(value: &api_types::mission::CatapultShellInfo) -> Self {
        Self {
            local_id: Some(value.local_id),
            x: Some(value.x),
            y: Some(value.y),
            z: Some(value.z),
        }
    }
}
