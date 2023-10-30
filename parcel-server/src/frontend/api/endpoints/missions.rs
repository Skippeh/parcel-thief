use actix_web::{get, post, web::Json};
use parcel_common::api_types::frontend::missions::EditMissionData;

use crate::frontend::{
    jwt_session::JwtSession,
    result::{ApiResponse, ApiResult},
};

#[post("missions")]
pub async fn create_mission(
    session: JwtSession,
    mission_data: Json<EditMissionData>,
) -> ApiResult<()> {
    log::info!("create_mission:\n{:#?}", mission_data);
    ApiResponse::ok(())
}

#[get("missions")]
pub async fn get_missions(session: JwtSession) -> ApiResult<()> {
    ApiResponse::ok(())
}
