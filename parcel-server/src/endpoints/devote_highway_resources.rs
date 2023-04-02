use actix_web::{
    post,
    web::{Data, Json},
};
use parcel_common::api_types::requests::devote_highway_resources::DevoteHighwayResourcesRequest;

use crate::{
    data::database::Database,
    endpoints::{EmptyResponse, InternalError},
    session::Session,
};

#[post("devoteHighwayResources")]
async fn devote_highway_resources(
    request: Json<DevoteHighwayResourcesRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<EmptyResponse, InternalError> {
    let conn = database.connect()?;
    let highway_resources = conn.highway_resources();

    highway_resources
        .devote_resources(&session.account_id, &request.put_histories)
        .await?;

    Ok(EmptyResponse)
}
