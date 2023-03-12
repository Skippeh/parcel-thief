use actix_web::{
    post,
    web::{Data, Json},
};
use parcel_common::api_types::requests::set_mission_progress::{
    SetMissionProgressRequest, SetMissionProgressResponse,
};

use crate::{
    data::database::Database, db::models::mission::ChangeMission, endpoints::InternalError,
    session::Session,
};

#[post("setMissionProgress")]
pub async fn set_mission_progress(
    request: Json<SetMissionProgressRequest>,
    _session: Session,
    database: Data<Database>,
) -> Result<Json<SetMissionProgressResponse>, InternalError> {
    let conn = database.connect()?;
    let missions = conn.missions();

    let mission = missions.get_by_id(&request.mission_online_id).await?;

    if let Some(mission) = mission {
        if mission.creator_id != request.creator_account_id {
            return Err(anyhow::anyhow!("Mismatched creator_account_id in request").into());
        }

        let delivered_info = request
            .dynamic_delivered_info
            .as_ref()
            .map(|info| info.into());

        let catapult_shell_info = request.catapult_shell_info.as_ref().map(|info| info.into());

        missions
            .update_mission(
                &mission.id,
                &ChangeMission {
                    qpid_delivered_location: Some(request.delivered_location_id),
                    progress_state: Some(request.progress_state),
                    qpid_id: Some(request.qpid_id),
                    ..Default::default()
                },
                request.baggages.as_deref().map(Some),
                delivered_info.as_ref().map(Some),
                catapult_shell_info.as_ref().map(Some),
            )
            .await?;

        let mission = missions
            .query_mission_data([mission])
            .await?
            .pop()
            .expect("There should be exactly one item in the vec");

        Ok(Json(mission.into_api_type()))
    } else {
        Err(anyhow::anyhow!("Mission not found").into())
    }
}
