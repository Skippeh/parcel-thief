use diesel::{Insertable, Queryable};
use parcel_common::api_types;

use crate::db::schema::wasted_baggages;

#[derive(Debug, Queryable)]
pub struct WastedBaggage {
    id: String,
    qpid_id: i32,
    creator_id: String,
    item_hash: i32,
    broken: bool,
    x: i32,
    y: i32,
    z: i32,
}

impl WastedBaggage {
    pub fn into_api_type(self) -> api_types::requests::get_wasted_baggages::WastedBaggage {
        api_types::requests::get_wasted_baggages::WastedBaggage {
            baggage_id: self.id,
            account_id: self.creator_id,
            qpid_id: self.qpid_id,
            item: api_types::requests::get_wasted_baggages::WastedItem {
                broken: self.broken,
                item_hash: self.item_hash,
                x: self.x,
                y: self.y,
                z: self.z,
            },
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = wasted_baggages)]
pub struct NewWastedBaggage<'a> {
    pub id: String,
    pub qpid_id: i32,
    pub creator_id: &'a str,
    pub item_hash: i32,
    pub broken: bool,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
