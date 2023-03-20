use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendLikeRequest {
    /// Some index? Not sure, value is seemingly always -1
    #[serde(rename = "idx")]
    pub index: i32,
    /// Amount of likes given automatically (by using structures or walking through signs)
    #[serde(rename = "lp2")]
    pub likes_auto: i32,
    /// Amount of likes given manually (by pressing the like button)
    #[serde(rename = "lp")]
    pub likes_manual: i32,
    /// Unknown purpose, seems to always be empty string
    #[serde(rename = "lt")]
    pub like_type: String,
    /// The online id of the object receiving likes.
    /// May or may not be unique depending on online_type.
    /// See online_type for more information.
    #[serde(rename = "oid")]
    pub online_id: String,
    /// The type of the object receiving likes.
    ///
    /// Incomplete list of possible values:
    /// - h = highway, online_id format = "h{construction_id}", where the number is the id of the highway segment
    /// - i = dummy/shared, value is either idummy or ishared, maybe its purpose is to send likes to a player "directly" instead of liking an object
    /// - any other = object, where the first letter is the object type (and also the first letter of the object id)
    #[serde(rename = "ot")]
    pub online_type: String,
    /// The account id of the player receiving the likes.
    /// Most likely only relevant when liking non unique items such as highway segments
    #[serde(rename = "u")]
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendLikeResponse;
