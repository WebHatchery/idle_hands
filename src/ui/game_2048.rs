use super::{hit, panel, text, Direction, Game2048Size};
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
    text("BOARD SIZE", 400., 145., 13., crate::theme::BRASS);
    for (index, board_size) in Game2048Size::ALL.iter().enumerate() {
        let rect = Rect::new(400. + index as f32 * 155., 150., 145., 34.);
        panel(
            rect,
            if *board_size == g.board_size {
                crate::theme::LEATHER
            } else {
                crate::theme::GAME_PANEL
            },
        );
        text(board_size.label(), rect.x + 43., rect.y + 22., 12., WHITE);
    }
    score_box(Rect::new(830., 68., 120., 66.), "SCORE", g.score);
    score_box(Rect::new(965., 68., 120., 66.), "BEST", g.best);
    panel(Rect::new(830., 160., 360., 380.), crate::theme::GAME_PANEL);
    let dimension = g.board_size.dimension();
    let tile_size = if dimension == 4 { 76. } else { 60. };
    let gap = 8.;
    let grid_side = tile_size * dimension as f32 + gap * (dimension - 1) as f32;
    let origin_x = 830. + (360. - grid_side) * 0.5;
    let origin_y = 160. + (380. - grid_side) * 0.5;
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
                if dimension == 4 {
                    30.
                } else {
                    24.
                }
            } else if v < 1000 {
                if dimension == 4 {
                    25.
                } else {
                    21.
                }
            } else {
                if dimension == 4 {
                    20.
                } else {
                    18.
                }
            };
            let tw = crate::ui::measure_text(&label, None, fs as u16, 1.0).width;
            text(
                &label,
                r.x + (r.w - tw) / 2.,
                r.y + 48.,
                fs,
                crate::theme::CREAM,
            );
        }
    }
    text(
        "Every move is touch-complete",
        830.,
        570.,
        17.,
        crate::theme::SECONDARY,
    );
    text(
        "Swipe the board or use a direction button",
        830.,
        594.,
        16.,
        Color::new(0.55, 0.50, 0.64, 1.),
    );
    for (i, label) in ["UP", "LEFT", "DOWN", "RIGHT"].iter().enumerate() {
        let r = Rect::new(830. + i as f32 * 90., 615., 78., 46.);
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
    panel(
        Rect::new(400., 190., 300., 160.),
        Color::new(0.09, 0.07, 0.14, 0.98),
    );
    text("Tap or drag to combine", 425., 230., 23., WHITE);
    text(
        "matching tiles into a larger tile.",
        425.,
        260.,
        17.,
        Color::new(0.72, 0.68, 0.80, 1.),
    );
    panel(Rect::new(400., 390., 140., 48.), crate::theme::SURFACE_DARK);
    text("UNDO", 438., 421., 17., WHITE);
    panel(Rect::new(560., 390., 140., 48.), crate::theme::SURFACE_DARK);
    text("NEW GAME", 575., 421., 17., WHITE);
    panel(Rect::new(400., 450., 140., 48.), crate::theme::SURFACE_DARK);
    text("HINT", 438., 481., 17., WHITE);
    if let Some(hint) = state.card_hint.as_deref() {
        text(hint, 400., 520., 14., Color::new(0.63, 0.95, 0.72, 1.));
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
pub(crate) fn game_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    let mut out = vec![];
    if hit(Rect::new(20., 20., 180., 50.), p) {
        out.push(UiAction::Cabinet)
    }
    if hit(Rect::new(400., 390., 140., 48.), p) && state.games.game.can_undo() {
        out.push(UiAction::Undo)
    }
    if hit(Rect::new(560., 390., 140., 48.), p) {
        out.push(UiAction::Restart)
    }
    if hit(Rect::new(400., 450., 140., 48.), p) {
        out.push(UiAction::Game2048Hint)
    }
    for (index, board_size) in Game2048Size::ALL.iter().enumerate() {
        if hit(Rect::new(400. + index as f32 * 155., 150., 145., 34.), p)
            && state.games.game.board_size != *board_size
        {
            out.push(UiAction::Game2048Size(*board_size));
        }
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
            if hit(Rect::new(830. + i as f32 * 90., 615., 78., 46.), p) {
                out.push(UiAction::Move(*d))
            }
        }
    }
    out
}
