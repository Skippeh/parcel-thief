use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::set_mission_progress::SetMissionProgressRequest;

#[post("setMissionProgress")]
pub async fn set_mission_progress(request: Json<SetMissionProgressRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
