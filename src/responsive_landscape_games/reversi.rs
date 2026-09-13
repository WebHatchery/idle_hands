use super::{back, panel, text};
use crate::{
    accessibility,
    reversi::{AiLevel, ReversiStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

const REV_BOARD: Rect = Rect {
    x: 10.,
    y: 20.,
    w: 350.,
    h: 350.,
};
pub fn draw_reversi(state: &AppState) {
    let game = &state.games.reversi;
    back();
    text(
        "REVERSI",
        100.,
        20.,
        accessibility::text_size(19., state.large_text),
        crate::theme::BRASS,
    );
    panel(
        REV_BOARD,
        if state.high_contrast {
            Color::new(0.02, 0.20, 0.16, 1.)
        } else {
            Color::new(0.10, 0.30, 0.24, 1.)
        },
    );
    let cell = REV_BOARD.w / 8.;
    let legal = if game.status == ReversiStatus::Playing {
        game.legal_moves(game.turn)
    } else {
        Vec::new()
    };
    for index in 0..64 {
        let row = index / 8;
        let col = index % 8;
        let rect = Rect::new(
            REV_BOARD.x + col as f32 * cell,
            REV_BOARD.y + row as f32 * cell,
            cell,
            cell,
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            accessibility::grid_line(state.high_contrast),
        );
        if game.board[index] == 0 && legal.contains(&index) {
            draw_circle(
                rect.center().x,
                rect.center().y,
                5.,
                if state.high_contrast {
                    WHITE
                } else {
                    Color::new(0.72, 0.95, 0.72, 0.75)
                },
            );
        }
        if game.board[index] != 0 {
            draw_circle(
                rect.center().x,
                rect.center().y,
                cell * 0.34,
                if game.board[index] == 1 {
                    accessibility::board_fill(state.high_contrast)
                } else {
                    if state.high_contrast {
                        WHITE
                    } else {
                        Color::new(0.92, 0.85, 0.66, 1.)
                    }
                },
            );
        }
    }
    text(
        &format!("DARK {}  •  LIGHT {}", game.score(1), game.score(2)),
        400.,
        55.,
        accessibility::text_size(15., state.large_text),
        crate::theme::BRASS,
    );
    text(
        match game.status {
            ReversiStatus::Playing if game.turn == 1 => "Your turn: glowing square",
            ReversiStatus::Playing => "Opponent is thinking",
            ReversiStatus::Won => "Board complete",
        },
        400.,
        82.,
        accessibility::text_size(14., state.large_text),
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(Rect::new(400., 120., 160., 44.), crate::theme::SURFACE_DARK);
    text(
        "PASS TURN",
        450.,
        148.,
        accessibility::text_size(12., state.large_text),
        WHITE,
    );
    panel(Rect::new(590., 120., 160., 44.), crate::theme::SURFACE);
    text(
        "NEW BOARD",
        635.,
        148.,
        accessibility::text_size(12., state.large_text),
        WHITE,
    );
    for (index, label) in [
        ("GENTLE", AiLevel::Gentle),
        ("SHARP", AiLevel::Sharp),
        ("2 PLAYER", AiLevel::TwoPlayer),
    ]
    .iter()
    .enumerate()
    {
        let rect = Rect::new(400. + index as f32 * 120., 190., 112., 44.);
        panel(
            rect,
            if game.ai_level == label.1 {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                crate::theme::SURFACE_DARK
            },
        );
        text(
            label.0,
            rect.x + 25.,
            rect.y + 29.,
            accessibility::text_size(10., state.large_text),
            WHITE,
        );
    }
}
pub fn reversi_clicks(_state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(Rect::new(400., 120., 160., 44.), p) {
        return vec![UiAction::ReversiPass];
    }
    if crate::ui::hit(Rect::new(590., 120., 160., 44.), p) {
        return vec![UiAction::ReversiNew];
    }
    for (index, level) in [AiLevel::Gentle, AiLevel::Sharp, AiLevel::TwoPlayer]
        .iter()
        .enumerate()
    {
        if crate::ui::hit(Rect::new(400. + index as f32 * 120., 190., 112., 44.), p) {
            return vec![UiAction::ReversiLevel(*level)];
        }
    }
    if !REV_BOARD.contains(p) {
        return vec![];
    }
    let cell = REV_BOARD.w / 8.;
    let col = ((p.x - REV_BOARD.x) / cell) as usize;
    let row = ((p.y - REV_BOARD.y) / cell) as usize;
    if row < 8 && col < 8 {
        vec![UiAction::ReversiPlace(row * 8 + col)]
    } else {
        vec![]
    }
}
