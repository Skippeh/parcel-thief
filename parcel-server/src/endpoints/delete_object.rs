use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::delete_object::DeleteObjectRequest;

#[post("/deleteObject")]
pub async fn delete_object(request: Json<DeleteObjectRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
