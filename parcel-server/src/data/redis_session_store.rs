use std::{fmt::Display, ops::Sub, sync::Arc};

use chrono::{TimeZone, Utc};
use parcel_common::api_types::auth::Provider;
use redis::{aio::Connection, AsyncCommands, RedisResult};

use crate::session::Session;

use super::redis_client::RedisClient;

pub struct RedisSessionStore {
    client: Arc<RedisClient>,
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
    pub fn new(client: Arc<RedisClient>, prefix: &str) -> Self {
        Self {
            client,
            prefix: prefix.into(),
        }
    }

    pub async fn save_session(&self, session: &Session) -> RedisResult<()> {
        let connection = &mut self.client.lock().await;
        let key = self.get_session_key(session.get_token());
        let value = serde_json::to_string(&session).unwrap();
        connection
            .pset_ex::<_, _, String>(
                &key,
                &value,
                session.expire_date.sub(Utc::now()).num_milliseconds() as usize,
            )
            .await?;
        self.set_reverse_lookup_token(
            connection,
            session.provider,
            &session.provider_id,
            session.get_token(),
            session.expire_date.sub(Utc::now()).num_milliseconds() as usize,
        )
        .await?;

        Ok(())
    }

    pub async fn load_session(&self, token: &str) -> RedisResult<Option<Session>> {
        let connection = &mut self.client.lock().await;
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

    pub async fn delete_session(&self, token: &str) -> RedisResult<()> {
        let session = self.load_session(token).await?;
        let connection = &mut self.client.lock().await;

        if session.is_none() {
            return Ok(());
        }

        let session = session.unwrap();

        connection
            .del::<_, i32>(self.get_session_key(token))
            .await?;
        connection
            .del::<_, i32>(
                self.get_session_reverse_lookup_key(session.provider, &session.provider_id),
            )
            .await?;

        Ok(())
    }

    pub async fn find_active_session_token(
        &self,
        provider: Provider,
        provider_id: &str,
    ) -> RedisResult<Option<String>> {
        let connection = &mut self.client.lock().await;

        self.get_reverse_lookup_token(connection, provider, provider_id)
            .await
    }

    async fn set_reverse_lookup_token(
        &self,
        conn: &mut Connection,
        provider: Provider,
        provider_id: &str,
        token: &str,
        expire_in_millis: usize,
    ) -> RedisResult<()> {
        let key = self.get_session_reverse_lookup_key(provider, provider_id);
        conn.pset_ex(&key, token, expire_in_millis).await
    }

    async fn get_reverse_lookup_token(
        &self,
        conn: &mut Connection,
        provider: Provider,
        provider_id: &str,
    ) -> RedisResult<Option<String>> {
        let key = self.get_session_reverse_lookup_key(provider, provider_id);
        let token = conn.get::<_, Option<String>>(&key).await?;

        Ok(token)
    }

    fn get_session_key(&self, token: &str) -> String {
        format!("{}session/{}", self.prefix, token)
    }

    fn get_session_reverse_lookup_key(&self, provider: Provider, provider_id: &str) -> String {
        format!(
            "{}session-reverse-lookup/{:?}_{}",
            self.prefix, provider, provider_id
        )
    }
}
