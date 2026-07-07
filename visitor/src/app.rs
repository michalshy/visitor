mod queue;
mod state;
mod command;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};
use anyhow::Result;

use state::State;
use queue::{process, Queue};
use command::Command;

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
        while !self.exit {
            process(&mut self.queue, &mut self.state)?;
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit = true,
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut entries: Vec<String> = Vec::new();

    }
}