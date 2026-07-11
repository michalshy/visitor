pub mod state;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};
use anyhow::{Ok, Result};

use state::State;
use crate::events::queue::{process, Queue};
use crate::events::command::Command;
use crate::tui::{Tui};

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
        queue.push(Command::LIST_DIR);
        Ok(App { state, queue, exit: false })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let tui = Tui::default();
        while !self.exit {
            process(&mut self.queue, &mut self.state)?;
            terminal.draw(|frame| tui.draw(&self.state, frame))?;
            self.handle_events(&tui)?;
        }
        Ok(())
    }

    fn handle_events(&mut self, tui: &Tui) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                return self.handle_key(key_event, tui);
            }
            _ => { }
        };
        Ok(())
    }

    fn handle_key(&mut self, key_event: KeyEvent, tui: &Tui) -> Result<()> {
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