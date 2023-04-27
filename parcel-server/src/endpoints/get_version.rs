use actix_web::{get, web::Json};
use parcel_common::api_types::requests::get_version::GetVersionResponse;

use crate::session::Session;

#[get("/getVersion")]
pub async fn get_version(_session: Session) -> Json<GetVersionResponse> {
    Json(GetVersionResponse {
        domain: "prod".into(),
        major: 0,
        minor: 12,
        version: "0.0.1".into(),
        zone: "ds".into(),
    })
}
