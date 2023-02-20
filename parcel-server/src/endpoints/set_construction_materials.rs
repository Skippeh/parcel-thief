use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::set_construction_materials::SetConstructionMaterialsRequest;

#[post("setConstructionMaterials")]
pub async fn set_construction_materials(
    request: Json<SetConstructionMaterialsRequest>,
) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
