pub mod aes;
pub mod frontend;
mod http_utility;
mod incoming;
pub mod logger;
mod outgoing;
mod proxy_response_handler;
pub mod server;

use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, net::IpAddr, path::PathBuf, process::ExitCode};
use tokio::select;
use tui::{backend::CrosstermBackend, Terminal};

use server::start_http_server;

use crate::frontend::start_frontend_ui;

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
}

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    let args = Options::parse();

    if args.cert.is_some() != args.key.is_some() {
        println!("Both certificate and private key paths need to be specified");
        return Ok(ExitCode::from(1));
    }

    let secure_options = match args.cert.is_some() {
        true => Some((args.cert.as_deref().unwrap(), args.key.as_deref().unwrap())),
        false => None,
    };

    enable_raw_mode()?; // disables some default console behaviour

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let run_result;

    select! {
        result = start_http_server(secure_options, args.listen_port, args.bind_interface, args.gateway_domain.as_deref()) => {
            run_result = result;
        }
        result = start_frontend_ui(&mut terminal) => {
            run_result = result;
        }
    };

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = run_result {
        eprintln!("Fatal error: {:?}", err);
    }

    Ok(ExitCode::from(0))
}
