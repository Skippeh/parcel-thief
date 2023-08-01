use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::Arc,
};

use notify::{EventHandler, PollWatcher, RecommendedWatcher, Watcher};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[async_trait::async_trait]
pub trait Persist<TData>
where
    TData: Send + Sync,
{
    async fn write_file(file_path: &Path, data: &TData) -> Result<(), anyhow::Error>;
    async fn read_file(file_path: &Path) -> Result<TData, anyhow::Error>;
}

pub struct Settings<TData, TDataPersist> {
    lock: RwLock<TData>,
    file_path: PathBuf,
    /// If true, the settings will be reloaded the next time `read()` is called.
    is_dirty: Arc<RwLock<bool>>,
    _watcher: RecommendedWatcher,
    _data_persist: PhantomData<TDataPersist>,
}

impl<TData, TPersist> Settings<TData, TPersist>
where
    TData: Default + Send + Sync,
    TPersist: Persist<TData>,
{
    /// Loads settings from the file at the specified path.
    /// If the file doesn't exist the default settings are saved to the path and then returned.
    ///
    /// If the file is edited manually the settings will be reloaded automatically the next time `read()` is called.
    pub async fn load_from_path(file_path: &Path) -> Result<Self, anyhow::Error> {
        let settings = if file_path.try_exists()? {
            log::debug!("Loading settings from file: {}", file_path.display());
            TPersist::read_file(file_path).await?
        } else {
            log::warn!("Settings file not found, using default settings");
            let values = TData::default();

            TPersist::write_file(file_path, &values).await?;

            values
        };

        let is_dirty = Arc::new(RwLock::new(false));
        let mut watcher = notify::recommended_watcher(SetBoolTrueEventHandler(is_dirty.clone()))?;

        watcher.watch(file_path, notify::RecursiveMode::NonRecursive)?;

        Ok(Self {
            lock: RwLock::new(settings),
            file_path: file_path.to_owned(),
            is_dirty,
            _watcher: watcher,
            _data_persist: PhantomData,
        })
    }

    /// Returns a read guard over the settings.
    /// It does not load settings from the file again unless the file was changed since the last time `read()` or `write()` was called.
    pub async fn read(&self) -> RwLockReadGuard<TData> {
        if *self.is_dirty.read().await {
            log::debug!("Reloading settings");

            let mut write_guard = self.lock.write().await;

            match TPersist::read_file(&self.file_path).await {
                Ok(settings) => {
                    *write_guard = settings;
                    *self.is_dirty.write().await = false;
                }
                Err(err) => {
                    log::error!("Error occurred when trying to reload settings: {}", err);
                }
            }
        }

        self.lock.read().await
    }

    /// Returns a callback in which you can change the setting fields.
    /// After the callback finishes the changes will be written back to the file.
    pub async fn write<F>(&self, callback: F) -> Result<(), anyhow::Error>
    where
        F: FnOnce(&mut RwLockWriteGuard<'_, TData>),
    {
        let mut settings = self.lock.write().await;
        callback(&mut settings);

        TPersist::write_file(&self.file_path, &*settings).await?;
        *self.is_dirty.write().await = false;

        Ok(())
    }
}

pub struct JsonPersist;

#[async_trait::async_trait]
impl<TData> Persist<TData> for JsonPersist
where
    TData: Serialize + DeserializeOwned + Send + Sync,
{
    async fn write_file(file_path: &Path, data: &TData) -> Result<(), anyhow::Error> {
        let bytes = serde_json::to_vec_pretty(data)?;
        tokio::fs::write(file_path, bytes).await?;

        Ok(())
    }

    async fn read_file(file_path: &Path) -> Result<TData, anyhow::Error> {
        let bytes = tokio::fs::read(file_path).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}

struct SetBoolTrueEventHandler(Arc<RwLock<bool>>);

impl EventHandler for SetBoolTrueEventHandler {
    fn handle_event(&mut self, event: notify::Result<notify::Event>) {
        if let Ok(event) = event {
            match event.kind {
                notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
                    futures::executor::block_on(async {
                        *self.0.write().await = true;
                    });
                }
                _ => {}
            }
        }
    }
}
