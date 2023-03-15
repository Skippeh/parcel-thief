use std::cmp::Ordering;

use actix_web::{
    post,
    web::{Data, Json},
};
use parcel_common::api_types::{
    mission::{MissionType, OnlineMissionType, ProgressState},
    requests::find_missions::{FindMissionsRequest, FindMissionsResponse},
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("findMissions")]
pub async fn find_missions(
    request: Json<FindMissionsRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<Json<FindMissionsResponse>, InternalError> {
    let request = request.into_inner();
    let conn = database.connect()?;
    let db_missions = conn.missions();

    const MISSION_TYPES: &[MissionType] = &[
        MissionType::Delivery,
        MissionType::Collect,
        MissionType::LostObject,
        MissionType::Supply,
        MissionType::Special,
        MissionType::Free,
    ];
    const ONLINE_MISSION_TYPES: &[OnlineMissionType] = &[
        OnlineMissionType::OnlineSupply,
        OnlineMissionType::Private,
        OnlineMissionType::Dynamic,
        OnlineMissionType::Static,
        OnlineMissionType::SharedLastStranding,
    ];
    const PROGRESS_STATES: &[ProgressState] = &[ProgressState::Available, ProgressState::Ready];
    let missions = db_missions
        .find_missions(
            ONLINE_MISSION_TYPES,
            MISSION_TYPES,
            &[&session.account_id],
            PROGRESS_STATES,
            &request.qpid_ids,
        )
        .await?;

    let mut missions = db_missions
        .query_mission_data(missions)
        .await?
        .into_iter()
        .collect::<Vec<_>>();

    if let Some(target_ids) = &request.target_ids {
        missions.sort_unstable_by(|a, b| {
            let contains_a = target_ids.contains(&a.mission.creator_id);
            let contains_b = target_ids.contains(&b.mission.creator_id);

            if contains_a && !contains_b {
                Ordering::Less
            } else if !contains_a && contains_b {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
    }

    // todo: limit number of missions based on request parameters: limit_pot_baggages, mission_limit_per_pot, private_limit_per_pot

    let res_missions = missions
        .into_iter()
        .take(request.limit as usize)
        .map(|m| m.into_api_type())
        .collect();

    Ok(Json(FindMissionsResponse {
        missions: res_missions,
    }))
}
