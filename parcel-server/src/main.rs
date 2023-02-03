mod endpoints;
mod middleware;

use std::net::IpAddr;

use actix_web::{middleware as actix_middleware, web::JsonConfig, App, HttpServer};
use anyhow::Result;
use clap::Parser;
use endpoints::configure_endpoints;

#[derive(Parser)]
struct Options {
    #[arg(long = "bind_addr", default_value = "0.0.0.0")]
    bind_address: IpAddr,

    #[arg(long = "port", default_value_t = 8080)]
    listen_port: u16,

    /// If specified encryption will be optional. This means that the client can decide if encryption should be used for responses and decryption for requests.
    ///
    /// The client decides by setting the Use-Encryption and Use-Decryption headers.
    ///
    /// NOTE: Should only be used for debugging/development purposes and not for a production server.
    #[arg(long = "opt_encryption", default_value_t = false)]
    optional_encryption: bool,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let args = Options::parse();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error,warn,info,debug"));
    HttpServer::new(move || {
        let json_config = JsonConfig::default().content_type_required(false); // don't require Content-Type: application/json header to parse json request body

        App::new()
            .app_data(json_config)
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
    })
    .bind((args.bind_address, args.listen_port))?
    .run()
    .await
    .map_err(|err| err.into())
}
