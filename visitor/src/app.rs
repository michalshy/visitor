mod logger;

use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, Event};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};
use anyhow::{Ok, Result};

use crate::app::logger::Logger;
use crate::action::Action;
use crate::input;
use crate::state::State;
use crate::update::update;
use crate::view::draw;

use std::sync::mpsc;

pub struct App
{
    state: State,
    _logger: Logger,
    exit: bool
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
                    if let Some(action) = input::map_key(k.code) {
                        update(&mut self.state, action)?;       // mutate state — the one call
                    }
                }
            }
        }
        Ok(())
    }
}