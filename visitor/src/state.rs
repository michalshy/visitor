use std::collections::VecDeque;
use std::path::{PathBuf};
use std::env::current_dir;
use anyhow::Result;

use libvisitor::VEntry;
use ratatui::widgets::ListState;

const DEFAULT_PREVIEW: u8 = 40;

pub struct State {
    pub current_dir: PathBuf,
    pub entries: Vec<VEntry>,
    picked: Option<Picked>,

    //ui
    pub indices: VecDeque<usize>, // saved indices
    pub list_state: ListState, // list of current entries
    pub preview_size: u8, // in percentage
}

impl State {
    pub fn init() -> Result<State> {
        let path = current_dir()?;
        let entries = Vec::new();
        Ok(State { 
            current_dir: path, 
            entries, 
            picked: None, 
            indices: VecDeque::new(), 
            list_state: ListState::default(),
            preview_size: DEFAULT_PREVIEW
        })
    }

    pub fn pick(&mut self, entry: VEntry, pick_type: PickType) {
        let picked = Picked { path: entry.path.clone(), pick_type };
        self.picked = Some(picked);
    }

    pub fn unpick(&mut self) {
        self.picked = None;
    }

    pub fn picked(self) -> Option<Picked> {
        self.picked
    }

    pub fn get_picked(&self) -> Option<Picked> {
        self.picked.clone()
    }
}

#[derive(Clone, Copy)]
pub enum PickType
{
    Copy,
    Cut
}

#[derive(Clone)]
pub struct Picked {
    pub path: PathBuf,
    pub pick_type: PickType,
}