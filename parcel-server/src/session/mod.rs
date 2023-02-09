pub mod redis;

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    sync::Arc,
};

use ::redis::RedisError;
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub struct SessionNotFound;

impl Display for SessionNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The session could not be found")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    RedisError(RedisError),
    SessionNotFound,
}

impl Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::RedisError(error) => write!(f, "A redis error occured: {}", error),
            SessionError::SessionNotFound => {
                write!(f, "Could not find a session from the specified token")
            }
        }
    }
}

pub struct Session {
    token: String,
    values: HashMap<String, String>,
    expire_date: DateTime<Utc>,
}

impl Session {
    pub fn new(token: String, expire_date: DateTime<Utc>) -> Self {
        Self {
            token,
            expire_date,
            values: HashMap::new(),
        }
    }

    pub fn with_values(
        values: HashMap<String, String>,
        token: String,
        expire_date: DateTime<Utc>,
    ) -> Session {
        Self {
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
