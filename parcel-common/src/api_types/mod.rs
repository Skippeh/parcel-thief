use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod area;
pub mod auth;
pub mod frontend;
pub mod mission;
pub mod object;
pub mod player_profile;
pub mod rank;
pub mod requests;
pub mod road;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptedData {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

pub trait IntoDsApiType {
    type ApiType: Serialize + DeserializeOwned;

    fn into_ds_api_type(self) -> Self::ApiType;
}

pub trait TryIntoDsApiType {
    type ApiType: Serialize + DeserializeOwned;
    type Error;

    fn try_into_ds_api_type(self) -> Result<Self::ApiType, Self::Error>;
}

pub trait IntoFrontendApiType {
    type ApiType: Serialize + DeserializeOwned;

    fn into_frontend_api_type(self) -> Self::ApiType;
}

pub trait TryIntoFrontendApiType {
    type ApiType: Serialize + DeserializeOwned;
    type Error;

    fn try_into_frontend_api_type(self) -> Result<Self::ApiType, Self::Error>;
}
