use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::state::PopUpState;

pub fn draw(buffer: &str, frame: &mut Frame, area: Rect) {
    let paragraph = Paragraph::new(buffer);
    frame.render_widget(paragraph, area);
}
