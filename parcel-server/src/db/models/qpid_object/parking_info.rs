use diesel::{Insertable, Queryable};
use parcel_common::api_types;

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

impl ParkingInfo {
    pub fn into_api_type(self) -> api_types::object::ParkingInfo {
        api_types::object::ParkingInfo {
            location_id: self.location_id,
            dynamic_location_id: self.dynamic_location_id,
            current_qpid_id: self.current_qpid_id,
            is_parking: self.is_parking,
        }
    }
}
