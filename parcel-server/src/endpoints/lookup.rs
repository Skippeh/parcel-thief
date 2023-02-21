use actix_web::{
    post,
    web::{Data, Json},
};
use parcel_common::api_types::requests::lookup::{LookupRequest, LookupResponse, LookupUserInfo};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

/// This request takes a list of account ids and returns the account info for them.
///
/// If one of the provided ids don't have an account it's not added to the response.
#[post("lookup")]
pub async fn lookup(
    request: Json<LookupRequest>,
    _session: Session,
    database: Data<Database>,
) -> Result<Json<LookupResponse>, InternalError> {
    let conn = database.connect()?;
    let users = conn
        .accounts()
        .get_by_ids(&request.account_ids)
        .await?
        .into_iter()
        .map(|acc| LookupUserInfo {
            account_id: acc.id,
            display_name: acc.display_name,
            provider_account_id: acc.provider_id,
            provider: acc.provider,
        })
        .collect();

    Ok(Json(LookupResponse { users }))
}
