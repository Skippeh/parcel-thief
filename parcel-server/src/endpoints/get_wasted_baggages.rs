use actix_web::{
    post,
    web::{Data, Json},
};
use anyhow::Context;
use parcel_common::api_types::{
    requests::get_wasted_baggages::{GetWastedBaggagesRequest, GetWastedBaggagesResponse},
    IntoDsApiType,
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("getWastedBaggages")]
pub async fn get_wasted_baggages(
    request: Json<GetWastedBaggagesRequest>,
    _session: Session,
    database: Data<Database>,
) -> Result<Json<GetWastedBaggagesResponse>, InternalError> {
    let conn = database.connect().await?;
    let wasted_baggages = conn.wasted_baggages();
    let mut baggages = Vec::new();

    for qpid_id in request.0.qpid_ids {
        let last_date = if qpid_id.last_login_time > 0 {
            Some(
                chrono::DateTime::from_timestamp_millis(qpid_id.last_login_time)
                    .context("Date out of range")?
                    .naive_utc(),
            )
        } else {
            None
        };
        let qpid_id = qpid_id.qpid_id;

        let qpid_baggages = wasted_baggages
            .get_wasted_baggages(qpid_id, last_date.as_ref())
            .await?
            .into_iter()
            .map(|wb| wb.into_ds_api_type());

        baggages.extend(qpid_baggages);
    }

    Ok(Json(GetWastedBaggagesResponse {
        update_date: chrono::Utc::now().timestamp_millis(),
        baggages,
    }))
}
