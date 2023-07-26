use std::{ops::Deref, time::Duration};

use actix_http::{header::Header, StatusCode};
use actix_web::{web::Data, FromRequest, ResponseError};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use chrono::{DateTime, TimeZone, Utc};
use diesel::ConnectionError;
use flagset::FlagSet;
use futures_util::{future::LocalBoxFuture, FutureExt};
use jwt::VerifyWithKey;
use moka::Expiry;
use parcel_common::api_types::frontend::auth::{FrontendPermissions, JwtPayload};

use crate::{
    data::{database::Database, jwt_secret::JwtSecret, memory_cache::MemoryCache},
    db::QueryError,
};

use super::error::ApiError;

pub type SessionBlacklistCache = MemoryCache<String, DateTime<Utc>>; // value = token expire date
pub type SessionPermissionsCache = MemoryCache<i64, FlagSet<FrontendPermissions>>;

pub struct JwtSession(JwtPayload, FlagSet<FrontendPermissions>);

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
    pub fn has_permissions(&self, permissions: impl Into<FlagSet<FrontendPermissions>>) -> bool {
        let permissions = permissions.into();

        if permissions.is_empty() {
            return true;
        }

        self.1.contains(permissions)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FromRequestError {
    #[error("Invalid or missing auth token")]
    Unauthorized,
    #[error("The auth token has expired")]
    Expired,
    #[error("{0}")]
    QueryError(QueryError),
    #[error("{0}")]
    DatabaseConnectError(ConnectionError),
}

impl From<QueryError> for FromRequestError {
    fn from(value: QueryError) -> Self {
        FromRequestError::QueryError(value)
    }
}

impl From<ConnectionError> for FromRequestError {
    fn from(value: ConnectionError) -> Self {
        FromRequestError::DatabaseConnectError(value)
    }
}

impl ResponseError for FromRequestError {
    fn status_code(&self) -> actix_http::StatusCode {
        match self {
            FromRequestError::Unauthorized => StatusCode::UNAUTHORIZED,
            FromRequestError::Expired => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_http::body::BoxBody> {
        let err = match self {
            FromRequestError::Unauthorized => ApiError::Unauthorized(anyhow::anyhow!("{}", self)),
            FromRequestError::Expired => ApiError::Unauthorized(anyhow::anyhow!("{}", self)),
            FromRequestError::QueryError(err) => ApiError::Internal(anyhow::anyhow!("{}", err)),
            FromRequestError::DatabaseConnectError(err) => {
                ApiError::Internal(anyhow::anyhow!("{}", err))
            }
        };

        err.error_response()
    }
}

impl FromRequest for JwtSession {
    type Error = FromRequestError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_http::Payload) -> Self::Future {
        let req = req.clone();
        let auth = Authorization::<Bearer>::parse(&req);

        async move {
            let jwt_secret = req
                .app_data::<Data<JwtSecret>>()
                .expect("JwtSecret should always exist");
            let session_blacklist_cache = req
                .app_data::<Data<SessionBlacklistCache>>()
                .expect("SessionBlacklistCache should always exist");
            let session_permissions_cache = req
                .app_data::<Data<SessionPermissionsCache>>()
                .expect("SessionPermissionsCache should always exist");
            let token = match auth {
                Ok(auth) => auth.into_scheme().token().to_owned(),
                Err(_) => return Err(FromRequestError::Unauthorized),
            };

            if session_blacklist_cache.contains_key(&token) {
                return Err(FromRequestError::Expired);
            }

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

            let permissions = {
                match session_permissions_cache.get(&payload.account_id) {
                    Some(permissions) => permissions,
                    None => {
                        let database = req
                            .app_data::<Data<Database>>()
                            .expect("Database should always exist");
                        let account = database
                            .connect()
                            .await?
                            .frontend_accounts()
                            .get_by_id(payload.account_id)
                            .await?;

                        match account {
                            None => return Err(FromRequestError::Unauthorized),
                            Some(account) => {
                                let permissions = FlagSet::new_truncated(account.permissions);
                                session_permissions_cache
                                    .insert(payload.account_id, permissions.clone())
                                    .await;

                                permissions
                            }
                        }
                    }
                }
            };

            Ok(Self(payload, permissions))
        }
        .boxed_local()
    }
}

pub struct SessionBlacklistCacheExpiry;

impl Expiry<String, DateTime<Utc>> for SessionBlacklistCacheExpiry {
    fn expire_after_create(
        &self,
        _key: &String,
        value: &DateTime<Utc>,
        current_time: std::time::Instant,
    ) -> Option<Duration> {
        get_expiry_duration(value, current_time)
    }

    fn expire_after_update(
        &self,
        _key: &String,
        value: &DateTime<Utc>,
        current_time: std::time::Instant,
        _current_duration: Option<Duration>,
    ) -> Option<Duration> {
        get_expiry_duration(value, current_time)
    }
}

fn get_expiry_duration(
    value: &DateTime<Utc>,
    current_time: std::time::Instant,
) -> Option<Duration> {
    let delta_time = value.timestamp_millis() - current_time.elapsed().as_millis() as i64;

    if delta_time < 0 {
        Some(Duration::default())
    } else {
        Some(Duration::from_millis(delta_time as u64))
    }
}
