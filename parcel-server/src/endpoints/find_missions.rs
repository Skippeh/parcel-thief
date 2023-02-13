use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::find_missions::FindMissionsRequest;

#[post("findMissions")]
pub async fn find_missions(request: Json<FindMissionsRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
