use std::{
    cmp::min,
    collections::{hash_map::Entry, HashMap, HashSet},
};

use actix_web::{
    post,
    web::{Data, Json},
};
use chrono::NaiveDateTime;
use parcel_common::api_types::requests::get_like_history::{
    GetLikeHistoryRequest, GetLikeHistoryResponse, LikeHistory,
};

use crate::{
    data::database::Database, db::models::like::Like, endpoints::InternalError, session::Session,
};

#[post("getLikeHistory")]
pub async fn get_like_history(
    request: Json<GetLikeHistoryRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<Json<GetLikeHistoryResponse>, InternalError> {
    let conn = database.connect()?;
    let likes = conn.likes();

    let given_likes = if request.since <= 0 {
        likes.get_unacknowleged_likes(&session.account_id).await?
    } else {
        let since = NaiveDateTime::from_timestamp_millis(request.since)
            .ok_or_else(|| anyhow::anyhow!("Could not convert since to datetime"))?;
        likes.get_likes_since(&session.account_id, &since).await?
    };

    // update likes to set acknowledged = true
    let like_ids = given_likes.iter().map(|like| like.id).collect::<Vec<_>>();
    likes.set_acknowledged(&like_ids, true).await?;

    // merge likes that have identical from_id, to_id, like_type, and online_id
    let mut merged_likes = HashMap::new();

    for like in given_likes {
        let key = (
            like.from_id.clone(),
            like.to_id.clone(),
            like.ty.clone(),
            like.online_id.clone(),
        );

        match merged_likes.entry(key) {
            Entry::Occupied(val) => {
                let val: &mut Like = val.into_mut();
                val.likes_auto += like.likes_auto;
                val.likes_manual += like.likes_manual;

                // Keep the time of the latest like
                if like.time > val.time {
                    val.time = like.time;
                }
            }
            Entry::Vacant(val) => {
                val.insert(like);
            }
        }
    }

    let mut given_likes = merged_likes.into_values().collect::<Vec<_>>();

    // sort by most likes
    given_likes.sort_unstable_by_key(|like| like.total_likes());

    let result = given_likes
        .drain(..min(given_likes.len(), 5))
        .map(|like| like.into_api_type())
        .collect::<Vec<_>>();

    let mut summarized_like = LikeHistory {
        account_id: "".into(),
        like_type: "".into(),
        likes_auto: 0,
        likes_manual: 0,
        online_id: "".into(),
        summarized_player_count: 0,
        time: 0,
    };
    let mut user_ids = HashSet::new();

    for like in given_likes {
        summarized_like.likes_auto += like.likes_auto;
        summarized_like.likes_manual += like.likes_manual;

        // update timestamp if it's newer
        let timestamp_millis = like.time.timestamp_millis();
        if timestamp_millis > summarized_like.time {
            summarized_like.time = timestamp_millis;
        }

        // add unique user ids to summarized_player_count
        if !user_ids.contains(&like.from_id) {
            user_ids.insert(like.from_id);
            summarized_like.summarized_player_count += 1;
        }
    }

    Ok(Json(GetLikeHistoryResponse {
        like_histories: result,
    }))
}
