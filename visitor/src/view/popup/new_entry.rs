use ratatui::{Frame, layout::{Alignment, Constraint, Rect}, style::{Modifier, Style}, text::{Line, Span}, widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap}};

use crate::view::{pallete::{BORDER, MUTED_TXT, PRIMARY_BG, PRIMARY_TXT, SELECTED_TXT}};

pub fn draw(buffer: &str, frame: &mut Frame, area: Rect) {
    let hint = "[Enter]-confirm [Esc]-close";
    let title = " New Entry ";

    // widest line drives the box width; 2 for borders, 2 for a little breathing room
    let content_width = buffer.chars().count()
        .max(hint.chars().count())
        .max(title.chars().count()) as u16;
    let width = (content_width + 4).min(area.width);

    let area = area.centered(Constraint::Length(width), Constraint::Length(4));
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(Span::styled(
            format!(" New Entry "),
            Style::default()
                .fg(SELECTED_TXT)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER))
        .style(Style::default().bg(PRIMARY_BG));

    let text = Paragraph::new(vec![
            Line::from(Span::styled(buffer, Style::default().fg(PRIMARY_TXT))),
            Line::from(Span::styled(
                "[Enter]-confirm [Esc]-close",
                Style::default().fg(MUTED_TXT),
            )),
        ])
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(text, area);
}
