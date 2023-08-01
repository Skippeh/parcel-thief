use actix_web::{
    post,
    web::{Data, Json},
};
use parcel_common::api_types::{
    requests::update_objects_tag::{UpdateObjectsTagRequest, UpdateObjectsTagResponse},
    TryIntoDsApiType,
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("updateObjectsTag")]
pub async fn update_objects_tag(
    request: Json<UpdateObjectsTagRequest>,
    _session: Session,
    database: Data<Database>,
) -> Result<Json<UpdateObjectsTagResponse>, InternalError> {
    let db = database.connect().await?;
    let objects = db.qpid_objects();

    objects
        .add_remove_tag_from_objects(
            &request.0.tag,
            request
                .0
                .add
                .as_ref()
                .map(|vec| vec.iter().map(|id| id as &str)),
            request
                .0
                .delete
                .as_ref()
                .map(|vec| vec.iter().map(|id| id as &str)),
        )
        .await?;

    let mut object_ids = Vec::new();

    if let Some(add) = request.0.add {
        object_ids.extend(add);
    }

    if let Some(delete) = request.0.delete {
        object_ids.extend(delete);
    }

    if !object_ids.is_empty() {
        let response_objects = objects.find_objects_by_id(&object_ids).await?;
        let response_objects = objects
            .query_object_data(response_objects)
            .await?
            .into_iter()
            .map(|obj| obj.try_into_ds_api_type())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Json(UpdateObjectsTagResponse {
            objects: response_objects,
        }))
    } else {
        Ok(Json(UpdateObjectsTagResponse {
            objects: Vec::new(),
        }))
    }
}
