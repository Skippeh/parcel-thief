use actix_web::{
    put,
    web::{Data, Json},
};
use parcel_common::api_types::requests::delete_missions::DeleteMissionsRequest;

use crate::{
    data::database::Database,
    endpoints::{EmptyResponse, InternalError},
    session::Session,
};

#[put("deleteMissions")]
pub async fn delete_missions(
    request: Json<DeleteMissionsRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<EmptyResponse, InternalError> {
    let conn = database.connect().await?;
    let missions = conn.missions();
    let mission_ids = request
        .0
        .missions
        .into_iter()
        .map(|mission| mission.mission_online_id)
        .collect::<Vec<_>>();

    let num_deleted = missions
        .delete_missions(&session.account_id, mission_ids.iter().map(|id| id as &str))
        .await?;

    if num_deleted != mission_ids.len() {
        log::warn!("Failed to delete {} out of {} missions (user might be trying to delete a mission that they didn't create or was created on the official server)", mission_ids.len() - num_deleted, mission_ids.len());
    }

    Ok(EmptyResponse)
}
