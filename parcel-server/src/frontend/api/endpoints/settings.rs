use actix_web::{
    get, put,
    web::{Data, Json},
};
use parcel_common::api_types::frontend::{auth::FrontendPermissions, settings::SettingsValues};

use crate::{
    frontend::{
        error::ApiError,
        jwt_session::JwtSession,
        result::{ApiResponse, ApiResult},
    },
    ServerSettings,
};

#[get("settings")]
pub async fn get_settings(
    session: JwtSession,
    settings: Data<ServerSettings>,
) -> ApiResult<SettingsValues> {
    // check that the session has access
    if !session.has_permissions(FrontendPermissions::ManageServerSettings) {
        return Err(ApiError::Forbidden);
    }

    ApiResponse::ok((*settings.read().await).clone())
}

#[put("settings")]
pub async fn set_settings(
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
