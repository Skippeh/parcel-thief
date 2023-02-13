use std::{collections::HashMap, fmt::Display};

use chrono::{TimeZone, Utc};
use parcel_common::api_types::auth::Provider;
use redis::{aio::Connection, AsyncCommands, Client, IntoConnectionInfo, RedisResult};

use super::{RedisError, Session};

pub struct RedisSessionStore {
    client: Client,
    prefix: String,
}

#[derive(Debug, thiserror::Error)]
pub struct InvalidTokenError;

impl Display for InvalidTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The token must be non zero length")
    }
}

impl RedisSessionStore {
    pub async fn new<T>(connection_string: T, prefix: &str) -> RedisResult<Self>
    where
        T: IntoConnectionInfo,
    {
        let client = Client::open(connection_string)?;
        let res = Self {
            client,
            prefix: prefix.into(),
        };

        // fail early if connection doesn't work
        res.connect().await?;

        Ok(res)
    }

    pub async fn save_session(&self, session: &Session) -> Result<(), RedisError> {
        let connection = &mut self.connect().await?;

        let key = self.get_session_key(session.get_token());
        let value = serde_json::to_string(&session).unwrap();
        connection.set::<_, _, String>(&key, &value).await?;
        connection
            .expire_at::<_, i32>(&key, session.expire_date.timestamp() as usize)
            .await?;
        self.set_reverse_lookup_token(
            connection,
            &session.provider,
            &session.provider_id,
            session.get_token(),
            session.expire_date.timestamp() as usize,
        )
        .await?;

        Ok(())
    }

    pub async fn load_session(&self, token: &str) -> Result<Option<Session>, RedisError> {
        let connection = &mut self.connect().await?;

        let key = self.get_session_key(token);
        let value = connection.get::<_, Option<String>>(&key).await;

        match value {
            Ok(value) => {
                let expires_in_secs = connection.pttl::<_, i64>(&key).await?;
                let epoch_now = Utc::now().timestamp_millis();

                if expires_in_secs < 0 {
                    connection.del(&key).await?;
                    return Ok(None);
                }

                let expiration_date = Utc
                    .timestamp_millis_opt(epoch_now + expires_in_secs)
                    .unwrap();

                match value {
                    Some(value) => {
                        let session = serde_json::from_str::<Session>(&value);

                        match session {
                            Ok(mut session) => {
                                session.expire_date = expiration_date;
                                session.token = token.to_owned();
                                Ok(Some(session))
                            }
                            Err(_) => {
                                // delete value if it can't be deserialized and return none
                                connection.del(&key).await?;
                                Ok(None)
                            }
                        }
                    }
                    None => Ok(None),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub async fn delete_session(&self, token: &str) -> Result<(), RedisError> {
        let connection = &mut self.connect().await?;
        let session = self.load_session(token).await?;

        if session.is_none() {
            return Ok(());
        }

        let session = session.unwrap();

        connection
            .del::<_, i32>(self.get_session_key(token))
            .await?;
        connection
            .del::<_, i32>(
                self.get_session_reverse_lookup_key(&session.provider, &session.provider_id),
            )
            .await?;

        Ok(())
    }

    pub async fn find_active_session_token(
        &self,
        provider: &Provider,
        provider_id: &str,
    ) -> Result<Option<String>, RedisError> {
        let connection = &mut self.connect().await?;
        let key = self.get_session_reverse_lookup_key(provider, provider_id);

        self.get_reverse_lookup_token(connection, provider, provider_id)
            .await
    }

    async fn set_reverse_lookup_token(
        &self,
        conn: &mut Connection,
        provider: &Provider,
        provider_id: &str,
        token: &str,
        expire_at_secs: usize,
    ) -> Result<(), RedisError> {
        let key = self.get_session_reverse_lookup_key(provider, provider_id);
        conn.set(&key, token).await?;
        conn.expire_at(&key, expire_at_secs).await?;

        Ok(())
    }

    async fn get_reverse_lookup_token(
        &self,
        conn: &mut Connection,
        provider: &Provider,
        provider_id: &str,
    ) -> Result<Option<String>, RedisError> {
        let key = self.get_session_reverse_lookup_key(provider, provider_id);
        let token = conn.get::<_, Option<String>>(&key).await?;

        Ok(token)
    }

    async fn connect(&self) -> RedisResult<Connection> {
        self.client.get_async_connection().await
    }

    fn get_session_key(&self, token: &str) -> String {
        format!("{}session/{}", self.prefix, token)
    }

    fn get_session_reverse_lookup_key(&self, provider: &Provider, provider_id: &str) -> String {
        format!(
            "{}session-reverse-lookup/{:?}_{}",
            self.prefix, provider, provider_id
        )
    }
}
