use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use parcel_common::api_types::IntoDsApiType;

use crate::db::schema::qpid_object_construction_materials;

#[derive(Debug, Queryable)]
pub struct ConstructionMaterials {
    pub id: i64,
    pub object_id: String,
    pub contributor_id: Option<String>,
    pub mats_0: i32,
    pub mats_1: i32,
    pub mats_2: i32,
    pub mats_3: i32,
    pub mats_4: i32,
    pub mats_5: i32,
    pub repair_0: i32,
    pub repair_1: i32,
    pub repair_2: i32,
    pub repair_3: i32,
    pub repair_4: i32,
    pub repair_5: i32,
    pub contribute_time: NaiveDateTime,
}

impl IntoDsApiType for ConstructionMaterials {
    type ApiType = parcel_common::api_types::object::ConstructionMaterials;

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
            materials_to_repair: [
                self.repair_0,
                self.repair_1,
                self.repair_2,
                self.repair_3,
                self.repair_4,
                self.repair_5,
            ],
            contribute_time: self.contribute_time.timestamp_millis(),
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = qpid_object_construction_materials)]
pub struct NewConstructionMaterials<'a> {
    pub object_id: &'a str,
    pub contributor_id: Option<&'a str>,
    pub mats_0: i32,
    pub mats_1: i32,
    pub mats_2: i32,
    pub mats_3: i32,
    pub mats_4: i32,
    pub mats_5: i32,
    pub repair_0: i32,
    pub repair_1: i32,
    pub repair_2: i32,
    pub repair_3: i32,
    pub repair_4: i32,
    pub repair_5: i32,
    pub contribute_time: &'a NaiveDateTime,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = qpid_object_construction_materials)]
pub struct ChangeConstructionMaterials<'a> {
    pub mats_0: i32,
    pub mats_1: i32,
    pub mats_2: i32,
    pub mats_3: i32,
    pub mats_4: i32,
    pub mats_5: i32,
    pub repair_0: i32,
    pub repair_1: i32,
    pub repair_2: i32,
    pub repair_3: i32,
    pub repair_4: i32,
    pub repair_5: i32,
    pub contribute_time: &'a NaiveDateTime,
}

impl<'a> From<&'a NewConstructionMaterials<'a>> for ChangeConstructionMaterials<'a> {
    fn from(mats: &'a NewConstructionMaterials) -> Self {
        Self {
            mats_0: mats.mats_0,
            mats_1: mats.mats_1,
            mats_2: mats.mats_2,
            mats_3: mats.mats_3,
            mats_4: mats.mats_4,
            mats_5: mats.mats_5,
            repair_0: mats.repair_0,
            repair_1: mats.repair_1,
            repair_2: mats.repair_2,
            repair_3: mats.repair_3,
            repair_4: mats.repair_4,
            repair_5: mats.repair_5,
            contribute_time: mats.contribute_time,
        }
    }
}
