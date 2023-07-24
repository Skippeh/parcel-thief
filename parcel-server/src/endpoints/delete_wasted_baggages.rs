use actix_web::{
    post,
    web::{Data, Json},
};
use parcel_common::api_types::requests::delete_wasted_baggages::DeleteWastedBaggagesRequest;

use crate::{data::database::Database, session::Session};

use super::{EmptyResponse, InternalError};

#[post("deleteWastedBaggages")]
pub async fn delete_wasted_baggages(
    request: Json<DeleteWastedBaggagesRequest>,
    _session: Session,
    database: Data<Database>,
) -> Result<EmptyResponse, InternalError> {
    let db = database.connect().await?;
    let wasted_baggages = db.wasted_baggages();

    wasted_baggages
        .delete_by_requests(request.0.delete_requests.iter())
        .await?;

    Ok(EmptyResponse)
}
