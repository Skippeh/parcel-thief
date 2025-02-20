use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRelationshipsRequest;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRelationshipsResponse {
    /// Last x people the player interacted with, ordered by last interaction time (latest first).
    ///
    /// The official server returns 10 players.
    #[serde(rename = "history")]
    pub history: Vec<RelationshipHistory>,
    /// Players that we have a strand contract with.
    #[serde(rename = "strand")]
    pub strand_contracts: Vec<StrandContract>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelationshipHistory {
    /// The time when this player was last interacted with
    /// expressed as epoch (milliseconds)
    #[serde(rename = "t")]
    pub last_interaction_time: i64,
    #[serde(rename = "uid")]
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrandContract {
    /// The time when this strand contract was made
    /// expressed as epoch (milliseconds)
    #[serde(rename = "t")]
    pub added_time: i64,
    /// The account id of the player that we have a strand contract with
    #[serde(rename = "uid")]
    pub account_id: String,
}
