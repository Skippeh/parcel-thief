pub mod redis;

use std::{collections::HashMap, fmt::Display};

use ::redis::RedisError;
use actix_http::{header::Header, StatusCode};
use actix_web::{web::Data, FromRequest, ResponseError};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use chrono::{DateTime, Utc};
use futures_util::future::LocalBoxFuture;
use parcel_common::api_types::auth::Provider;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::response_error::{impl_response_error, CommonResponseError};

use self::redis::RedisSessionStore;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub provider: Provider,
    pub provider_id: String,
    pub account_id: String,
    #[serde(skip)]
    token: String,
    values: HashMap<String, String>,
    #[serde(skip)]
    expire_date: DateTime<Utc>,
}

impl Session {
    pub fn new(
        provider: &Provider,
        provider_id: &str,
        account_id: &str,
        token: String,
        expire_date: DateTime<Utc>,
    ) -> Self {
        Self {
            provider: provider.clone(),
            provider_id: provider_id.into(),
            account_id: account_id.into(),
            token,
            expire_date,
            values: HashMap::new(),
        }
    }

    pub fn with_values(
        provider: &Provider,
        provider_id: &str,
        account_id: &str,
        values: HashMap<String, String>,
        token: String,
        expire_date: DateTime<Utc>,
    ) -> Session {
        Self {
            provider: provider.clone(),
            provider_id: provider_id.into(),
            account_id: account_id.into(),
            token,
            expire_date,
            values,
        }
    }

    /// Gets the token that identifies this session. It is the value that is sent to and read from the client.
    #[inline]
    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub async fn set_expiration(&mut self, date_time: chrono::DateTime<Utc>) {
        self.expire_date = date_time;
    }

    pub fn get<T>(&self, key: &str) -> Result<Option<T>, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        let serialized_value = self.values.get(key);

        match serialized_value {
            None => Ok(None),
            Some(serialized_value) => Ok(serde_json::from_str(serialized_value)?),
        }
    }

    pub fn get_raw(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    pub fn set<T>(&mut self, key: &str, val: &T) -> Result<(), serde_json::Error>
    where
        T: Serialize,
    {
        let serialized_value = serde_json::to_string(val)?;
        self.values.insert(key.into(), serialized_value);
        Ok(())
    }

    pub fn set_raw(&mut self, key: &str, val: &str) {
        self.values.insert(key.into(), val.into());
    }
}

#[derive(Debug)]
pub enum FromRequestError {
    UnknownToken,
    Unauthorized,
    RedisError(RedisError),
}

impl Display for FromRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromRequestError::UnknownToken => write!(f, "Unknown token"),
            FromRequestError::RedisError(err) => write!(f, "A redis error occured: {:?}", err),
            FromRequestError::Unauthorized => write!(f, "No token specified"),
        }
    }
}

impl_response_error!(FromRequestError);
impl CommonResponseError for FromRequestError {
    fn get_status_code(&self) -> String {
        match self {
            FromRequestError::UnknownToken => "AU-UT",
            FromRequestError::RedisError(_) => "SV-IT",
            FromRequestError::Unauthorized => "AU-UA",
        }
        .into()
    }

    fn get_http_status_code(&self) -> actix_http::StatusCode {
        match self {
            FromRequestError::UnknownToken | FromRequestError::Unauthorized => {
                StatusCode::UNAUTHORIZED
            }
            FromRequestError::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn get_message(&self) -> String {
        match self {
            FromRequestError::UnknownToken => "bad token",
            FromRequestError::RedisError(_) => "internal error",
            FromRequestError::Unauthorized => "no permission",
        }
        .into()
    }
}

impl FromRequest for Session {
    type Error = FromRequestError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_http::Payload) -> Self::Future {
        let req = req.clone();
        let auth = Authorization::<Bearer>::parse(&req);

        Box::pin(async move {
            let token = match auth {
                Ok(auth) => auth.into_scheme().token().to_owned(),
                Err(_) => return Err(FromRequestError::Unauthorized),
            };

            let session_store = req.app_data::<Data<RedisSessionStore>>().unwrap();
            let session = session_store
                .load_session(&token)
                .await
                .map_err(FromRequestError::RedisError)?;

            match session {
                Some(session) => Ok(session),
                None => Err(FromRequestError::UnknownToken),
            }
        })
    }
}
