use std::fmt;

use actix_http::{header::Header, StatusCode};
use actix_web::{web::Data, FromRequest, HttpRequest};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};

use crate::{
    response_error::{impl_response_error, CommonResponseError},
    session::redis::RedisSessionStore,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct DsSession {
    pub account_id: String,
    token: String,
}

impl DsSession {
    pub async fn from_store(
        store: &RedisSessionStore,
        token: &str,
    ) -> Result<Option<Self>, anyhow::Error> {
        let session = store.load_session(token).await?;

        match session {
            Some(session) => {
                let account_id = session
                    .get_raw("account_id")
                    .ok_or_else(|| anyhow::anyhow!("No account id found in session"))?
                    .clone();

                Ok(Some(DsSession {
                    account_id,
                    token: session.get_token().to_owned(),
                }))
            }
            None => Ok(None),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DsSessionError {
    UnknownToken,
}

impl fmt::Display for DsSessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DsSessionError::UnknownToken => write!(f, "Token is expired or unknown"),
        }
    }
}

impl_response_error!(DsSessionError);
impl CommonResponseError for DsSessionError {
    fn get_status_code(&self) -> String {
        match self {
            DsSessionError::UnknownToken => "AU-UT",
        }
        .into()
    }

    fn get_http_status_code(&self) -> actix_http::StatusCode {
        match self {
            DsSessionError::UnknownToken => StatusCode::UNAUTHORIZED,
        }
    }

    fn get_message(&self) -> String {
        "unknown token".into()
    }
}

impl FromRequest for DsSession {
    type Error = DsSessionError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_http::Payload) -> Self::Future {
        let req = req.clone();
        let auth = Authorization::<Bearer>::parse(&req);

        Box::pin(async move {
            let token = match auth {
                Ok(auth) => auth.into_scheme().token().to_owned(),
                Err(_) => return Err(DsSessionError::UnknownToken),
            };

            let session_store = req.app_data::<Data<RedisSessionStore>>().unwrap();

            match DsSession::from_store(session_store.as_ref(), &token).await {
                Ok(session) => Ok(session.ok_or(DsSessionError::UnknownToken)?),
                Err(_) => Err(DsSessionError::UnknownToken),
            }
        })
    }
}
