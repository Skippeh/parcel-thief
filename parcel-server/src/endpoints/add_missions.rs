use actix_web::{
    put,
    web::{Data, Json},
};
use parcel_common::api_types::{
    requests::add_missions::{AddMissionsRequest, AddMissionsResponse},
    IntoDsApiType,
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[put("addMissions")]
pub async fn add_missions(
    request: Json<AddMissionsRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<Json<AddMissionsResponse>, InternalError> {
    let conn = database.connect().await?;
    let missions = conn.missions();
    let mut saved_missions = Vec::new();

    for mission in &request.missions {
        saved_missions.push(
            missions
                .save_mission(mission, &session.account_id)
                .await?
                .into_ds_api_type(),
        );
    }

    Ok(Json(AddMissionsResponse {
        missions: saved_missions,
    }))
}
