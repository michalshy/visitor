use std::path::{PathBuf};

use crate::app::state::PickType;

pub enum Command {
    ListDir,
    MoveToParent,
    GetFileDetails { idx: usize },
    Execute { idx: usize },
    Pick { idx: usize, pick_type: PickType },
    ActPicked,
}