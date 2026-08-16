//! Oak, moss, and cream presentation shared by the collection UI.

use crate::state::GameId;
use macroquad::prelude::Color;

pub const BACKGROUND: Color = Color::new(0.184, 0.165, 0.141, 1.0);
pub const BACKGROUND_DEEP: Color = Color::new(0.105, 0.086, 0.067, 1.0);
pub const SURFACE: Color = Color::new(0.420, 0.322, 0.239, 1.0);
pub const SURFACE_DARK: Color = Color::new(0.250, 0.185, 0.132, 1.0);
pub const MOSS: Color = Color::new(0.478, 0.545, 0.353, 1.0);
pub const MOSS_DARK: Color = Color::new(0.255, 0.310, 0.188, 1.0);
pub const BRASS: Color = Color::new(0.784, 0.663, 0.420, 1.0);
pub const CREAM: Color = Color::new(0.957, 0.922, 0.847, 1.0);
pub const SECONDARY: Color = Color::new(0.659, 0.604, 0.518, 1.0);
pub const INK: Color = Color::new(0.125, 0.102, 0.078, 1.0);
pub const BORDER: Color = Color::new(0.600, 0.480, 0.310, 0.78);
pub const LEATHER: Color = Color::new(0.355, 0.157, 0.110, 1.0);
pub const WALNUT: Color = Color::new(0.390, 0.235, 0.105, 1.0);
pub const PARCHMENT_BROWN: Color = Color::new(0.520, 0.390, 0.230, 1.0);
pub const SLATE_BRONZE: Color = Color::new(0.220, 0.285, 0.260, 1.0);
pub const PAPER: Color = Color::new(0.835, 0.755, 0.620, 1.0);
pub const PAPER_LIGHT: Color = Color::new(0.900, 0.835, 0.720, 1.0);

// Shared drawer chrome keeps every game in the cabinet's oak, moss, brass,
// and cream family while leaving room for restrained game-piece colors.
pub const GAME_PANEL: Color = SURFACE_DARK;

pub fn category_surface(game: GameId, active: bool) -> Color {
    let base = match crate::cabinet_status::category_filter(game) {
        3 => LEATHER,
        4 => MOSS_DARK,
        5 => WALNUT,
        6 => PARCHMENT_BROWN,
        7 => SLATE_BRONZE,
        _ => SURFACE_DARK,
    };
    if active {
        lighten(base, 0.07)
    } else {
        base
    }
}

pub fn lighten(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r + amount).min(1.0),
        (color.g + amount).min(1.0),
        (color.b + amount).min(1.0),
        color.a,
    )
}

pub fn text_color(color: Color) -> Color {
    if color.r > 0.95 && color.g > 0.95 && color.b > 0.95 {
        CREAM
    } else if color.r > color.b * 0.9 && color.b > color.g * 1.15 && color.b > 0.5 {
        SECONDARY
    } else {
        color
    }
}

/// Translates legacy drawer chrome into the cabinet palette. Gameplay pieces
/// keep their own signal colors; this is intended for neutral panels only.
pub fn drawer_surface(color: Color) -> Color {
    if color.r < 0.19 && color.g < 0.14 && color.b > 0.18 {
        GAME_PANEL
    } else if color.r < 0.28 && color.g < 0.20 && color.b > 0.25 {
        SURFACE_DARK
    } else if color.r > 0.40 && color.r < 0.55 && color.g < 0.32 && color.b > 0.35 {
        LEATHER
    } else {
        color
    }
}
