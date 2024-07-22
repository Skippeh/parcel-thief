mod background_jobs;
mod data;
mod db;
mod embedded;
mod endpoints;
mod frontend;
mod middleware;
mod response_error;
mod session;
mod settings;
mod whitelist;

use std::{
    fs::File,
    io::BufReader,
    net::IpAddr,
    path::{Path, PathBuf},
    time::Duration,
};

use actix_web::{
    middleware::NormalizePath,
    web::{self},
    App, HttpServer,
};
use anyhow::{Context, Result};
use clap::Parser;
use data::{
    database::Database,
    hash_secret::HashSecret,
    jwt_secret::JwtSecret,
    memory_cache::PersistentCache,
    platforms::{epic::Epic, steam::Steam},
};
use diesel::{pg::Pg, Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use fern::{
    colors::{Color, ColoredLevelConfig},
    DateBased,
};
use frontend::{
    api::endpoints::auth::FrontendAuthCache,
    jwt_session::{
        SessionBlacklistCache, SessionBlacklistCacheExpiry, SessionPermissionsCache,
        BLACKLIST_CACHE_PATH,
    },
};
use moka::future::CacheBuilder;
use parcel_common::api_types::frontend::settings::SettingsValues;
use parcel_game_data::GameData;
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use settings::Settings;

use crate::{data::session_store::SessionStore, middleware::wrap_errors};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type ServerSettings = Settings<SettingsValues, settings::JsonPersist>;
pub type WhitelistSettings = Settings<whitelist::Whitelist, whitelist::WhitelistPersist>;

/// A custom server implementation for Death Stranding Directory's Cut.
///
/// It's designed for small groups of people. All objects, missions, etc, are synced between all players,
/// so there's no chance of objects missing in one player's world unless they deleted it themselves or it's built too close to another object.
#[derive(Parser)]
pub struct Options {
    /// The address of the network interface to bind to, usually 0.0.0.0 to bind to all interfaces
    #[arg(long = "bind_addr", default_value = "0.0.0.0", env = "BIND_ADDRESS")]
    bind_address: IpAddr,

    /// The port to listen on, usually 80 or 443 depending on whether or not SSL is used
    #[arg(long = "port", default_value_t = 8080, env = "LISTEN_PORT")]
    listen_port: u16,

    /// Optional path to the private key for the server's certificate. The private key should be in PKCS#8 format
    ///
    /// Only needed if secure/SSL connections should be used
    #[arg(long = "cert-private-key", env = "CERT_PRIVATE_KEY")]
    cert_private_key: Option<PathBuf>,

    /// Optional path to the public key for the server's certificate
    ///
    /// Only needed if secure/SSL connections should be used
    #[arg(long = "cert-public-key", env = "CERT_PUBLIC_KEY")]
    cert_public_key: Option<PathBuf>,

    /// If enabled encryption will be optional. This means that the client can decide if encryption should be used for responses and decryption for requests
    ///
    /// The client decides by setting the Use-Encryption and Use-Decryption headers to true/false
    ///
    /// NOTE: Should only be used for debugging/development purposes and not for a production server
    #[arg(
        long = "opt-encryption",
        default_value_t = false,
        env = "OPT_ENCRYPTION"
    )]
    optional_encryption: bool,

    /// The Steam web api key used for authenticating and getting user info for Steam players. The key can be found here: https://steamcommunity.com/dev/apikey
    #[arg(long = "steam-api-key", env = "STEAM_API_KEY")]
    steam_api_key: String,

    /// The optional connection string to a postgresql database. This is where all data will be stored
    ///
    /// If unspecified then a portable version of postgresql will be downloaded and configured automatically for you.
    /// This is the easiest way to setup a local postgresql instance if you don't have an existing one.
    ///
    /// Example: postgres://localhost/parcels?user=postgres&password=1234
    #[arg(long = "database-url", env = "DATABASE_URL")]
    database_url: Option<String>,

    /// The public url that people can reach this server from. Do not end the url with a '/'
    ///
    /// If unspecified, this will default to the endpoint that the client is connecting from
    ///
    /// Example: https://ds.mydomain.com
    #[arg(long = "gateway-url", env = "GATEWAY_URL")]
    gateway_url: Option<String>,

    /// If enabled request logs will also include decrypted request body and response.
    /// This is a lot slower than normal logging so don't use this in production
    #[arg(long, default_value_t = false, env = "DEEP_LOGGING")]
    deep_logging: bool,

    #[arg(long, default_value = "data/game_data.json", env = "GAME_DATA_PATH")]
    game_data_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct GatewayUrl(String);

impl std::ops::Deref for GatewayUrl {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<GatewayUrl> for String {
    fn from(value: GatewayUrl) -> Self {
        value.0
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = Options::parse();

    // Create 'logs' directory
    std::fs::create_dir_all("logs").context("Could not create logs directory")?;

    let colors = ColoredLevelConfig::new()
        .trace(Color::White)
        .info(Color::BrightGreen)
        .debug(Color::Blue)
        .warn(Color::Yellow)
        .error(Color::BrightRed);
    let log_file = DateBased::new("logs/", "%Y-%m-%d.log");
    const LOG_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";
    fern::Dispatch::new()
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    let now = chrono::Local::now().format(LOG_TIME_FORMAT).to_string();
                    out.finish(format_args!(
                        "[{} {} {}] {}",
                        now,
                        colors.color(record.level()),
                        record.target(),
                        message
                    ))
                })
                .chain(std::io::stdout()),
        )
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    let now = chrono::Local::now().format(LOG_TIME_FORMAT).to_string();
                    out.finish(format_args!(
                        "[{} {} {}] {}",
                        now,
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .chain(log_file),
        )
        .level(log::LevelFilter::Info)
        .level_for("parcel_common", log::LevelFilter::Debug)
        .level_for("parcel_server", log::LevelFilter::Debug)
        .level_for("parcel_game_data", log::LevelFilter::Debug)
        .apply()?;

    log::info!("Server starting...");

    if args.cert_private_key.is_some() != args.cert_public_key.is_some() {
        anyhow::bail!("Both or none of the public and private keys needs to be specified");
    }

    // make sure data directory exists
    std::fs::create_dir_all("data").context("Could not create data directory")?;

    let database_url = embedded::postgresql::setup_postgresql(&args)
        .await
        .context("Failed to setup and launch postgresql server")?;

    // Create potentially mutable data outside of the HttpService factory, otherwise each worker thread will not share the same data globally.
    let steam_data = web::Data::new(
        Steam::new(args.steam_api_key.clone()).context("Could not create steam web api client")?,
    );
    let epic_data = web::Data::new(Epic::new().context("Could not create epic web api client")?);
    let session_store =
        web::Data::new(SessionStore::load_or_create(Path::new("data/sessions")).await);
    let session_store_clone = session_store.clone();
    let database = web::Data::new(Database::new(&database_url));
    let frontend_auth_cache = web::Data::new(FrontendAuthCache::with_time_to_live_secs(
        "FrontendAuthCache",
        60 * 2,
    ));
    let session_blacklist_cache = web::Data::new(
        SessionBlacklistCache::from_builder(
            CacheBuilder::default()
                .name("SessionBlacklistCache")
                .expire_after(SessionBlacklistCacheExpiry),
        )
        .load_from_file(Path::new(BLACKLIST_CACHE_PATH))
        .await
        .context("Could not load session blacklist")?,
    );
    let session_blacklist_cache_clone = session_blacklist_cache.clone();
    let session_permissions_cache = web::Data::new(SessionPermissionsCache::from_builder(
        CacheBuilder::default()
            .name("SessionPermissionsCache")
            .time_to_idle(Duration::from_secs(60 * 5)),
    ));
    let jwt_secret = web::Data::new(
        JwtSecret::load_or_generate_secret()
            .await
            .context("Failed to load jwt secret")?,
    );
    let hash_secret = web::Data::new(
        HashSecret::load_or_generate_secret()
            .await
            .context("Failed to load hash secret")?,
    );
    let game_data = web::Data::new(
        load_gamedata_from_file(&args.game_data_path).context("Could not load game data")?,
    );
    let server_settings = web::Data::new(
        ServerSettings::load_from_path(Path::new("data/settings.json"))
            .await
            .context("Could not load settings")?,
    );
    let whitelist_settings = web::Data::new(
        WhitelistSettings::load_from_path(Path::new("data/whitelist.txt"))
            .await
            .context("Could not load whitelist")?,
    );

    migrate_database(&database_url).context("Could not apply pending database migrations")?;
    create_admin_account_if_not_exists(&*database, &*hash_secret)
        .await
        .context("Could not check for or create admin account")?;

    let gateway_url = args.gateway_url.as_ref().map(|url| format!("{}/ds", url));

    if let Some(gateway_url) = gateway_url.as_ref() {
        log::info!(
            "Launching server with the public gateway url set to \"{}\"",
            gateway_url
        );
    } else {
        log::info!("Launching server on port {} with the public gateway url being inferred from the incoming connection", args.listen_port);
    }

    let mut background_job_scheduler =
        background_jobs::create_scheduler(database.clone().into_inner()).await?;
    background_job_scheduler
        .start()
        .await
        .context("Could not start scheduler for background jobs")?;

    let mut builder = HttpServer::new(move || {
        App::new()
            .app_data(steam_data.clone())
            .app_data(epic_data.clone())
            .app_data(session_store.clone())
            .app_data(database.clone())
            .app_data(web::Data::new(
                gateway_url.as_ref().map(|url| GatewayUrl(url.clone())),
            ))
            .app_data(frontend_auth_cache.clone())
            .app_data(session_blacklist_cache.clone())
            .app_data(session_permissions_cache.clone())
            .app_data(jwt_secret.clone())
            .app_data(hash_secret.clone())
            .app_data(game_data.clone())
            .app_data(server_settings.clone())
            .app_data(whitelist_settings.clone())
            .service(
                actix_web::web::scope("/ds/e")
                    .configure(endpoints::configure_endpoints)
                    .wrap(middleware::deep_logger::DeepLogger {
                        enabled: args.deep_logging,
                    })
                    // Make sure this is last middleware so that the data is decrypted before doing anything else that interacts with the encrypted data
                    .wrap(middleware::encryption::DataEncryption {
                        optional_encryption: args.optional_encryption,
                    })
                    .wrap(wrap_errors::WrapErrors),
            )
            .service(endpoints::auth::auth)
            .service(endpoints::auth::me::me)
            .service(actix_web::web::scope("/frontend").configure(frontend::configure_endpoints))
            .wrap(NormalizePath::trim())
            .wrap(actix_web::middleware::Logger::default())
    });

    if args.cert_public_key.is_some() {
        let ssl_config = load_rustls_config(
            args.cert_private_key.as_ref().unwrap(),
            args.cert_public_key.as_ref().unwrap(),
        )
        .context("Could not load ssl config")?;
        builder = builder.bind_rustls((args.bind_address, args.listen_port), ssl_config)?;
    } else {
        builder = builder.bind((args.bind_address, args.listen_port))?;
    }

    let result = builder.run().await.map_err(|err| err.into());

    // Stop embedded programs if they're running
    let stop_pg_result = embedded::postgresql::stop_postgresql().await;

    if let Err(err) = &stop_pg_result {
        log::error!("Could not gracefully stop postgresql server: {}", err);
        log::info!("Note: postgresql server has been stopped even if there are errors above");
    }

    let mut scheduler_stop_failed = false;
    if let Err(err) = background_job_scheduler.shutdown().await {
        log::error!("Could not shutdown scheduler for background jobs: {}", err);
        scheduler_stop_failed = true;
    }

    session_store_clone.save_to_file().await?;
    session_blacklist_cache_clone
        .save_to_file(Path::new(BLACKLIST_CACHE_PATH))
        .await?;

    log::info!("Server stopped\n");

    if scheduler_stop_failed {
        panic!("Could not shutdown background jobs");
    }

    result
}

fn load_rustls_config(
    private_key_path: &Path,
    public_key_path: &Path,
) -> Result<rustls::ServerConfig> {
    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let key_file = &mut BufReader::new(File::open(private_key_path)?);
    let cert_file = &mut BufReader::new(File::open(public_key_path)?);

    let cert_chain = certs(cert_file)?.into_iter().map(Certificate).collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)?
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        anyhow::bail!(
            "Could not load private keys from file. Make sure the key is of PKCS#8 format."
        );
    }

    Ok(config.with_single_cert(cert_chain, keys.remove(0))?)
}

fn migrate_database(database_url: &str) -> Result<(), anyhow::Error> {
    let mut pg_conn =
        PgConnection::establish(database_url).context("Could not connect to database")?;

    let pending_migrations =
        MigrationHarness::<Pg>::pending_migrations(&mut pg_conn, MIGRATIONS)
            .map_err(|err| anyhow::anyhow!("Could not get pending migrations: {}", err))?;

    log::info!("Pending database migrations: {}", pending_migrations.len());

    MigrationHarness::<Pg>::run_pending_migrations(&mut pg_conn, MIGRATIONS)
        .map_err(|err| anyhow::anyhow!("Could not run migrations: {}", err))?;

    if !pending_migrations.is_empty() {
        log::info!("Applied pending database migrations successfully");
    }

    Ok(())
}

fn load_gamedata_from_file(game_data_path: &Path) -> Result<GameData, anyhow::Error> {
    log::info!("Loading game data");

    let bytes = std::fs::read(game_data_path)?;
    let game_data = serde_json::from_slice(&bytes)?;

    Ok(game_data)
}

async fn create_admin_account_if_not_exists(
    database: &Database,
    hash_secret: &HashSecret,
) -> Result<(), anyhow::Error> {
    let conn = database.connect().await?;
    if let Some((username, password)) = conn
        .frontend_accounts()
        .create_admin_account_if_not_exists(hash_secret)
        .await?
    {
        log::warn!("Could not find an existing admin account");
        log::info!(
            "Created admin account with username \"{username}\" and password \"{password}\""
        );
    }

    Ok(())
}
