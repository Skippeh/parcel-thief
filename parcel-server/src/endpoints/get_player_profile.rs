use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::get_player_profile::GetPlayerProfileRequest;

#[post("/getPlayerProfile")]
pub async fn get_player_profile(request: Json<GetPlayerProfileRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
