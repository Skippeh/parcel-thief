use actix_web::{
    post,
    web::{Data, Json},
};
use base64::Engine;
use diesel::ConnectionError;
use parcel_common::api_types::requests::get_road_data::{GetRoadDataRequest, GetRoadDataResponse};

use crate::{
    data::database::Database,
    db::QueryError,
    endpoints::InternalError,
    response_error::{impl_response_error, CommonResponseError},
    session::Session,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Road not found")]
    RoadNotFound,
    #[error("{0}")]
    Internal(InternalError),
}

impl_response_error!(Error);
impl CommonResponseError for Error {
    fn get_status_code(&self) -> String {
        match self {
            Error::RoadNotFound => "RD_NF".into(),
            Error::Internal(err) => err.get_status_code(),
        }
    }

    fn get_http_status_code(&self) -> actix_http::StatusCode {
        match self {
            Error::RoadNotFound => actix_http::StatusCode::NOT_FOUND,
            Error::Internal(err) => err.get_http_status_code(),
        }
    }

    fn get_message(&self) -> String {
        match self {
            Error::RoadNotFound => "road not found".into(),
            Error::Internal(err) => err.get_message(),
        }
    }
}

impl From<QueryError> for Error {
    fn from(err: QueryError) -> Self {
        Self::Internal(InternalError(err.into()))
    }
}

impl From<ConnectionError> for Error {
    fn from(value: ConnectionError) -> Self {
        Self::Internal(InternalError(value.into()))
    }
}

#[post("getRoadData")]
pub async fn get_road_data(
    request: Json<GetRoadDataRequest>,
    _session: Session,
    database: Data<Database>,
) -> Result<Json<GetRoadDataResponse>, Error> {
    let conn = database.connect()?;
    let roads = conn.roads();

    let road_data = roads.get_road_data(&request.0.road_id).await?;

    if let Some(road_data) = road_data {
        let b64_data = base64::engine::general_purpose::STANDARD.encode(road_data.data);
        Ok(Json(GetRoadDataResponse { data: b64_data }))
    } else {
        Err(Error::RoadNotFound)
    }
}
