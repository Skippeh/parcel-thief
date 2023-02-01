use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::get_wasted_baggages::GetWastedBaggagesRequest;

#[post("e/getWastedBaggages")]
pub async fn get_wasted_baggages(request: Json<GetWastedBaggagesRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
