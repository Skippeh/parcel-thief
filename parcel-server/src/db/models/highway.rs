use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use parcel_common::api_types;

use crate::db::schema::{devoted_highway_resources, total_highway_resources};

#[derive(Debug, Queryable)]
pub struct DevotedHighwayResources {
    pub id: i64,
    pub account_id: String,
    pub construction_id: i32,
    pub time: NaiveDateTime,
    pub resource_id: i16,
    pub num_resources: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = devoted_highway_resources)]
pub struct NewDevotedHighwayResources<'a> {
    pub account_id: &'a str,
    pub construction_id: i32,
    pub time: &'a NaiveDateTime,
    pub resource_id: i16,
    pub num_resources: i32,
}

#[derive(Debug, Queryable)]
pub struct TotalHighwayResources {
    pub id: i32,
    pub construction_id: i32,
    pub resource_id: i16,
    pub num_resources: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = total_highway_resources)]
pub struct NewTotalHighwayResources {
    pub construction_id: i32,
    pub resource_id: i16,
    pub num_resources: i64,
}
