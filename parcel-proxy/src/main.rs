mod http_utility;
mod incoming;
pub mod logger;
mod outgoing;
mod proxy_response_handler;
pub mod server;

use clap::Parser;
use lazy_static::lazy_static;
use std::{net::IpAddr, path::PathBuf, process::ExitCode, str::FromStr, sync::Arc};
use tokio::{select, sync::RwLock};

use server::start_http_server;

lazy_static! {
    pub static ref LOG_DIRECTORY: Arc<RwLock<PathBuf>> =
        Arc::new(RwLock::new(PathBuf::from_str("./logs").unwrap()));
}

#[derive(Debug, Parser)]
struct Options {
    /// Path to the file that contains the public certificate
    cert: Option<PathBuf>,
    /// Path to the file that contains the private key for the public certificate
    key: Option<PathBuf>,
    /// The port to listen on. Default value uses 80 if cert and key are not set, and 443 if they are
    #[arg(long)]
    listen_port: Option<u16>,
    /// The network interface to bind on
    #[arg(long, name = "bind_interface", default_value = "0.0.0.0")]
    bind_interface: IpAddr,
    /// The domain or ip of the gateway. This will be part of the public url returned to the game when authenticating.
    /// If unspecified the value depends on the bind interface (if set to 0.0.0.0/:: it will be localhost, otherwise it'll match the interface address)
    #[arg(long)]
    gateway_domain: Option<String>,
    /// The path to the directory where logs should be saved. Default is a subfolder in current working directory called "logs"
    #[arg(long)]
    logs_dir: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    let args = Options::parse();

    if args.cert.is_some() != args.key.is_some() {
        println!("Both certificate and private key paths need to be specified");
        return Ok(ExitCode::from(1));
    }

    {
        let mut log_directory = LOG_DIRECTORY.write().await;
        if let Some(logs_dir) = args.logs_dir {
            if !logs_dir.is_dir() {
                println!("Logs directory does not point to a directory. Does the directory exist?");
                return Ok(ExitCode::from(1));
            } else {
                let abs_path = std::fs::canonicalize(&logs_dir)?;
                println!("Logs will be saved in \"{}\"", abs_path.display());
                *log_directory = abs_path;
            }
        }
    }

    let secure_options = match args.cert.is_some() {
        true => Some((args.cert.as_deref().unwrap(), args.key.as_deref().unwrap())),
        false => None,
    };

    select! {
        result = start_http_server(secure_options, args.listen_port, args.bind_interface, args.gateway_domain.as_deref()) => {
            if let Err(err) = result {
                eprintln!("{:?}", err);
            }
        }
        _ = tokio::signal::ctrl_c() => {}
    };

    Ok(ExitCode::from(0))
}
