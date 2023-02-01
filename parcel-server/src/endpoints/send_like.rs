use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::send_like::SendLikeRequest;

#[post("e/sendLike")]
pub async fn send_like(request: Json<SendLikeRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
