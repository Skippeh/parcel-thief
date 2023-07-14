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
    // note: at the moment the typescript generator doesn't support serde_repr/c style enums. So this is currently unusable in TS without a workaround
    #[derive(PartialOrd, Ord, Hash, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
    #[cfg_attr(feature = "ts", derive(TypeDef))]
    #[repr(u32)]
    pub enum JwtPermissions: u32 {
        None = 0,
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
        permissions: JwtPermissions,
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
    pub account_id: String,
}
