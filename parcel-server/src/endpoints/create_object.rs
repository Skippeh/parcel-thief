use actix_web::{
    post,
    web::{Data, Json},
};

use parcel_common::api_types::requests::create_object::{
    CreateObjectRequest, CreateObjectResponse,
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("createObject")]
pub async fn create_object(
    request: Json<CreateObjectRequest>,
    database: Data<Database>,
    session: Session,
) -> Result<Json<CreateObjectResponse>, InternalError> {
    let db = database.connect()?;
    let qpid_objects = db.qpid_objects();
    let result = qpid_objects
        .create_from_request(&request, &session.account_id)
        .await?
        .try_into_api_type()?;
    Ok(Json(result))
}
