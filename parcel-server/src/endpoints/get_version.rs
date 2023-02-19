use actix_web::{get, HttpResponse, Responder};
use parcel_common::api_types::requests::get_version::GetVersionResponse;

use crate::session::Session;

#[get("/getVersion")]
pub async fn get_version(_session: Session) -> impl Responder {
    HttpResponse::Ok().json(GetVersionResponse::current_version())
}
