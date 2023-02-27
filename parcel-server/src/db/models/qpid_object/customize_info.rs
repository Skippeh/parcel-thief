use diesel::{Insertable, Queryable};
use parcel_common::api_types;

use crate::db::schema::qpid_object_customize_infos;

#[derive(Debug, Queryable)]
pub struct CustomizeInfo {
    pub object_id: String,
    pub customize_param: i32,
    pub customize_color: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = qpid_object_customize_infos, primary_key(object_id))]
pub struct NewCustomizeInfo<'a> {
    pub object_id: &'a str,
    pub customize_param: i32,
    pub customize_color: i32,
}

impl CustomizeInfo {
    pub fn into_api_type(self) -> api_types::object::CustomizeInfo {
        api_types::object::CustomizeInfo {
            customize_param: self.customize_param as u32,
            customize_color: self.customize_color as u32,
        }
    }
}
