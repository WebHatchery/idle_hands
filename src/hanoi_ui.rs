//! Responsive touch presentation for Towers of Hanoi.

use crate::{
    hanoi::{Hanoi, HanoiPhase},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    pegs: [Rect; 3],
    hint: Rect,
    undo: Rect,
    new_game: Rect,
    disk_choices: [Rect; 3],
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        let board = Rect::new(230., 48., 370., 270.);
        Layout {
            board,
            pegs: peg_rects(board),
            hint: Rect::new(635., 220., 105., 44.),
            undo: Rect::new(635., 110., 105., 44.),
            new_game: Rect::new(635., 165., 140., 44.),
            disk_choices: [
                Rect::new(625., 50., 67., 44.),
                Rect::new(697., 50., 67., 44.),
                Rect::new(769., 50., 67., 44.),
            ],
        }
    } else if crate::ui::is_portrait() {
        let board = Rect::new(15., 105., 300., 285.);
        Layout {
            board,
            pegs: peg_rects(board),
            hint: Rect::new(15., 455., 145., 44.),
            undo: Rect::new(15., 510., 145., 44.),
            new_game: Rect::new(170., 510., 145., 44.),
            disk_choices: [
                Rect::new(15., 400., 90., 44.),
                Rect::new(112., 400., 90., 44.),
                Rect::new(209., 400., 90., 44.),
            ],
        }
    } else {
        let board = Rect::new(300., 100., 500., 330.);
        Layout {
            board,
            pegs: peg_rects(board),
            hint: Rect::new(850., 245., 120., 44.),
            undo: Rect::new(850., 190., 120., 44.),
            new_game: Rect::new(990., 190., 145., 44.),
            disk_choices: [
                Rect::new(850., 125., 90., 44.),
                Rect::new(950., 125., 90., 44.),
                Rect::new(1050., 125., 90., 44.),
            ],
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    for (peg, rect) in l.pegs.iter().enumerate() {
        if rect.contains(point) {
            return vec![UiAction::HanoiPeg(peg)];
        }
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::HanoiHint];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::HanoiUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::HanoiNew];
    }
    for (index, disks) in [3, 5, 7].into_iter().enumerate() {
        if crate::ui::hit(l.disk_choices[index], point) {
            return vec![UiAction::HanoiDisks(disks)];
        }
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.hanoi;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let title_x = if compact {
        70.
    } else if portrait {
        25.
    } else {
        400.
    };
    let title_y = if compact {
        28.
    } else if portrait {
        62.
    } else {
        58.
    };
    text("‹ CABINET", 8., 30., 13., muted());
    text("HANOI", title_x, title_y, title_size(), accent());
    text(
        &format!(
            "{} disks  •  Moves {}/{}  •  {}",
            game.disks,
            game.moves,
            game.optimal_moves(),
            if game.won() {
                game.clear_rank()
            } else {
                "MOVE THE DISKS"
            }
        ),
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        body_size(),
        muted(),
    );
    draw_board(l.board, game);
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or(status_text(game.phase)),
        if compact { 230. } else { title_x },
        if portrait {
            575.
        } else if compact {
            340.
        } else {
            470.
        },
        body_size(),
        muted(),
    );
    button(l.hint, "HINT");
    button(l.undo, "UNDO");
    button(l.new_game, "RESTART");
    for (rect, disks) in l.disk_choices.iter().zip([3, 5, 7]) {
        button_selected(*rect, &format!("{} DISKS", disks), game.disks == disks);
    }
}

fn peg_rects(board: Rect) -> [Rect; 3] {
    let step = board.w / 3.;
    [
        Rect::new(board.x, board.y, step, board.h),
        Rect::new(board.x + step, board.y, step, board.h),
        Rect::new(board.x + step * 2., board.y, step, board.h),
    ]
}

fn draw_board(board: Rect, game: &Hanoi) {
    draw_rectangle(
        board.x,
        board.y,
        board.w,
        board.h,
        Color::new(0.14, 0.10, 0.22, 1.),
    );
    draw_rectangle_lines(board.x, board.y, board.w, board.h, 2., accent());
    let base_y = board.y + board.h - 35.;
    let post_top = board.y + 35.;
    for peg in 0..3 {
        let center = board.x + board.w * (peg as f32 * 2. + 1.) / 6.;
        draw_line(center, post_top, center, base_y, 5., line_color());
        draw_line(
            center - board.w / 7.,
            base_y,
            center + board.w / 7.,
            base_y,
            5.,
            accent(),
        );
        if game.selected == Some(peg) {
            draw_circle(center, post_top - 12., 8., accent());
        } else if let Some(source) = game.selected {
            draw_circle_lines(
                center,
                post_top - 12.,
                9.,
                3.,
                if game.can_move(source, peg) {
                    Color::new(0.45, 0.90, 0.58, 1.)
                } else {
                    Color::new(0.95, 0.35, 0.38, 1.)
                },
            );
        }
        for (level, disk) in game.stacks[peg].iter().enumerate() {
            let width = board.w * (0.08 + *disk as f32 * 0.025);
            let y = base_y - (level as f32 + 1.) * disk_height();
            draw_rectangle(
                center - width / 2.,
                y,
                width,
                disk_height() - 3.,
                disk_color(*disk),
            );
            draw_rectangle_lines(center - width / 2., y, width, disk_height() - 3., 1., WHITE);
            center_text(
                &disk.to_string(),
                Rect::new(center - width / 2., y, width, disk_height() - 3.),
                11.,
                crate::theme::BACKGROUND,
            );
        }
    }
}

fn disk_height() -> f32 {
    if crate::ui::is_portrait() {
        26.
    } else {
        29.
    }
}

fn disk_color(disk: u8) -> Color {
    let colors = [
        Color::new(0.38, 0.64, 0.95, 1.),
        Color::new(0.55, 0.85, 0.62, 1.),
        Color::new(0.95, 0.63, 0.35, 1.),
        Color::new(0.84, 0.48, 0.80, 1.),
        crate::theme::BRASS,
        Color::new(0.40, 0.84, 0.84, 1.),
        Color::new(0.94, 0.44, 0.56, 1.),
    ];
    colors[(disk.saturating_sub(1) as usize).min(colors.len() - 1)]
}

fn status_text(phase: HanoiPhase) -> &'static str {
    match phase {
        HanoiPhase::Playing => "Tap a source peg, then a destination peg",
        HanoiPhase::Won => "Tower complete • Choose a disk count or tap RESTART",
    }
}

fn button(rect: Rect, label: &str) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(label, rect, 11., WHITE);
}

fn button_selected(rect: Rect, label: &str, selected: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            crate::theme::SURFACE
        },
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(label, rect, 9., WHITE);
}

fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = crate::ui::measure_text(label, None, size as u16, 1.);
    crate::ui::draw_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + rect.h * 0.63,
        size,
        color,
    );
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
fn title_size() -> f32 {
    if crate::ui::is_compact_landscape() {
        20.
    } else if crate::ui::is_portrait() {
        23.
    } else {
        29.
    }
}
fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        10.
    } else {
        12.
    }
}
fn accent() -> Color {
    crate::theme::BRASS
}
fn muted() -> Color {
    crate::theme::SECONDARY
}
fn line_color() -> Color {
    Color::new(0.45, 0.38, 0.65, 0.8)
}
