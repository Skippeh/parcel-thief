mod endpoints;
mod middleware;

use std::net::IpAddr;

use actix_web::{middleware as actix_middleware, web::JsonConfig, App, HttpServer};
use anyhow::Result;
use clap::Parser;
use endpoints::configure_endpoints;

#[derive(Parser)]
struct Options {
    #[arg(default_value = "0.0.0.0")]
    bind_address: IpAddr,

    #[arg(default_value_t = 8080)]
    listen_port: u16,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let args = Options::parse();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error,warn,info,debug"));

    HttpServer::new(|| {
        let json_config = JsonConfig::default().content_type_required(false); // don't require Content-Type: application/json header to parse json request body

        App::new()
            .app_data(json_config)
            .configure(configure_endpoints)
            .wrap(actix_middleware::Logger::default())
            .service(
                actix_web::web::scope("/e")
                    .configure(configure_endpoints)
                    // Make sure this is last middleware so that the data is decrypted before doing anything else
                    .wrap(middleware::encryption::DataEncryption::default()),
            )
    })
    .bind((args.bind_address, args.listen_port))?
    .run()
    .await
    .map_err(|err| err.into())
}
