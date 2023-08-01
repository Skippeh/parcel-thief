use diesel::{AsChangeset, Insertable, Queryable};
use parcel_common::api_types::{self, IntoDsApiType};

use crate::db::schema::qpid_object_stone_infos;

#[derive(Debug, Queryable)]
pub struct StoneInfo {
    pub object_id: String,
    pub resting_count: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = qpid_object_stone_infos, primary_key(object_id))]
pub struct NewStoneInfo<'a> {
    pub object_id: &'a str,
    pub resting_count: i32,
}

impl IntoDsApiType for StoneInfo {
    type ApiType = api_types::object::StoneInfo;

    fn into_ds_api_type(self) -> Self::ApiType {
        Self::ApiType {
            resting_count: self.resting_count,
        }
    }
}

#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = qpid_object_stone_infos)]
pub struct ChangeStoneInfo {
    pub resting_count: Option<i32>,
}

impl From<&api_types::object::StoneInfo> for ChangeStoneInfo {
    fn from(value: &api_types::object::StoneInfo) -> Self {
        Self {
            resting_count: Some(value.resting_count),
        }
    }
}
