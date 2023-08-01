use diesel::{Insertable, Queryable};
use parcel_common::api_types::{self, IntoDsApiType};

use crate::db::schema::mission_supply_infos;

#[derive(Debug, Queryable)]
pub struct SupplyInfo {
    pub mission_id: String,
    pub item_hash: i64,
    pub amount: i32,
}

impl IntoDsApiType for SupplyInfo {
    type ApiType = api_types::mission::SupplyInfo;

    fn into_ds_api_type(self) -> Self::ApiType {
        Self::ApiType {
            item_hash: self.item_hash,
            amount: self.amount,
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = mission_supply_infos, primary_key(mission_id))]
pub struct NewSupplyInfo<'a> {
    pub mission_id: &'a str,
    pub item_hash: i64,
    pub amount: i32,
}
