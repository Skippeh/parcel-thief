use diesel::{Insertable, Queryable};

use crate::db::schema::mission_supply_infos;

#[derive(Debug, Queryable)]
pub struct SupplyInfo {
    pub mission_id: String,
    pub item_hash: i64,
    pub amount: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = mission_supply_infos, primary_key(mission_id))]
pub struct NewSupplyInfo<'a> {
    pub mission_id: &'a str,
    pub item_hash: i64,
    pub amount: i32,
}
