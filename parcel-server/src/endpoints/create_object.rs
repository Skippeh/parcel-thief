use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::create_object::CreateObjectRequest;

#[post("e/createObject")]
pub async fn create_object(request: Json<CreateObjectRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
