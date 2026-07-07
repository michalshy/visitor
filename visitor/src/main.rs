mod app;
mod tui;
use crate::app::App;
use anyhow::Result;

fn main() -> Result<()> {
    ratatui::run(|terminal| 
        App::init().unwrap().run(terminal))
}
