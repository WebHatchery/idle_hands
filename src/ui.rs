//! Touch-first cabinet and 2048 presentation.

use crate::{
    data::GameData,
    state::{AppState, Direction, GameId, Screen},
};
use macroquad::prelude::*;
pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;
#[derive(Debug, Clone, Copy)]
pub enum UiAction {
    Open(usize),
    Cabinet,
    Help,
    Settings,
    Move(Direction),
    Undo,
    Restart,
    ConfirmRestart,
    Cancel,
    ToggleSound,
    ToggleMotion,
}
pub fn mouse() -> Vec2 {
    vec2(
        mouse_position().0 * LOGICAL_WIDTH / screen_width(),
        mouse_position().1 * LOGICAL_HEIGHT / screen_height(),
    )
}
pub fn clicks(state: &AppState) -> Vec<UiAction> {
    let p = mouse();
    match state.screen {
        Screen::Cabinet => {
            let mut out = vec![];
            for i in 0..8 {
                if cabinet_rect(i).contains(p) {
                    out.push(UiAction::Open(i));
                }
            }
            if Rect::new(1040., 28., 90., 42.).contains(p) {
                out.push(UiAction::Help)
            }
            if Rect::new(1140., 28., 110., 42.).contains(p) {
                out.push(UiAction::Settings)
            }
            out
        }
        Screen::Game(GameId::Game2048) => game_clicks(state, p),
        Screen::Help => {
            if Rect::new(1030., 635., 180., 48.).contains(p) {
                vec![UiAction::Cabinet]
            } else {
                vec![]
            }
        }
        Screen::Settings => settings_clicks(p),
        Screen::Game(_) => vec![],
    }
}
pub fn draw(state: &AppState, data: &GameData, loaded_assets: usize) {
    match state.screen {
        Screen::Cabinet => draw_cabinet(state, data, loaded_assets),
        Screen::Game(GameId::Game2048) => draw_2048(state),
        Screen::Help => draw_help(),
        Screen::Settings => draw_settings(state),
        Screen::Game(_) => draw_cabinet(state, data, loaded_assets),
    }
}
fn text(s: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(s, x, y, size, color);
}
fn panel(r: Rect, fill: Color) {
    draw_rectangle(r.x, r.y, r.w, r.h, fill);
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., Color::new(0.45, 0.38, 0.65, 0.65))
}
fn draw_cabinet(state: &AppState, data: &GameData, loaded: usize) {
    text(
        "IDLE HANDS",
        46.,
        70.,
        48.,
        Color::new(0.95, 0.83, 0.45, 1.),
    );
    text(
        "A small collection for quiet minutes",
        48.,
        98.,
        20.,
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    text(
        &format!(
            "{}  •  {} games waiting at the cabinet",
            state.profile_name,
            GameId::ALL.len()
        ),
        48.,
        130.,
        18.,
        Color::new(0.60, 0.56, 0.72, 1.),
    );
    for i in 0..8 {
        let r = cabinet_rect(i);
        let active = i == 4;
        panel(
            r,
            if active {
                Color::new(0.17, 0.12, 0.27, 1.)
            } else {
                Color::new(0.09, 0.075, 0.15, 1.)
            },
        );
        text(
            GameId::ALL[i].title(),
            r.x + 18.,
            r.y + 40.,
            25.,
            if active {
                Color::new(0.98, 0.82, 0.42, 1.)
            } else {
                WHITE
            },
        );
        text(
            if active { "PLAY NOW" } else { "COMING SOON" },
            r.x + 18.,
            r.y + 70.,
            14.,
            if active {
                Color::new(0.55, 1., 0.72, 1.)
            } else {
                Color::new(0.58, 0.54, 0.66, 1.)
            },
        );
        text(
            GameId::ALL[i].subtitle(),
            r.x + 18.,
            r.y + 102.,
            15.,
            Color::new(0.69, 0.65, 0.78, 1.),
        );
        draw_circle(
            r.right() - 34.,
            r.y + 40.,
            18.,
            if active {
                Color::new(0.85, 0.55, 0.28, 1.)
            } else {
                Color::new(0.22, 0.18, 0.31, 1.)
            },
        );
        text(
            &format!("{}", i + 1),
            r.right() - 39.,
            r.y + 46.,
            16.,
            Color::new(0.08, 0.05, 0.12, 1.),
        );
    }
    text("HELP", 1054., 55., 17., WHITE);
    text("SETTINGS", 1151., 55., 17., WHITE);
    text(
        &format!("Cabinet online  •  {} textures ready", loaded),
        48.,
        686.,
        16.,
        Color::new(0.52, 0.48, 0.64, 1.),
    );
    let _ = data;
}
fn cabinet_rect(i: usize) -> Rect {
    let col = i % 4;
    let row = i / 4;
    Rect::new(
        48. + col as f32 * 300.,
        165. + row as f32 * 210.,
        270.,
        178.,
    )
}
fn draw_2048(state: &AppState) {
    let g = &state.game;
    text("‹ CABINET", 40., 55., 20., Color::new(0.78, 0.70, 0.92, 1.));
    text("2048", 40., 105., 52., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        "Slide, merge, breathe",
        44.,
        132.,
        18.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    score_box(Rect::new(830., 68., 120., 66.), "SCORE", g.score);
    score_box(Rect::new(965., 68., 120., 66.), "BEST", g.best);
    panel(
        Rect::new(830., 160., 360., 380.),
        Color::new(0.10, 0.07, 0.16, 1.),
    );
    for i in 0..16 {
        let r = Rect::new(
            850. + (i % 4) as f32 * 84.,
            180. + (i / 4) as f32 * 84.,
            76.,
            76.,
        );
        let v = g.cells[i];
        draw_rectangle(r.x, r.y, r.w, r.h, tile_color(v));
        if v > 0 {
            let label = v.to_string();
            let fs = if v < 100 {
                30.
            } else if v < 1000 {
                25.
            } else {
                20.
            };
            let tw = measure_text(&label, None, fs as u16, 1.0).width;
            text(
                &label,
                r.x + (r.w - tw) / 2.,
                r.y + 48.,
                fs,
                if v < 8 {
                    Color::new(0.25, 0.18, 0.20, 1.)
                } else {
                    WHITE
                },
            );
        }
    }
    text(
        "Every move is touch-complete",
        830.,
        570.,
        17.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    text(
        "Swipe the board or use a direction button",
        830.,
        594.,
        16.,
        Color::new(0.55, 0.50, 0.64, 1.),
    );
    for (i, label) in ["↑", "←", "↓", "→"].iter().enumerate() {
        let r = Rect::new(830. + i as f32 * 90., 615., 78., 46.);
        panel(r, Color::new(0.18, 0.12, 0.28, 1.));
        text(
            label,
            r.x + 28.,
            r.y + 33.,
            26.,
            Color::new(0.98, 0.83, 0.45, 1.),
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
    panel(
        Rect::new(400., 390., 140., 48.),
        Color::new(0.18, 0.12, 0.28, 1.),
    );
    text("UNDO", 438., 421., 17., WHITE);
    panel(
        Rect::new(560., 390., 140., 48.),
        Color::new(0.18, 0.12, 0.28, 1.),
    );
    text("NEW GAME", 575., 421., 17., WHITE);
    if state.confirm_restart {
        panel(
            Rect::new(330., 270., 440., 150.),
            Color::new(0.16, 0.09, 0.20, 1.),
        );
        text("Start a new board?", 375., 315., 25., WHITE);
        panel(
            Rect::new(380., 340., 150., 44.),
            Color::new(0.25, 0.16, 0.32, 1.),
        );
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
fn tile_color(v: u16) -> Color {
    match v {
        0 => Color::new(0.14, 0.10, 0.20, 1.),
        2 => Color::new(0.35, 0.25, 0.32, 1.),
        4 => Color::new(0.45, 0.30, 0.29, 1.),
        8 => Color::new(0.72, 0.40, 0.22, 1.),
        16 => Color::new(0.83, 0.50, 0.20, 1.),
        32 => Color::new(0.82, 0.32, 0.20, 1.),
        64 => Color::new(0.75, 0.20, 0.25, 1.),
        128 => Color::new(0.65, 0.40, 0.72, 1.),
        256 => Color::new(0.50, 0.36, 0.78, 1.),
        512 => Color::new(0.35, 0.45, 0.80, 1.),
        1024 => Color::new(0.30, 0.65, 0.70, 1.),
        _ => Color::new(0.72, 0.62, 0.25, 1.),
    }
}
fn game_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    let mut out = vec![];
    if Rect::new(20., 20., 180., 50.).contains(p) {
        out.push(UiAction::Cabinet)
    }
    if Rect::new(400., 390., 140., 48.).contains(p) && state.game.can_undo() {
        out.push(UiAction::Undo)
    }
    if Rect::new(560., 390., 140., 48.).contains(p) {
        out.push(UiAction::Restart)
    }
    if state.confirm_restart {
        if Rect::new(380., 340., 150., 44.).contains(p) {
            out.push(UiAction::Cancel)
        }
        if Rect::new(550., 340., 150., 44.).contains(p) {
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
            if Rect::new(830. + i as f32 * 90., 615., 78., 46.).contains(p) {
                out.push(UiAction::Move(*d))
            }
        }
    }
    out
}
fn draw_help() {
    panel(
        Rect::new(120., 80., 1040., 560.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text(
        "HOW TO PLAY",
        170.,
        145.,
        42.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    text(
        "Idle Hands is a cabinet of calm, tactile games.",
        170.,
        200.,
        24.,
        WHITE,
    );
    text(
        "Tap a cabinet object to open it. The highlighted 2048 drawer is ready now.",
        170.,
        245.,
        19.,
        Color::new(0.75, 0.70, 0.84, 1.),
    );
    text(
        "In 2048, swipe the board or tap the visible arrows. Matching tiles merge.",
        170.,
        285.,
        19.,
        Color::new(0.75, 0.70, 0.84, 1.),
    );
    text(
        "All future games remain reachable and clearly marked while they are built.",
        170.,
        325.,
        19.,
        Color::new(0.75, 0.70, 0.84, 1.),
    );
    panel(
        Rect::new(1030., 635., 180., 48.),
        Color::new(0.25, 0.16, 0.32, 1.),
    );
    text("BACK", 1090., 666., 18., WHITE)
}
fn draw_settings(state: &AppState) {
    panel(
        Rect::new(240., 100., 800., 500.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text(
        "SETTINGS",
        290.,
        170.,
        42.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    text(
        &format!("Profile: {}", state.profile_name),
        290.,
        235.,
        22.,
        WHITE,
    );
    text(
        &format!("Sound: {}", if state.sound { "On" } else { "Off" }),
        290.,
        295.,
        20.,
        WHITE,
    );
    text(
        &format!(
            "Reduced motion: {}",
            if state.reduced_motion { "On" } else { "Off" }
        ),
        290.,
        355.,
        20.,
        WHITE,
    );
    text(
        "Settings are saved per profile in the collection shell.",
        290.,
        430.,
        17.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    panel(
        Rect::new(290., 490., 150., 48.),
        Color::new(0.25, 0.16, 0.32, 1.),
    );
    text("BACK", 340., 521., 17., WHITE)
}
fn settings_clicks(p: Vec2) -> Vec<UiAction> {
    let mut o = vec![];
    if Rect::new(290., 490., 150., 48.).contains(p) {
        o.push(UiAction::Cabinet)
    }
    if Rect::new(290., 270., 250., 45.).contains(p) {
        o.push(UiAction::ToggleSound)
    }
    if Rect::new(290., 330., 300., 45.).contains(p) {
        o.push(UiAction::ToggleMotion)
    }
    o
}
