use actix_web::{
    post,
    web::{Data, Json},
};
use parcel_common::api_types::{
    requests::get_ordered_missions::GetOrderedMissionsResponse, IntoDsApiType,
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("getOrderedMissions")]
pub async fn get_ordered_missions(
    session: Session,
    database: Data<Database>,
) -> Result<Json<GetOrderedMissionsResponse>, InternalError> {
    let conn = database.connect()?;
    let missions = conn.missions();
    let ordered_missions = missions.get_ordered_missions(&session.account_id).await?;
    let ordered_missions = missions
        .query_mission_data(ordered_missions)
        .await?
        .into_iter()
        .map(|mission| mission.into_ds_api_type())
        .collect::<Vec<_>>();

    Ok(Json(GetOrderedMissionsResponse {
        missions: ordered_missions,
    }))
}
