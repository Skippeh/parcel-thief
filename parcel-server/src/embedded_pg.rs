use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use futures_util::lock::Mutex;
use pg_embed::{
    pg_fetch::{PgFetchSettings, PG_V15},
    postgres::{PgEmbed, PgSettings},
};

use crate::Options;

lazy_static::lazy_static! {
    static ref PG_SERVER: Arc<Mutex<Option<PgEmbed>>> = Arc::new(Mutex::new(None));
}

/// Sets up local postgresql server (if necessary) and returns the database connection string.
pub async fn setup_postgresql(args: &Options) -> Result<String, anyhow::Error> {
    if PG_SERVER.lock().await.is_some() {
        return Err(anyhow::anyhow!("PostgreSQL server is already running"));
    }

    match args.database_url.as_ref() {
        Some(url) => Ok(url.clone()),
        None => {
            let settings: PgSettings = PgSettings {
                database_dir: PathBuf::from("data/db"),
                port: portpicker::pick_unused_port().context("Failed to find an unused port")?,
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

            let mut pg = PgEmbed::new(settings, fetch_settings).await?;

            log::info!("Downloading and setting up PostgreSQL...");
            pg.setup().await?;

            log::info!("Starting PostgreSQL server...");
            pg.start_db().await?;

            log::info!("Creating database if it doesn't exist...");
            if !pg.database_exists("parcels").await? {
                pg.create_database("parcels").await?;
            }

            let url = pg.full_db_uri("parcels");

            PG_SERVER.lock().await.replace(pg);

            Ok(url)
        }
    }
}
