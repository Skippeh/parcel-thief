use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::find_qpid_objects::FindQpidObjectsRequest;

#[post("e/findQpidObjects")]
pub async fn find_qpid_objects(request: Json<FindQpidObjectsRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
