mod pallete;
mod convert;

use anyhow::{Ok, Result};
use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style, Styled}, widgets::{Block, Borders, List, ListState, Padding, Paragraph, Widget, canvas::Line}};
use crate::app::state::State;
use crate::events::command::Command;
use convert::{to_list_item, highlighted};

#[derive(Default)]
pub struct Tui {
    list_state: ListState,
}

impl Tui {
    pub fn new() -> Tui {
        Tui { list_state: ListState::default().with_selected(Some(0)) }
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

    pub fn handle_input(&self, key: KeyCode) -> Result<Option<Command>> {
        Ok(None)
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
        let items = state.entries
            .iter()
            .enumerate()
            .map(|(idx, e)| 
                to_list_item(e, highlighted(idx, self.list_state.selected())));

        let style = Style::default()
            .bg(pallete::SECONDARY_BG);

        let list = List::new(items)
            .style(style)
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, rect, &mut self.list_state);
    }
    
    fn draw_footer(&self, state: &State, frame: &mut Frame, rect: Rect) {
        let bg = Style::default().bg(pallete::PRIMARY_BG);
    
        let paragraph = Paragraph::new("")
            .style(bg);
    
        frame.render_widget(paragraph, rect);
    }
}