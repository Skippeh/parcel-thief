use actix_web::{
    post,
    web::{Data, Json},
};
use parcel_common::api_types::{
    requests::create_road::{CreateRoadRequest, CreateRoadResponse},
    IntoDsApiType,
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("createRoad")]
pub async fn create_road(
    request: Json<CreateRoadRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<Json<CreateRoadResponse>, InternalError> {
    let db = database.connect()?;
    let roads = db.roads();

    let road = roads
        .create_road_from_request(&session.account_id, &request.into_inner())
        .await
        .map_err(|err| InternalError(err.into()))? // todo: replace internal error with proper error
        .into_ds_api_type();

    Ok(Json(road))
}
