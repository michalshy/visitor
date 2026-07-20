use ratatui::style::Color;

use crate::state::Mode;

// multipliers
const DIMMED_MULT: f32 = 0.5; // 1.0 = no change, 0.0 = black

pub fn is_dimmed(color: Color, mode: &Mode) -> Color {
    if mode == &Mode::Normal {
        return color
    }

    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            (r as f32 * DIMMED_MULT) as u8,
            (g as f32 * DIMMED_MULT) as u8,
            (b as f32 * DIMMED_MULT) as u8,
        ),
        other => other,
    }
}

pub fn dim(color: Color) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            (r as f32 * DIMMED_MULT) as u8,
            (g as f32 * DIMMED_MULT) as u8,
            (b as f32 * DIMMED_MULT) as u8,
        ),
        other => other,
    }
}

// colors
pub const PRIMARY_BG: Color = Color::Rgb(35, 36, 48);
pub const SECONDARY_BG: Color = Color::Rgb(26, 27, 35);
pub const BORDER: Color = Color::Rgb(51, 52, 74);
pub const SELECTED: Color = Color::Rgb(60, 63, 88);

// texts
pub const SELECTED_TXT: Color = Color::Rgb(255, 255, 255);
pub const PRIMARY_TXT: Color = Color::Rgb(216, 218, 229);
pub const SECONDARY_TXT: Color = Color::Rgb(168, 176, 200);
pub const MUTED_TXT: Color = Color::Rgb(107, 110, 133);
pub const DIMMED_TXT: Color = Color::Rgb(102, 104, 121);

// elements
pub const DIR: Color = Color::Rgb(127, 180, 232);
pub const SYMLINK: Color = Color::Rgb(232, 162, 107);
