use actix_web::{
    get,
    web::{Data, Query},
};
use parcel_common::api_types::frontend::baggages::{
    ListLostBaggagesResponse, LocalizedBaggageData,
};
use parcel_game_data::{GameData, Language, QpidArea};
use serde::Deserialize;

use crate::frontend::{
    jwt_session::JwtSession,
    result::{ApiResponse, ApiResult},
};

#[get("gameData/qpidAreas")]
pub async fn list_qpid_areas(
    _session: JwtSession,
    game_data: Data<GameData>,
) -> ApiResult<Vec<QpidArea>> {
    ApiResponse::ok(game_data.qpid_areas.values().map(|q| q.clone()).collect())
}

#[derive(Deserialize)]
pub struct LostBaggagesQuery {
    #[serde(rename = "lang")]
    language: Language,
}

#[get("gameData/lostBaggages")]
pub async fn list_lost_baggages(
    _session: JwtSession,
    game_data: Data<GameData>,
    query: Query<LostBaggagesQuery>,
) -> ApiResult<ListLostBaggagesResponse> {
    let qpid_ids = game_data
        .qpid_areas
        .values()
        .map(|q| q.qpid_id)
        .collect::<Vec<_>>();

    let qpid_baggages = game_data
        .get_lost_baggages(&qpid_ids)
        .into_iter()
        // clone baggages
        .map(|(qpid_id, baggages)| {
            (
                qpid_id,
                baggages
                    .into_iter()
                    .map(|b| LocalizedBaggageData::from_baggage_data(b.clone(), query.language))
                    .collect(),
            )
        })
        .collect();

    ApiResponse::ok(ListLostBaggagesResponse {
        qpid_baggages,
        generic_baggages: vec![],
    })
}
