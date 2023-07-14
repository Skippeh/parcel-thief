use std::collections::HashMap;

use actix_web::{get, web::Data};
use parcel_common::api_types::{
    frontend::baggages::{
        ListLostCargoResponse, ListSharedCargoResponse, LostCargoListItem, SharedCargoListItem,
    },
    mission::{MissionType, OnlineMissionType, ProgressState},
};
use parcel_game_data::{GameData, Language};

use crate::{
    data::database::Database,
    frontend::{
        error::ApiError,
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

    let data_missions = missions
        .find_missions(
            &[OnlineMissionType::Private], // private = shared cargo, dynamic = lost cargo. we only want shared cargo here
            &[MissionType::LostObject],
            &[], // no excluded accounts
            &[ProgressState::Available, ProgressState::Ready],
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

            let mut item_name = game_data
                .baggage_name(baggage.name_hash as u32, Language::English)
                .map(|n| n.to_owned())
                .unwrap_or_else(|| baggage.name_hash.to_string());

            // Replace '{0}' with the amount
            item_name = item_name.replace("{0}", baggage.amount.to_string().as_str());

            let category = baggage_data
                .map(|b| b.baggage_metadata.type_contents)
                .map(|t| format!("{:?}", t))
                .unwrap_or_else(|| "Unknown".into());

            let location_name = game_data
                .qpid_area_name(mission.mission.qpid_id, Language::English)
                .map(|n| n.to_owned())
                .unwrap_or_else(|| mission.mission.qpid_id.to_string());

            baggages.push(SharedCargoListItem {
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

#[get("baggages/lostCargo")]
pub async fn list_lost_cargo(
    _session: JwtSession,
    database: Data<Database>,
    game_data: Data<GameData>,
) -> ApiResult<ListLostCargoResponse> {
    let conn = database.connect()?;
    let missions = conn.missions();

    let data_missions = missions
        .find_missions(
            &[OnlineMissionType::Dynamic], // private = shared cargo, dynamic = lost cargo. we only want lost cargo here
            &[MissionType::LostObject],
            &[], // no excluded accounts
            &[ProgressState::Available, ProgressState::Ready],
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

            let mut item_name = game_data
                .baggage_name(baggage.name_hash as u32, Language::English)
                .map(|n| n.to_owned())
                .unwrap_or_else(|| baggage.name_hash.to_string());

            // Replace '{0}' with the amount
            item_name = item_name.replace("{0}", baggage.amount.to_string().as_str());

            let category = baggage_data
                .map(|b| b.baggage_metadata.type_contents)
                .map(|t| format!("{:?}", t))
                .unwrap_or_else(|| "Unknown".into());

            let location_name = game_data
                .qpid_area_name(mission.mission.qpid_id, Language::English)
                .map(|n| n.to_owned())
                .unwrap_or_else(|| mission.mission.qpid_id.to_string());

            baggages.push(LostCargoListItem {
                name: item_name,
                category,
                amount: baggage.amount,
                location: location_name,
                creator: creator.clone(),
            })
        }
    }

    ApiResponse::ok(ListLostCargoResponse { baggages })
}
