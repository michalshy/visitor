use std::{fs, io::Read};
use anyhow::{Result};
use libvisitor::{VKind, act_copy, act_create_dir, act_create_file, act_create_symlink, act_delete_file, act_move, list_dir};
use tracing::{info, warn};
use crate::{action::Action, state::{Mode, NewEntryKind::{Dir, File, Symlink}, PickType, PopUpState, Preview, State}};

const MAX_PREVIEW_BYTES: usize = 64 * 1024;

/**
We determine which type of preview we want to generate and populate according
state field, as we need to carry some information for the view

For dir type of preview, it will be only info about type, since we rebuild list
each iteration,
for text we will carry lines to render but synchronously,
for other, more sophisticated types we will process asynchronously and display
spinner if info is not yet ready
*/
pub fn update_preview(state: &mut State) -> Result<()> {
    if let Some(idx) = state.list_state.selected() {
        state.preview = Preview::Empty;
        let path = &state.entries[idx].path;
        if path.is_dir() {
            state.preview = Preview::Dir(list_dir(path)?);
        } else {
            let file = fs::File::open(path)?;
            let mut buf = Vec::new();
            file.take(MAX_PREVIEW_BYTES as u64).read_to_end(&mut buf)?;
            state.preview = classify(&buf);
        }
    }
    Ok(())
}

fn classify(buf: &[u8]) -> Preview {
    if buf.contains(&0) {
        Preview::Empty // to be defined
    } else {
        Preview::Text(String::from_utf8_lossy(buf).into_owned())
    }
}

pub fn update(state: &mut State, action: Action) -> Result<()> {
    let dir = state.current_dir.clone();
    let idx = state.list_state.selected();

    match action {
        Action::CursorDown => {
            state.list_state.select_next();
        }
        Action::CursorUp => {
            state.list_state.select_previous();
        }
        Action::ExecuteCursor => {
            if let Some(idx) = state.list_state.selected() {
                return act_execute(state, idx)
            }
        }
        Action::GetFileDetails => {
            // tbd
        }
        Action::MoveToParent =>{
            return act_move_parent(state)
        }
        Action::Paste => {
            if let Some(idx) = state.list_state.selected() {
                return act_paste(state)
            }
        }
        Action::Pick { pick_type } => {
            if let Some(idx) = state.list_state.selected() {
                return act_pick(state, idx, pick_type);
            }
        }
        Action::ResizePreview { bigger } => {
            act_resize(state, bigger);
        }
        Action::Delete => {
            if let Some(idx) = state.list_state.selected() {
                act_delete_file(state.entries[idx].path.clone())?;
            }
            update_entries(state)?
        }
        Action::StartNewEntry { kind } => {
            let new_state = PopUpState::NewEntry { kind, buffer: String::new() };
            state.mode = Mode::PopUp { state: new_state };
        }
        Action::Exit => {
            state.exit = true
        }
        Action::PopupCancel => {
            state.mode = Mode::Normal;
        }
        Action::PopupConfirm => {
            if let Mode::PopUp { state: PopUpState::NewEntry { kind, buffer } } = &mut state.mode {
                let new = state.current_dir.join(buffer);
                match kind {
                    Dir => {
                        act_create_dir(new)?;
                    }
                    File => {
                        act_create_file(new)?;
                    }   
                    Symlink => {
                        act_create_symlink();
                    }
                }
                update_entries(state)?;
                state.mode = Mode::Normal;
            }
        }
        Action::PopupRevert => {
            if let Mode::PopUp { state: PopUpState::NewEntry { kind: _, buffer } } = &mut state.mode {
                buffer.pop();
            }
        }
        Action::PopupType { c } => {
            if let Mode::PopUp { state: PopUpState::NewEntry { kind: _, buffer } } = &mut state.mode {
                buffer.push(c);
            }
        }
    }

    if idx != state.list_state.selected() || dir != state.current_dir {
        update_preview(state);
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

        let idx = state.indices.pop_back();
        match idx {
            Some(_) => {
                state.list_state.select(idx);
            }
            _ => ()
        }
    }
    Ok(())
}

fn act_execute(state: &mut State, idx: usize) -> Result<()> {
    match &state.entries[idx].kind {
        VKind::Dir => {
            state.indices.push_back(idx);

            let new_dir = &state.entries[idx].name;
            let old = state.current_dir.clone();
            let new = state.current_dir.join(new_dir);
            state.current_dir = new;
            if let Err(err) = update_entries(state) {
                warn!(?err, "Could not move to directory");
                state.current_dir = old;
                state.indices.pop_back();
            } else {
                if !state.entries.is_empty() {
                    state.list_state.select_first();
                }
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