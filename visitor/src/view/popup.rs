mod new_entry;

use ratatui::{Frame, layout::{Constraint, Rect}, widgets::{Block, Clear, Paragraph}};
use crate::state::PopUpState::{self, NewEntry};

pub fn draw(state: &PopUpState, frame: &mut Frame, rect: Rect) {
    let centered_area = rect.centered(Constraint::Percentage(60), Constraint::Percentage(20));
    frame.render_widget(Clear, centered_area);
    
    match state {
        NewEntry { kind, buffer } => {
            new_entry::draw(buffer, frame, centered_area);
        }
    }
}