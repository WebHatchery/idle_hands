//! Responsive presentation and touch routing for Mahjong Solitaire.

use crate::{
    accessibility,
    mahjong_solitaire::{MahjongStatus, Tile},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    tile_w: f32,
    tile_h: f32,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(20., 48., 280., 240.),
            tile_w: 32.,
            tile_h: 42.,
            hint: Rect::new(350., 275., 290., 42.),
            undo: Rect::new(350., 220., 120., 42.),
            new_game: Rect::new(490., 220., 150., 42.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 115., 340., 240.),
            tile_w: 38.,
            tile_h: 42.,
            hint: Rect::new(20., 530., 330., 42.),
            undo: Rect::new(20., 475., 145., 42.),
            new_game: Rect::new(185., 475., 165., 42.),
        }
    } else {
        Layout {
            board: Rect::new(360., 82., 560., 480.),
            tile_w: 58.,
            tile_h: 62.,
            hint: Rect::new(950., 495., 290., 44.),
            undo: Rect::new(950., 555., 120., 44.),
            new_game: Rect::new(1090., 555., 150., 44.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if back_rect().contains(point) {
        return vec![UiAction::Cabinet];
    }
    if layout.undo.contains(point) {
        return vec![UiAction::MahjongSolitaireUndo];
    }
    if layout.new_game.contains(point) {
        return vec![UiAction::MahjongSolitaireNew];
    }
    if layout.hint.contains(point) {
        return vec![UiAction::MahjongSolitaireHint];
    }
    for (index, tile) in state.mahjong_solitaire.tiles.iter().enumerate().rev() {
        if tile.removed {
            continue;
        }
        let rect = tile_rect(*tile, layout);
        if rect.contains(point) {
            return vec![UiAction::MahjongSolitaireTap(index)];
        }
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.mahjong_solitaire;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let header_x = if compact {
        120.
    } else if portrait {
        10.
    } else {
        360.
    };
    let header_y = if compact {
        30.
    } else if portrait {
        68.
    } else {
        60.
    };
    text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    text(
        "MAHJONG SOLITAIRE",
        header_x,
        header_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let instruction = state
        .card_hint
        .as_deref()
        .unwrap_or(status_text(game.status));
    text(
        instruction,
        if compact { 350. } else { header_x },
        if compact { 30. } else { header_y + 25. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    for (index, tile) in game.tiles.iter().enumerate() {
        if !tile.removed {
            draw_tile(
                *tile,
                index,
                game.selected,
                layout,
                state.high_contrast,
                state.large_text,
            );
        }
    }
    text(
        "TAP TWO MATCHING FREE TILES",
        if compact {
            350.
        } else if portrait {
            10.
        } else {
            360.
        },
        if compact {
            150.
        } else if portrait {
            400.
        } else {
            620.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    text(
        &format!("Pairs {}  •  Free tiles can move", game.moves),
        if compact {
            350.
        } else if portrait {
            10.
        } else {
            360.
        },
        if compact {
            175.
        } else if portrait {
            425.
        } else {
            650.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    button(layout.undo, "UNDO", state.large_text);
    button(layout.new_game, "NEW BOARD", state.large_text);
    button(layout.hint, "HINT", state.large_text);
}

fn draw_tile(
    tile: Tile,
    index: usize,
    selected: Option<usize>,
    layout: Layout,
    high_contrast: bool,
    large_text: bool,
) {
    let rect = tile_rect(tile, layout);
    let shade = 0.10 + f32::from(tile.layer) * 0.04;
    draw_rectangle(
        rect.x + f32::from(tile.layer) * 3.,
        rect.y - f32::from(tile.layer) * 3.,
        rect.w,
        rect.h,
        if high_contrast {
            Color::new(0.22 + shade, 0.18 + shade, 0.34 + shade, 1.)
        } else {
            Color::new(0.18 + shade, 0.12 + shade, 0.28 + shade, 1.)
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected == Some(index) { 3. } else { 1. },
        if selected == Some(index) {
            accent()
        } else {
            accessibility::grid_line(high_contrast)
        },
    );
    text(
        &format!("{}", tile.kind + 1),
        rect.x + rect.w * 0.4,
        rect.y + rect.h * 0.62,
        accessibility::text_size(if crate::ui::is_portrait() { 13. } else { 17. }, large_text),
        WHITE,
    );
}

fn tile_rect(tile: Tile, layout: Layout) -> Rect {
    Rect::new(
        layout.board.x + f32::from(tile.x) * layout.tile_w,
        layout.board.y + f32::from(tile.y) * layout.tile_h - f32::from(tile.layer) * 8.,
        layout.tile_w - 3.,
        layout.tile_h - 3.,
    )
}
fn status_text(status: MahjongStatus) -> &'static str {
    match status {
        MahjongStatus::Playing => "Find the quiet pairs",
        MahjongStatus::Won => "The tiles are clear",
        MahjongStatus::Stuck => "No free pair remains",
    }
}
fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(
        label,
        rect.x + 12.,
        rect.y + 28.,
        accessibility::text_size(11., large_text),
        WHITE,
    );
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        23.
    } else {
        29.
    }
}
fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        11.
    } else {
        13.
    }
}
fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}
fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}
fn back_rect() -> Rect {
    Rect::new(0., 0., 110., 42.)
}
