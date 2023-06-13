use actix_web::{
    post,
    web::{Data, Json},
};

use parcel_common::api_types::{
    object::ObjectType,
    requests::create_object::{CreateObjectRequest, CreateObjectResponse},
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("createObject")]
pub async fn create_object(
    request: Json<CreateObjectRequest>,
    database: Data<Database>,
    session: Session,
) -> Result<Json<CreateObjectResponse>, InternalError> {
    // Make sure that if object_type is unknown, then it is exactly 1 character long
    // todo: return bad request error instead of internal error
    if let ObjectType::Unknown(val) = &request.object_type {
        if val.len() != 1 || !val.is_ascii() {
            return Err(anyhow::anyhow!("Invalid object type: {}", val).into());
        }

        log::warn!("Creating object with unknown type: {}", val);
    }

    let db = database.connect()?;
    let qpid_objects = db.qpid_objects();
    let result = qpid_objects
        .create_from_request(&request, &session.account_id)
        .await?
        .try_into_api_type()?;
    Ok(Json(result))
}
