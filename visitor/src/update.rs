use anyhow::{Result};
use libvisitor::{VKind, act_copy, act_delete, act_move, list_dir};
use tracing::info;

use crate::{action::Action, state::{State, PickType}};

pub fn update(state: &mut State, action: Action) -> Result<()> {
    match action {
        Action::CursorDown => {
            state.list_state.select_next();
        },
        Action::CursorUp => {
            state.list_state.select_previous();
        },
        Action::ExecuteCursor => {
            if let Some(idx) = state.list_state.selected() {
                return act_execute(state, idx)
            }
        },
        Action::GetFileDetails => {
            // tbd
        },
        Action::MoveToParent =>{
            return act_move_parent(state)
        },
        Action::Paste => {
            if let Some(idx) = state.list_state.selected() {
                return act_paste(state)
            }
        },
        Action::Pick { pick_type } => {
            if let Some(idx) = state.list_state.selected() {
                return act_pick(state, idx, pick_type);
            }
        },
        Action::ResizePreview { bigger } => {
            act_resize(state, bigger);
        },
        Action::Delete => {
            if let Some(idx) = state.list_state.selected() {
                act_delete(state.entries[idx].path.clone())?;
            }
            update_entries(state)?
        }
        Action::NewEntry => {
            
        }
    }
    Ok(())
}

fn update_entries(state: &mut State) -> Result<()> {
    let entries = list_dir(&state.current_dir)?;
    state.entries = entries;
    Ok(())
}

fn act_move_parent(state: &mut State) -> Result<()> {
    if let Some(path) = state.current_dir.parent() {
        state.current_dir = path.to_path_buf();
        update_entries(state)?;

        let idx = state.indices.pop_front();
        match idx {
            Some(i) => {
                info!("Popped index from indices stack: {}", i);
                state.list_state.select(idx);
            },
            None => {
                info!("Nothing to pop out of indices stack");
            }
        }
    }
    Ok(())
}

fn act_execute(state: &mut State, idx: usize) -> Result<()> {
    match &state.entries[idx].kind {
        VKind::Dir => {
            state.indices.push_back(idx);
            info!("Pushed index: {}", idx);   

            let new_dir = &state.entries[idx].name;
            let new_path = state.current_dir.join(new_dir);
            state.current_dir = new_path;
            update_entries(state)?;
            if !state.entries.is_empty() {
                state.list_state.select_first();
            }
        }
        VKind::Symlink { target, broken } => {
            // tbd
        }
        VKind::File => {
            // tbd
        }
    }
    Ok(())
}

fn act_pick(state: &mut State, idx: usize, pick_type: PickType) -> Result<()>  {
    let entry = state.entries[idx].clone();
    state.pick(entry, pick_type);
    Ok(())
}

fn act_paste(state: &mut State) -> Result<()> {
    match state.get_picked() {
        Some(p) => {
            let file_name = p.path.file_name()
                .unwrap_or_default().to_string_lossy().into_owned();
            match p.pick_type {
                PickType::Copy => {
                    act_copy(p.path, state.current_dir.clone().join(file_name))?; // unused return
                },
                PickType::Cut => {
                    act_move(p.path, state.current_dir.clone().join(file_name))?;
                }
            }
            update_entries(state)?;
        },
        None => {}
    }
    Ok(())
}

fn act_resize(state: &mut State, bigger: bool) {
    if bigger {
        if state.preview_size < 99 {
            state.preview_size = state.preview_size.saturating_add(1);
        }
    }
    else {
        if state.preview_size > 2 {
            state.preview_size = state.preview_size.saturating_sub(1);
        }
    }
}