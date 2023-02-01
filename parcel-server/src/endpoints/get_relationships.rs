use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::get_relationships::GetRelationshipsRequest;

#[post("/getRelationships")]
pub async fn get_relationships(request: Json<GetRelationshipsRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
