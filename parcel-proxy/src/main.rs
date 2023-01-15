pub mod aes;
pub mod frontend;
mod http_utility;
mod incoming;
pub mod logger;
mod outgoing;
mod proxy_response_handler;
pub mod server;
pub mod state;

use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, net::IpAddr, path::PathBuf, process::ExitCode, sync::Arc};
use tokio::{
    select,
    sync::{mpsc, Mutex},
};
use tui::{backend::CrosstermBackend, Terminal};

use server::start_http_server;

use crate::frontend::start_frontend_ui;

pub type AppState = Arc<Mutex<state::AppState>>;

#[derive(Debug)]
pub enum UiMessage {
    LogMessage(String),
}

lazy_static::lazy_static! {
    pub static ref UI_MESSAGE_SENDER: Arc<std::sync::Mutex<Option<mpsc::UnboundedSender<UiMessage>>>> = Arc::new(std::sync::Mutex::new(None));
}

/// Logs a message to the tui console
macro_rules! cprintln {
    () => {
        {
            let sender = crate::UI_MESSAGE_SENDER.lock().expect("UI_MESSAGE_SENDER receiver poisoned or closed");
            if sender.is_some() {
                let _ = sender
                    .as_ref()
                    .unwrap()
                    .send(crate::UiMessage::LogMessage(String::new()));
            }
        }
    };
    ( $x:expr ) => {
        {
            let sender = crate::UI_MESSAGE_SENDER.lock().expect("UI_MESSAGE_SENDER receiver poisoned or closed");
            if sender.is_some() {
                let _ = sender
                    .as_ref()
                    .unwrap()
                    .send(crate::UiMessage::LogMessage($x.into()));
            }
        }
    };
    ( $($x:expr),+ ) => {
        {
            let sender = crate::UI_MESSAGE_SENDER.lock().expect("UI_MESSAGE_SENDER receiver poisoned or closed");
            if sender.is_some() {
                let msg = format!($($x,)*);
                let _ = sender
                    .as_ref()
                    .unwrap()
                    .send(crate::UiMessage::LogMessage(msg));
            }
        }
    };
}

pub(crate) use cprintln;

#[derive(Debug, Parser)]
pub struct Options {
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

    let app_state = Arc::new(Mutex::new(state::AppState::default()));
    let (ui_tx, ui_rx) = mpsc::unbounded_channel::<UiMessage>();
    *UI_MESSAGE_SENDER.lock().unwrap() = Some(ui_tx.clone());

    enable_raw_mode()?; // disables some default console behaviour

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let run_result;

    select! {
        result = tokio::spawn(start_http_server(args, app_state.clone(), ui_tx)) => {
            // this is never reached at the moment because start_frontend_ui is not really async,
            // which means the ui thread blocks until it's done.
            // effectively this means that if the networking thread returns an error we'll never know :)
            run_result = result?;
        }
        result = start_frontend_ui(&mut terminal, app_state.clone(), ui_rx) => {
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
