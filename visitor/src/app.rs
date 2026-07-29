mod logger;

use std::time::Duration;

use crossterm::event::{self, KeyEvent, KeyEventKind, Event};
use ratatui::{DefaultTerminal};
use anyhow::{Ok, Result};
use tracing::warn;

use crate::app::logger::Logger;
use crate::input;
use crate::state::State;
use crate::update::{update, update_preview};
use crate::view::draw;

pub struct App
{
    state: State,
    _logger: Logger,
}

impl App {
    pub fn init() -> Result<App> {
        let _logger = Logger::new();
        let state = State::init()?;
        Ok(App { state, _logger })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        if let Err(err) = update_preview(&mut self.state) {
            warn!(?err, "Could not update preview")
        }
        while !self.state.exit {
            terminal.draw(|frame| draw(&mut self.state, frame))?;
            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(k) = event::read()? {
                    self.handle_event(k);       
                }
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: KeyEvent) {
        if event.kind == KeyEventKind::Press {
            match event.code {
                _ => {
                    if let Some(action) = input::map_key(event.code, &self.state.mode) {
                        if let Err(err) = update(&mut self.state, action) {
                            warn!(?err, "Update failed");
                        }
                    }
                }
            }
        }
    }
}