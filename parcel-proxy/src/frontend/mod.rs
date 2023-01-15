use std::io::Stdout;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use tokio::{select, sync::mpsc};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{AppState, UiMessage};

pub async fn start_frontend_ui(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app_state: AppState,
    mut ui_rx: mpsc::UnboundedReceiver<UiMessage>,
) -> Result<()> {
    loop {
        select! {
            event = read_event_async() => {
                if !handle_event(event?)? {
                    break;
                }
            }
            ui_message = ui_rx.recv() => {
                match ui_message {
                    Some(ui_message) => handle_ui_message(ui_message, &app_state).await?,
                    None => break,
                }
            }
        }
    }

    Ok(())
}

/// Handles a console event. Returns false if the user wants to exit the app.
fn handle_event(event: Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('q') => return Ok(false),
            _ => return Ok(true),
        }
    }

    Ok(true)
}

async fn handle_ui_message(message: UiMessage, app_state: &AppState) -> Result<()> {
    match message {
        UiMessage::LogMessage(message) => {
            println!("{}", message);
            app_state.lock().await.log_messages.push(message);
        }
    }

    Ok(())
}

async fn read_event_async() -> core::result::Result<Event, std::io::Error> {
    tokio::spawn(async move { event::read() }).await?
}
