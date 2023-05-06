use actix_web::{post, web::Json};
use parcel_common::api_types::object::Object;

use crate::endpoints::InternalError;

#[post("setRecycleMaterials")]
pub async fn set_recycle_materials() -> Result<Json<Object>, InternalError> {
    //  todo: implement setRecycleMaterials endpoint
    Err(anyhow::anyhow!("Not implemented").into())
}
