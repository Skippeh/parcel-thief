mod endpoints;

use std::net::IpAddr;

use actix_web::{middleware, web::JsonConfig, App, HttpServer};
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
            .wrap(middleware::Logger::default())
            .app_data(json_config)
            .configure(configure_endpoints)
    })
    .bind((args.bind_address, args.listen_port))?
    .run()
    .await
    .map_err(|err| err.into())
}
