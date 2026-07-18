use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style, Styled}, widgets::{Block, Borders, List, ListState, Padding, Paragraph, Widget, canvas::Line}};
use crate::{state::{PickType, State}};
use tracing::{info, warn, error, debug};
use crate::view::pallete;

pub fn draw(state: &State, frame: &mut Frame, rect: Rect) {
    let style = Style::default()
        .bg(pallete::SECONDARY_BG);

    let block = Block::default().style(style);
    frame.render_widget(block, rect);
}