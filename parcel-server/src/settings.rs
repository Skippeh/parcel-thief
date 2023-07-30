use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use notify::{EventHandler, ReadDirectoryChangesWatcher, Watcher};
use parcel_common::api_types::frontend::settings::SettingsValues;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct Settings {
    lock: RwLock<SettingsValues>,
    file_path: PathBuf,
    /// If true, the settings will be reloaded the next time `read()` is called.
    is_dirty: Arc<RwLock<bool>>,
    _watcher: ReadDirectoryChangesWatcher,
}

impl Settings {
    /// Loads settings from the file at the specified path.
    /// If the file doesn't exist the default settings are saved to the path and then returned.
    ///
    /// If the file is edited manually the settings will be reloaded automatically the next time `read()` is called.
    pub async fn load_from_path(file_path: &Path) -> Result<Self, anyhow::Error> {
        let settings = if file_path.try_exists()? {
            log::debug!("Loading settings from file");
            read_file(file_path).await?
        } else {
            log::warn!("Settings file not found, using default settings");
            let values = SettingsValues::default();

            write_file(&values, file_path).await?;

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
        })
    }

    /// Returns a read guard over the settings.
    /// It does not load settings from the file again unless the file was changed since the last time `read()` or `write()` was called.
    pub async fn read(&self) -> RwLockReadGuard<SettingsValues> {
        if *self.is_dirty.read().await {
            log::debug!("Reloading settings");

            let mut write_guard = self.lock.write().await;

            match read_file(&self.file_path).await {
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
        F: FnOnce(&mut RwLockWriteGuard<'_, SettingsValues>),
    {
        let mut settings = self.lock.write().await;
        callback(&mut settings);

        write_file(&*settings, &self.file_path).await?;
        *self.is_dirty.write().await = false;

        Ok(())
    }
}

async fn read_file(path: &Path) -> Result<SettingsValues, anyhow::Error> {
    let bytes = std::fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

async fn write_file(settings: &SettingsValues, path: &Path) -> Result<(), anyhow::Error> {
    let bytes = serde_json::to_vec_pretty(&settings)?;
    std::fs::write(path, bytes)?;

    Ok(())
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
