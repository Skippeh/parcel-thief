use actix_web::{post, web::Json, Responder};
use parcel_common::api_types::requests::get_relationships::GetRelationshipsResponse;

use crate::session::Session;

#[post("getRelationships")]
pub async fn get_relationships(_session: Session) -> impl Responder {
    Json(GetRelationshipsResponse {
        history: Vec::new(),
        strand_contracts: Vec::new(),
    })
}
