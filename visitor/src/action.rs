use crate::state::{PickType};

pub enum Action {
    CursorUp,
    CursorDown,
    ResizePreview { bigger: bool },
    ExecuteCursor,
    MoveToParent,
    GetFileDetails,
    Pick { pick_type: PickType },
    Paste,
    Delete,
    NewEntry,

    PopupType{ c: char },
    PopupRevert,
    PopupConfirm,
    PopupCancel
}