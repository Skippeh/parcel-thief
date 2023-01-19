use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetLikeHistoryRequest {
    /// Return likes that were given after this date, expressed as either 0 or epoch (milliseconds)
    ///
    /// Not sure what time is for the cut off point if value is 0.
    /// Maybe the last time it was called with 0 (stored serverside)?
    #[serde(rename = "since")]
    pub since: i64,
    /// The account id, it's always an empty string
    #[serde(rename = "uid")]
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetLiekHistoryResponse {
    #[serde(rename = "like_histories")]
    pub like_histories: Vec<LikeHistory>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LikeHistory {
    /// Amount of likes given automatically (by using structures or walking through signs)
    #[serde(rename = "lp2")]
    pub likes_auto: i32,
    /// Amount of likes given manually (by pressing the like button)
    #[serde(rename = "lp")]
    pub likes_manual: i32,
    /// Not sure what this is, it's seemingly always an empty string
    #[serde(rename = "lt")]
    pub like_type: String,
    /// The online id of the object that was liked
    #[serde(rename = "oid")]
    pub online_id: String,
    /// Not sure what this is. It's not manual + auto likes.
    /// It's almost always zero seemingly, but not always.
    #[serde(rename = "sc")]
    pub summarized_count: i32,
    /// The time when the likes were given, expressed in epoch (milliseconds)
    #[serde(rename = "t")]
    pub time: i64,
    /// The account id of the person who gave the likes
    #[serde(rename = "uid")]
    pub account_id: String,
}
