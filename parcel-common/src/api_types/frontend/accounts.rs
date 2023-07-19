use serde::{Deserialize, Serialize};

#[cfg(feature = "ts")]
use typescript_type_def::TypeDef;
use validator::Validate;

use crate::api_types::auth::Provider;

use super::auth::FrontendPermissions;

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct FrontendAccountListItem {
    pub id: i64,
    pub game_id: Option<String>,
    pub name: String,
    pub permissions: Vec<FrontendPermissions>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct GameAccountListItem {
    pub game_id: String,
    pub name: String,
    pub provider: Provider,
    pub provider_id: String,
    pub last_login: String,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub enum ListAccountsType {
    Frontend,
    Game,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ListAccountsResponse {
    #[serde(rename_all = "camelCase")]
    Frontend {
        accounts: Vec<FrontendAccountListItem>,
    },
    #[serde(rename_all = "camelCase")]
    Game { accounts: Vec<GameAccountListItem> },
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct LocalAccount {
    pub username: String,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct ProviderConnection {
    pub provider: Provider,
    pub provider_id: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct FrontendAccount {
    pub id: i64,
    pub game_id: Option<String>,
    pub permissions: Vec<FrontendPermissions>,
    pub provider_connection: Option<ProviderConnection>,
    pub local_account: Option<LocalAccount>,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct SetAccountPermissionsRequest {
    pub permissions: Vec<FrontendPermissions>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialsRequest {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
}
