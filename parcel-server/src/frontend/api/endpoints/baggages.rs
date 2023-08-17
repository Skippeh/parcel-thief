use std::collections::HashMap;

use actix_web::{get, web::Data};
use anyhow::Context;
use parcel_common::api_types::{
    frontend::{
        accounts::GameAccountSummary,
        baggages::{
            Baggage, ListLostCargoResponse, ListSharedCargoResponse, ListWastedCargoResponse,
            LostCargoListItem, SharedCargoListItem, WastedCargoListItem,
        },
    },
    mission::{MissionType, OnlineMissionType, ProgressState},
};
use parcel_game_data::{ContentsType, GameData, Language};

use crate::{
    data::database::Database,
    frontend::{
        jwt_session::JwtSession,
        result::{ApiResponse, ApiResult},
    },
};

#[get("baggages/list/sharedCargo")]
pub async fn list_shared_cargo(
    _session: JwtSession,
    database: Data<Database>,
    game_data: Data<GameData>,
) -> ApiResult<ListSharedCargoResponse> {
    let conn = database.connect().await?;
    let missions = conn.missions(); // shared and lost cargo are saved as missions

    let data_missions = missions
        .find_missions(
            &[OnlineMissionType::Private, OnlineMissionType::Dynamic],
            &[MissionType::LostObject],
            &[], // no excluded accounts
            &[ProgressState::Available, ProgressState::Ready],
            None,
        )
        .await?
        .into_iter()
        .filter(|m| m.qpid_end_location == -1)
        .collect::<Vec<_>>();

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
            });
        }
    }

    ApiResponse::ok(ListSharedCargoResponse { baggages })
}

#[get("baggages/list/lostCargo")]
pub async fn list_lost_cargo(
    _session: JwtSession,
    database: Data<Database>,
    game_data: Data<GameData>,
) -> ApiResult<ListLostCargoResponse> {
    let conn = database.connect().await?;
    let missions = conn.missions();

    let data_missions = missions
        .find_missions(
            &[OnlineMissionType::Private, OnlineMissionType::Dynamic],
            &[MissionType::LostObject],
            &[], // no excluded accounts
            &[ProgressState::Available, ProgressState::Ready],
            None,
        )
        .await?
        .into_iter()
        .filter(|m| m.qpid_end_location != -1)
        .collect::<Vec<_>>();

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

            let target_location_name = game_data
                .qpid_area_name(mission.mission.qpid_end_location, Language::English)
                .map(|n| n.to_owned())
                .unwrap_or_else(|| mission.mission.qpid_end_location.to_string());

            baggages.push(LostCargoListItem {
                name: item_name,
                category,
                amount: baggage.amount,
                location: location_name,
                end_location: target_location_name,
                creator: creator.clone(),
            });
        }
    }

    ApiResponse::ok(ListLostCargoResponse { baggages })
}

#[get("baggages/list/wastedCargo")]
pub async fn list_wasted_cargo(
    _session: JwtSession,
    database: Data<Database>,
    game_data: Data<GameData>,
) -> ApiResult<ListWastedCargoResponse> {
    let conn = database.connect().await?;
    let wasteds = conn.wasted_baggages();
    let data_baggages = wasteds.get_all_baggages().await?;

    let mut account_ids = data_baggages
        .iter()
        .map(|baggage| baggage.creator_id.clone())
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

    let mut baggages = Vec::new();

    for baggage in data_baggages {
        let creator = accounts
            .get(&baggage.creator_id)
            .map(|acc| acc.display_name.clone())
            .unwrap_or_else(|| "Deleted account".into());

        let baggage_data = game_data.baggages.get(&(baggage.item_hash as u32));

        let item_name = game_data
            .baggage_name(baggage.item_hash as u32, Language::English)
            .map(|n| n.to_owned())
            .unwrap_or_else(|| baggage.item_hash.to_string());

        let category = baggage_data
            .map(|b| b.baggage_metadata.type_contents)
            .map(|t| format!("{:?}", t))
            .unwrap_or_else(|| "Unknown".into());

        let location_name = game_data
            .qpid_area_name(baggage.qpid_id, Language::English)
            .map(|n| n.to_owned())
            .unwrap_or_else(|| baggage.qpid_id.to_string());

        baggages.push(WastedCargoListItem {
            name: item_name,
            category,
            broken: baggage.broken,
            location: location_name,
            creator: creator.clone(),
        });
    }

    ApiResponse::ok(ListWastedCargoResponse { baggages })
}

#[get("baggages")]
pub async fn list_cargo(
    _session: JwtSession,
    database: Data<Database>,
    game_data: Data<GameData>,
) -> ApiResult<Vec<Baggage>> {
    let mut result = Vec::new();
    let conn = database.connect().await?;
    let missions = conn.missions();
    let wasteds = conn.wasted_baggages();
    let accounts = conn.accounts();

    let data_missions = missions
        .find_missions(
            &[OnlineMissionType::Private, OnlineMissionType::Dynamic],
            &[MissionType::LostObject],
            &[], // no excluded accounts
            &[ProgressState::Available, ProgressState::Ready],
            None,
        )
        .await?;
    let data_missions = missions.query_mission_data(data_missions).await?;

    log::debug!("{} missions", data_missions.len());

    for mission in data_missions {
        let creator = accounts
            .get_by_ids(&[&mission.mission.creator_id])
            .await?
            .into_iter()
            .next()
            .map(|acc| acc.display_name)
            .unwrap_or_else(|| "Deleted account".into());

        for baggage in mission.baggages {
            let mut item_name = game_data
                .baggage_name(baggage.name_hash as u32, Language::English)
                .map(|n| n.to_owned())
                .unwrap_or_else(|| baggage.name_hash.to_string());

            // Replace '{0}' with the amount
            item_name = item_name.replace("{0}", baggage.amount.to_string().as_str());

            let baggage_data = game_data.baggages.get(&(baggage.name_hash as u32));

            let category = baggage_data
                .map(|b| b.baggage_metadata.type_contents)
                .context("Missing baggage data")?;

            let target_location_id = if mission.mission.qpid_end_location == -1 {
                None
            } else {
                Some(mission.mission.qpid_end_location)
            };

            let target_location_name = game_data
                .qpid_area_name(mission.mission.qpid_end_location, Language::English)
                .map(|n| n.to_owned());

            result.push(Baggage {
                mission_id: mission.mission.id.clone(),
                id: baggage.id,
                creator: GameAccountSummary {
                    id: mission.mission.creator_id.clone(),
                    name: creator.clone(),
                },
                amount: baggage.amount,
                category,
                location: (
                    (baggage.x as f64 / 100f64) as f32,
                    (baggage.y as f64 / 100f64) as f32,
                    (baggage.z as f64 / 100f64) as f32,
                ),
                name: item_name,
                target_location_id,
                target_location_name,
                is_broken: false,
                is_wasted: false,
            })
        }
    }

    ApiResponse::ok(result)
}
