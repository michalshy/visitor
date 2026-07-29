use std::collections::VecDeque;
use std::path::{PathBuf};
use std::env::current_dir;
use anyhow::Result;

use libvisitor::{VEntry, list_dir};
use ratatui::widgets::ListState;

use crate::state::Mode::Normal;

const DEFAULT_PREVIEW: u8 = 40;

pub struct State {
    // app
    pub exit: bool,

    // fs
    pub current_dir: PathBuf,
    pub entries: Vec<VEntry>,
    picked: Option<Picked>,

    //ui
    pub indices: VecDeque<usize>, // saved indices
    pub list_state: ListState, // list of current entries
    pub preview_size: u8, // in percentage
    pub mode: Mode, // for displaying popups
    pub preview: Preview, // for previews
}

impl State {
    pub fn init() -> Result<State> {
        let current_dir = current_dir()?;
        let entries = list_dir(&current_dir.clone())?;
        let mut list_state = ListState::default();
        if !entries.is_empty() {
            list_state.select_first();
        }

        Ok(State { 
            exit: false,
            current_dir, 
            entries, 
            picked: None, 
            indices: VecDeque::new(), 
            list_state,
            preview_size: DEFAULT_PREVIEW,
            mode: Normal,
            preview: Preview::Empty,
        })
    }

    pub fn pick(&mut self, entry: VEntry, pick_type: PickType) {
        let picked = Picked { path: entry.path.clone(), pick_type };
        self.picked = Some(picked);
    }

    pub fn unpick(&mut self) {
        self.picked = None;
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum NewEntryKind {
    Dir,
    Symlink,
    File
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum PopUpState {
    NewEntry { kind: NewEntryKind, buffer: String }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Mode
{
    Normal,
    PopUp { state: PopUpState }
}

pub enum Preview
{
    Dir(Vec<VEntry>),
    Text(String),
    Empty
}