use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::get_road_data::GetRoadDataRequest;

#[post("e/getRoadData")]
pub async fn get_road_data(request: Json<GetRoadDataRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
