use actix_web::{post, web::Json, Responder};
use chrono::Utc;
use parcel_common::api_types::{
    rank::RankingRewards,
    requests::get_player_ranking_records::{
        GetPlayerRankingRecordsRequest, GetPlayerRankingRecordsResponse,
    },
};

#[post("getPlayerRankingRecords")]
pub async fn get_player_ranking_records(
    _request: Json<GetPlayerRankingRecordsRequest>,
) -> impl Responder {
    Json(GetPlayerRankingRecordsResponse {
        update_time: Utc::now().timestamp_millis(),
        rewards: RankingRewards {
            medals: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
        records: Vec::new(),
    })
}
