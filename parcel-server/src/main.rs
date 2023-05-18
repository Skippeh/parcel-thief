mod data;
mod db;
mod embedded_pg;
mod endpoints;
mod middleware;
mod response_error;
mod session;

use std::{
    fs::File,
    io::BufReader,
    net::IpAddr,
    path::{Path, PathBuf},
};

use actix_web::{
    web::{self},
    App, HttpServer,
};
use anyhow::{Context, Result};
use clap::Parser;
use data::{
    database::Database,
    platforms::{epic::Epic, steam::Steam},
    redis_client::RedisClient,
};
use diesel::{pg::Pg, Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use endpoints::configure_endpoints;
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};

use crate::{data::redis_session_store::RedisSessionStore, middleware::wrap_errors};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

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

    /// The connection string to a redis database. This is where cached data and session info will be stored
    ///
    /// If unspecified then an embedded redis instance will be launched and used.
    /// This is the easiest way to setup a redis server if you don't have an existing one.
    ///
    /// Example: redis://localhost
    #[arg(long = "redis-url", env = "REDIS_URL")]
    redis_url: Option<String>,

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
}

#[derive(Debug, Clone)]
pub struct GatewayUrl(String);

impl<'a> From<&'a GatewayUrl> for &'a str {
    fn from(value: &'a GatewayUrl) -> Self {
        &value.0
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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error,warn,info,debug"));

    if args.cert_private_key.is_some() != args.cert_public_key.is_some() {
        anyhow::bail!("Both or none of the public and private keys needs to be specified");
    }

    let redis_url = setup_redis(&args)
        .await
        .context("Failed to launch redis server")?;
    let database_url = embedded_pg::setup_postgresql(&args)
        .await
        .context("Failed to setup and launch postgresql server")?;

    // Create potentially mutable data outside of the HttpService factory, otherwise each worker thread will not share the same data globally.
    let redis_client_data = web::Data::new(
        RedisClient::connect(redis_url)
            .await
            .context("Could not connect to redis server")?,
    );

    let redis_client = redis_client_data.clone().into_inner();
    let steam_data = web::Data::new(
        Steam::new(args.steam_api_key.clone()).context("Could not create steam web api client")?,
    );
    let epic_data = web::Data::new(Epic::new().context("Could not create epic web api client")?);
    let session_store = web::Data::new(RedisSessionStore::new(redis_client.clone(), "ds-session/"));
    let database = web::Data::new(Database::new(&database_url));

    migrate_database(&database_url)
        .await
        .context("Could not apply pending database migrations")?;

    let gateway_url = args.gateway_url.as_ref().map(|url| format!("{}/ds", url));

    if let Some(gateway_url) = gateway_url.as_ref() {
        log::info!(
            "Launching server with the public gateway url set to \"{}\"",
            gateway_url
        );
    } else {
        log::info!("Launching server with the public gateway url being inferred from the incoming connection");
    }

    let mut builder = HttpServer::new(move || {
        App::new()
            .app_data(redis_client_data.clone())
            .app_data(steam_data.clone())
            .app_data(epic_data.clone())
            .app_data(session_store.clone())
            .app_data(database.clone())
            .app_data(web::Data::new(
                gateway_url.as_ref().map(|url| GatewayUrl(url.clone())),
            ))
            .service(
                actix_web::web::scope("/ds/e")
                    .configure(configure_endpoints)
                    .wrap(middleware::deep_logger::DeepLogger {
                        enabled: args.deep_logging,
                    })
                    // Make sure this is last middleware so that the data is decrypted before doing anything else that interacts with the encrypted data
                    .wrap(middleware::encryption::DataEncryption {
                        optional_encryption: args.optional_encryption,
                    }),
            )
            .service(endpoints::auth::auth)
            .service(endpoints::auth::me::me)
            .wrap(wrap_errors::WrapErrors::default())
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

    builder.run().await.map_err(|err| err.into())
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

async fn migrate_database(database_url: &str) -> Result<(), anyhow::Error> {
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

/// Sets up local redis server (if necessary) and returns the redis connection string.
async fn setup_redis(args: &Options) -> Result<String, anyhow::Error> {
    match args.redis_url.as_ref() {
        Some(url) => Ok(url.clone()),
        None => {
            todo!()
        }
    }
}
