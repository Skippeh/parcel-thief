use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::get_like_history::GetLikeHistoryRequest;

#[post("/getLikeHistory")]
pub async fn get_like_history(request: Json<GetLikeHistoryRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
