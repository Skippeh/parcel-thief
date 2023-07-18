use flagset::flags;

#[cfg(feature = "ts")]
use typescript_type_def::TypeDef;

use crate::api_types::auth::Provider;

#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct AuthRequest {
    pub provider: Provider,
}

#[derive(Debug, serde::Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct InitAuthResponse {
    pub redirect_url: String,
}

#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct CheckAuthRequest {
    pub callback_token: String,
}

flags! {
    #[derive(PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
    #[cfg_attr(feature = "ts", derive(TypeDef))]
    #[repr(i64)]
    #[serde(rename_all = "camelCase")]
    pub enum FrontendPermissions: i64 {
        ManageAccounts,
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum CheckAuthResponse {
    #[serde(rename_all = "camelCase")]
    Success {
        name: String,
        avatar_url: String,
        auth_token: String,
        game_account_id: Option<String>,
        permissions: Vec<FrontendPermissions>,
    },
    Failure {
        error: String,
    },
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    pub expires_at: i64,
    /// Frontend account id, not game account id
    pub account_id: i64,
    /// Use `FlagSet<FrontendPermissions>` to read flags
    pub permissions: i64,
}
