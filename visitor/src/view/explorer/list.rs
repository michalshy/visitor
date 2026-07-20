use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style, Styled}, widgets::{Block, Borders, List, ListState, Padding, Paragraph, Widget, canvas::Line}};
use crate::{state::{Mode, PickType, State}, view::pallete::is_dimmed};
use tracing::{info, warn, error, debug};
use crate::view::pallete;
use crate::view::convert::{to_list_item, highlighted};

pub fn draw(state: &mut State, frame: &mut Frame, rect: Rect) {
    let items = state.entries
        .iter()
        .enumerate()
        .map(|(idx, e)| 
            to_list_item(e, highlighted(idx, state.list_state.selected()), state.mode == Mode::Normal));

    let style = Style::default()
        .bg(is_dimmed(pallete::SECONDARY_BG, &state.mode));

    let list = List::new(items)
        .style(style);

    frame.render_stateful_widget(list, rect, &mut state.list_state);
}