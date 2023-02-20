use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::get_ordered_missions::GetOrderedMissionsRequest;

#[post("getOrderedMissions")]
pub async fn get_ordered_missions(
    request: Option<Json<GetOrderedMissionsRequest>>,
) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
