use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::devote_highway_resources::DevoteHighwayResourcesRequest;

#[post("devoteHighwayResources")]
async fn devote_highway_resources(request: Json<DevoteHighwayResourcesRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
