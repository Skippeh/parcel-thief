use std::fmt::Display;

use actix_http::StatusCode;
use actix_web::{
    post,
    web::{Data, Json},
};
use diesel::ConnectionError;
use parcel_common::api_types::requests::send_like::SendLikeRequest;

use crate::{
    data::database::{likes::LikeTarget, Database},
    db::QueryError,
    endpoints::EmptyResponse,
    response_error::{impl_response_error, CommonResponseError},
    session::Session,
};

use super::InternalError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    Internal(InternalError),
    UnknownObject(String),
    UnexpectedValue(anyhow::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Internal(err) => err.fmt(f),
            Error::UnknownObject(id) => write!(f, "Unknown object id: {id}"),
            Error::UnexpectedValue(err) => write!(f, "Unexpected value in request: {err}"),
        }
    }
}

impl From<QueryError> for Error {
    fn from(value: QueryError) -> Self {
        Self::Internal(value.into())
    }
}

impl From<ConnectionError> for Error {
    fn from(value: ConnectionError) -> Self {
        Self::Internal(value.into())
    }
}

impl_response_error!(Error);
impl CommonResponseError for Error {
    fn get_status_code(&self) -> String {
        match self {
            Error::Internal(err) => err.get_status_code(),
            Error::UnknownObject(_) => "SL-UO".into(),
            Error::UnexpectedValue(_) => "SL-UV".into(),
        }
    }

    fn get_http_status_code(&self) -> actix_http::StatusCode {
        match self {
            Error::Internal(err) => err.get_http_status_code(),
            Error::UnknownObject(_) => StatusCode::BAD_REQUEST,
            Error::UnexpectedValue(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn get_message(&self) -> String {
        match self {
            Error::Internal(err) => err.get_message(),
            Error::UnknownObject(_) => "unknown object".into(),
            Error::UnexpectedValue(_) => "unexpected request value".into(),
        }
    }
}

#[post("sendLike")]
pub async fn send_like(
    request: Json<SendLikeRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<EmptyResponse, Error> {
    let conn = database.connect()?;
    let likes = conn.likes();
    let accounts = conn.accounts();

    if request.index != -1 {
        return Err(Error::UnexpectedValue(anyhow::anyhow!(
            "Expected index = -1, but it was {}",
            request.index
        )));
    }

    if request.likes_manual <= 0 && request.likes_auto <= 0 {
        return Err(Error::UnexpectedValue(anyhow::anyhow!(
            "Both manual and auto likes is <= 0"
        )));
    }

    if request.likes_auto < 0 {
        return Err(Error::UnexpectedValue(anyhow::anyhow!("likes_auto < 0")));
    }

    if request.likes_manual < 0 {
        return Err(Error::UnexpectedValue(anyhow::anyhow!("likes_manual < 0")));
    }

    if request.account_id == session.account_id {
        return Err(Error::UnexpectedValue(anyhow::anyhow!(
            "Target account matches current session account, giving likes to self owned objects should not be possible"
        )));
    }

    let like_target = LikeTarget::try_from(request.online_id.as_ref())
        .map_err(|_| Error::UnknownObject(request.online_id.clone()))?;

    if let LikeTarget::Object(id) = &like_target {
        let objects = conn.qpid_objects();
        let object = objects.get_by_id(id).await?;

        match &object {
            Some(object) => {
                if object.creator_id != request.account_id {
                    return Err(Error::UnexpectedValue(anyhow::anyhow!(
                        "Object creator id does not match request account id"
                    )));
                }
            }
            None => return Err(Error::UnknownObject(request.online_id.clone())),
        }
    }

    likes
        .give_likes(
            request.likes_auto,
            request.likes_manual,
            &request.like_type,
            &session.account_id,
            &request.account_id,
            like_target,
        )
        .await?;

    accounts
        .add_relationship_history(
            &session.account_id,
            &request.account_id,
            &chrono::Utc::now().naive_utc(),
        )
        .await?;

    Ok(EmptyResponse)
}
