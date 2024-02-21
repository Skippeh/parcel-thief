use std::{borrow::Cow, collections::HashMap};

use actix_web::{
    get, post,
    web::{Data, Json},
};
use diesel_async::scoped_futures::ScopedFutureExt;
use parcel_common::api_types::{
    frontend::missions::{BaggageAmount, EditMissionData},
    mission::{Baggage as MissionBaggage, MissionType, OnlineMissionType},
    requests::add_missions::NewMission,
};
use parcel_game_data::{Baggage, GameData};
use validator::{ValidationError, ValidationErrors};

use crate::{
    data::database::Database,
    db::models::custom_mission::CustomMissionType,
    frontend::{
        error::ApiError,
        jwt_session::JwtSession,
        result::{ApiResponse, ApiResult},
    },
};

#[post("missions")]
pub async fn create_mission(
    session: JwtSession,
    mission_data: Json<EditMissionData>,
    game_data: Data<GameData>,
    database: Data<Database>,
) -> ApiResult<()> {
    log::info!("create_mission:\n{:#?}", mission_data);

    match mission_data.into_inner() {
        EditMissionData::Delivery {
            start_qpid_id,
            end_qpid_id,
            baggage_amounts,
            reward_amounts,
        } => {
            let mut validation_errors = ValidationErrors::new();
            let mut baggages = HashMap::new();
            let mut unknown_cargo = Vec::new();
            let mut unknown_rewards = Vec::new();

            find_add_baggages(
                &game_data,
                baggage_amounts.iter().map(|b| b.name_hash),
                &mut baggages,
                &mut unknown_cargo,
            );
            find_add_baggages(
                &game_data,
                reward_amounts.iter().map(|b| b.name_hash),
                &mut baggages,
                &mut unknown_rewards,
            );

            if !unknown_cargo.is_empty() {
                let mut error = ValidationError::new("unknownHashes");
                error.add_param(Cow::Borrowed("hashes"), &unknown_cargo);
                validation_errors.add("baggageAmounts", error);
            }

            if !unknown_rewards.is_empty() {
                let mut error = ValidationError::new("unknownHashes");
                error.add_param(Cow::Borrowed("hashes"), &unknown_rewards);
                validation_errors.add("rewardAmounts", error);
            }

            let start_qpid_area = game_data.qpid_areas.get(&start_qpid_id);
            let end_qpid_area = game_data.qpid_areas.get(&end_qpid_id);

            if start_qpid_area.is_none() {
                validation_errors.add("startQpidId", ValidationError::new("unknownId"));
            }

            if end_qpid_area.is_none() {
                validation_errors.add("endQpidId", ValidationError::new("unknownId"));
            }

            let area_hash = start_qpid_area.unwrap().metadata.area.try_into();

            if area_hash.is_err() {
                validation_errors.add("area", ValidationError::new("unknownArea"));
            }

            if !validation_errors.is_empty() {
                return Err(ApiError::ValidationErrors(validation_errors));
            }

            let start_qpid_area = start_qpid_area.unwrap();
            let area_hash = area_hash.unwrap();

            // todo: Add new custom mission to database
            let conn = database.connect().await?;

            conn.transaction(|conn| {
                async {
                    let account = conn.accounts().get_or_create_server_account().await?;
                    let custom_missions = conn.custom_missions();
                    let mission = custom_missions
                        .create_mission(CustomMissionType::Delivery, session.account_id)
                        .await?;

                    let game_missions = conn.missions();
                    // todo: for each baggage * amount: create game mission in shared box at start_qpid_id
                    for BaggageAmount { name_hash, amount } in &baggage_amounts {
                        for _ in 0..*amount {
                            game_missions
                                .save_mission(
                                    &NewMission {
                                        area_hash,
                                        qpid_id: start_qpid_id,
                                        qpid_start_location: start_qpid_id,
                                        qpid_end_location: end_qpid_id,
                                        mission_static_id: 0,
                                        mission_type: MissionType::LostObject,
                                        online_mission_type: OnlineMissionType::Private,
                                        supply_info: None,
                                        dynamic_start_info: None,
                                        dynamic_end_info: None,
                                        dynamic_mission_info: None,
                                        catapult_shell_info: None,
                                        baggages: Some(vec![MissionBaggage {
                                            amount: *amount as i32,
                                            name_hash: *name_hash as i32,
                                            user_index: 0,
                                            x: (start_qpid_area.metadata.location.0 * 100.).round()
                                                as i32,
                                            y: (start_qpid_area.metadata.location.1 * 100.).round()
                                                as i32,
                                            z: (start_qpid_area.metadata.location.2 * 100.).round()
                                                as i32,
                                            is_returned: false,
                                            ammo_info: None,
                                        }]),
                                    },
                                    &account.id,
                                    Some(mission.id),
                                )
                                .await?;
                        }
                    }

                    // todo: save reward baggages

                    Ok(())
                }
                .scope_boxed()
            })
            .await?;

            ApiResponse::ok(())
        }
        EditMissionData::Collection {
            target_qpid_id,
            baggage_amounts,
            reward_amounts,
        } => todo!(),
        EditMissionData::Recovery {
            target_qpid_id,
            baggages,
            reward_amounts,
        } => todo!(),
    }
}

#[get("missions")]
pub async fn get_missions(session: JwtSession) -> ApiResult<()> {
    ApiResponse::ok(())
}

/// Map baggages by name hash. Hashes not found are ignored.
fn find_add_baggages(
    game_data: &GameData,
    hashes: impl Iterator<Item = u32>,
    map: &mut HashMap<String, Baggage>,
    unknown_hashes: &mut Vec<u32>,
) {
    for hash in hashes {
        if let Some(b) = game_data.baggages.get(&hash) {
            map.insert(b.name_hash.to_string(), b.clone());
        } else {
            unknown_hashes.push(hash);
        }
    }
}
