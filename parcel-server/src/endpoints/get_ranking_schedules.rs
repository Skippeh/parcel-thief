use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::get_ranking_schedules::{
    GetRankingSchedulesRequest, GetRankingSchedulesResponse,
};

#[post("getRankingSchedules")]
pub async fn get_ranking_schedules(request: Json<GetRankingSchedulesRequest>) -> impl Responder {
    Json(GetRankingSchedulesResponse {
        updated_time: 0,
        schedules: Vec::new(),
    })
}
