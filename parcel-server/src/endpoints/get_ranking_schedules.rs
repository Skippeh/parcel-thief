use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::get_ranking_schedules::GetRankingSchedulesRequest;

#[post("e/getRankingSchedules")]
pub async fn get_ranking_schedules(request: Json<GetRankingSchedulesRequest>) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
