use anyhow::Result;
use crossterm::event::KeyCode;

use crate::{action::Action, state::State};

pub fn map_key(key: KeyCode) -> Option<Action> {
    match key {
        
    }

    None
}

pub fn handle_events(state: &mut State) -> Result<()> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            return handle_key(key_event);
        }
        _ => { }
    };
    Ok(())
}

fn handle_key(key_event: KeyEvent, state: &mut State) -> Result<()> {
    match key_event.code {
        KeyCode::Esc => self.exit = true,
        code => { 
            if let Some(cmd) = handle_input(code)? {
                self.queue.push(cmd);
            }
        }
    }
    Ok(())
}

pub fn handle_input(key: KeyCode) -> Result<Option<Action>> {
    let mut stale_file = false;
    match key {
        KeyCode::Up => {
            self.list_state.select_previous();
            stale_file = true;
        },
        KeyCode::Down => {
            self.list_state.select_next();
            stale_file = true;
        },
        KeyCode::Char(']') => {
            if self.preview_size > 2 {
                self.preview_size = self.preview_size.saturating_sub(1);
            }
        },
        KeyCode::Char('[') => {
            if self.preview_size < 99 {
                self.preview_size = self.preview_size.saturating_add(1);
            }
        },
        KeyCode::Char('c') => {
            let idx = self.list_state.selected().unwrap_or(0);
            return Ok(Some(Command::Pick{ idx, pick_type: PickType::Copy }))  
        },
        KeyCode::Char('x') => {
            let idx = self.list_state.selected().unwrap_or(0);
            return Ok(Some(Command::Pick{ idx, pick_type: PickType::Cut }))  
        },
        KeyCode::Char('v') => {
            return Ok(Some(Command::ActPicked))
        }
        KeyCode::Backspace => {
            return Ok(Some(Command::MoveToParent));
        },
        KeyCode::Enter => {
            if let Some(idx) = self.list_state.selected() {
                return Ok(Some(Command::Execute { idx }));
            } else {
                return Ok(None)
            }
        }
        _ => {}
    }
    if stale_file {
        if let Some(idx) = self.list_state.selected() {
            return Ok(Some(Command::GetFileDetails { idx }));
        } else {
            return Ok(None)
        }
    } 
    return Ok(None)
}