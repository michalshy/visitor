use crate::events::command::Command::{self};
use crate::app::state::State;
use std::collections::VecDeque;
use libvisitor::list_dir;
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

pub fn process(queue: &mut Queue, state: &mut State) -> Result<()> {
    if queue.is_empty() {
        return Ok(());
    }

    let command = queue.pop();
    if command.is_some() {
        dispatch(command.unwrap(), state, queue)?;
    }
    Ok(())
}

fn dispatch(command: Command, state: &mut State, queue: &mut Queue) -> Result<()> {
    match command {
        Command::ListDir => {
            let entries = list_dir(&state.current_dir)?;
            state.entries = entries;
        },
        Command::MoveUp => {
            if let Some(path) = state.current_dir.parent() {
                state.current_dir = path.to_path_buf();
                queue.push(Command::ListDir);
            }
        },
        Command::GetFileDetails { idx: _ } => {
            // tbd
        }
    }
    Ok(())
}