use std::borrow::Cow;

use actix_http::StatusCode;
use actix_web::{web::BufMut, ResponseError};
use validator::{ValidationError, ValidationErrors};

use crate::db::QueryError;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    Internal(anyhow::Error),
    #[error("{0}")]
    BadRequest(anyhow::Error),
    #[error("You lack the permissions to access to this resource")]
    Forbidden,
    #[error("{0}")]
    Unauthorized(anyhow::Error),
    #[error("The resource could not be found")]
    NotFound,
    #[error("{0}")]
    Unprocessable(anyhow::Error),
    #[error("One or more fields have validation errors")]
    ValidationErrors(validator::ValidationErrors),
}

impl ApiError {
    /// Creates a new `ApiError` from a slice of validation errors.
    ///
    /// The error tuples are in the format (field_name, error_code).
    ///
    /// The error code should be an identifier for the clientside to use to display the correct message and not an error message.
    pub fn validation_errors(errors: &[(&'static str, &'static str)]) -> Self {
        let mut validation_errors = ValidationErrors::new();

        for (field_name, error_code) in errors {
            let mut error = ValidationError::new("");
            error.code = Cow::Borrowed(error_code);
            validation_errors.add(field_name, error);
        }

        ApiError::ValidationErrors(validation_errors)
    }
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

impl From<QueryError> for ApiError {
    fn from(err: QueryError) -> Self {
        ApiError::Internal(err.into())
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    status_code: u16,
    error: String,
    form_errors: Option<ValidationErrors>,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Unprocessable(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::ValidationErrors(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_http::body::BoxBody> {
        let res = actix_web::HttpResponse::new(self.status_code());
        let mut error_response = ErrorResponse {
            status_code: self.status_code().as_u16(),
            error: match &self {
                ApiError::Internal(_) => "An internal error occurred".into(),
                err => format!("{}", err),
            },
            form_errors: None,
        };

        if let ApiError::ValidationErrors(errors) = &self {
            error_response.form_errors = Some(errors.clone());
        }

        let json_response =
            serde_json::to_vec(&error_response).expect("Json serialization should never fail");

        let mut buf = actix_web::web::BytesMut::with_capacity(json_response.len());
        buf.put_slice(&json_response);

        res.set_body(actix_http::body::BoxBody::new(buf))
    }
}
