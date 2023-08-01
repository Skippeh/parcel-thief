use diesel::{Insertable, Queryable};
use parcel_common::api_types::{self, IntoDsApiType};

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

impl IntoDsApiType for BridgeInfo {
    type ApiType = api_types::object::BridgeInfo;

    fn into_ds_api_type(self) -> Self::ApiType {
        Self::ApiType { angle: self.angle }
    }
}
