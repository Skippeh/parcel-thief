use std::collections::HashMap;

use actix_web::{get, web::Data};
use parcel_common::api_types::{
    frontend::baggages::{BaggageListItem, ListSharedCargoResponse},
    mission::{MissionType, OnlineMissionType, ProgressState},
};
use parcel_game_data::{GameData, Language};

use crate::{
    data::database::Database,
    frontend::{
        jwt_session::JwtSession,
        result::{ApiResponse, ApiResult},
    },
};

#[get("baggages/sharedCargo")]
pub async fn list_shared_cargo(
    _session: JwtSession,
    database: Data<Database>,
    game_data: Data<GameData>,
) -> ApiResult<ListSharedCargoResponse> {
    let conn = database.connect()?;
    let missions = conn.missions(); // shared and lost cargo are saved as missions

    const ONLINE_MISSION_TYPES: &[OnlineMissionType] = &[OnlineMissionType::Private]; // private = shared cargo, dynamic = lost cargo. we only want shared cargo here
    const MISSION_TYPES: &[MissionType] = &[MissionType::LostObject];
    const PROGRESS_STATES: &[ProgressState] = &[ProgressState::Available, ProgressState::Ready];

    let data_missions = missions
        .find_missions(
            &ONLINE_MISSION_TYPES,
            &MISSION_TYPES,
            &[], // no excluded accounts
            &PROGRESS_STATES,
            None,
        )
        .await?;

    let mut account_ids = data_missions
        .iter()
        .map(|mission| mission.creator_id.clone())
        .collect::<Vec<_>>();

    // Remove duplicate ids (sort first otherwise dedup doesn't work)
    account_ids.sort_unstable();
    account_ids.dedup();

    let accounts = conn.accounts();
    let accounts = accounts
        .get_by_ids(&account_ids)
        .await?
        .into_iter()
        .map(|acc| (acc.id.clone(), acc))
        .collect::<HashMap<_, _>>();

    let data_missions = missions.query_mission_data(data_missions).await?;
    let mut baggages = Vec::new();

    for mission in data_missions {
        let creator = accounts
            .get(&mission.mission.creator_id)
            .map(|acc| acc.display_name.clone())
            .unwrap_or_else(|| "Deleted account".into());

        for baggage in mission.baggages {
            let baggage_data = game_data.baggages.get(&(baggage.name_hash as u32));

            let mut item_name = baggage_data
                .map(|b| b.names.get(&Language::English).map(|n| n.clone()))
                .flatten()
                .unwrap_or_else(|| baggage.name_hash.to_string());

            item_name = item_name.replace("{0}", baggage.amount.to_string().as_str());

            let category = baggage_data
                .map(|b| b.baggage_metadata.type_contents)
                .map(|t| format!("{:?}", t))
                .unwrap_or_else(|| "Unknown".into());

            let location_name = game_data
                .qpid_areas
                .get(&mission.mission.qpid_id)
                .map(|a| a.names.get(&Language::English).map(|n| n.clone()))
                .flatten()
                .unwrap_or_else(|| mission.mission.qpid_id.to_string());

            baggages.push(BaggageListItem {
                name: item_name,
                category,
                amount: baggage.amount,
                location: location_name,
                creator: creator.clone(),
            })
        }
    }

    ApiResponse::ok(ListSharedCargoResponse { baggages })
}
