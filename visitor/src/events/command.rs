use std::path::{PathBuf};

pub enum Command {
    ListDir,
    MoveUp,
    GetFileDetails { idx: usize }
}