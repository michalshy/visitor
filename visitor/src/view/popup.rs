mod new_entry;

use ratatui::{Frame, layout::{Constraint, Rect}, widgets::{Block, Clear, Paragraph}};
use crate::state::{PopUpState};

pub fn draw(state: &PopUpState, frame: &mut Frame, rect: Rect) {
    let popup_block = Block::bordered().title("New");
    let centered_area = rect.centered(Constraint::Percentage(60), Constraint::Percentage(20));
    // clears out any background in the area before rendering the popup
    frame.render_widget(Clear, centered_area);
    let paragraph = Paragraph::new("Lorem ipsum").block(popup_block);
    frame.render_widget(paragraph, centered_area);
}