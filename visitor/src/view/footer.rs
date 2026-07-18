use ratatui::{Frame, layout::Rect, style::Style, widgets::Paragraph};
use crate::{state::{State}};
use crate::view::pallete;

pub fn draw(_: &State, frame: &mut Frame, rect: Rect) {
    let style: Style = Style::default()
        .bg(pallete::PRIMARY_BG)
        .fg(pallete::MUTED_TXT);

    let paragraph = Paragraph::new(
        "[new (dir d) (file f) (symlink s)] [delete q] [rename r] [copy c] [cut x] [paste v] [enter ↵] [move up ⏎] [exit ESC]")
        .style(style).alignment(ratatui::layout::HorizontalAlignment::Left);

    frame.render_widget(paragraph, rect);
}