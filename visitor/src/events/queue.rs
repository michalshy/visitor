use crate::events::callback::Callback;
use crate::events::command::Command::{self};
use crate::app::state::State;
use std::collections::VecDeque;
use std::sync::mpsc::Sender;
use libvisitor::{VKind, list_dir};
use anyhow::{Error, Ok, Result};

#[derive(Default)]
pub struct Queue {
    q: VecDeque<Command>
}

impl Queue {
    pub fn push(&mut self, command: Command) {
        self.q.push_back(command);
    }

    fn is_empty(&self) -> bool {
        self.q.is_empty()
    }

    fn pop(&mut self) -> Option<Command> {
        self.q.pop_front()
    }
}

pub fn process(
    queue: &mut Queue, 
    state: &mut State, 
    tx: Sender<Callback>) 
-> Result<()> {
    if queue.is_empty() {
        return Ok(());
    }

    let command = queue.pop();
    if command.is_some() {
        let result = dispatch(command.unwrap(), state, queue)?;
        if let Some(callback) = result {
            tx.send(callback)?;
        }
    }
    Ok(())
}

fn event_list_dir(state: &mut State) -> Result<()> {
    let entries = list_dir(&state.current_dir)?;
    state.entries = entries;
    Ok(())
}

fn event_move_parent(state: &mut State) -> Result<Option<Callback>> {
    if let Some(path) = state.current_dir.parent() {
        state.current_dir = path.to_path_buf();
        event_list_dir(state)?;
        return Ok(Some(Callback::MoveToParent))
    }
    Ok(None)
}

fn event_execute(state: &mut State, idx: usize) -> Result<Option<Callback>> {
    match &state.entries[idx].kind {
        VKind::Dir => {
            let new_dir = &state.entries[idx].name;
            let new_path = state.current_dir.join(new_dir);
            state.current_dir = new_path;
            event_list_dir(state)?;
            return Ok(Some(Callback::MoveToChild))
        }
        VKind::Symlink { target, broken } => {

        }
        VKind::File => {

        }
    }
    Ok(None)
}

fn dispatch(command: Command, state: &mut State, queue: &mut Queue) -> Result<Option<Callback>> {
    match command {
        Command::ListDir => {
            event_list_dir(state)?;
        },
        Command::MoveToParent => {
            return Ok(event_move_parent(state)?);
        },
        Command::Execute { idx } => {
            return Ok(event_execute(state, idx)?);
        },
        Command::GetFileDetails { idx: _ } => {
            // tbd
        }
    }
    Ok(None)
}