use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::delete_missions::DeleteMissionsRequest;

#[post("/deleteMissions")]
pub async fn delete_missions(request: Json<DeleteMissionsRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
