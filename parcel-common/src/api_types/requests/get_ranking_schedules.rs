use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRankingSchedulesRequest {
    /// The current time in epoch (milliseconds)
    #[serde(rename = "ct")]
    pub client_time: u64,
    /// Not sure what this is, but the value is seemingly always 4.
    /// Maybe it's the amount of schedules that should be returned?
    #[serde(rename = "rn")]
    pub request_num: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRankingSchedulesResponse {
    #[serde(rename = "rs")]
    pub schedules: Vec<RankingSchedule>,
    /// The time when this data was updated, expressed in epoch (milliseconds)
    #[serde(rename = "ut")]
    pub updated_time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RankingSchedule {
    /// Not sure what this is, but it seems to increment by 1 from each previous ranking
    #[serde(rename = "bi")]
    pub basement_index: i32,
    #[serde(rename = "e")]
    /// The end date expressed in epoch (milliseconds)
    pub end_date: i64,
    #[serde(rename = "s")]
    /// The start date expressed in epoch (milliseconds)
    pub start_date: i64,
}
