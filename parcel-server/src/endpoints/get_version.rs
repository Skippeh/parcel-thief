use actix_web::{post, HttpResponse, Responder};
use parcel_common::api_types::requests::get_version::GetVersionResponse;

#[post("/getVersion")]
pub async fn get_version() -> impl Responder {
    HttpResponse::Ok().json(GetVersionResponse::current_version())
}
