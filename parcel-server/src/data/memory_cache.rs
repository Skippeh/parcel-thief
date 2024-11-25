use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    path::Path,
    time::Duration,
};

use bincode::Options;
use moka::future::{Cache as MokaCache, CacheBuilder};
use serde::{de::DeserializeOwned, Serialize};

pub struct MemoryCache<K, V>
where
    K: Eq + PartialEq + std::hash::Hash + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    cache: MokaCache<K, V>,
}

impl<K, V> MemoryCache<K, V>
where
    K: Eq + PartialEq + std::hash::Hash + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    pub fn from_builder(builder: CacheBuilder<K, V, moka::future::Cache<K, V>>) -> Self {
        let cache = builder.build();
        Self { cache }
    }

    pub fn with_time_to_live_secs(name: &str, time_to_live: u64) -> Self {
        Self::from_builder(
            CacheBuilder::new(u64::MAX)
                .time_to_live(Duration::from_secs(time_to_live))
                .name(name),
        )
    }
}

impl<K, V> Deref for MemoryCache<K, V>
where
    K: Eq + PartialEq + std::hash::Hash + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    type Target = MokaCache<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

impl<K, V> DerefMut for MemoryCache<K, V>
where
    K: Eq + PartialEq + std::hash::Hash + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cache
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    /// Serialize or deserialize error
    #[error("{0}")]
    Serialize(bincode::Error),
    #[error("{0}")]
    Io(std::io::Error),
}

#[async_trait::async_trait]
pub trait PersistentCache: Sized {
    async fn save_to_file(&self, file_path: &Path) -> Result<(), CacheError>;
    async fn load_from_file(self, file_path: &Path) -> Result<Self, CacheError>;
}

#[async_trait::async_trait]
impl<K, V> PersistentCache for MemoryCache<K, V>
where
    K: Serialize + DeserializeOwned + Eq + PartialEq + std::hash::Hash + Send + Sync + 'static,
    V: Serialize + DeserializeOwned + Send + Sync + Clone + 'static,
{
    async fn save_to_file(&self, file_path: &Path) -> Result<(), CacheError> {
        let values = self.cache.iter().collect::<HashMap<_, _>>();
        let serialized = get_bincode_options()
            .serialize(&values)
            .map_err(CacheError::Serialize)?;

        tokio::fs::write(file_path, serialized)
            .await
            .map_err(CacheError::Io)?;

        Ok(())
    }

    /// Loads cache values from the specified file path. If the file does not exist the current values in the cache are saved to the file and then `self` is returned unchanged.
    async fn load_from_file(self, file_path: &Path) -> Result<Self, CacheError> {
        if !tokio::fs::try_exists(file_path)
            .await
            .map_err(CacheError::Io)?
        {
            self.save_to_file(file_path).await?;
            return Ok(self);
        }

        let deserialized = tokio::fs::read(file_path).await.map_err(CacheError::Io)?;
        let values: HashMap<K, V> = get_bincode_options()
            .deserialize(&deserialized)
            .map_err(CacheError::Serialize)?;

        let result = self;

        for (key, value) in values {
            result.cache.insert(key, value).await;
        }

        Ok(result)
    }
}

fn get_bincode_options() -> impl bincode::Options {
    bincode::DefaultOptions::new()
        .with_varint_encoding()
        .with_little_endian()
}
