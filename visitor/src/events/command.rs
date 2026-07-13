use std::path::{PathBuf};

pub enum Command {
    ListDir,
    MoveToParent,
    GetFileDetails { idx: usize },
    Execute { idx: usize }
}