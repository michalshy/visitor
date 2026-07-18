use crossterm::event::KeyCode;

use crate::state::PickType;

pub enum Action {
    CursorUp,
    CursorDown,
    ExecuteCursor { idx: usize },
    ListDir,
    MoveToParent,
    GetFileDetails { idx: usize },
    Pick { idx: usize, pick_type: PickType },
    ActPicked,
}