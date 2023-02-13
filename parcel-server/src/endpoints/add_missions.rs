use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::add_missions::AddMissionsRequest;

#[post("addMissions")]
pub async fn add_missions(request: Json<AddMissionsRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
