use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

use crate::db::schema::mission_relations;

#[derive(Debug, Queryable)]
pub struct Relation {
    pub id: i64,
    pub mission_id: String,
    pub account_id: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = mission_relations)]
pub struct NewRelation<'a> {
    pub mission_id: &'a str,
    pub account_id: &'a str,
    pub updated_at: &'a NaiveDateTime,
}
