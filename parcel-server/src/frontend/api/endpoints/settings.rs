use actix_web::{
    get, put,
    web::{Data, Json},
};
use parcel_common::api_types::frontend::{
    auth::FrontendPermissions,
    settings::{SettingsValues, WhitelistEntry},
};

use crate::{
    frontend::{
        error::ApiError,
        jwt_session::JwtSession,
        result::{ApiResponse, ApiResult},
    },
    ServerSettings, WhitelistSettings,
};

#[get("settings/server")]
pub async fn get_server_settings(
    session: JwtSession,
    settings: Data<ServerSettings>,
) -> ApiResult<SettingsValues> {
    // check that the session has access
    if !session.has_permissions(FrontendPermissions::ManageServerSettings) {
        return Err(ApiError::Forbidden);
    }

    ApiResponse::ok((*settings.read().await).clone())
}

#[put("settings/server")]
pub async fn set_server_settings(
    session: JwtSession,
    request_settings: Json<SettingsValues>,
    settings: Data<ServerSettings>,
) -> ApiResult<SettingsValues> {
    // check that the session has access
    if !session.has_permissions(FrontendPermissions::ManageServerSettings) {
        return Err(ApiError::Forbidden);
    }

    settings
        .write(|settings| {
            **settings = request_settings.clone();
        })
        .await?;

    ApiResponse::ok(request_settings.into_inner())
}

#[get("settings/whitelist")]
pub async fn get_whitelist(
    session: JwtSession,
    whitelist: Data<WhitelistSettings>,
) -> ApiResult<Vec<WhitelistEntry>> {
    // check that the session has access
    if !session.has_permissions(FrontendPermissions::ManageServerSettings) {
        return Err(ApiError::Forbidden);
    }

    ApiResponse::ok(whitelist.read().await.clone().into_inner())
}

#[put("settings/whitelist")]
pub async fn set_whitelist(
    session: JwtSession,
    mut request_whitelist: Json<Vec<WhitelistEntry>>,
    whitelist: Data<WhitelistSettings>,
) -> ApiResult<Vec<WhitelistEntry>> {
    // check that the session has access
    if !session.has_permissions(FrontendPermissions::ManageServerSettings) {
        return Err(ApiError::Forbidden);
    }

    // normalize formatting on request data
    for entry in request_whitelist.iter_mut() {
        entry.provider_id = entry.provider_id.trim().to_string();
        entry.name_reference = entry.name_reference.as_ref().map(|s| s.trim().to_string());
    }

    whitelist
        .write(|whitelist| {
            whitelist.clear();
            whitelist.append(&mut request_whitelist.clone());
        })
        .await?;

    ApiResponse::ok(request_whitelist.into_inner())
}
