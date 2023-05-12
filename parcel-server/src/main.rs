mod data;
mod db;
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

#[derive(Parser)]
struct Options {
    #[arg(long = "bind_addr", default_value = "0.0.0.0", env = "BIND_ADDRESS")]
    bind_address: IpAddr,

    #[arg(long = "port", default_value_t = 8080, env = "LISTEN_PORT")]
    listen_port: u16,

    #[arg(long = "cert-private-key", env = "CERT_PRIVATE_KEY")]
    cert_private_key: Option<PathBuf>,

    #[arg(long = "cert-public-key", env = "CERT_PUBLIC_KEY")]
    cert_public_key: Option<PathBuf>,

    /// If specified encryption will be optional. This means that the client can decide if encryption should be used for responses and decryption for requests.
    ///
    /// The client decides by setting the Use-Encryption and Use-Decryption headers to true/false.
    ///
    /// NOTE: Should only be used for debugging/development purposes and not for a production server.
    #[arg(long = "opt-encryption", default_value_t = false)]
    optional_encryption: bool,

    /// The Steam web api key used for authenticating and getting user info for Steam players. The key can be found here: https://steamcommunity.com/dev/apikey
    ///
    /// If unspecified the STEAM_API_KEY environment variable will be used.
    #[arg(long = "steam-api-key", env = "STEAM_API_KEY")]
    steam_api_key: String,

    #[arg(long = "redis-conn-string", env = "REDIS_CONNECTION_STRING")]
    redis_connection_string: String,

    #[arg(long = "database-url", env = "DATABASE_URL")]
    database_url: String,

    /// The public url that people can reach this server from. Do not end the url with a '/'.
    ///
    /// Example: https://ds.mydomain.com
    #[arg(long = "gateway-url", env = "GATEWAY_URL")]
    gateway_url: String,

    /// If set, request logs will also include decrypted request body and response.
    /// This is a lot slower than the normal logging, so don't use this in production.
    #[arg(long, default_value_t = false)]
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

    // Create potentially mutable data outside of the HttpService factory, otherwise each worker thread will not share the same data globally.
    let redis_client_data = web::Data::new(
        RedisClient::connect(args.redis_connection_string)
            .await
            .context("Could not connect to redis server")?,
    );

    let redis_client = redis_client_data.clone().into_inner();
    let steam_data = web::Data::new(
        Steam::new(
            args.steam_api_key.clone(),
            redis_client.clone(),
            "platform/steam/",
        )
        .context("Could not create steam web api client")?,
    );
    let epic_data = web::Data::new(
        Epic::new(redis_client.clone(), "platform/epic/")
            .context("Could not create epic web api client")?,
    );
    let session_store = web::Data::new(RedisSessionStore::new(redis_client.clone(), "ds-session/"));
    let database = web::Data::new(Database::new(&args.database_url));

    migrate_database(&args.database_url)
        .await
        .context("Could not apply pending database migrations")?;

    let gateway_url = format!("{}/ds", args.gateway_url);

    log::info!(
        "Launching server with the public gateway url set to \"{}\"",
        gateway_url
    );

    let mut builder = HttpServer::new(move || {
        App::new()
            .app_data(redis_client_data.clone())
            .app_data(steam_data.clone())
            .app_data(epic_data.clone())
            .app_data(session_store.clone())
            .app_data(database.clone())
            .app_data(web::Data::new(GatewayUrl(gateway_url.clone())))
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
            "Could not load private keys from file. Make sure the key is of PKCS8 format."
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
