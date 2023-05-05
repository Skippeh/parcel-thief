use actix_web::{
    post,
    web::{Data, Json},
};
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
    let db = database.connect()?;
    let accounts = db.accounts();

    // todo: implement a way to run queries from DatabaseConnection within a transaction
    // so that we can call multiple functions from database "helpers" without risking
    // data loss/corruption when only a partial amount of data is updated

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

    Ok(EmptyResponse)
}
