use std::fmt::Display;

use actix_web::{
    post,
    web::{Data, Json},
};
use diesel::ConnectionError;
use parcel_common::api_types::requests::update_object::UpdateObjectRequest;

use crate::{
    data::database::{qpid_objects::ChangeInfo, Database},
    db::QueryError,
    response_error::{impl_response_error, CommonResponseError},
    session::Session,
};

use super::{EmptyResponse, InternalError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    InternalError(InternalError),
    ObjectNotFound(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalError(err) => err.fmt(f),
            Error::ObjectNotFound(object_id) => write!(f, "No object found with id: {}", object_id),
        }
    }
}

impl_response_error!(Error);
impl CommonResponseError for Error {
    fn get_status_code(&self) -> String {
        match self {
            Error::InternalError(err) => err.get_status_code(),
            Error::ObjectNotFound(_) => "UO-NF".into(),
        }
    }

    fn get_http_status_code(&self) -> actix_http::StatusCode {
        match self {
            Error::InternalError(err) => err.get_http_status_code(),
            Error::ObjectNotFound(_) => actix_http::StatusCode::BAD_REQUEST,
        }
    }

    fn get_message(&self) -> String {
        match self {
            Error::InternalError(err) => err.get_message(),
            Error::ObjectNotFound(_) => "object not found".into(),
        }
    }
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

impl From<InternalError> for Error {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}

#[post("updateObject")]
pub async fn update_object(
    request: Json<UpdateObjectRequest>,
    _session: Session,
    database: Data<Database>,
) -> Result<EmptyResponse, Error> {
    let conn = database.connect()?;
    let objects = conn.qpid_objects();

    let object = objects.get_by_id(&request.object_id).await?;

    if let Some(object) = object {
        if let Some(stone_info) = &request.stone_info {
            objects
                .update_info(&object.id, ChangeInfo::Stone(&stone_info.into()))
                .await?;
        } else if let Some(parking_info) = &request.parking_info {
            objects
                .update_info(&object.id, ChangeInfo::Parking(&parking_info.into()))
                .await?;
        } else if let Some(vehicle_info) = &request.vehicle_info {
            objects
                .update_info(&object.id, ChangeInfo::Vehicle(&vehicle_info.into()))
                .await?;
        } else if let Some(customize_info) = &request.customize_info {
            objects
                .update_info(&object.id, ChangeInfo::Customize(&customize_info.into()))
                .await?;
        } else if let Some(extra_info) = &request.extra_info {
            objects
                .update_info(&object.id, ChangeInfo::Extra(&extra_info.into()))
                .await?;
        }

        Ok(EmptyResponse)
    } else {
        Err(Error::ObjectNotFound(request.into_inner().object_id))
    }
}
