use anyhow::Result;
use crossterm::event::KeyCode;
use crate::state::PickType;

use crate::{action::Action};

pub fn map_key(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Up => Some(Action::CursorUp),
        KeyCode::Down => Some(Action::CursorDown),
        KeyCode::Char('[') => Some(Action::ResizePreview { bigger: true }),
        KeyCode::Char(']') => Some(Action::ResizePreview { bigger: false }),
        KeyCode::Char('c') => Some(Action::Pick { pick_type: PickType::Copy }),
        KeyCode::Char('x') => Some(Action::Pick { pick_type: PickType::Cut }),
        KeyCode::Char('v') => Some(Action::Paste),
        KeyCode::Char('q') => Some(Action::Delete),
        KeyCode::Char('s') | KeyCode::Char('d') | KeyCode::Char('f') => Some(Action::NewEntry),
        KeyCode::Backspace => Some(Action::MoveToParent),
        KeyCode::Enter => Some(Action::ExecuteCursor),
        _ => None
    }   
}