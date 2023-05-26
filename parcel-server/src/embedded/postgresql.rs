use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use futures_util::lock::Mutex;
use indicatif::ProgressBar;
use pg_embed::{
    pg_fetch::{PgFetchSettings, PG_V15},
    postgres::{PgEmbed, PgSettings},
};

use crate::Options;

use super::{EmbeddedSoftware, EmbeddedSoftwareInstaller, SetupStep};

lazy_static::lazy_static! {
    static ref PG_SERVER: Arc<Mutex<Option<EmbeddedSoftwareInstaller<Postgres, (), anyhow::Error, (), anyhow::Error, anyhow::Error>>>> = Arc::new(Mutex::new(None));
}

const DB_NAME: &str = "parcels";

/// Sets up local postgresql server (if necessary) and returns the database connection string.
pub async fn setup_postgresql(args: &Options) -> Result<String, anyhow::Error> {
    match args.database_url.as_ref() {
        Some(url) => Ok(url.clone()),
        None => {
            let pg = &mut *PG_SERVER.lock().await;

            match pg {
                Some(_) => {
                    anyhow::bail!("PostgreSQL server is already running")
                }
                None => {
                    let settings: PgSettings = PgSettings {
                        database_dir: PathBuf::from("data/postgresql/data"),
                        port: portpicker::pick_unused_port()
                            .context("Failed to find an unused port")?,
                        user: "ds".into(),
                        password: "ds_1234".into(),
                        auth_method: pg_embed::pg_enums::PgAuthMethod::Plain,
                        persistent: true,
                        timeout: None,
                        migration_dir: None,
                    };

                    let fetch_settings: PgFetchSettings = PgFetchSettings {
                        version: PG_V15,
                        ..Default::default()
                    };

                    let pg_embed = PgEmbed::new(settings, fetch_settings).await?;
                    let mut installer = EmbeddedSoftwareInstaller::new(Postgres(pg_embed));

                    installer.setup().await?;
                    installer.start().await?;

                    let url = installer.software.0.full_db_uri(DB_NAME);
                    pg.replace(installer);
                    Ok(url)
                }
            }
        }
    }
}

pub async fn stop_postgresql() -> Result<(), anyhow::Error> {
    let server = &mut *PG_SERVER.lock().await;

    match server {
        Some(server) => {
            log::info!("Stopping PostgreSQL server...");
            server.stop().await?;
            Ok(())
        }
        None => Ok(()),
    }
}

struct Postgres(PgEmbed);

#[async_trait::async_trait]
impl EmbeddedSoftware for Postgres {
    type SetupState = ();
    type SetupError = anyhow::Error;
    type RunValue = ();
    type RunError = anyhow::Error;
    type StopError = anyhow::Error;

    fn get_name(&self) -> String {
        "PostgreSQL".into()
    }

    fn is_installed(&self) -> bool {
        false
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
                    Software = Postgres,
                >,
            >,
        >,
    ) {
        ((), vec![Box::new(PgSetupStep)])
    }

    async fn start(&mut self) -> Result<Self::RunValue, Self::RunError> {
        let pg = &mut self.0;
        pg.start_db().await?;

        log::info!("Creating database if it doesn't exist...");
        if !pg.database_exists(DB_NAME).await? {
            pg.create_database(DB_NAME).await?;
        }

        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Self::StopError> {
        match self.0.stop_db().await {
            Ok(_) => Ok(()),
            Err(err) => {
                if err.source.is_none() && err.message.is_none() {
                    Ok(())
                } else {
                    Err(err.into())
                }
            }
        }
    }
}

struct PgSetupStep;

#[async_trait::async_trait]
impl SetupStep for PgSetupStep {
    type SetupState = ();
    type SetupError = anyhow::Error;
    type Software = Postgres;

    fn step_name(&self) -> String {
        "Download and configure".into()
    }

    async fn execute(
        &mut self,
        software: &mut Self::Software,
        _state: Self::SetupState,
        progress_bar: &mut ProgressBar,
    ) -> Result<Self::SetupState, Self::SetupError> {
        progress_bar.set_length(1);

        software.0.setup().await?;
        progress_bar.finish();

        Ok(())
    }
}
