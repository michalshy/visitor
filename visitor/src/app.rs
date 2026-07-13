pub mod state;

use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, Event};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};
use anyhow::{Ok, Result};

use state::State;
use crate::events::queue::{process, Queue};
use crate::events::command::Command;
use crate::events::callback::{self, Callback};
use crate::tui::{Tui};

use std::sync::mpsc;

pub struct App
{
    pub state: State,
    pub queue: Queue,
    pub exit: bool
}

impl App {
    pub fn init() -> Result<App> {
        let state = State::init()?;
        let mut queue = Queue::default();
        queue.push(Command::ListDir);
        Ok(App { state, queue, exit: false })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let (tx, rx) = mpsc::channel::<Callback>();
        let mut tui = Tui::new(rx);
        while !self.exit {
            process(&mut self.queue, &mut self.state, tx.clone())?;
            tui.update()?;
            terminal.draw(|frame| tui.draw(&self.state, frame))?;
            if event::poll(Duration::from_millis(16))? {
                self.handle_events(&mut tui)?;
            }
        }
        Ok(())
    }

    fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                return self.handle_key(key_event, tui);
            }
            _ => { }
        };
        Ok(())
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