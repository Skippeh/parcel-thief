use actix_web::{
    put,
    web::{Data, Json},
};
use parcel_common::api_types::requests::set_mission_progress::{
    SetMissionProgressRequest, SetMissionProgressResponse,
};

use crate::{
    data::database::Database, db::models::mission::ChangeMission, endpoints::InternalError,
    session::Session,
};

#[put("setMissionProgress")]
pub async fn set_mission_progress(
    request: Json<SetMissionProgressRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<Json<SetMissionProgressResponse>, InternalError> {
    let conn = database.connect()?;
    let missions = conn.missions();
    let accounts = conn.accounts();

    let mission = missions.get_by_id(&request.mission_online_id).await?;

    if let Some(mission) = mission {
        if mission.creator_id != request.creator_account_id {
            return Err(anyhow::anyhow!("Mismatched creator_account_id in request").into());
        }

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
                request.dynamic_delivered_info.as_ref().map(Some),
                request.catapult_shell_info.as_ref().map(Some),
            )
            .await?;

        if session.account_id != mission.creator_id {
            accounts
                .add_relationship_history(
                    &session.account_id,
                    &mission.creator_id,
                    &chrono::Utc::now().naive_utc(),
                )
                .await?;
        }

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
