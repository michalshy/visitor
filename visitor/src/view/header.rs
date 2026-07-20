use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style, Styled}, widgets::{Block, Borders, List, ListState, Padding, Paragraph, Widget, canvas::Line}};
use crate::{state::State, view::pallete::is_dimmed};
use crate::view::pallete;

pub fn draw(state: &State, frame: &mut Frame, rect: Rect) {
    let style = Style::default()
        .bg(is_dimmed(pallete::PRIMARY_BG, &state.mode))
        .fg(is_dimmed(pallete::PRIMARY_TXT, &state.mode));
    
    let title =  state.current_dir.to_string_lossy();
    let paragraph = Paragraph::new(title)
        .style(style);

    frame.render_widget(paragraph, rect);
}