use actix_web::{
    post,
    web::{Data, Json},
};
use anyhow::Context;
use parcel_common::api_types::{
    object::Object, requests::set_recycle_materials::SetRecycleMaterialsRequest, TryIntoDsApiType,
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("setRecycleMaterials")]
pub async fn set_recycle_materials(
    request: Json<SetRecycleMaterialsRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<Json<Object>, InternalError> {
    let db = database.connect()?;
    let qpid_objects = db.qpid_objects();

    let object = qpid_objects
        .get_by_id(&request.object_id)
        .await?
        .with_context(|| format!("Object not found: {}", &request.object_id))?;

    // Null contributor means the owner of the object is the contributor.
    let contributor_id: Option<&str> = if object.creator_id == session.account_id {
        None
    } else {
        Some(&session.account_id)
    };

    qpid_objects
        .contribute_recycle_materials(contributor_id, &request.object_id, &request.materials)
        .await?;

    let object = qpid_objects
        .query_object_data([object])
        .await?
        .into_iter()
        .next()
        .context("Object not found (but shouldn't ever happen at this point)")?
        .try_into_ds_api_type()?;

    Ok(Json(object))
}
