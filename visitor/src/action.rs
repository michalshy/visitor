use crate::state::{NewEntryKind, PickType};

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
    StartNewEntry { kind: NewEntryKind },
    Exit,

    PopupType{ c: char },
    PopupRevert,
    PopupConfirm,
    PopupCancel,
}