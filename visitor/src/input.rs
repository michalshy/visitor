use anyhow::Result;
use crossterm::event::KeyCode;
use crate::state::Mode::{Normal, PopUp};
use crate::state::NewEntryKind::{Dir, File, Symlink};
use crate::state::{Mode, PickType};

use crate::{action::Action};

pub fn map_key(key: KeyCode, mode: &Mode) -> Option<Action> {
    match mode {
        Normal => map_normal(key),
        PopUp { .. } => map_popup(key)
    } 
}

pub fn map_normal(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Esc => Some(Action::Exit),
        KeyCode::Up => Some(Action::CursorUp),
        KeyCode::Down => Some(Action::CursorDown),
        KeyCode::Char('[') => Some(Action::ResizePreview { bigger: true }),
        KeyCode::Char(']') => Some(Action::ResizePreview { bigger: false }),
        KeyCode::Char('c') => Some(Action::Pick { pick_type: PickType::Copy }),
        KeyCode::Char('x') => Some(Action::Pick { pick_type: PickType::Cut }),
        KeyCode::Char('v') => Some(Action::Paste),
        KeyCode::Char('q') => Some(Action::Delete),
        KeyCode::Char('s') => Some(Action::StartNewEntry { kind: Symlink }),
        KeyCode::Char('d') => Some(Action::StartNewEntry { kind: Dir }),
        KeyCode::Char('f') => Some(Action::StartNewEntry { kind: File }),
        KeyCode::Backspace => Some(Action::MoveToParent),
        KeyCode::Enter => Some(Action::ExecuteCursor),
        _ => None
    }   
}

pub fn map_popup(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Char(c) => Some(Action::PopupType{c}),
        KeyCode::Backspace => Some(Action::PopupRevert),
        KeyCode::Esc => Some(Action::PopupCancel),
        KeyCode::Enter => Some(Action::PopupConfirm),
        _ => None
    }
}