use actix_web::{get, web::Json};
use parcel_common::api_types::requests::get_version::GetVersionResponse;

use crate::session::Session;

#[get("/getVersion")]
pub async fn get_version(_session: Session) -> Json<GetVersionResponse> {
    Json(GetVersionResponse::current_version())
}
