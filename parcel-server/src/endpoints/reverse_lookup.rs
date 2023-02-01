use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::reverse_lookup::ReverseLookupRequest;

#[post("e/reverseLookup")]
pub async fn reverse_lookup(request: Json<ReverseLookupRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
