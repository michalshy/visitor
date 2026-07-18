mod preview;
mod list;

use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style, Styled}, widgets::{Block, Borders, List, ListState, Padding, Paragraph, Widget, canvas::Line}};
use crate::{state::{PickType, State}};
use tracing::{info, warn, error, debug};
use crate::view::pallete;

pub fn draw(state: &mut State, frame: &mut Frame, rect: Rect) {
    let list_size = 100 - state.preview_size;
    let [list, preview] = Layout::horizontal([
        Constraint::Percentage(list_size as u16),
        Constraint::Percentage(state.preview_size as u16)
    ]).spacing(1).areas(rect);

    list::draw(state, frame, list);
    preview::draw(state, frame, preview);
}