use std::{
    cmp::min,
    fs::canonicalize,
    io::Write,
    path::{Path, PathBuf},
    process::{Child, Stdio},
    sync::{Arc, Mutex},
};

use anyhow::Context;
use futures_util::StreamExt;
use indicatif::ProgressStyle;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

use crate::{embedded::EmbeddedSoftwareInstaller, Options};

use super::{EmbeddedSoftware, SetupStep};

type RedisInstaller = EmbeddedSoftwareInstaller<
    Redis,
    SetupState,
    SetupError,
    RedisInstance,
    anyhow::Error,
    anyhow::Error,
>;

lazy_static::lazy_static! {
    static ref REDIS_CLIENT: Arc<Mutex<Option<RedisInstaller>>> = Arc::new(Mutex::new(None));
}

/// Sets up local redis server (if necessary) and returns the redis connection string.
pub async fn setup_redis(args: &Options) -> Result<String, anyhow::Error> {
    match args.redis_url.as_ref() {
        Some(url) => Ok(url.clone()),
        None => {
            let mut installer = EmbeddedSoftwareInstaller::new(Redis::default());
            installer.setup().await?;
            let instance = installer.start().await?;

            *REDIS_CLIENT.lock().unwrap() = Some(installer);

            // wait for 3 seconds to allow redis to start
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    }
}

pub async fn stop_redis() -> Result<(), anyhow::Error> {
    let client = &mut *REDIS_CLIENT.lock().unwrap();

    match client {
        Some(installer) => {
            log::info!("Stopping redis server...");
            installer.stop().await?;
            client.take();
            Ok(())
        }
        None => Ok(()),
    }
}

#[derive(Default)]
struct Redis {
    child: Option<Child>,
}

#[derive(Debug, Copy, Clone)]
struct RedisInstance {
    pub listen_port: u16,
    pub process_id: u32,
}

struct DownloadStep;
struct UnarchiveStep;

#[derive(thiserror::Error, Debug)]
pub enum SetupError {
    #[error("Could not download required files: {0}")]
    DownloadError(reqwest::Error),
    #[error("Could not extract files: {0}")]
    ExtractError(anyhow::Error),
    #[error("Not implemented")]
    NotImplemented,
    #[error("Support for embedded Redis server is not implemented for the running operating system. Please provide a connection url to a running redis server")]
    UnsupportedOs,
    #[error("IO error: {0}")]
    IoError(tokio::io::Error),
    #[error("{0}")]
    OtherError(anyhow::Error),
}

impl From<tokio::io::Error> for SetupError {
    fn from(value: tokio::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<reqwest::Error> for SetupError {
    fn from(value: reqwest::Error) -> Self {
        Self::DownloadError(value)
    }
}

impl From<zip::result::ZipError> for SetupError {
    fn from(value: zip::result::ZipError) -> Self {
        Self::ExtractError(value.into())
    }
}

pub enum SetupState {
    Downloading,
    Extracting(PathBuf),
    Finished,
}

#[async_trait::async_trait]
impl EmbeddedSoftware for Redis {
    type SetupError = SetupError;
    type SetupState = SetupState;
    type RunValue = RedisInstance;
    type RunError = anyhow::Error;
    type StopError = anyhow::Error;

    fn get_name(&self) -> String {
        "Redis".into()
    }

    fn is_installed(&self) -> bool {
        Path::new("data/redis/redis-server.exe").exists()
            && Path::new("data/redis/redis.conf").exists()
    }

    async fn setup(
        &mut self,
    ) -> (
        Self::SetupState,
        Vec<
            Box<
                dyn SetupStep<
                    SetupState = Self::SetupState,
                    SetupError = Self::SetupError,
                    Software = Self,
                >,
            >,
        >,
    ) {
        (
            SetupState::Downloading,
            vec![Box::new(DownloadStep), Box::new(UnarchiveStep)],
        )
    }

    async fn start(&mut self) -> Result<Self::RunValue, Self::RunError> {
        if self.child.is_some() {
            return Err(anyhow::anyhow!("Redis is already running"));
        }

        // make sure logs dir exists
        std::fs::create_dir_all("logs")?;

        let listen_port =
            portpicker::pick_unused_port().context("Could not find an unused port")?;
        let stdout = Stdio::from(std::fs::File::create("logs/redis_stdout.log")?);
        let stderr = Stdio::from(std::fs::File::create("logs/redis_stderr.log")?);
        let child_process = std::process::Command::new("data/redis/redis-server.exe")
            .arg("redis.conf")
            .arg("--port")
            .arg(format!("{}", listen_port))
            .current_dir(canonicalize("data/redis")?)
            .stdout(stdout)
            .stderr(stderr)
            .stdin(Stdio::piped())
            .spawn()?;

        log::debug!("redis pid: {}", child_process.id());

        let instance = RedisInstance {
            listen_port,
            process_id: child_process.id(),
        };

        self.child = Some(child_process);

        Ok(instance)
    }

    async fn stop(&mut self) -> Result<(), Self::StopError> {
        match &mut self.child {
            Some(child) => {
                let stdin = child.stdin.as_mut().unwrap();
                let _ = stdin.write_all(b"\x03"); // send ^C to stdin
                let _ = child.wait();

                self.child = None;
                Ok(())
            }
            None => Err(anyhow::anyhow!("Redis is not running")),
        }
    }
}

impl DownloadStep {
    fn get_download_url() -> Result<String, SetupError> {
        if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
            Ok("https://github.com/tporadowski/redis/releases/download/v5.0.14.1/Redis-x64-5.0.14.1.zip"
            .into())
        } else {
            Err(SetupError::UnsupportedOs)
        }
    }
}

#[async_trait::async_trait]
impl SetupStep for DownloadStep {
    type SetupState = SetupState;
    type SetupError = SetupError;
    type Software = Redis;

    fn step_name(&self) -> String {
        "Download files".into()
    }

    async fn execute(
        &mut self,
        software: &mut Self::Software,
        state: Self::SetupState,
        progress_bar: &mut indicatif::ProgressBar,
    ) -> Result<Self::SetupState, Self::SetupError> {
        match state {
            SetupState::Downloading => {
                progress_bar.set_style(
                    ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap().progress_chars("#>-"),
                );

                let download_url = Self::get_download_url()?;

                // create data/redis directory if it doesn't exist
                tokio::fs::create_dir_all("data/redis").await?;

                let client = Client::new();
                let response = client.get(&download_url).send().await?;
                let total_length = response
                    .content_length()
                    .ok_or_else(|| SetupError::OtherError(anyhow::anyhow!("No content length")))?;
                progress_bar.set_length(total_length);

                let mut file = tokio::fs::File::create("data/redis.zip").await?;
                let mut stream = response.bytes_stream();
                let mut downloaded_bytes = 0u64;

                while let Some(chunk) = stream.next().await {
                    let chunk = chunk?;
                    file.write_all(&chunk).await?;
                    downloaded_bytes += chunk.len() as u64;
                    progress_bar.set_position(min(downloaded_bytes, total_length));
                }

                progress_bar.finish();

                Ok(SetupState::Extracting(PathBuf::from("data/redis.zip")))
            }
            _ => Err(SetupError::OtherError(anyhow::anyhow!(
                "Unexpected setup state"
            ))),
        }
    }
}

#[async_trait::async_trait]
impl SetupStep for UnarchiveStep {
    type SetupState = SetupState;
    type SetupError = SetupError;
    type Software = Redis;

    fn step_name(&self) -> String {
        "Extract files".into()
    }

    async fn execute(
        &mut self,
        software: &mut Self::Software,
        state: Self::SetupState,
        progress_bar: &mut indicatif::ProgressBar,
    ) -> Result<Self::SetupState, Self::SetupError> {
        match state {
            SetupState::Extracting(file_path) => {
                progress_bar.set_length(1);

                match file_path
                    .extension()
                    .map(|ext| ext.to_string_lossy())
                    .as_deref()
                {
                    Some("zip") => {
                        // extract and then delete zip file
                        let mut file = zip::ZipArchive::new(std::fs::File::open(&file_path)?)?;
                        file.extract("data/redis")?;
                        std::fs::remove_file(file_path)?;

                        // Write embedded config file to data/redis/redis.conf
                        let conf_bytes = std::include_bytes!("redis.conf");
                        let mut file = tokio::fs::File::create("data/redis/redis.conf").await?;
                        file.write_all(conf_bytes).await?;

                        progress_bar.finish();

                        Ok(SetupState::Finished)
                    }
                    _ => Err(SetupError::OtherError(anyhow::anyhow!(
                        "Unexpected file extension"
                    ))),
                }
            }
            _ => Err(SetupError::OtherError(anyhow::anyhow!(
                "Unexpected setup state"
            ))),
        }
    }
}
