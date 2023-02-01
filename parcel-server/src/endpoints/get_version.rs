use actix_web::{post, HttpResponse, Responder};

#[post("/getVersion")]
pub async fn get_version() -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
