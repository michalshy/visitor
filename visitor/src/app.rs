mod logger;

use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, Event};
use ratatui::{DefaultTerminal};
use anyhow::{Ok, Result};

use crate::app::logger::Logger;
use crate::input;
use crate::state::{Mode, State};
use crate::update::update;
use crate::view::draw;

pub struct App
{
    state: State,
    _logger: Logger,
    exit: bool,
}

impl App {
    pub fn init() -> Result<App> {
        let _logger = Logger::new();
        let state = State::init()?;
        Ok(App { state, _logger, exit: false })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| draw(&mut self.state, frame))?;
            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(k) = event::read()? {
                    self.handle_event(k, mode)?;       
                }
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: KeyEvent, mode: &Mode) -> Result<()> {
        if event.kind == KeyEventKind::Press {
            match event.code {
                KeyCode::Esc => self.exit = true,
                _ => {
                    if let Some(action) = input::map_key(event.code, mode) {
                        update(&mut self.state, action)?;
                    }
                }
            }
        }
        Ok(())
    }
}