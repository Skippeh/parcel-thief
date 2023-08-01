use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use parcel_common::api_types::{self, IntoDsApiType};

use crate::db::schema::{
    likes::{self},
    total_highway_likes,
};

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

impl Like {
    pub fn total_likes(&self) -> i32 {
        self.likes_manual + self.likes_auto
    }
}

impl IntoDsApiType for Like {
    type ApiType = api_types::requests::get_like_history::LikeHistory;

    fn into_ds_api_type(self) -> Self::ApiType {
        Self::ApiType {
            likes_auto: self.likes_auto,
            likes_manual: self.likes_manual,
            like_type: self.ty,
            online_id: self.online_id,
            summarized_player_count: 0,
            time: self.time.timestamp_millis(),
            account_id: self.from_id,
        }
    }
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

#[derive(Debug, Queryable)]
pub struct TotalHighwayLikes {
    pub account_id: String,
    pub likes: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = total_highway_likes)]
pub struct NewTotalHighwayLikes<'a> {
    pub account_id: &'a str,
    pub likes: i64,
}
