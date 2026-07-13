mod pallete;
mod convert;

use anyhow::{Ok, Result};
use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style, Styled}, widgets::{Block, Borders, List, ListState, Padding, Paragraph, Widget, canvas::Line}};
use crate::app::state::State;
use crate::events::command::Command;
use convert::{to_list_item, highlighted};

const DEFAULT_PREVIEW: u8 = 40;

#[derive(Default)]
pub struct Tui {
    list_state: ListState,
    preview_size: u8 // in percentage
}

impl Tui {
    pub fn new() -> Tui {
        Tui { list_state: ListState::default().with_selected(Some(0)), preview_size: DEFAULT_PREVIEW }
    }

    pub fn draw(&mut self, state: &State, frame: &mut Frame) {
        let outer = Block::default()
            .borders(Borders::ALL)
            .padding(Padding { left: 1, right: 1, top: 0, bottom: 0 })
            .style(Style::default()
                .fg(pallete::BORDER)
                .bg(pallete::PRIMARY_BG)
            );
    
        let inner = outer.inner(frame.area());
        frame.render_widget(outer, frame.area());
    
        let [header, main, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]).areas(inner); 
    
        self.draw_header(state, frame, header);
        self.draw_main(state, frame, main);
        self.draw_footer(state, frame, footer);
    }

    pub fn handle_input(&mut self, key: KeyCode) -> Result<Option<Command>> {
        let mut stale_file = false;
        match key {
            KeyCode::Up => {
                self.list_state.select_previous();
                stale_file = true;
            },
            KeyCode::Down => {
                self.list_state.select_next();
                stale_file = true;
            },
            KeyCode::Char(']') => {
                if self.preview_size > 2 {
                    self.preview_size = self.preview_size.saturating_sub(1);
                }
            },
            KeyCode::Char('[') => {
                if self.preview_size < 99 {
                    self.preview_size = self.preview_size.saturating_add(1);
                }
            },
            KeyCode::Backspace => {
                return Ok(Some(Command::MoveUp));
            }
            _ => {}
        }
        if stale_file {
            if let Some(idx) = self.list_state.selected() {
                return Ok(Some(Command::GetFileDetails { idx }));
            } else {
                return Ok(None)
            }
        } 
        return Ok(None)
    }
    
    fn draw_header(&self, state: &State, frame: &mut Frame, rect: Rect) {
        let style = Style::default()
            .bg(pallete::PRIMARY_BG).fg(pallete::PRIMARY_TXT);
        let title =  state.current_dir.to_string_lossy();
        let paragraph = Paragraph::new(title)
            .style(style);
    
        frame.render_widget(paragraph, rect);
    }
    
    fn draw_main(&mut self, state: &State, frame: &mut Frame, rect: Rect) {
        let list_size = 100 - self.preview_size;
        let [list, preview] = Layout::horizontal([
            Constraint::Percentage(list_size as u16),
            Constraint::Percentage(self.preview_size as u16)
        ]).spacing(1).areas(rect);

        self.draw_list(state, frame, list);
        self.draw_preview(state, frame, preview);
    }

    fn draw_list(&mut self, state: &State, frame: &mut Frame, rect: Rect) {
        let items = state.entries
            .iter()
            .enumerate()
            .map(|(idx, e)| 
                to_list_item(e, highlighted(idx, self.list_state.selected())));

        let style = Style::default()
            .bg(pallete::SECONDARY_BG);

        let list = List::new(items)
            .style(style);

        frame.render_stateful_widget(list, rect, &mut self.list_state);
    }

    fn draw_preview(&mut self, state: &State, frame: &mut Frame, rect: Rect) {
        let style = Style::default()
            .bg(pallete::SECONDARY_BG);

        let block = Block::default().style(style);
        frame.render_widget(block, rect);
    }
    
    fn draw_footer(&self, state: &State, frame: &mut Frame, rect: Rect) {
        let style: Style = Style::default()
            .bg(pallete::PRIMARY_BG)
            .fg(pallete::MUTED_TXT);
    
        let paragraph = Paragraph::new("⏎ backspace")
            .style(style).alignment(ratatui::layout::HorizontalAlignment::Right);
    
        frame.render_widget(paragraph, rect);
    }
}