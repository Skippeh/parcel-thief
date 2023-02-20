use actix_web::{post, web::Json, Responder};
use parcel_common::api_types::requests::get_wasted_baggages::{
    GetWastedBaggagesRequest, GetWastedBaggagesResponse,
};

use crate::session::Session;

#[post("getWastedBaggages")]
pub async fn get_wasted_baggages(
    request: Json<GetWastedBaggagesRequest>,
    _session: Session,
) -> impl Responder {
    Json(GetWastedBaggagesResponse {
        update_date: 0,
        baggages: Vec::new(),
    })
}
