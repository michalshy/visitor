mod convert;
mod explorer;
mod footer;
mod header;
mod pallete;
mod popup;

use crate::{state::{Mode::{Normal, PopUp}, State}, view::pallete::is_dimmed};
use ratatui::{Frame, layout::{Constraint, Layout}, style::Style, widgets::{Block, Borders, Padding}};


pub fn draw(state: &mut State, frame: &mut Frame) {
    let outer = Block::default()
        .borders(Borders::ALL)
        .padding(Padding { left: 1, right: 1, top: 0, bottom: 0 })
        .style(Style::default()
            .fg(is_dimmed(pallete::BORDER, &state.mode))
            .bg(is_dimmed(pallete::PRIMARY_BG, &state.mode))
        );

    let inner = outer.inner(frame.area());
    frame.render_widget(outer, frame.area());

    //main section drawing, always visible
    let [header, explorer, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ]).areas(inner); 

    header::draw(state, frame, header);
    explorer::draw(state, frame, explorer);
    footer::draw(state, frame, footer);

    match &state.mode {
        Normal => {
            // nothing more to draw
        }
        PopUp { state } => {
            popup::draw(state, frame, inner);
        }
    }
}