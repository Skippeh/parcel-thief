use actix_web::{put, web::Json, Responder};
use parcel_common::api_types::requests::set_player_profile::{
    SetPlayerProfileRequest, SetPlayerProfileResponse,
};

use crate::session::Session;

#[put("setPlayerProfile")]
pub async fn set_player_profile(
    request: Json<SetPlayerProfileRequest>,
    session: Session,
) -> impl Responder {
    Json(SetPlayerProfileResponse {
        account_id: session.account_id,
        basic: request.0.basic,
    })
}
