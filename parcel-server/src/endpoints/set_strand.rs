use actix_web::{
    post,
    web::{Data, Json},
};
use diesel_async::scoped_futures::ScopedFutureExt;
use parcel_common::api_types::requests::set_strand::SetStrandRequest;

use crate::{
    data::database::Database,
    endpoints::{EmptyResponse, InternalError},
    session::Session,
};

#[post("setStrand")]
pub async fn set_strand(
    request: Json<SetStrandRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<EmptyResponse, InternalError> {
    let db = database.connect().await?;

    db.transaction(|db| {
        async move {
            let accounts = db.accounts();

            if let Some(add) = &request.0.add_account_ids {
                accounts
                    .add_strand_contracts(&session.account_id, add.iter().map(|id| id as &str))
                    .await?;
            }

            if let Some(del) = &request.0.del_account_ids {
                accounts
                    .remove_strand_contracts(&session.account_id, del.iter().map(|id| id as &str))
                    .await?;
            }

            Ok(())
        }
        .scope_boxed()
    })
    .await?;

    Ok(EmptyResponse)
}
