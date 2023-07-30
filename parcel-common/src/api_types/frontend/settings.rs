use serde::{Deserialize, Serialize};

#[cfg(feature = "ts")]
use typescript_type_def::TypeDef;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct SettingsValues {
    /// If true, anyone who knows of the server address will be able to
    /// log in to the game server. Otherwise they must first be added to
    /// the whitelist.
    pub public_server: bool,
    /// If true, any user with an existing game account can log in to the
    /// frontend. Otherwise an admin must first create a frontend account
    /// for the user.
    pub allow_frontend_login: bool,
}

// This could be implemented automatically with macro trait at the moment,
// but we're not doing that in case any non default value fields are added
impl Default for SettingsValues {
    fn default() -> Self {
        Self {
            public_server: false,
            allow_frontend_login: false,
        }
    }
}
