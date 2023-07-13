use actix_web::{
    post,
    web::{Data, Json},
};
use parcel_common::api_types::{
    object::QpidObjectsResponse,
    requests::get_qpid_objects::{GetQpidObjectsRequest, GetQpidObjectsResponse},
    TryIntoDsApiType,
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("getQpidObjects")]
pub async fn get_qpid_objects(
    request: Json<GetQpidObjectsRequest>,
    _session: Session,
    database: Data<Database>,
) -> Result<Json<GetQpidObjectsResponse>, InternalError> {
    let conn = database.connect()?;
    let objects = conn.qpid_objects();

    let db_objects = objects.find_objects_by_id(&request.object_ids).await?;
    let api_objects = objects
        .query_object_data(db_objects)
        .await?
        .into_iter()
        .map(|obj| obj.try_into_ds_api_type())
        .collect::<Result<_, _>>()?;

    Ok(Json(GetQpidObjectsResponse {
        normal: QpidObjectsResponse {
            object_p: Some(api_objects),
            ..Default::default()
        },
    }))
}
