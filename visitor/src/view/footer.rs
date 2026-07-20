use ratatui::{Frame, layout::Rect, style::Style, widgets::Paragraph};
use crate::{state::State, view::pallete::is_dimmed};
use crate::view::pallete;

pub fn draw(state: &State, frame: &mut Frame, rect: Rect) {
    let style: Style = Style::default()
        .bg(is_dimmed(pallete::PRIMARY_BG, &state.mode))
        .fg(is_dimmed(pallete::MUTED_TXT, &state.mode));

    let paragraph = Paragraph::new(
        "[new (dir d) (file f) (symlink s)] [delete q] [rename r] [copy c] [cut x] [paste v] [enter ↵] [move up ⏎] [exit ESC]")
        .style(style).alignment(ratatui::layout::HorizontalAlignment::Left);

    frame.render_widget(paragraph, rect);
}