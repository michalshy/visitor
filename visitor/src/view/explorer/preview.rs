use anyhow::Result;
use crossterm::event::KeyCode;
use libvisitor::VEntry;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style, Styled}, widgets::{Block, Borders, List, ListState, Padding, Paragraph, Widget, canvas::Line}};
use crate::{state::{Mode, PickType, Preview, State}, view::{convert::to_list_item, pallete::is_dimmed}};
use tracing::{info, warn, error, debug};
use crate::view::pallete;

pub fn draw(state: &State, frame: &mut Frame, rect: Rect) {
    let style = Style::default()
        .bg(is_dimmed(pallete::SECONDARY_BG, &state.mode));

    internal_draw(state, frame, rect);

    let block = Block::default().style(style);
    frame.render_widget(block, rect);
}

fn internal_draw(state: &State, frame: &mut Frame, rect: Rect) {
    match &state.preview {
        Preview::Dir(entries) => {
            dir_prev(entries, state.mode.clone(), frame, rect);
        }
        Preview::Text(lines) => {
            text_prev(lines, state.mode.clone(), frame, rect);
        }
        Preview::Empty => { /* nothing */ }
    }
}

fn dir_prev(entries: &Vec<VEntry>, mode: Mode, frame: &mut Frame, rect: Rect) {
    let items = entries
        .iter()
        .enumerate()
        .map(|(_, e)| 
            to_list_item(e, false, mode == Mode::Normal));

    let style = Style::default()
        .bg(is_dimmed(pallete::SECONDARY_BG, &mode));

    let list = List::new(items)
        .style(style);

    frame.render_widget(list, rect);
}

fn text_prev(lines: &String, mode: Mode, frame: &mut Frame, rect: Rect) {

}