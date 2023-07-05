use actix_http::StatusCode;
use actix_web::{web::BufMut, ResponseError};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Internal error: {0}")]
    Internal(anyhow::Error),
    #[error("Bad request: {0}")]
    BadRequest(anyhow::Error),
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::Internal(err)
    }
}

impl From<diesel::ConnectionError> for ApiError {
    fn from(err: diesel::ConnectionError) -> Self {
        ApiError::Internal(err.into())
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(err: diesel::result::Error) -> Self {
        ApiError::Internal(err.into())
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    status_code: u16,
    error: String,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_http::body::BoxBody> {
        let res = actix_web::HttpResponse::new(self.status_code());
        let json_response = serde_json::to_vec(&ErrorResponse {
            status_code: self.status_code().as_u16(),
            error: format!("{}", self),
        })
        .expect("Json serialization should never fail");

        let mut buf = actix_web::web::BytesMut::with_capacity(json_response.len());
        buf.put_slice(&json_response);

        res.set_body(actix_http::body::BoxBody::new(buf))
    }
}
