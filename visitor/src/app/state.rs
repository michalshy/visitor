use std::path::{PathBuf};
use std::env::current_dir;
use anyhow::Result;

use libvisitor::VEntry;

pub struct State {
    pub current_dir: PathBuf,
    pub entries: Vec<VEntry>
}

impl State {
    pub fn init() -> Result<State> {
        let path = current_dir()?;
        let entries = Vec::new();
        Ok(State { current_dir: path, entries })
    }
}