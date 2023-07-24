use actix_web::{
    post,
    web::{Data, Json},
};
use parcel_common::api_types::{
    requests::get_relationships::GetRelationshipsResponse, IntoDsApiType,
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("getRelationships")]
pub async fn get_relationships(
    session: Session,
    database: Data<Database>,
) -> Result<Json<GetRelationshipsResponse>, InternalError> {
    let db = database.connect().await?;
    let accounts = db.accounts();

    const HISTORY_LIMIT: i64 = 10;
    let history = accounts
        .get_relationship_history(&session.account_id, Some(HISTORY_LIMIT))
        .await?
        .into_iter()
        .map(|history| history.into_ds_api_type())
        .collect::<Vec<_>>();

    let strand_contracts = accounts
        .get_strand_contracts(&session.account_id)
        .await?
        .into_iter()
        .map(|strand_contract| strand_contract.into_ds_api_type())
        .collect::<Vec<_>>();

    Ok(Json(GetRelationshipsResponse {
        history,
        strand_contracts,
    }))
}
