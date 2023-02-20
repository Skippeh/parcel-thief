use actix_web::{post, web::Json, Responder};
use parcel_common::api_types::requests::get_player_ranking_records::{
    GetPlayerRankingRecordsRequest, GetPlayerRankingRecordsResponse,
};

#[post("getPlayerRankingRecords")]
pub async fn get_player_ranking_records(
    request: Json<GetPlayerRankingRecordsRequest>,
) -> impl Responder {
    Json(GetPlayerRankingRecordsResponse {
        update_time: 0,
        rewards: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        records: Vec::new(),
    })
}
