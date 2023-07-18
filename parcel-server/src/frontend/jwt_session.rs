use std::ops::Deref;

use actix_http::{header::Header, StatusCode};
use actix_web::{web::Data, FromRequest, ResponseError};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use chrono::{TimeZone, Utc};
use flagset::FlagSet;
use futures_util::future::LocalBoxFuture;
use jwt::VerifyWithKey;
use parcel_common::api_types::frontend::auth::{FrontendPermissions, JwtPayload};

use crate::data::jwt::JwtSecret;

pub struct JwtSession(JwtPayload);

impl Deref for JwtSession {
    type Target = JwtPayload;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<JwtPayload> for JwtSession {
    fn as_ref(&self) -> &JwtPayload {
        &self.0
    }
}

impl JwtSession {
    pub async fn has_permissions(
        &self,
        permissions: impl Into<FlagSet<FrontendPermissions>>,
    ) -> bool {
        let permissions = permissions.into();

        if permissions.is_empty() {
            return true;
        }

        let my_permissions = FlagSet::<FrontendPermissions>::new_truncated(self.permissions);

        my_permissions.contains(permissions)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FromRequestError {
    #[error("Invalid or missing auth token")]
    Unauthorized,
    #[error("The auth token has expired")]
    Expired,
}

impl ResponseError for FromRequestError {
    fn status_code(&self) -> actix_http::StatusCode {
        match self {
            FromRequestError::Unauthorized => StatusCode::UNAUTHORIZED,
            FromRequestError::Expired => StatusCode::UNAUTHORIZED,
        }
    }
}

impl FromRequest for JwtSession {
    type Error = FromRequestError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_http::Payload) -> Self::Future {
        let req = req.clone();
        let auth = Authorization::<Bearer>::parse(&req);

        Box::pin(async move {
            let jwt_secret = req
                .app_data::<Data<JwtSecret>>()
                .expect("JwtSecret should always exist");
            let token = match auth {
                Ok(auth) => auth.into_scheme().token().to_owned(),
                Err(_) => return Err(FromRequestError::Unauthorized),
            };

            let payload: JwtPayload = token
                .verify_with_key(jwt_secret.as_ref())
                .map_err(|_| FromRequestError::Unauthorized)?;

            let expire_date = Utc
                .timestamp_opt(payload.expires_at, 0)
                .earliest()
                .ok_or_else(|| FromRequestError::Expired)?;

            if Utc::now() >= expire_date {
                return Err(FromRequestError::Expired);
            }

            Ok(Self(payload))
        })
    }
}
