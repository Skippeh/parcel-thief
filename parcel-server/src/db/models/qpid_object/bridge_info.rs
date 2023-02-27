use diesel::{Insertable, Queryable};
use parcel_common::api_types;

use crate::db::schema::qpid_object_bridge_infos;

#[derive(Debug, Queryable)]
pub struct BridgeInfo {
    pub object_id: String,
    pub angle: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = qpid_object_bridge_infos, primary_key(object_id))]
pub struct NewBridgeInfo<'a> {
    pub object_id: &'a str,
    pub angle: i32,
}

impl BridgeInfo {
    pub fn into_api_type(self) -> api_types::object::BridgeInfo {
        api_types::object::BridgeInfo { angle: self.angle }
    }
}
