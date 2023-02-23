use std::fmt::Display;

use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use parcel_common::api_types::player_profile::BasicPlayerProfile;

use crate::db::schema::player_profiles;

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = player_profiles, primary_key(account_id))]
pub struct PlayerProfile {
    pub account_id: String,
    pub banner_id: i32,
    pub num_bronze_medals: i32,
    pub num_bronze_medals_large: i32,
    pub delivered_baggage: i32,
    pub delivery_rank: i32,
    pub delivered_weight: i32,
    pub evaluation_bridge: i32,
    pub evaluation_delivery: i32,
    pub evaluation_safety: i32,
    pub evaluation_speed: i32,
    pub evaluation_service: i32,
    pub num_gold_medals: i32,
    pub num_gold_medals_large: i32,
    pub legend_count: i32,
    pub last_login: NaiveDateTime,
    pub distance_traveled: i32,
    pub music_open_tracks: i64,
    pub name_hash: i32,
    pub num_platinum_medals: i32,
    pub num_platinum_medals_large: i32,
    pub num_likes_received_npc: i32,
    pub num_likes_received_online: i32,
    pub num_rainbow_medals: i32,
    pub num_rainbow_medals_large: i32,
    pub super_legend_count: i32,
    pub num_silver_medals: i32,
    pub num_silver_medals_large: i32,
    pub ss_legend_count: i32,
}

#[derive(Debug, thiserror::Error)]
pub struct DateOutOfRange;

impl Display for DateOutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The epoch time stamp could not be converted to NaiveDateTime due to being out of range")
    }
}

impl TryFrom<(String, &BasicPlayerProfile)> for PlayerProfile {
    type Error = DateOutOfRange;

    fn try_from(value: (String, &BasicPlayerProfile)) -> Result<Self, Self::Error> {
        let (account_id, value) = value;
        Ok(Self {
            account_id,
            banner_id: value.banner_id,
            num_bronze_medals: value.num_bronze_medals,
            num_bronze_medals_large: value.num_bronze_medals_large,
            delivered_baggage: value.delivered_baggage,
            delivery_rank: value.delivery_rank,
            delivered_weight: value.delivered_weight,
            evaluation_bridge: value.evaluation_bridge,
            evaluation_delivery: value.evaluation_delivery,
            evaluation_safety: value.evaluation_safety,
            evaluation_speed: value.evaluation_speed,
            evaluation_service: value.evaluation_service,
            num_gold_medals: value.num_gold_medals,
            num_gold_medals_large: value.num_gold_medals_large,
            legend_count: value.legend_count,
            last_login: NaiveDateTime::from_timestamp_opt(value.last_login, 0)
                .ok_or(DateOutOfRange)?,
            distance_traveled: value.distance_traveled,
            music_open_tracks: value.music_open_tracks as i64,
            name_hash: value.name_hash,
            num_platinum_medals: value.num_platinum_medals,
            num_platinum_medals_large: value.num_platinum_medals_large,
            num_likes_received_npc: value.num_likes_received_npc,
            num_likes_received_online: value.num_likes_received_online,
            num_rainbow_medals: value.num_rainbow_medals,
            num_rainbow_medals_large: value.num_rainbow_medals_large,
            super_legend_count: value.super_legend_count,
            num_silver_medals: value.num_silver_medals,
            num_silver_medals_large: value.num_silver_medals_large,
            ss_legend_count: value.ss_legend_count,
        })
    }
}

impl TryFrom<(String, BasicPlayerProfile)> for PlayerProfile {
    type Error = DateOutOfRange;

    fn try_from(value: (String, BasicPlayerProfile)) -> Result<Self, Self::Error> {
        Self::try_from((value.0, &value.1))
    }
}

impl From<&PlayerProfile> for BasicPlayerProfile {
    fn from(value: &PlayerProfile) -> Self {
        Self {
            banner_id: value.banner_id,
            num_bronze_medals: value.num_bronze_medals,
            num_bronze_medals_large: value.num_bronze_medals_large,
            delivered_baggage: value.delivered_baggage,
            delivery_rank: value.delivery_rank,
            delivered_weight: value.delivered_weight,
            evaluation_bridge: value.evaluation_bridge,
            evaluation_delivery: value.evaluation_delivery,
            evaluation_safety: value.evaluation_safety,
            evaluation_speed: value.evaluation_speed,
            evaluation_service: value.evaluation_service,
            num_gold_medals: value.num_gold_medals,
            num_gold_medals_large: value.num_gold_medals_large,
            legend_count: value.legend_count,
            last_login: value.last_login.timestamp(),
            distance_traveled: value.distance_traveled,
            music_open_tracks: value.music_open_tracks as u64,
            name_hash: value.name_hash,
            num_platinum_medals: value.num_platinum_medals,
            num_platinum_medals_large: value.num_platinum_medals_large,
            num_likes_received_npc: value.num_likes_received_npc,
            num_likes_received_online: value.num_likes_received_online,
            num_rainbow_medals: value.num_rainbow_medals,
            num_rainbow_medals_large: value.num_rainbow_medals_large,
            super_legend_count: value.super_legend_count,
            num_silver_medals: value.num_silver_medals,
            num_silver_medals_large: value.num_silver_medals_large,
            ss_legend_count: value.ss_legend_count,
        }
    }
}

impl From<PlayerProfile> for BasicPlayerProfile {
    fn from(value: PlayerProfile) -> Self {
        Self::from(&value)
    }
}

impl PlayerProfile {
    pub fn assign_from_basic_profile(
        &mut self,
        value: &BasicPlayerProfile,
    ) -> Result<(), DateOutOfRange> {
        self.banner_id = value.banner_id;
        self.num_bronze_medals = value.num_bronze_medals;
        self.num_bronze_medals_large = value.num_bronze_medals_large;
        self.delivered_baggage = value.delivered_baggage;
        self.delivery_rank = value.delivery_rank;
        self.delivered_weight = value.delivered_weight;
        self.evaluation_bridge = value.evaluation_bridge;
        self.evaluation_delivery = value.evaluation_delivery;
        self.evaluation_safety = value.evaluation_safety;
        self.evaluation_speed = value.evaluation_speed;
        self.evaluation_service = value.evaluation_service;
        self.num_gold_medals = value.num_gold_medals;
        self.num_gold_medals_large = value.num_gold_medals_large;
        self.legend_count = value.legend_count;
        self.last_login =
            NaiveDateTime::from_timestamp_opt(value.last_login, 0).ok_or(DateOutOfRange)?;
        self.distance_traveled = value.distance_traveled;
        self.music_open_tracks = value.music_open_tracks as i64;
        self.name_hash = value.name_hash;
        self.num_platinum_medals = value.num_platinum_medals;
        self.num_platinum_medals_large = value.num_platinum_medals_large;
        self.num_likes_received_npc = value.num_likes_received_npc;
        self.num_likes_received_online = value.num_likes_received_online;
        self.num_rainbow_medals = value.num_rainbow_medals;
        self.num_rainbow_medals_large = value.num_rainbow_medals_large;
        self.super_legend_count = value.super_legend_count;
        self.num_silver_medals = value.num_silver_medals;
        self.num_silver_medals_large = value.num_silver_medals_large;
        self.ss_legend_count = value.ss_legend_count;

        Ok(())
    }
}
