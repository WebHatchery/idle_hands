//! Tile palette helpers for the selectable 2048 board themes.

use macroquad::prelude::Color;

pub fn tile_color(v: u16, theme: u8) -> Color {
    match theme % 3 {
        1 => match v {
            0 => crate::theme::MOSS_DARK,
            2 => crate::theme::MOSS_DARK,
            4 => crate::theme::MOSS,
            8 => crate::theme::WALNUT,
            16 => crate::theme::BRASS,
            32 => crate::theme::LEATHER,
            64 => crate::theme::PARCHMENT_BROWN,
            128 => crate::theme::MOSS,
            256 => crate::theme::SLATE_BRONZE,
            512 => crate::theme::WALNUT,
            1024 => crate::theme::MOSS_DARK,
            _ => crate::theme::BRASS,
        },
        2 => match v {
            0 => crate::theme::BACKGROUND_DEEP,
            2 => crate::theme::SURFACE_DARK,
            4 => crate::theme::PARCHMENT_BROWN,
            8 => crate::theme::WALNUT,
            16 => crate::theme::BRASS,
            32 => crate::theme::LEATHER,
            64 => crate::theme::PARCHMENT_BROWN,
            128 => crate::theme::WALNUT,
            256 => crate::theme::SURFACE,
            512 => crate::theme::SLATE_BRONZE,
            1024 => crate::theme::MOSS_DARK,
            _ => crate::theme::BRASS,
        },
        _ => match v {
            0 => crate::theme::BACKGROUND_DEEP,
            2 => crate::theme::SURFACE_DARK,
            4 => crate::theme::PARCHMENT_BROWN,
            8 => crate::theme::WALNUT,
            16 => crate::theme::BRASS,
            32 => crate::theme::LEATHER,
            64 => crate::theme::LEATHER,
            128 => crate::theme::MOSS_DARK,
            256 => crate::theme::SLATE_BRONZE,
            512 => crate::theme::WALNUT,
            1024 => crate::theme::MOSS,
            _ => crate::theme::BRASS,
        },
    }
}
