use std::fmt::Display;

use actix_web::{
    post,
    web::{Data, Json},
};
use diesel::ConnectionError;
use parcel_common::api_types::requests::delete_object::DeleteObjectRequest;

use crate::{
    data::database::Database,
    db::QueryError,
    endpoints::{EmptyResponse, InternalError},
    response_error::{impl_response_error, CommonResponseError},
    session::Session,
};

#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    NoIdSpecified,
    InternalError(InternalError),
    ObjectNotFound(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoIdSpecified => write!(f, "No id specified"),
            Error::InternalError(err) => write!(f, "{}", err),
            Error::ObjectNotFound(object_id) => {
                write!(f, "Could not find an object with id: {}", object_id)
            }
        }
    }
}

impl From<InternalError> for Error {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}

impl From<ConnectionError> for Error {
    fn from(value: ConnectionError) -> Self {
        Self::InternalError(value.into())
    }
}

impl From<QueryError> for Error {
    fn from(value: QueryError) -> Self {
        Self::InternalError(value.into())
    }
}

impl_response_error!(Error);
impl CommonResponseError for Error {
    fn get_status_code(&self) -> String {
        match self {
            Error::NoIdSpecified => "DO-NI".into(),
            Error::ObjectNotFound(_) => "DO-NF".into(),
            Error::InternalError(err) => err.get_status_code(),
        }
    }

    fn get_http_status_code(&self) -> actix_http::StatusCode {
        match self {
            Error::NoIdSpecified | Error::ObjectNotFound(_) => actix_http::StatusCode::BAD_REQUEST,
            Error::InternalError(err) => err.get_http_status_code(),
        }
    }

    fn get_message(&self) -> String {
        match self {
            Error::NoIdSpecified => "no id specified".into(),
            Error::InternalError(err) => err.get_message(),
            Error::ObjectNotFound(_) => "object not found".into(),
        }
    }
}

#[post("deleteObject")]
pub async fn delete_object(
    request: Json<DeleteObjectRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<EmptyResponse, Error> {
    if request.object_id.is_empty() {
        return Err(Error::NoIdSpecified);
    }

    let conn = database.connect().await?;
    let objects = conn.qpid_objects();

    let object = objects.get_by_id(&request.object_id).await?;

    // Note that it's important we don't remove the object from the database.
    // This is because some objects can hold items (like postboxes) that might
    // hold items that other players have donated, or otherwise may rely on.
    if let Some(object) = object {
        objects
            .mark_deleted_for_account(&object.id, &session.account_id)
            .await?;
        Ok(EmptyResponse)
    } else {
        Err(Error::ObjectNotFound(request.object_id.clone()))
    }
}
