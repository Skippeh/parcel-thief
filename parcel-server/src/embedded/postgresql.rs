use std::{path::PathBuf, sync::Arc};

use futures_util::lock::Mutex;
use postgresql_embedded::{PostgreSQL, Settings, VersionReq};

use crate::Options;

lazy_static::lazy_static! {
    static ref PG_EMBED: Arc<Mutex<Option<PostgreSQL>>> = Arc::new(Mutex::new(None));
}

const DB_NAME: &str = "parcels";

/// Sets up local postgresql server (if necessary) and returns the database connection string.
pub async fn setup_postgresql(args: &Options) -> Result<String, anyhow::Error> {
    match args.database_url.as_ref() {
        Some(url) => Ok(url.clone()),
        None => {
            let static_pg_embed = &mut *PG_EMBED.lock().await;

            match static_pg_embed {
                Some(_) => {
                    anyhow::bail!("PostgreSQL server is already running")
                }
                None => {
                    let settings = Settings {
                        version: VersionReq::parse("=16.3.0").unwrap(),
                        installation_dir: PathBuf::from("data/postgresql/bin"),
                        temporary: false,
                        data_dir: PathBuf::from("data/postgresql/data"),
                        password: "ds_1234".into(),
                        password_file: PathBuf::from("data/postgresql/pw"),
                        timeout: None,
                        ..Default::default()
                    };
                    let mut pg_embed = PostgreSQL::new(settings);
                    pg_embed.setup().await?;
                    pg_embed.start().await?;

                    log::debug!("Creating database if it doesn't exist...");
                    if !pg_embed.database_exists(DB_NAME).await? {
                        pg_embed.create_database(DB_NAME).await?;
                    }

                    let url = pg_embed.settings().url(DB_NAME);
                    static_pg_embed.replace(pg_embed);
                    Ok(url)
                }
            }
        }
    }
}

pub async fn stop_postgresql() -> Result<(), anyhow::Error> {
    let server = &mut *PG_EMBED.lock().await;

    match server {
        Some(server) => {
            log::info!("Stopping PostgreSQL server...");
            server.stop().await?;
            Ok(())
        }
        None => Ok(()),
    }
}
