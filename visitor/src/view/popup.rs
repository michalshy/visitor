mod new_entry;

use ratatui::{Frame, layout::{Constraint, Rect}, widgets::{Block, Clear, Paragraph}};
use crate::state::PopUpState::{self, NewEntry};

pub fn draw(state: &PopUpState, frame: &mut Frame, rect: Rect) {
    match state {
        NewEntry { kind, buffer } => {
            new_entry::draw(buffer, frame, rect);
        }
    }
}