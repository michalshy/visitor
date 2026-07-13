use libvisitor::{VEntry, VKind::{Symlink, File, Dir}};
use ratatui::{style::{Color, Style}, text::{Line, Span}, widgets::ListItem};
use crate::tui::pallete;
use chrono::{DateTime, Local};

pub fn highlighted(idx: usize, selected: Option<usize>) -> bool {
    if selected.is_none() {
        return false
    }
    idx == selected.unwrap()
}

pub fn to_list_item(entry: &VEntry, highlighted: bool) -> ListItem<'_> {
    let modified: DateTime<Local> = entry.modified.into();

    let bg_color = if highlighted {
        pallete::SELECTED
    } else {
        pallete::SECONDARY_BG
    };

    let fg_color = if highlighted {
        pallete::SELECTED_TXT
    } else {
        line_color(entry)
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

fn line_color(entry: &VEntry) -> Color {
    match &entry.kind {
        File => { 
            match entry.hidden {
                true => { return pallete::DIMMED_TXT }
                _ => { return pallete::PRIMARY_TXT }
            }
        }
        Dir => { return pallete::DIR }
        Symlink { target: _, broken: _ } => { return pallete::SYMLINK }
    }
}