use std::fmt::Display;

use actix_web::{
    put,
    web::{Data, Json},
};
use diesel::ConnectionError;
use parcel_common::api_types::requests::set_player_profile::{
    SetPlayerProfileRequest, SetPlayerProfileResponse,
};

use crate::{
    data::database::Database,
    db::{models::player_profile::PlayerProfile, QueryError},
    response_error::{impl_response_error, CommonResponseError},
    session::Session,
};

use super::InternalError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    InternalError(InternalError),
    InvalidProfileValue(anyhow::Error),
}

impl From<QueryError> for Error {
    fn from(value: QueryError) -> Self {
        Self::InternalError(value.into())
    }
}

impl From<ConnectionError> for Error {
    fn from(value: ConnectionError) -> Self {
        Self::InternalError(value.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalError(err) => err.fmt(f),
            Error::InvalidProfileValue(err) => write!(f, "Invalid profile value: {}", err),
        }
    }
}

impl_response_error!(Error);
impl CommonResponseError for Error {
    fn get_status_code(&self) -> String {
        match self {
            Error::InternalError(err) => err.get_status_code(),
            Error::InvalidProfileValue(_) => "SPP_IPV".into(),
        }
    }

    fn get_http_status_code(&self) -> actix_http::StatusCode {
        match self {
            Error::InternalError(err) => err.get_http_status_code(),
            Error::InvalidProfileValue(_) => actix_http::StatusCode::BAD_REQUEST,
        }
    }

    fn get_message(&self) -> String {
        match self {
            Error::InternalError(err) => err.get_message(),
            Error::InvalidProfileValue(_) => "invalid profile values".into(),
        }
    }
}

#[put("setPlayerProfile")]
pub async fn set_player_profile(
    request: Json<SetPlayerProfileRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<Json<SetPlayerProfileResponse>, Error> {
    let account_id = session.account_id.clone();
    let conn = database.connect()?;
    let profiles = conn.player_profiles();

    profiles
        .add_or_update_profile(
            &PlayerProfile::try_from((account_id.clone(), &request.basic))
                .map_err(|err| Error::InvalidProfileValue(err.into()))?,
        )
        .await?;

    Ok(Json(SetPlayerProfileResponse {
        account_id,
        basic: request.into_inner().basic,
    }))
}
