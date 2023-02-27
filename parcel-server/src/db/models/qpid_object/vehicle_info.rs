use diesel::{Insertable, Queryable};
use parcel_common::api_types;

use crate::db::schema::qpid_object_vehicle_infos;

#[derive(Debug, Queryable)]
pub struct VehicleInfo {
    pub object_id: String,
    pub location_id: i32,
    pub dynamic_location_id: String,
    pub current_qpid_id: i32,
    pub is_parking: bool,
    pub is_lost: bool,
    pub is_race: bool,
    pub customize_type: i32,
    pub customize_color: i32,
    pub new_pos_x: i32,
    pub new_pos_y: i32,
    pub new_pos_z: i32,
    pub new_rot_x: i32,
    pub new_rot_y: i32,
    pub new_rot_z: i32,
    pub exponent: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = qpid_object_vehicle_infos, primary_key(object_id))]
pub struct NewVehicleInfo<'a> {
    pub object_id: &'a str,
    pub location_id: i32,
    pub dynamic_location_id: &'a str,
    pub current_qpid_id: i32,
    pub is_parking: bool,
    pub is_lost: bool,
    pub is_race: bool,
    pub customize_type: i32,
    pub customize_color: i32,
    pub new_pos_x: i32,
    pub new_pos_y: i32,
    pub new_pos_z: i32,
    pub new_rot_x: i32,
    pub new_rot_y: i32,
    pub new_rot_z: i32,
    pub exponent: i32,
}

impl VehicleInfo {
    pub fn into_api_type(self) -> api_types::object::VehicleInfo {
        api_types::object::VehicleInfo {
            location_id: self.location_id,
            dynamic_location_id: self.dynamic_location_id,
            current_qpid_id: self.current_qpid_id,
            is_parking: self.is_parking,
            is_lost: self.is_lost,
            is_race: self.is_race,
            customize_type: self.customize_type,
            customize_color: self.customize_color,
            new_position: (self.new_pos_x, self.new_pos_y, self.new_pos_z),
            new_rotation: (self.new_rot_x, self.new_rot_y, self.new_rot_z),
            exponent: self.exponent,
        }
    }
}
