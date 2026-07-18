mod app;
mod state;
mod view;
mod action;
mod update;
mod input;
mod events;

use crate::app::App;
use anyhow::Result;

fn main() -> Result<()> {
    ratatui::run(|terminal| 
        App::init().unwrap().run(terminal))
}
