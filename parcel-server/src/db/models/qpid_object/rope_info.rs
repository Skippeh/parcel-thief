use diesel::{Insertable, Queryable};
use parcel_common::api_types;

use crate::db::schema::qpid_object_rope_infos;

#[derive(Debug, Queryable)]
pub struct RopeInfo {
    pub object_id: String,
    pub pitch: i32,
    pub heading: i32,
    pub len: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = qpid_object_rope_infos, primary_key(object_id))]
pub struct NewRopeInfo<'a> {
    pub object_id: &'a str,
    pub pitch: i32,
    pub heading: i32,
    pub len: i32,
}

impl RopeInfo {
    pub fn into_api_type(self) -> api_types::object::RopeInfo {
        api_types::object::RopeInfo {
            pitch: self.pitch,
            heading: self.heading,
            length: self.len,
        }
    }
}
