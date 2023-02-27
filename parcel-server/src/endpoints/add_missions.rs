use actix_web::{put, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::add_missions::AddMissionsRequest;

#[put("addMissions")]
pub async fn add_missions(request: Json<AddMissionsRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
