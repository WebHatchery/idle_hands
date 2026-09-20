use super::{hit, panel, text, Direction};
use crate::{palette_ui, state::AppState, ui_action::UiAction};
use macroquad::prelude::*;

pub(crate) fn draw_2048(state: &AppState) {
    let g = &state.games.game;
    text("‹ CABINET", 40., 55., 20., Color::new(0.78, 0.70, 0.92, 1.));
    text("2048", 40., 105., 52., crate::theme::BRASS);
    text(
        "Slide, merge, breathe",
        44.,
        132.,
        18.,
        crate::theme::SECONDARY,
    );
    score_box(Rect::new(320., 48., 112., 58.), "SCORE", g.score);
    score_box(Rect::new(444., 48., 112., 58.), "BEST", g.best);
    let board = Rect::new(320., 120., 560., 560.);
    panel(board, crate::theme::GAME_PANEL);
    let dimension = g.board_size.dimension();
    let gap = 12.;
    let tile_size = (board.w - 40. - gap * (dimension - 1) as f32) / dimension as f32;
    let grid_side = tile_size * dimension as f32 + gap * (dimension - 1) as f32;
    let origin_x = board.x + (board.w - grid_side) * 0.5;
    let origin_y = board.y + (board.h - grid_side) * 0.5;
    for i in 0..g.cells.len() {
        let r = Rect::new(
            origin_x + (i % dimension) as f32 * (tile_size + gap),
            origin_y + (i / dimension) as f32 * (tile_size + gap),
            tile_size,
            tile_size,
        );
        let v = g.cells[i];
        draw_rectangle(
            r.x,
            r.y,
            r.w,
            r.h,
            palette_ui::tile_color(v, state.board_theme),
        );
        if v > 0 {
            let label = v.to_string();
            let fs = if v < 100 {
                (tile_size * 0.30).min(34.)
            } else if v < 1000 {
                (tile_size * 0.26).min(29.)
            } else {
                (tile_size * 0.22).min(25.)
            };
            let tw = crate::ui::measure_text(&label, None, fs as u16, 1.0).width;
            text(
                &label,
                r.x + (r.w - tw) / 2.,
                r.y + (r.h + fs * 0.36) * 0.5,
                fs,
                crate::theme::CREAM,
            );
        }
    }
    text("MOVE", 930., 150., 14., crate::theme::BRASS);
    for (i, label) in ["UP", "LEFT", "DOWN", "RIGHT"].iter().enumerate() {
        let r = Rect::new(
            930. + (i % 2) as f32 * 98.,
            166. + (i / 2) as f32 * 58.,
            88.,
            46.,
        );
        panel(r, crate::theme::SURFACE_DARK);
        let width = crate::ui::measure_text(label, None, 14, 1.0).width;
        text(
            label,
            r.x + (r.w - width) * 0.5,
            r.y + 30.,
            14.,
            crate::theme::BRASS,
        );
    }
    action_button(Rect::new(930., 300., 186., 48.), "UNDO");
    action_button(Rect::new(930., 360., 186., 48.), "HINT");
    if let Some(hint) = state.card_hint.as_deref() {
        text(hint, 930., 505., 14., Color::new(0.63, 0.95, 0.72, 1.));
    }
    if state.confirm_restart {
        panel(
            Rect::new(330., 270., 440., 150.),
            Color::new(0.16, 0.09, 0.20, 1.),
        );
        text("Start a new board?", 375., 315., 25., WHITE);
        panel(Rect::new(380., 340., 150., 44.), crate::theme::MOSS_DARK);
        text("CANCEL", 417., 368., 16., WHITE);
        panel(
            Rect::new(550., 340., 150., 44.),
            Color::new(0.45, 0.22, 0.25, 1.),
        );
        text("START", 598., 368., 16., WHITE);
    }
}
fn score_box(r: Rect, label: &str, value: u32) {
    panel(r, Color::new(0.12, 0.08, 0.19, 1.));
    text(
        label,
        r.x + 14.,
        r.y + 22.,
        13.,
        Color::new(0.62, 0.55, 0.72, 1.),
    );
    text(&value.to_string(), r.x + 14., r.y + 51., 24., WHITE)
}
fn action_button(rect: Rect, label: &str) {
    panel(rect, crate::theme::SURFACE_DARK);
    let width = crate::ui::measure_text(label, None, 17, 1.0).width;
    text(
        label,
        rect.x + (rect.w - width) * 0.5,
        rect.y + 31.,
        17.,
        WHITE,
    );
}
pub(crate) fn game_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    let mut out = vec![];
    if hit(Rect::new(20., 20., 180., 50.), p) {
        out.push(UiAction::Cabinet)
    }
    if hit(Rect::new(930., 300., 186., 48.), p) && state.games.game.can_undo() {
        out.push(UiAction::Undo)
    }
    if hit(Rect::new(930., 420., 186., 48.), p) {
        out.push(UiAction::Restart)
    }
    if hit(Rect::new(930., 360., 186., 48.), p) {
        out.push(UiAction::Game2048Hint)
    }
    if state.confirm_restart {
        if hit(Rect::new(380., 340., 150., 44.), p) {
            out.push(UiAction::Cancel)
        }
        if hit(Rect::new(550., 340., 150., 44.), p) {
            out.push(UiAction::ConfirmRestart)
        }
    } else {
        for (i, d) in [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ]
        .iter()
        .enumerate()
        {
            if hit(
                Rect::new(
                    930. + (i % 2) as f32 * 98.,
                    166. + (i / 2) as f32 * 58.,
                    88.,
                    46.,
                ),
                p,
            ) {
                out.push(UiAction::Move(*d))
            }
        }
    }
    out
}
