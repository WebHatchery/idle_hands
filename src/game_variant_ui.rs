//! Shared rule-card control drawn above every game drawer.

use crate::{
    game_variants,
    state::{AppState, Screen},
    variant_card_data,
};
use macroquad::prelude::*;

pub fn button_rect() -> Rect {
    let (width, _) = crate::ui::layout_size();
    variant_card_data::layout(
        width,
        crate::ui::is_portrait(),
        crate::ui::is_compact_landscape(),
    )
    .rect
}

pub fn clicks(state: &AppState, point: Vec2) -> bool {
    matches!(state.screen, Screen::Game(_)) && crate::ui::hit(button_rect(), point)
}

pub fn draw(state: &AppState) {
    let Screen::Game(game) = state.screen else {
        return;
    };
    let rect = button_rect();
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let layout = variant_card_data::layout(crate::ui::layout_size().0, portrait, compact);
    crate::ui::draw_rounded_panel(
        rect,
        7.,
        crate::theme::GAME_PANEL,
        if state.high_contrast {
            WHITE
        } else {
            crate::theme::BRASS
        },
    );
    let label = macroquad_toolkit::ui::truncate_text_to_width(
        &game_variants::label(state, game),
        layout.label_rect.w,
        layout.label_size,
    );
    crate::ui::draw_text(
        "RULE CARD  ›",
        rect.x + 9.,
        layout.title_baseline,
        layout.title_size,
        crate::theme::BRASS,
    );
    crate::ui::draw_text(
        &label,
        layout.label_rect.x,
        layout.label_baseline,
        layout.label_size,
        crate::theme::CREAM,
    );
    draw_icon(game, rect);
}

fn draw_icon(game: crate::state::GameId, rect: Rect) {
    let center = vec2(rect.right() - 17., rect.y + rect.h * 0.52);
    let ink = crate::theme::BRASS;
    match game {
        crate::state::GameId::Solitaire
        | crate::state::GameId::FreeCell
        | crate::state::GameId::Yahtzee
        | crate::state::GameId::KlondikeGolf
        | crate::state::GameId::Blackjack
        | crate::state::GameId::SpiderSolitaire
        | crate::state::GameId::Pyramid
        | crate::state::GameId::TriPeaks => {
            draw_rectangle_lines(center.x - 9., center.y - 11., 18., 22., 1.5, ink);
            draw_line(
                center.x - 5.,
                center.y - 4.,
                center.x + 5.,
                center.y - 4.,
                1.5,
                ink,
            );
            draw_circle(center.x, center.y + 5., 2.5, ink);
        }
        crate::state::GameId::Hangman
        | crate::state::GameId::WordSearch
        | crate::state::GameId::WordGrid
        | crate::state::GameId::WordLadder
        | crate::state::GameId::Nonogram
        | crate::state::GameId::Mastermind => {
            for row in 0..3 {
                draw_line(
                    center.x - 9.,
                    center.y - 7. + row as f32 * 7.,
                    center.x + 9.,
                    center.y - 7. + row as f32 * 7.,
                    1.5,
                    ink,
                );
            }
            draw_circle(center.x - 5., center.y - 7., 1.5, ink);
            draw_circle(center.x - 5., center.y, 1.5, ink);
            draw_circle(center.x - 5., center.y + 7., 1.5, ink);
        }
        crate::state::GameId::Breakout
        | crate::state::GameId::Snake
        | crate::state::GameId::TinyTowerDefence
        | crate::state::GameId::SpaceInvaders
        | crate::state::GameId::Asteroids
        | crate::state::GameId::Frogger
        | crate::state::GameId::MunchMaze
        | crate::state::GameId::BlockStack
        | crate::state::GameId::TerrainCannon
        | crate::state::GameId::FlingFury
        | crate::state::GameId::PaddleDuel => {
            draw_rectangle(center.x - 9., center.y - 8., 18., 3., ink);
            draw_circle(center.x, center.y + 2., 3., ink);
            draw_line(
                center.x - 9.,
                center.y + 9.,
                center.x + 9.,
                center.y + 9.,
                2.,
                ink,
            );
        }
        _ => {
            draw_rectangle_lines(center.x - 9., center.y - 9., 18., 18., 1.5, ink);
            draw_line(center.x - 8., center.y, center.x + 8., center.y, 1.5, ink);
            draw_line(center.x, center.y - 8., center.x, center.y + 8., 1.5, ink);
        }
    }
}
