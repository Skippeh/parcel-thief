use actix_web::{
    post,
    web::{Data, Json},
};

use parcel_common::api_types::requests::reverse_lookup::{
    ReverseLookupRequest, ReverseLookupResponse,
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

/// This request takes a list of provider account ids and returns the account id.
///
/// If an account id doesn't have an account it's not added to the response.
#[post("reverseLookup")]
pub async fn reverse_lookup(
    request: Json<ReverseLookupRequest>,
    _session: Session,
    database: Data<Database>,
) -> Result<Json<ReverseLookupResponse>, InternalError> {
    let conn = database.connect().await?;
    let accounts = conn.accounts();

    let account_ids = accounts
        .get_by_provider_ids(request.provider, &request.provider_account_ids)
        .await?
        .into_iter()
        .map(|acc| acc.id)
        .collect();

    Ok(Json(ReverseLookupResponse { account_ids }))
}
