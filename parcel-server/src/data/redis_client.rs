use std::{ops::Deref, sync::Arc};

use futures_util::lock::Mutex;
use redis::{aio::Connection, Client, IntoConnectionInfo};

#[derive(Debug)]
pub struct RedisClient(Arc<Mutex<Connection>>);

impl Deref for RedisClient {
    type Target = Arc<Mutex<Connection>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RedisClient {
    pub async fn connect(
        connection_string: impl IntoConnectionInfo,
    ) -> Result<Self, redis::RedisError> {
        let client = Client::open(connection_string)?;
        let conn = client.get_tokio_connection().await?;

        Ok(Self(Arc::new(Mutex::new(conn))))
    }
}
