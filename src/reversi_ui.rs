//! Touch-first Reversi board and pass controls.

use crate::{
    reversi::{AiLevel, ReversiStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

const BOARD: Rect = Rect {
    x: 70.,
    y: 150.,
    w: 480.,
    h: 480.,
};
fn panel(rect: Rect, fill: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        Color::new(0.45, 0.38, 0.65, 0.65),
    );
}
fn button(rect: Rect, label: &str) {
    panel(rect, Color::new(0.18, 0.12, 0.28, 1.));
    draw_text(label, rect.x + 20., rect.y + 30., 16., WHITE);
}

pub fn draw_reversi(state: &AppState) {
    let game = &state.reversi;
    draw_text("‹ CABINET", 40., 55., 20., Color::new(0.78, 0.70, 0.92, 1.));
    draw_text("REVERSI", 40., 105., 44., Color::new(0.98, 0.83, 0.45, 1.));
    draw_text(
        "Turn the board, one quiet move at a time",
        44.,
        132.,
        18.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    panel(BOARD, Color::new(0.10, 0.30, 0.24, 1.));
    let cell = BOARD.w / 8.;
    for index in 0..64 {
        let row = index / 8;
        let col = index % 8;
        let rect = Rect::new(
            BOARD.x + col as f32 * cell,
            BOARD.y + row as f32 * cell,
            cell,
            cell,
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            Color::new(0.48, 0.75, 0.55, 0.7),
        );
        if game.board[index] == 0
            && game.status == ReversiStatus::Playing
            && game.legal_moves(game.turn).contains(&index)
        {
            draw_circle(
                rect.center().x,
                rect.center().y,
                6.,
                Color::new(0.72, 0.95, 0.72, 0.75),
            );
        }
        if game.board[index] != 0 {
            draw_circle(
                rect.center().x,
                rect.center().y,
                cell * 0.34,
                if game.board[index] == 1 {
                    Color::new(0.08, 0.06, 0.12, 1.)
                } else {
                    Color::new(0.92, 0.85, 0.66, 1.)
                },
            );
            draw_circle_lines(
                rect.center().x,
                rect.center().y,
                cell * 0.34,
                2.,
                Color::new(0.75, 0.62, 0.35, 0.8),
            );
        }
    }
    draw_text(
        format!("YOU  {}", game.score(1)),
        650.,
        190.,
        26.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    draw_text(
        format!("OPPONENT  {}", game.score(2)),
        650.,
        230.,
        22.,
        Color::new(0.82, 0.76, 0.88, 1.),
    );
    draw_text(
        match game.status {
            ReversiStatus::Playing if game.ai_level == AiLevel::TwoPlayer && game.turn == 1 => {
                "Player 1 — tap a glowing square"
            }
            ReversiStatus::Playing if game.ai_level == AiLevel::TwoPlayer => {
                "Player 2 — tap a glowing square"
            }
            ReversiStatus::Playing if game.turn == 1 => "Your turn — tap a glowing square",
            ReversiStatus::Playing => "Opponent is thinking",
            ReversiStatus::Won => match game.winner {
                Some(1) => "You win the board",
                Some(2) => "The opponent takes the board",
                _ => "The board is tied",
            },
        },
        650.,
        285.,
        18.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    draw_text(
        match game.ai_level {
            AiLevel::Gentle => "Opponent: Gentle",
            AiLevel::Sharp => "Opponent: Sharp",
            AiLevel::TwoPlayer => "Same-device two player",
        },
        650.,
        325.,
        17.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    button(Rect::new(650., 390., 180., 48.), "PASS TURN");
    button(Rect::new(650., 455., 180., 48.), "NEW BOARD");
    button(Rect::new(850., 390., 180., 48.), "GENTLE AI");
    button(Rect::new(850., 455., 180., 48.), "SHARP AI");
    button(Rect::new(850., 520., 180., 48.), "TWO PLAYER");
    draw_text(
        "A pass is available when no legal move remains.",
        650.,
        600.,
        15.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
    draw_text(
        "Your dark discs face the light opponent discs.",
        650.,
        625.,
        15.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
}

pub fn reversi_clicks(_state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(20., 20., 180., 50.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if Rect::new(650., 390., 180., 48.).contains(p) {
        return vec![UiAction::ReversiPass];
    }
    if Rect::new(650., 455., 180., 48.).contains(p) {
        return vec![UiAction::ReversiNew];
    }
    if Rect::new(850., 390., 180., 48.).contains(p) {
        return vec![UiAction::ReversiLevel(AiLevel::Gentle)];
    }
    if Rect::new(850., 455., 180., 48.).contains(p) {
        return vec![UiAction::ReversiLevel(AiLevel::Sharp)];
    }
    if Rect::new(850., 520., 180., 48.).contains(p) {
        return vec![UiAction::ReversiLevel(AiLevel::TwoPlayer)];
    }
    if !BOARD.contains(p) {
        return vec![];
    }
    let cell = BOARD.w / 8.;
    let col = ((p.x - BOARD.x) / cell) as usize;
    let row = ((p.y - BOARD.y) / cell) as usize;
    if row < 8 && col < 8 {
        vec![UiAction::ReversiPlace(row * 8 + col)]
    } else {
        vec![]
    }
}
