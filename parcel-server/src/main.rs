mod data;
mod endpoints;
mod middleware;
mod response_error;

use std::{io::Write, net::IpAddr};

use actix_web::{
    middleware as actix_middleware,
    web::{self, JsonConfig},
    App, HttpServer,
};
use anyhow::Result;
use clap::Parser;
use data::steam::Steam;
use endpoints::configure_endpoints;

#[derive(Parser)]
struct Options {
    #[arg(long = "bind_addr", default_value = "0.0.0.0")]
    bind_address: IpAddr,

    #[arg(long = "port", default_value_t = 8080)]
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
}

#[actix_web::main]
async fn main() -> Result<()> {
    let args = Options::parse();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error,warn,info,debug"));

    // Create potentially mutable data outside of the HttpService factory, otherwise each worker thread will not share the same data globally.
    let steam_data = web::Data::new(Steam::new(args.steam_api_key.clone()).unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(JsonConfig::default().content_type_required(false)) // don't require Content-Type: application/json header to parse json request body
            .app_data(steam_data.clone())
            .configure(configure_endpoints)
            .wrap(actix_middleware::Logger::default())
            .service(
                actix_web::web::scope("/e")
                    .configure(configure_endpoints)
                    // Make sure this is last middleware so that the data is decrypted before doing anything else
                    .wrap(middleware::encryption::DataEncryption {
                        optional_encryption: args.optional_encryption,
                    }),
            )
            .service(endpoints::auth::auth)
    })
    .bind((args.bind_address, args.listen_port))?
    .run()
    .await
    .map_err(|err| err.into())
}
