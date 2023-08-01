use actix_web::web::Json;
use serde::Serialize;

use super::error::ApiError;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T: Serialize> {
    pub status_code: u16,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Result<Json<Self>, ApiError> {
        Ok(Json(Self {
            status_code: 200,
            data,
        }))
    }
}

pub type ApiResult<T> = Result<Json<ApiResponse<T>>, ApiError>;
