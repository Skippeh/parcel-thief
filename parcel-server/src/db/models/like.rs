use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

use crate::db::schema::likes::{self};

#[derive(Debug, Queryable)]
pub struct Like {
    pub id: i64,
    pub time: NaiveDateTime,
    pub from_id: String,
    pub to_id: String,
    pub online_id: String,
    pub likes_manual: i32,
    pub likes_auto: i32,
    pub ty: String,
    pub acknowledged: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = likes)]
pub struct NewLike<'a> {
    pub time: &'a NaiveDateTime,
    pub from_id: &'a str,
    pub to_id: &'a str,
    pub online_id: &'a str,
    pub likes_manual: i32,
    pub likes_auto: i32,
    #[diesel(column_name = type_)]
    pub ty: &'a str,
    pub acknowledged: bool,
}
