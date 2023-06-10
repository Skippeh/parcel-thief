use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use futures_util::lock::Mutex;
use pg_embed::{
    pg_fetch::{PgFetchSettings, PG_V15},
    postgres::{PgEmbed, PgSettings},
};

use crate::Options;

lazy_static::lazy_static! {
    static ref PG_EMBED: Arc<Mutex<Option<PgEmbed>>> = Arc::new(Mutex::new(None));
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

                    let mut pg_embed = PgEmbed::new(settings, fetch_settings).await?;
                    pg_embed.setup().await?;
                    pg_embed.start_db().await?;

                    log::debug!("Creating database if it doesn't exist...");
                    if !pg_embed.database_exists(DB_NAME).await? {
                        pg_embed.create_database(DB_NAME).await?;
                    }

                    let url = pg_embed.full_db_uri(DB_NAME);
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
            server.stop_db().await?;
            Ok(())
        }
        None => Ok(()),
    }
}
