use actix_web::{
    get,
    web::{Data, Json},
};
use parcel_common::api_types::{
    frontend::baggages::ListItemsResponse,
    mission::{MissionType, OnlineMissionType, ProgressState},
};

use crate::{
    data::database::Database,
    frontend::{error::ApiError, jwt_session::JwtSession},
};

#[get("baggages/sharedCargo")]
pub async fn list_shared_cargo(
    _session: JwtSession,
    database: Data<Database>,
) -> Result<Json<ListItemsResponse>, ApiError> {
    let conn = database.connect()?;
    let missions = conn.missions(); // shared and lost cargo are saved as missions

    const ONLINE_MISSION_TYPES: &[OnlineMissionType] = &[OnlineMissionType::Private]; // private = shared cargo, dynamic = lost cargo. we only want shared cargo here
    const MISSION_TYPES: &[MissionType] = &[MissionType::LostObject];
    const PROGRESS_STATES: &[ProgressState] = &[ProgressState::Available, ProgressState::Ready];

    let data_missions = missions
        .find_missions(
            &ONLINE_MISSION_TYPES,
            &MISSION_TYPES,
            &[], // no excluded accounts
            &PROGRESS_STATES,
            None,
        )
        .await?;

    let data_missions = missions.query_mission_data(data_missions).await?;

    Err(ApiError::Internal(anyhow::anyhow!("Not implemented")))
}
