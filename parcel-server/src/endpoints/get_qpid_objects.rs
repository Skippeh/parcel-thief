use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::get_qpid_objects::GetQpidObjectsRequest;

#[post("e/getQpidObjects")]
pub async fn get_qpid_objects(request: Json<GetQpidObjectsRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
