use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::create_road::CreateRoadRequest;

#[post("/createRoad")]
pub async fn create_road(request: Json<CreateRoadRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("internal error")
}
