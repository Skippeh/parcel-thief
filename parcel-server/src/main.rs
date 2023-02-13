#![allow(dead_code)] // todo: remove this when things are starting to come together

mod data;
mod db;
mod endpoints;
mod middleware;
mod response_error;
mod session;

use std::net::IpAddr;

use actix_web::{
    middleware as actix_middleware,
    web::{self, JsonConfig},
    App, HttpServer,
};
use anyhow::{Context, Result};
use clap::Parser;
use data::{database::Database, steam::Steam};
use endpoints::configure_endpoints;
use session::redis::RedisSessionStore;

use crate::middleware::wrap_errors;

#[derive(Parser)]
struct Options {
    #[arg(long = "bind_addr", default_value = "0.0.0.0", env = "BIND_ADDRESS")]
    bind_address: IpAddr,

    #[arg(long = "port", default_value_t = 8080, env = "LISTEN_PORT")]
    listen_port: u16,

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

    // Create potentially mutable data outside of the HttpService factory, otherwise each worker thread will not share the same data globally.
    let steam_data = web::Data::new(Steam::new(args.steam_api_key.clone()).unwrap());

    let session_store = web::Data::new(
        RedisSessionStore::new(args.redis_connection_string, "ds-session/")
            .await
            .context("could not connect to redis server")?,
    );

    let database = web::Data::new(Database::new(&args.database_url));

    // Test database connection
    database
        .connect()
        .context("Could not connect to database")?;

    log::info!(
        "Launching server with the public gateway url set to \"{}/e\"",
        args.gateway_url
    );

    HttpServer::new(move || {
        App::new()
            .app_data(JsonConfig::default().content_type_required(false)) // don't require Content-Type: application/json header to parse json request body
            .app_data(steam_data.clone())
            .app_data(session_store.clone())
            .app_data(database.clone())
            .app_data(web::Data::new(GatewayUrl(format!(
                "{}/e",
                args.gateway_url
            ))))
            .service(
                actix_web::web::scope("/e")
                    .configure(configure_endpoints)
                    // Make sure this is last middleware so that the data is decrypted before doing anything else that interacts with the encrypted data
                    .wrap(middleware::encryption::DataEncryption {
                        optional_encryption: args.optional_encryption,
                    }),
            )
            .service(endpoints::auth::auth)
            .service(endpoints::auth::me::me)
            .wrap(wrap_errors::WrapErrors::default())
            .wrap(actix_middleware::Logger::default())
    })
    .bind((args.bind_address, args.listen_port))?
    .run()
    .await
    .map_err(|err| err.into())
}
