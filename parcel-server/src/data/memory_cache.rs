use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use moka::future::{Cache as MokaCache, CacheBuilder};

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
