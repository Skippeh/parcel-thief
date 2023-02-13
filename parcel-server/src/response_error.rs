use std::fmt::Display;

use actix_http::StatusCode;

pub trait CommonResponseError {
    fn get_status_code(&self) -> String;
    fn get_http_status_code(&self) -> StatusCode;
    fn get_message(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonError {
    pub status: String,
    #[serde(
        rename = "code",
        serialize_with = "serialize_status_code",
        deserialize_with = "deserialize_status_code"
    )]
    pub status_code: StatusCode,
    pub message: String,
}

fn serialize_status_code<S>(val: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(val.as_u16())
}

fn deserialize_status_code<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
where
    D: Deserializer<'de>,
{
    let val: u16 = serde::de::Deserialize::deserialize(deserializer)?;
    StatusCode::from_u16(val).map_err(serde::de::Error::custom)
}

macro_rules! impl_response_error {
    ( $ty:ty ) => {
        impl actix_web::ResponseError for $ty {
            #[inline]
            fn status_code(&self) -> actix_http::StatusCode {
                self.get_http_status_code()
            }

            fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
                let status_code = self.status_code();
                let mut res = actix_web::HttpResponse::new(status_code);
                let data = $crate::response_error::CommonError {
                    message: self.get_message(),
                    status: self.get_status_code(),
                    status_code,
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
use serde::{Deserialize, Deserializer, Serialize, Serializer};
