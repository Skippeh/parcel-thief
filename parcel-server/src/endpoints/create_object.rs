use actix_web::{post, web::Json};
use anyhow::anyhow;
use parcel_common::api_types::requests::create_object::{
    CreateObjectRequest, CreateObjectResponse,
};

use crate::endpoints::InternalError;

#[post("createObject")]
pub async fn create_object(
    request: Json<CreateObjectRequest>,
) -> Result<Json<CreateObjectResponse>, InternalError> {
    Err(anyhow!("Not implemented").into())
}
