use actix_web::{post, web::Json, Responder};
use parcel_common::api_types::requests::reverse_lookup::{
    ReverseLookupRequest, ReverseLookupResponse,
};

use crate::session::Session;

#[post("reverseLookup")]
pub async fn reverse_lookup(
    request: Json<ReverseLookupRequest>,
    _session: Session,
) -> impl Responder {
    Json(ReverseLookupResponse {
        account_ids: Vec::new(),
    })
}
