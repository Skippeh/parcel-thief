use actix_web::{post, web::Json, Responder};
use chrono::Utc;
use parcel_common::api_types::requests::get_ranking_schedules::{
    GetRankingSchedulesRequest, GetRankingSchedulesResponse,
};

#[post("getRankingSchedules")]
pub async fn get_ranking_schedules(_request: Json<GetRankingSchedulesRequest>) -> impl Responder {
    Json(GetRankingSchedulesResponse {
        updated_time: Utc::now().timestamp_millis(),
        schedules: Vec::new(),
    })
}
