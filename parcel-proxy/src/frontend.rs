use std::io::Stdout;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use tui::{backend::CrosstermBackend, Terminal};

pub async fn start_frontend_ui(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    Ok(())
}
