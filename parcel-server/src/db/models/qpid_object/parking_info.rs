use diesel::{AsChangeset, Insertable, Queryable};
use parcel_common::api_types::{self, IntoDsApiType};

use crate::db::schema::qpid_object_parking_infos;

#[derive(Debug, Queryable)]
pub struct ParkingInfo {
    pub object_id: String,
    pub location_id: i32,
    pub dynamic_location_id: String,
    pub current_qpid_id: i32,
    pub is_parking: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = qpid_object_parking_infos, primary_key(object_id))]
pub struct NewParkingInfo<'a> {
    pub object_id: &'a str,
    pub location_id: i32,
    pub dynamic_location_id: &'a str,
    pub current_qpid_id: i32,
    pub is_parking: bool,
}

impl IntoDsApiType for ParkingInfo {
    type ApiType = api_types::object::ParkingInfo;

    fn into_ds_api_type(self) -> Self::ApiType {
        Self::ApiType {
            location_id: self.location_id,
            dynamic_location_id: self.dynamic_location_id,
            current_qpid_id: self.current_qpid_id,
            is_parking: self.is_parking,
        }
    }
}

#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = qpid_object_parking_infos)]
pub struct ChangeParkingInfo<'a> {
    pub location_id: Option<i32>,
    pub dynamic_location_id: Option<&'a str>,
    pub current_qpid_id: Option<i32>,
    pub is_parking: Option<bool>,
}

impl<'a> From<&'a api_types::object::ParkingInfo> for ChangeParkingInfo<'a> {
    fn from(value: &'a api_types::object::ParkingInfo) -> Self {
        Self {
            location_id: Some(value.location_id),
            dynamic_location_id: Some(&value.dynamic_location_id),
            current_qpid_id: Some(value.current_qpid_id),
            is_parking: Some(value.is_parking),
        }
    }
}
