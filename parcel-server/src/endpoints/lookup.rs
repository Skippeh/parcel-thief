use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::lookup::LookupRequest;

#[post("e/lookup")]
pub async fn lookup(request: Json<LookupRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
