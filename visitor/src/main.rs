mod app;
mod tui;
mod events;

use crate::app::App;
use anyhow::Result;

fn main() -> Result<()> {
    ratatui::run(|terminal| 
        App::init().unwrap().run(terminal))
}
