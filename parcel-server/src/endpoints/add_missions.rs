use actix_web::{put, web::Json};
use parcel_common::api_types::requests::add_missions::{AddMissionsRequest, AddMissionsResponse};

use crate::endpoints::InternalError;

#[put("addMissions")]
pub async fn add_missions(
    request: Json<AddMissionsRequest>,
) -> Result<Json<AddMissionsResponse>, InternalError> {
    Err(anyhow::anyhow!("Not implemented").into())
}
