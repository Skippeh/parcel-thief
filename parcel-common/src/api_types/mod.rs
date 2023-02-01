use serde::{Deserialize, Serialize};

pub mod area;
pub mod auth;
pub mod baggage;
pub mod mission;
pub mod object;
pub mod player_profile;
pub mod rank;
pub mod requests;
pub mod road;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptedData {
    pub data: Option<String>,
}
