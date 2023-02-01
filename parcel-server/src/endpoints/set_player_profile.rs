use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::set_player_profile::SetPlayerProfileRequest;

#[post("e/setPlayerProfile")]
pub async fn set_player_profile(request: Json<SetPlayerProfileRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
