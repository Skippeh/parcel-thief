use actix_web::{post, web::Json, HttpResponse, Responder};
use parcel_common::api_types::requests::get_player_ranking_records::GetPlayerRankingRecordsRequest;

#[post("/getPlayerRankingRecords")]
pub async fn get_player_ranking_records(
    request: Json<GetPlayerRankingRecordsRequest>,
) -> impl Responder {
    HttpResponse::InternalServerError().body("not implemented")
}
