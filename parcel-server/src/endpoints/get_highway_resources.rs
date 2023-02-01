use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::get_highway_resources::GetHighwayResourcesRequest;

#[post("getHighwayResources")]
pub async fn get_highway_resources(request: Json<GetHighwayResourcesRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
