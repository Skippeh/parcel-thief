use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use parcel_common::api_types::IntoDsApiType;

use crate::db::schema::qpid_object_recycle_materials;

#[derive(Debug, Queryable)]
pub struct RecycleMaterials {
    pub id: i64,
    pub object_id: String,
    pub contributor_id: Option<String>,
    pub mats_0: i32,
    pub mats_1: i32,
    pub mats_2: i32,
    pub mats_3: i32,
    pub mats_4: i32,
    pub mats_5: i32,
    pub recycle_time: NaiveDateTime,
}

impl IntoDsApiType for RecycleMaterials {
    type ApiType = parcel_common::api_types::object::RecycleMaterials;

    fn into_ds_api_type(self) -> Self::ApiType {
        Self::ApiType {
            // None means the owner of the object is the contributor, and the api should return an empty string if this is the case
            contributor_account_id: self.contributor_id.unwrap_or_default(),
            materials: [
                self.mats_0,
                self.mats_1,
                self.mats_2,
                self.mats_3,
                self.mats_4,
                self.mats_5,
            ],
            recycle_time: self.recycle_time.and_utc().timestamp_millis(),
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = qpid_object_recycle_materials)]
pub struct NewRecycleMaterials<'a> {
    pub object_id: &'a str,
    pub contributor_id: Option<&'a str>,
    pub mats_0: i32,
    pub mats_1: i32,
    pub mats_2: i32,
    pub mats_3: i32,
    pub mats_4: i32,
    pub mats_5: i32,
    pub recycle_time: &'a NaiveDateTime,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = qpid_object_recycle_materials)]
pub struct ChangeRecycleMaterials<'a> {
    pub mats_0: i32,
    pub mats_1: i32,
    pub mats_2: i32,
    pub mats_3: i32,
    pub mats_4: i32,
    pub mats_5: i32,
    pub recycle_time: &'a NaiveDateTime,
}

impl<'a> From<&'a NewRecycleMaterials<'a>> for ChangeRecycleMaterials<'a> {
    fn from(mats: &'a NewRecycleMaterials) -> Self {
        Self {
            mats_0: mats.mats_0,
            mats_1: mats.mats_1,
            mats_2: mats.mats_2,
            mats_3: mats.mats_3,
            mats_4: mats.mats_4,
            mats_5: mats.mats_5,
            recycle_time: mats.recycle_time,
        }
    }
}
