use libvisitor::{VEntry, VKind::{Symlink, File, Dir}};
use ratatui::{style::Color, text::{Line, Span}, widgets::ListItem};
use crate::tui::pallete;
use chrono::{DateTime, Local};

pub fn highlighted(idx: usize, selected: Option<usize>) -> bool {
    if selected.is_none() {
        return false
    }
    idx == selected.unwrap()
}

pub fn to_list_item(entry: &VEntry, highlighted: bool) -> ListItem {
    let modified: DateTime<Local> = entry.modified.into();

    let name = Span::default()
        .content(entry.name.clone());

    let path = Span::default().content(entry.path.clone());
    let size = Span::default().content(entry.size.to_string());
    let modified = Span::default().content(modified.format("%d/%m/%Y %T").to_string());
    let permission = Span::default().content(format!("{}", entry.permissions));
    
    let line = Line::default().spans([
        name,
        path,
        size,
        modified,
        permission,
    ]);

    ListItem::new(line)
}

fn span_color(entry: &VEntry) -> Color {
    match &entry.kind {
        File => { 
            match entry.hidden {
                true => { return pallete::DIMMED_TXT }
                _ => { return pallete::PRIMARY_TXT }
            }
        }
        Dir => { return pallete::DIR }
        Symlink { target, broken } => { return pallete::SYMLINK }
    }
}