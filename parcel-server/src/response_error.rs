use actix_http::StatusCode;

pub trait CommonResponseError {
    fn get_status_code(&self) -> String;
    fn get_http_status_code(&self) -> StatusCode;
    fn get_message(&self) -> String;
}

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub status: String,
    pub status_code: u16,
    pub message: String,
}

macro_rules! impl_response_error {
    ( $ty:ty ) => {
        impl actix_web::ResponseError for $ty {
            #[inline]
            fn status_code(&self) -> actix_http::StatusCode {
                self.get_http_status_code()
            }

            fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
                let status_code = self.status_code();
                let mut res = HttpResponse::new(status_code);
                let data = $crate::response_error::CommonError {
                    message: self.get_message(),
                    status: self.get_status_code(),
                    status_code: status_code.into(),
                };

                res.headers_mut().insert(
                    actix_http::header::CONTENT_TYPE,
                    actix_http::header::HeaderValue::from_static("application/json"),
                );

                let json = serde_json::to_string(&data).unwrap(); // unwrap is safe because the CommonError struct does not have any potential to fail serialization
                res.set_body(actix_web::body::BoxBody::new(json))
            }
        }
    };
}

pub(crate) use impl_response_error;
use serde::Serialize;
