mod convert;
mod explorer;
mod footer;
mod header;
mod pallete;

use crate::state::State;
use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style, Styled}, widgets::{Block, Borders, List, ListState, Padding, Paragraph, Widget, canvas::Line}};
use crate::{state::{PickType}, events::callback::Callback};
use tracing::{info, warn, error, debug};


pub fn draw(state: &mut State, frame: &mut Frame) {
    let outer = Block::default()
        .borders(Borders::ALL)
        .padding(Padding { left: 1, right: 1, top: 0, bottom: 0 })
        .style(Style::default()
            .fg(pallete::BORDER)
            .bg(pallete::PRIMARY_BG)
        );

    let inner = outer.inner(frame.area());
    frame.render_widget(outer, frame.area());

    let [header, explorer, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ]).areas(inner); 

    header::draw(state, frame, header);
    explorer::draw(state, frame, explorer);
    footer::draw(state, frame, footer);
}