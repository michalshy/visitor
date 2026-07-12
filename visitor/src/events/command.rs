use std::path::{PathBuf};

pub enum Command {
    ListDir,
    GetFileDetails { idx: usize }
}