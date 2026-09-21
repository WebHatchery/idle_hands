//! Responsive touch presentation for Tiny Tower Defence.

use crate::{
    state::AppState,
    tiny_tower_defence::{
        EnemyKind, TinyTowerDefence, TowerCellAvailability, TowerKind, TowerPhase,
    },
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Layout {
    pub board: Rect,
    pub wave: Rect,
    pub hint: Rect,
    pub undo: Rect,
    pub new_game: Rect,
    pub kinds: [Rect; 3],
}

pub fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(220., 70., 560., 280.),
            wave: Rect::new(18., 120., 165., 44.),
            hint: Rect::new(18., 278., 120., 40.),
            undo: Rect::new(18., 175., 120., 44.),
            new_game: Rect::new(18., 228., 145., 44.),
            kinds: [
                Rect::new(18., 66., 50., 44.),
                Rect::new(72., 66., 50., 44.),
                Rect::new(126., 66., 56., 44.),
            ],
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 150., 340., 300.),
            wave: Rect::new(20., 485., 165., 44.),
            hint: Rect::new(20., 595., 145., 42.),
            undo: Rect::new(20., 540., 145., 44.),
            new_game: Rect::new(195., 540., 165., 44.),
            kinds: [
                Rect::new(20., 102., 100., 44.),
                Rect::new(130., 102., 100., 44.),
                Rect::new(240., 102., 110., 44.),
            ],
        }
    } else {
        Layout {
            board: Rect::new(350., 125., 560., 400.),
            wave: Rect::new(950., 170., 170., 46.),
            hint: Rect::new(950., 345., 120., 44.),
            undo: Rect::new(950., 230., 120., 44.),
            new_game: Rect::new(950., 288., 155., 44.),
            kinds: [
                Rect::new(950., 108., 52., 44.),
                Rect::new(1006., 108., 52., 44.),
                Rect::new(1062., 108., 58., 44.),
            ],
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    for (index, rect) in l.kinds.iter().enumerate() {
        if crate::ui::hit(*rect, point) {
            return vec![UiAction::TowerSelectKind(TowerKind::ALL[index])];
        }
    }
    if let Some(index) = state.tower_inspection {
        if crate::ui::hit(inspection_build_rect(inspection_rect()), point) {
            return vec![UiAction::TowerBuild(index)];
        }
        if crate::ui::hit(inspection_cancel_rect(inspection_rect()), point) {
            return vec![UiAction::TowerCancelInspection];
        }
        return Vec::new();
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
        return vec![UiAction::TowerInspect(index)];
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.tiny_tower_defence;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let title_x = if compact {
        crate::ui::COMPACT_HEADER_TITLE_X
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
        if compact { compact_status_x() } else { title_x },
        if compact { 30. } else { title_y + 25. },
        body_size(),
        muted(),
    );
    let grid = crate::grid::GridLayout::new(
        l.board,
        TinyTowerDefence::width(),
        TinyTowerDefence::height(),
    );
    for (index, rect) in l.kinds.iter().enumerate() {
        let kind = TowerKind::ALL[index];
        kind_button(
            *rect,
            kind,
            game.selected_kind == kind,
            kind.cost_at_level(0).unwrap_or_default(),
        );
    }
    for index in 0..game.towers.len() {
        let Some(rect) = grid.cell_rect(index) else {
            continue;
        };
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            cell_fill(index, game.towers[index], game.tower_kind(index)),
        );
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., line_color());
        if game.towers[index] > 0 {
            let kind = game.tower_kind(index);
            let label = format!("{}{}", tower_glyph(kind), game.towers[index]);
            center_text(&label, rect, cell_size(), tower_color(kind));
        } else if index % TinyTowerDefence::width() == 0 {
            center_text("IN", rect, small_size(), muted());
        } else if index % TinyTowerDefence::width() == TinyTowerDefence::width() - 1 {
            center_text("BASE", rect, small_size(), accent());
        }
    }
    for enemy in &game.enemies {
        let index = usize::from(enemy.row) * TinyTowerDefence::width() + usize::from(enemy.column);
        if let Some(rect) = grid.cell_rect(index) {
            let label = format!("{}{}", enemy_glyph(enemy.kind), enemy.health);
            draw_circle(
                rect.x + rect.w * 0.5,
                rect.y + rect.h * 0.5,
                rect.w.min(rect.h) * 0.26,
                enemy_color(enemy.kind),
            );
            if enemy.kind == EnemyKind::Armored {
                draw_circle_lines(
                    rect.x + rect.w * 0.5,
                    rect.y + rect.h * 0.5,
                    rect.w.min(rect.h) * 0.31,
                    2.,
                    accent(),
                );
            }
            center_text(&label, rect, small_size(), WHITE);
        }
    }
    text(
        &status_text(game.phase, game.score),
        if compact { 220. } else { title_x },
        if portrait {
            468.
        } else if compact {
            365.
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
            660.
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
    if state.tower_inspection.is_some() {
        draw_inspection(state, game);
    }
}

pub fn wave_label(phase: TowerPhase, paused: bool) -> &'static str {
    match phase {
        TowerPhase::Build => "START WAVE",
        TowerPhase::Wave if paused => "RESUME",
        TowerPhase::Wave => "PAUSE",
        TowerPhase::Won => "WAVE 8 CLEAR",
        TowerPhase::Lost => "LANES LOST",
    }
}

pub fn status_text(phase: TowerPhase, score: u32) -> String {
    match phase {
        TowerPhase::Build => format!("Prepare the lanes  •  Score {}", score),
        TowerPhase::Wave => format!("Stop the invaders  •  Score {}", score),
        TowerPhase::Won => format!("The tower holds  •  Score {}", score),
        TowerPhase::Lost => format!("The gate fell  •  Score {}", score),
    }
}

pub fn cell_fill(index: usize, tower: u8, kind: TowerKind) -> Color {
    if tower > 0 {
        match kind {
            TowerKind::Bolt => Color::new(0.16, 0.30, 0.27, 1.),
            TowerKind::Frost => Color::new(0.14, 0.25, 0.36, 1.),
            TowerKind::Burst => Color::new(0.36, 0.22, 0.14, 1.),
        }
    } else if index.is_multiple_of(TinyTowerDefence::width()) {
        Color::new(0.23, 0.18, 0.29, 1.)
    } else if index % TinyTowerDefence::width() == TinyTowerDefence::width() - 1 {
        Color::new(0.32, 0.23, 0.17, 1.)
    } else {
        Color::new(0.10, 0.08, 0.17, 1.)
    }
}

pub fn kind_button(rect: Rect, kind: TowerKind, selected: bool, cost: u16) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            cell_fill(1, 1, kind)
        } else {
            crate::theme::SURFACE
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected { 2. } else { 1. },
        tower_color(kind),
    );
    center_text(
        kind.label(),
        Rect::new(rect.x, rect.y, rect.w, rect.h * 0.58),
        8.,
        WHITE,
    );
    center_text(
        &format!("{}G", cost),
        Rect::new(rect.x, rect.y + rect.h * 0.42, rect.w, rect.h * 0.58),
        8.,
        tower_color(kind),
    );
}

pub fn inspection_rect() -> Rect {
    let (width, height) = crate::ui::layout_size();
    if crate::ui::is_compact_landscape() {
        Rect::new(18., 116., 185., (height - 126.).min(260.))
    } else if crate::ui::is_portrait() {
        Rect::new(
            10.,
            455.,
            (width - 20.).min(370.),
            (height - 465.).max(285.),
        )
    } else {
        Rect::new(
            920.,
            390.,
            (width - 940.).min(300.),
            (height - 400.).max(250.),
        )
    }
}

pub fn inspection_build_rect(panel: Rect) -> Rect {
    if crate::ui::is_compact_landscape() {
        Rect::new(panel.x + 10., panel.bottom() - 104., panel.w - 20., 44.)
    } else {
        let width = (panel.w - 36.) * 0.5;
        Rect::new(panel.x + 12., panel.bottom() - 56., width, 44.)
    }
}

pub fn inspection_cancel_rect(panel: Rect) -> Rect {
    if crate::ui::is_compact_landscape() {
        Rect::new(panel.x + 10., panel.bottom() - 54., panel.w - 20., 44.)
    } else {
        let width = (panel.w - 36.) * 0.5;
        Rect::new(panel.x + 24. + width, panel.bottom() - 56., width, 44.)
    }
}

pub fn draw_inspection(state: &AppState, game: &TinyTowerDefence) {
    let Some(index) = state.tower_inspection else {
        return;
    };
    let panel = inspection_rect();
    draw_rectangle(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        Color::new(0.08, 0.06, 0.14, 0.98),
    );
    draw_rectangle_lines(panel.x, panel.y, panel.w, panel.h, 2., accent());
    let availability = game.cell_availability(index);
    let (title, kind, level, action_label, cost, available) = match availability {
        TowerCellAvailability::Blocked => (
            format!("CELL {}  RESERVED", index),
            None,
            0,
            "UNAVAILABLE",
            None,
            false,
        ),
        TowerCellAvailability::Build { kind, cost } => (
            format!("CELL {}  EMPTY", index),
            Some(kind),
            1,
            "BUILD",
            Some(cost),
            game.phase == TowerPhase::Build && game.gold >= cost,
        ),
        TowerCellAvailability::Upgrade { kind, level, cost } => (
            format!("CELL {}  {} LEVEL {}", index, kind.label(), level),
            Some(kind),
            level.saturating_add(1),
            "UPGRADE",
            Some(cost),
            game.phase == TowerPhase::Build && game.gold >= cost,
        ),
        TowerCellAvailability::MaxLevel { kind, level } => (
            format!("CELL {}  {} LEVEL {}", index, kind.label(), level),
            Some(kind),
            level,
            "MAX LEVEL",
            None,
            false,
        ),
    };
    text(&title, panel.x + 12., panel.y + 25., body_size(), accent());
    if let Some(kind) = kind {
        text(
            &format!("{}  •  {}", kind.label(), kind.role()),
            panel.x + 12.,
            panel.y + 48.,
            body_size(),
            tower_color(kind),
        );
        text(
            &tower_effect(kind, level),
            panel.x + 12.,
            panel.y + 70.,
            body_size(),
            muted(),
        );
    } else {
        text(
            "ENTRY / BASE LANES CANNOT BUILD",
            panel.x + 12.,
            panel.y + 52.,
            body_size(),
            muted(),
        );
    }
    let detail_y = panel.y
        + if crate::ui::is_compact_landscape() {
            98.
        } else {
            94.
        };
    let detail = if game.phase != TowerPhase::Build {
        "PAUSE THE WAVE TO CHANGE TOWERS".to_owned()
    } else if let Some(cost) = cost {
        if game.gold < cost {
            format!("NEED {}G  •  HAVE {}G", cost, game.gold)
        } else if action_label == "BUILD" {
            format!("COST {}G  •  GOLD AFTER {}G", cost, game.gold - cost)
        } else {
            format!("NEXT {}G  •  GOLD AFTER {}G", cost, game.gold - cost)
        }
    } else {
        "NO FURTHER UPGRADE AVAILABLE".to_owned()
    };
    text(&detail, panel.x + 12., detail_y, body_size(), muted());
    inspection_button(inspection_build_rect(panel), action_label, available, state);
    inspection_button(inspection_cancel_rect(panel), "CANCEL", true, state);
}

pub fn tower_effect(kind: TowerKind, level: u8) -> String {
    match kind {
        TowerKind::Bolt => format!("DAMAGE {}  •  RANGE 3", level),
        TowerKind::Frost => format!("DAMAGE 1  •  SLOW {} TICKS", 1 + level / 2),
        TowerKind::Burst => format!("DAMAGE {}  •  AREA SPLASH", 1 + level.saturating_sub(1) / 2),
    }
}

pub fn inspection_button(rect: Rect, label: &str, enabled: bool, state: &AppState) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if enabled {
            crate::theme::SURFACE
        } else {
            crate::theme::SURFACE_DARK
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if state.high_contrast { 2. } else { 1. },
        if enabled { accent() } else { muted() },
    );
    center_text(label, rect, 10., if enabled { WHITE } else { muted() });
}

pub fn tower_glyph(kind: TowerKind) -> &'static str {
    match kind {
        TowerKind::Bolt => "B",
        TowerKind::Frost => "F",
        TowerKind::Burst => "X",
    }
}

pub fn tower_color(kind: TowerKind) -> Color {
    match kind {
        TowerKind::Bolt => Color::new(0.62, 1., 0.78, 1.),
        TowerKind::Frost => Color::new(0.48, 0.82, 1., 1.),
        TowerKind::Burst => Color::new(1., 0.70, 0.32, 1.),
    }
}

pub fn enemy_glyph(kind: EnemyKind) -> &'static str {
    match kind {
        EnemyKind::Grunt => "G",
        EnemyKind::Swift => "S",
        EnemyKind::Armored => "A",
    }
}

pub fn enemy_color(kind: EnemyKind) -> Color {
    match kind {
        EnemyKind::Grunt => Color::new(0.62, 0.22, 0.35, 1.),
        EnemyKind::Swift => Color::new(0.72, 0.38, 0.78, 1.),
        EnemyKind::Armored => Color::new(0.55, 0.42, 0.30, 1.),
    }
}

pub fn button(rect: Rect, label: &str) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(label, rect, 11., WHITE);
}

pub fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = crate::ui::measure_text(label, None, size as u16, 1.);
    crate::ui::draw_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + rect.h * 0.62,
        size,
        color,
    );
}

pub fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}

pub fn title_size() -> f32 {
    if crate::ui::is_compact_landscape() {
        19.
    } else if crate::ui::is_portrait() {
        18.
    } else {
        27.
    }
}

pub fn compact_status_x() -> f32 {
    280.
}

pub fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        10.
    } else {
        12.
    }
}

pub fn cell_size() -> f32 {
    if crate::ui::is_portrait() {
        19.
    } else {
        25.
    }
}

pub fn small_size() -> f32 {
    if crate::ui::is_portrait() {
        8.
    } else {
        10.
    }
}

pub fn accent() -> Color {
    crate::theme::BRASS
}

pub fn muted() -> Color {
    crate::theme::SECONDARY
}

pub fn line_color() -> Color {
    Color::new(0.45, 0.38, 0.65, 0.8)
}
