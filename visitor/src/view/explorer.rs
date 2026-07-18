mod preview;
mod list;

use ratatui::{Frame, layout::{Constraint, Layout, Rect}};
use crate::{state::State};

pub fn draw(state: &mut State, frame: &mut Frame, rect: Rect) {
    let list_size = 100 - state.preview_size;
    let [list, preview] = Layout::horizontal([
        Constraint::Percentage(list_size as u16),
        Constraint::Percentage(state.preview_size as u16)
    ]).spacing(1).areas(rect);

    list::draw(state, frame, list);
    preview::draw(state, frame, preview);
}