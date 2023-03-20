use actix_web::{
    post,
    web::{Data, Json},
};
use chrono::NaiveDateTime;
use parcel_common::api_types::requests::get_like_history::{
    GetLikeHistoryRequest, GetLikeHistoryResponse,
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("getLikeHistory")]
pub async fn get_like_history(
    request: Json<GetLikeHistoryRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<Json<GetLikeHistoryResponse>, InternalError> {
    let conn = database.connect()?;
    let likes = conn.likes();
    let given_likes;

    if request.since <= 0 {
        given_likes = likes.get_unacknowleged_likes(&session.account_id).await?;
    } else {
        let since = NaiveDateTime::from_timestamp_millis(request.since)
            .ok_or_else(|| anyhow::anyhow!("Could not convert since to datetime"))?;
        given_likes = likes.get_likes_since(&session.account_id, &since).await?;
    }

    // todo: implement the rest

    Err(anyhow::anyhow!("Not implemented").into())
}
