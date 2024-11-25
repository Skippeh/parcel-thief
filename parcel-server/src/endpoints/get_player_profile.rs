use std::collections::HashMap;

use actix_web::{
    post,
    web::{Data, Json},
};
use parcel_common::api_types::{
    auth::Provider,
    player_profile::BasicPlayerProfile,
    requests::get_player_profile::{
        GetPlayerProfileRequest, GetPlayerProfileResponse, ProfileResponse,
    },
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("getPlayerProfile")]
pub async fn get_player_profile(
    request: Json<GetPlayerProfileRequest>,
    database: Data<Database>,
    _session: Session,
) -> Result<Json<GetPlayerProfileResponse>, InternalError> {
    if request.profiles.is_empty() {
        return Ok(Json(GetPlayerProfileResponse { profiles: vec![] }));
    }

    let conn = database.connect().await?;
    let accounts = conn.accounts();
    let profiles = conn.player_profiles();

    // Make sure all flags equal 1 or return error
    // (Not sure if flags are ever any other value)
    for profile in &request.profiles {
        if profile.flags != 1 {
            return Err(anyhow::anyhow!("Unknown profile flags: {}", profile.flags).into());
        }
    }

    // The owned string is the account id, and the borrowed string is the request id, which is returned in the response
    let mut account_ids = HashMap::<String, &str>::new();

    // non account ids to query, grouped by provider type and coupled with their original request id (used in response)
    // the hashmap key and value is (provider_id, original_id)
    let mut id_queries = HashMap::<Provider, HashMap<&str, &str>>::new();

    for profile in &request.profiles {
        if let Some((provider_str, id)) = profile.id.split_once('_') {
            match provider_str {
                "zygo" => {
                    account_ids.insert(profile.id.clone(), &profile.id);
                }
                "steam" => {
                    id_queries
                        .entry(Provider::Steam)
                        .or_default()
                        .insert(id, &profile.id);
                }
                "epic" => {
                    id_queries
                        .entry(Provider::Epic)
                        .or_default()
                        .insert(id, &profile.id);
                }
                other => return Err(anyhow::anyhow!("Unexpected id type: {:?}", other).into()),
            }
        }
    }

    // Query for account ids
    for (provider, provider_ids_with_original_id) in &id_queries {
        let provider_ids: Vec<&str> = provider_ids_with_original_id
            .iter()
            .map(|(provider_id, _)| *provider_id)
            .collect();

        for account in accounts
            .get_by_provider_ids(*provider, &provider_ids)
            .await?
        {
            let original_id = *provider_ids_with_original_id
                .get(account.provider_id.as_str())
                .unwrap();
            account_ids.insert(account.id, original_id);
        }
    }

    // Query db and transform results into response types
    let profiles: Vec<ProfileResponse> = profiles
        .get_by_account_ids(
            &account_ids
                .keys()
                .map(|id| id.as_str())
                .collect::<Vec<&str>>(),
        )
        .await?
        .into_iter()
        .map(|profile| {
            let original_id = account_ids
                .get(&profile.account_id)
                .map(|s| (*s).to_owned())
                .unwrap();
            ProfileResponse {
                basic: BasicPlayerProfile::from(profile),
                id: original_id,
            }
        })
        .collect();

    Ok(Json(GetPlayerProfileResponse { profiles }))
}
