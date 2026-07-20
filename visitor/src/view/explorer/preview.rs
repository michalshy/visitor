use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style, Styled}, widgets::{Block, Borders, List, ListState, Padding, Paragraph, Widget, canvas::Line}};
use crate::{state::{PickType, State}, view::pallete::is_dimmed};
use tracing::{info, warn, error, debug};
use crate::view::pallete;

pub fn draw(state: &State, frame: &mut Frame, rect: Rect) {
    let style = Style::default()
        .bg(is_dimmed(pallete::SECONDARY_BG, &state.mode));

    let block = Block::default().style(style);
    frame.render_widget(block, rect);
}