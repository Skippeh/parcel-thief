use actix_web::{
    post,
    web::{Data, Json},
};
use parcel_common::api_types::requests::put_wasted_baggages::PutWastedBaggagesRequest;

use crate::{
    data::database::Database,
    endpoints::{EmptyResponse, InternalError},
    session::Session,
};

#[post("putWastedBaggages")]
pub async fn put_wasted_baggages(
    request: Json<PutWastedBaggagesRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<EmptyResponse, InternalError> {
    let conn = database.connect().await?;
    let wasted_baggages = conn.wasted_baggages();

    wasted_baggages
        .add_wasted_items(
            request.0.qpid_id,
            &session.account_id,
            &request.0.wasted_items,
        )
        .await?;

    Ok(EmptyResponse)
}
