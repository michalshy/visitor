use libvisitor::{VEntry, VKind::{Symlink, File, Dir}};
use ratatui::{style::{Color, Style}, text::{Line, Span}, widgets::ListItem};
use crate::{state::Mode, view::pallete::{self, dim}};
use chrono::{DateTime, Local};

pub fn highlighted(idx: usize, selected: Option<usize>) -> bool {
    if selected.is_none() {
        return false
    }
    idx == selected.unwrap()
}

pub fn to_list_item(entry: &VEntry, highlighted: bool, should_dim: bool) -> ListItem<'_> {
    let modified: DateTime<Local> = entry.modified.into();

    let bg_color = if highlighted {
        if should_dim {pallete::SELECTED} else {dim(pallete::SELECTED)}
    } else {
        if should_dim {pallete::SECONDARY_BG} else {dim(pallete::SECONDARY_BG)}
    };

    let fg_color = if highlighted {
        if should_dim {pallete::SELECTED_TXT} else {dim(pallete::SELECTED_TXT)}
    } else {
        line_color(entry, should_dim)
    };

    let name = Span::raw(format!("{:<30}", entry.name.clone()));
    let size = Span::raw(format!("{:<15}", entry.size.to_string()));
    let modified = Span::default().content(modified.format("%d/%m/%Y %T     ").to_string());
    let permission = Span::default().content(format!("{:<10}", entry.permissions));

    let style = Style::default().bg(bg_color).fg(fg_color);
    
    let line = Line::default().spans([
        name,
        size,
        modified,
        permission,
    ]).style(style);

    ListItem::new(line)
}

fn line_color(entry: &VEntry, should_dim: bool) -> Color {
    match &entry.kind {
        File => { 
            match entry.hidden {
                true => { 
                    if should_dim {pallete::DIMMED_TXT} else {dim(pallete::DIMMED_TXT)} 
                }
                _ => { 
                    if should_dim {pallete::PRIMARY_TXT} else {dim(pallete::PRIMARY_TXT)} 
                }
            }
        }
        Dir => { 
            if should_dim {pallete::DIR} else {dim(pallete::DIR)} 
        }
        Symlink { target: _, broken: _ } => { 
            if should_dim {pallete::SYMLINK} else {dim(pallete::SYMLINK)} 
        }
    }
}