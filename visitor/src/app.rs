mod logger;

use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, Event};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};
use anyhow::{Ok, Result};

use crate::app::logger::Logger;
use crate::events::queue::{process, Queue};
use crate::events::callback::{self, Callback};
use crate::action::Action;
use crate::state::State;
use crate::view::draw;

use std::sync::mpsc;

pub struct App
{
    state: State,
    queue: Queue,
    _logger: Logger,
    exit: bool
}

impl App {
    pub fn init() -> Result<App> {
        let _logger = Logger::new();
        let state = State::init()?;
        let mut queue = Queue::default();
        queue.push(Action::ListDir);
        Ok(App { state, queue, _logger, exit: false })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let (tx, rx) = mpsc::channel::<Callback>();
        while !self.exit {
            process(&mut self.queue, &mut self.state, tx.clone())?;
            terminal.draw(|frame| draw(&mut self.state, frame))?;
            if event::poll(Duration::from_millis(16))? {
                self.handle_events(&mut tui)?;
            }
        }
        Ok(())
    }

    fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        
    }

    fn handle_key(&mut self, key_event: KeyEvent, tui: &mut Tui) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => self.exit = true,
            code => { 
                if let Some(cmd) = tui.handle_input(code)? {
                    self.queue.push(cmd);
                }
            }
        }
        Ok(())
    }
}