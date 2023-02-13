use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::set_strand::SetStrandRequest;

#[post("setStrand")]
pub async fn set_strand(request: Json<SetStrandRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
