use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style, Styled}, widgets::{Block, Borders, List, ListState, Padding, Paragraph, Widget, canvas::Line}};
use crate::{state::{PickType, State}, events::callback::Callback};
use tracing::{info, warn, error, debug};
use crate::view::pallete;

pub fn draw(state: &State, frame: &mut Frame, rect: Rect) {
    let style: Style = Style::default()
        .bg(pallete::PRIMARY_BG)
        .fg(pallete::MUTED_TXT);

    let paragraph = Paragraph::new("move up ⏎")
        .style(style).alignment(ratatui::layout::HorizontalAlignment::Right);

    frame.render_widget(paragraph, rect);
}