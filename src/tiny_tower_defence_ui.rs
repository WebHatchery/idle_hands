//! Responsive touch presentation for Tiny Tower Defence.

use crate::{
    state::AppState,
    tiny_tower_defence::{TinyTowerDefence, TowerPhase},
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    wave: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(220., 70., 560., 280.),
            wave: Rect::new(18., 120., 165., 44.),
            hint: Rect::new(18., 278., 120., 40.),
            undo: Rect::new(18., 175., 120., 44.),
            new_game: Rect::new(18., 228., 145., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(20., 140., 360., 300.),
            wave: Rect::new(20., 470., 165., 44.),
            hint: Rect::new(20., 580., 145., 42.),
            undo: Rect::new(20., 525., 145., 44.),
            new_game: Rect::new(195., 525., 165., 44.),
        }
    } else {
        Layout {
            board: Rect::new(350., 125., 560., 400.),
            wave: Rect::new(950., 170., 170., 46.),
            hint: Rect::new(950., 345., 120., 44.),
            undo: Rect::new(950., 230., 120., 44.),
            new_game: Rect::new(950., 288., 155., 44.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    if l.wave.contains(point) {
        return vec![UiAction::TowerWave];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::TowerUndo];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::TowerHint];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::TowerNew];
    }
    if let Some(index) = crate::grid::GridLayout::new(
        l.board,
        crate::tiny_tower_defence::TinyTowerDefence::width(),
        crate::tiny_tower_defence::TinyTowerDefence::height(),
    )
    .index_at(point)
    {
        return vec![UiAction::TowerCell(index)];
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.tiny_tower_defence;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let title_x = if compact {
        70.
    } else if portrait {
        10.
    } else {
        400.
    };
    let title_y = if compact {
        30.
    } else if portrait {
        68.
    } else {
        60.
    };
    text("‹ CABINET", 8., 30., 13., muted());
    let title = if compact {
        "TOWER DEFENCE"
    } else {
        "TINY TOWER DEFENCE"
    };
    text(title, title_x, title_y, title_size(), accent());
    text(
        &format!(
            "Gold {}  •  Lives {}  •  Wave {} / {}",
            game.gold,
            game.lives,
            game.wave,
            TinyTowerDefence::target_wave()
        ),
        if compact { 430. } else { title_x },
        if compact { 30. } else { title_y + 25. },
        body_size(),
        muted(),
    );
    let grid = crate::grid::GridLayout::new(
        l.board,
        TinyTowerDefence::width(),
        TinyTowerDefence::height(),
    );
    for index in 0..game.towers.len() {
        let rect = grid.cell_rect(index).unwrap();
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            cell_fill(index, game.towers[index]),
        );
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., line_color());
        if game.towers[index] > 0 {
            let label = format!("T{}", game.towers[index]);
            center_text(&label, rect, cell_size(), Color::new(0.62, 1., 0.78, 1.));
        } else if index % TinyTowerDefence::width() == 0 {
            center_text("IN", rect, small_size(), muted());
        } else if index % TinyTowerDefence::width() == TinyTowerDefence::width() - 1 {
            center_text("BASE", rect, small_size(), accent());
        }
    }
    for enemy in &game.enemies {
        let index = usize::from(enemy.row) * TinyTowerDefence::width() + usize::from(enemy.column);
        if let Some(rect) = grid.cell_rect(index) {
            let label = format!("E{}", enemy.health);
            draw_circle(
                rect.x + rect.w * 0.5,
                rect.y + rect.h * 0.5,
                rect.w.min(rect.h) * 0.26,
                Color::new(0.62, 0.22, 0.35, 1.),
            );
            center_text(&label, rect, small_size(), WHITE);
        }
    }
    text(
        &status_text(game.phase, game.score),
        if compact { 220. } else { title_x },
        if portrait {
            458.
        } else if compact {
            375.
        } else {
            560.
        },
        body_size(),
        muted(),
    );
    text(
        state.card_hint.as_deref().unwrap_or(if game.paused {
            "Wave paused — tap RESUME"
        } else {
            "Tap empty cells to build • tap towers to upgrade"
        }),
        if compact { 220. } else { title_x },
        if portrait {
            640.
        } else if compact {
            383.
        } else {
            580.
        },
        body_size(),
        muted(),
    );
    button(l.wave, wave_label(game.phase, game.paused));
    button(l.hint, "HINT");
    button(l.undo, "UNDO");
    button(l.new_game, "NEW TOWER");
}

fn wave_label(phase: TowerPhase, paused: bool) -> &'static str {
    match phase {
        TowerPhase::Build => "START WAVE",
        TowerPhase::Wave if paused => "RESUME",
        TowerPhase::Wave => "PAUSE",
        TowerPhase::Won => "WAVE 8 CLEAR",
        TowerPhase::Lost => "LANES LOST",
    }
}

fn status_text(phase: TowerPhase, score: u32) -> String {
    match phase {
        TowerPhase::Build => format!("Prepare the lanes  •  Score {}", score),
        TowerPhase::Wave => format!("Stop the quiet invaders  •  Score {}", score),
        TowerPhase::Won => format!("The tower holds  •  Score {}", score),
        TowerPhase::Lost => format!("The gate fell  •  Score {}", score),
    }
}

fn cell_fill(index: usize, tower: u8) -> Color {
    if tower > 0 {
        Color::new(0.16, 0.30, 0.27, 1.)
    } else if index.is_multiple_of(TinyTowerDefence::width()) {
        Color::new(0.23, 0.18, 0.29, 1.)
    } else if index % TinyTowerDefence::width() == TinyTowerDefence::width() - 1 {
        Color::new(0.32, 0.23, 0.17, 1.)
    } else {
        Color::new(0.10, 0.08, 0.17, 1.)
    }
}

fn button(rect: Rect, label: &str) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(label, rect, 11., WHITE);
}

fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = crate::ui::measure_text(label, None, size as u16, 1.);
    crate::ui::draw_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + rect.h * 0.62,
        size,
        color,
    );
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}

fn title_size() -> f32 {
    if crate::ui::is_compact_landscape() {
        19.
    } else if crate::ui::is_portrait() {
        20.
    } else {
        27.
    }
}

fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        10.
    } else {
        12.
    }
}

fn cell_size() -> f32 {
    if crate::ui::is_portrait() {
        19.
    } else {
        25.
    }
}

fn small_size() -> f32 {
    if crate::ui::is_portrait() {
        8.
    } else {
        10.
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
