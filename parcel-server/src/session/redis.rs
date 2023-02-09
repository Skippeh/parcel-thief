use std::{collections::HashMap, fmt::Display, sync::Arc};

use chrono::{DateTime, TimeZone, Utc};
use redis::{aio::Connection, AsyncCommands, Client, IntoConnectionInfo, RedisResult};
use serde::{Deserialize, Serialize};

use super::{RedisError, Session, SessionError};

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
        let mut connection = self.connect().await?;

        let key = self.get_key(session.get_token());
        let value = serde_json::to_string(&session.values).unwrap();
        let result_str = connection.set::<_, _, String>(&key, &value).await?;
        connection
            .expire_at::<_, i32>(&key, session.expire_date.timestamp() as usize)
            .await?;

        if &result_str == "OK" {
            Ok(())
        } else {
            panic!("Unexpected result: {}", result_str)
        }
    }

    pub async fn load_session(&self, token: &str) -> Result<Option<Session>, RedisError> {
        let mut connection = self.connect().await?;

        let key = self.get_key(token);
        let value = connection.get::<_, Option<String>>(&key).await;

        match value {
            Ok(value) => {
                let expires_in_secs = connection.pttl::<_, i64>(&key).await?;

                if expires_in_secs < 0 {
                    connection.del(&key).await?;
                    return Ok(None);
                }

                let epoch_now = Utc::now().timestamp_millis();
                let expiration_date = Utc
                    .timestamp_millis_opt(epoch_now + expires_in_secs)
                    .unwrap();

                match value {
                    Some(value) => {
                        let values = serde_json::from_str::<HashMap<String, String>>(&value);

                        match values {
                            Ok(values) => {
                                let session =
                                    Session::with_values(values, token.to_owned(), expiration_date);
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

    pub async fn delete_session(&self, token: &str) -> Result<(), SessionError> {
        todo!()
    }

    async fn connect(&self) -> RedisResult<Connection> {
        self.client.get_async_connection().await
    }

    fn get_key(&self, token: &str) -> String {
        format!("{}{}", self.prefix, token)
    }
}
