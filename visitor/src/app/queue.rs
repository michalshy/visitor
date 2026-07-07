use crate::app::{command::Command::{self, LIST_DIR}, state::State};
use std::collections::VecDeque;
use libvisitor::list_dir;
use anyhow::{Ok, Result};

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
        dispatch(command.unwrap(), state)?;
    }
    Ok(())
}

fn dispatch(command: Command, state: &mut State) -> Result<()> {
    match command {
        LIST_DIR => {
            let entries = list_dir(&state.current_dir)?;
            state.entries = entries;
        }
    }
    Ok(())
}