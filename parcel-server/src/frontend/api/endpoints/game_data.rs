use actix_web::{get, web::Data};
use parcel_game_data::{GameData, QpidArea};

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
