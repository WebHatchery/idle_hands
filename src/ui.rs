//! Touch-first cabinet and 2048 presentation.

use crate::fivefold_ui;
use crate::freecell_ui;
use crate::input::Viewport;
use crate::nonogram_ui;
use crate::records_ui;
use crate::reversi_ui;
use crate::solitaire_ui;
use crate::sudoku_ui;
use crate::{
    data::GameData,
    minesweeper::{Cell, MinePreset, MineStatus},
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
    Records,
    Settings,
    Save,
    Load,
    Move(Direction),
    Undo,
    Restart,
    ConfirmRestart,
    Cancel,
    ToggleSound,
    ToggleMotion,
    MineReveal(usize),
    MineFlag(usize),
    MineChord(usize),
    MineRestart,
    MineFlagMode,
    MinePreset(crate::minesweeper::MinePreset),
    SudokuCell(usize),
    SudokuNumber(u8),
    SudokuErase,
    SudokuNoteMode,
    SudokuDifficulty(crate::sudoku::SudokuDifficulty),
    SudokuUndo,
    NonogramCell(usize),
    NonogramMode,
    NonogramUndo,
    NonogramPreset(crate::nonogram::NonogramPreset),
    SolitaireStock,
    SolitaireTableau(usize, usize),
    SolitaireWaste,
    SolitaireFoundation(usize),
    SolitaireUndo,
    SolitaireNew,
    FreeCellCell(usize),
    FreeCellCascade(usize, usize),
    FreeCellFoundation(usize),
    FreeCellUndo,
    FreeCellNew,
    FivefoldRoll,
    FivefoldHold(usize),
    FivefoldCategory(crate::fivefold::Category),
    FivefoldNew,
    ReversiPlace(usize),
    ReversiPass,
    ReversiNew,
    ReversiLevel(crate::reversi::AiLevel),
}
pub fn viewport() -> Viewport {
    Viewport::new(
        screen_width(),
        screen_height(),
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
    )
}
pub fn mouse() -> Vec2 {
    viewport()
        .screen_to_logical(vec2(mouse_position().0, mouse_position().1))
        .unwrap_or(vec2(-1000., -1000.))
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
            if Rect::new(940., 28., 90., 42.).contains(p) {
                out.push(UiAction::Help)
            }
            if Rect::new(1040., 28., 90., 42.).contains(p) {
                out.push(UiAction::Records)
            }
            if Rect::new(1140., 28., 110., 42.).contains(p) {
                out.push(UiAction::Settings)
            }
            out
        }
        Screen::Game(GameId::Game2048) => game_clicks(state, p),
        Screen::Game(GameId::Minesweeper) => mine_clicks(state, p),
        Screen::Game(GameId::Sudoku) => sudoku_ui::sudoku_clicks(state, p),
        Screen::Game(GameId::Nonogram) => nonogram_ui::nonogram_clicks(state, p),
        Screen::Game(GameId::Solitaire) => solitaire_ui::solitaire_clicks(state, p),
        Screen::Game(GameId::FreeCell) => freecell_ui::freecell_clicks(state, p),
        Screen::Game(GameId::Yahtzee) => fivefold_ui::fivefold_clicks(state, p),
        Screen::Game(GameId::Reversi) => reversi_ui::reversi_clicks(state, p),
        Screen::Help => {
            if Rect::new(1030., 635., 180., 48.).contains(p) {
                vec![UiAction::Cabinet]
            } else {
                vec![]
            }
        }
        Screen::Records => records_ui::records_clicks(p),
        Screen::Settings => settings_clicks(p),
    }
}
pub fn draw(state: &AppState, data: &GameData, loaded_assets: usize) {
    match state.screen {
        Screen::Cabinet => draw_cabinet(state, data, loaded_assets),
        Screen::Game(GameId::Game2048) => draw_2048(state),
        Screen::Game(GameId::Minesweeper) => draw_minesweeper(state),
        Screen::Game(GameId::Sudoku) => sudoku_ui::draw_sudoku(state),
        Screen::Game(GameId::Nonogram) => nonogram_ui::draw_nonogram(state),
        Screen::Game(GameId::Solitaire) => solitaire_ui::draw_solitaire(state),
        Screen::Game(GameId::FreeCell) => freecell_ui::draw_freecell(state),
        Screen::Game(GameId::Yahtzee) => fivefold_ui::draw_fivefold(state),
        Screen::Game(GameId::Reversi) => reversi_ui::draw_reversi(state),
        Screen::Help => draw_help(),
        Screen::Records => records_ui::draw_records(state),
        Screen::Settings => draw_settings(state),
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
        let active = matches!(
            GameId::ALL[i],
            GameId::Game2048
                | GameId::Minesweeper
                | GameId::Sudoku
                | GameId::Nonogram
                | GameId::Solitaire
                | GameId::FreeCell
                | GameId::Yahtzee
                | GameId::Reversi
        );
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
    text("HELP", 954., 55., 17., WHITE);
    text("RECORDS", 1048., 55., 17., WHITE);
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
    text("BACK", 340., 521., 17., WHITE);
    panel(
        Rect::new(470., 490., 150., 48.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("SAVE NOW", 500., 521., 16., WHITE);
    panel(
        Rect::new(650., 490., 150., 48.),
        Color::new(0.22, 0.18, 0.35, 1.),
    );
    text("LOAD", 699., 521., 16., WHITE)
}
fn settings_clicks(p: Vec2) -> Vec<UiAction> {
    let mut o = vec![];
    if Rect::new(290., 490., 150., 48.).contains(p) {
        o.push(UiAction::Cabinet)
    }
    if Rect::new(470., 490., 150., 48.).contains(p) {
        o.push(UiAction::Save)
    }
    if Rect::new(650., 490., 150., 48.).contains(p) {
        o.push(UiAction::Load)
    }
    if Rect::new(290., 270., 250., 45.).contains(p) {
        o.push(UiAction::ToggleSound)
    }
    if Rect::new(290., 330., 300., 45.).contains(p) {
        o.push(UiAction::ToggleMotion)
    }
    o
}

fn draw_minesweeper(state: &AppState) {
    let game = &state.minesweeper;
    text("‹ CABINET", 40., 55., 20., Color::new(0.78, 0.70, 0.92, 1.));
    text(
        "MINESWEEPER",
        40.,
        105.,
        42.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    text(
        "Read the quiet field",
        44.,
        132.,
        18.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    let board = Rect::new(350., 155., 450., 450.);
    panel(board, Color::new(0.10, 0.07, 0.16, 1.));
    let cell_size = (board.w - 24.) / game.width as f32;
    for index in 0..game.cells.len() {
        let rect = Rect::new(
            board.x + 12. + (index % game.width) as f32 * cell_size,
            board.y + 12. + (index / game.width) as f32 * cell_size,
            cell_size - 2.,
            cell_size - 2.,
        );
        let cell = game.cells[index];
        let revealed = matches!(cell, Cell::Revealed(value) if value < 9)
            || matches!(game.status, MineStatus::Lost)
                && matches!(cell, Cell::Mine | Cell::FlaggedMine | Cell::Revealed(9));
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if revealed {
                Color::new(0.24, 0.19, 0.30, 1.)
            } else {
                Color::new(0.15, 0.11, 0.23, 1.)
            },
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            Color::new(0.48, 0.40, 0.60, 0.7),
        );
        match cell {
            Cell::Flagged | Cell::FlaggedMine => text(
                "⚑",
                rect.x + 12.,
                rect.y + 31.,
                25.,
                Color::new(0.98, 0.46, 0.38, 1.),
            ),
            Cell::Mine if matches!(game.status, MineStatus::Lost) => text(
                "✹",
                rect.x + 11.,
                rect.y + 31.,
                24.,
                Color::new(0.98, 0.45, 0.32, 1.),
            ),
            Cell::Revealed(value) if value > 0 && value < 9 => text(
                &value.to_string(),
                rect.x + cell_size * 0.35,
                rect.y + cell_size * 0.68,
                (cell_size * 0.48).min(23.),
                Color::new(0.76, 0.90, 1.0, 1.),
            ),
            _ => {}
        }
    }
    text(
        &format!("Mines: {} / {}", game.flagged_count(), game.mines),
        850.,
        215.,
        22.,
        Color::new(0.82, 0.75, 0.90, 1.),
    );
    text(
        &format!("Time: {:03}s", game.elapsed_whole_seconds()),
        850.,
        175.,
        22.,
        Color::new(0.82, 0.75, 0.90, 1.),
    );
    for (index, preset) in MinePreset::ALL.iter().enumerate() {
        let rect = Rect::new(820. + index as f32 * 110., 285., 100., 32.);
        panel(
            rect,
            if *preset == game.preset {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(preset.label(), rect.x + 8., rect.y + 21., 11., WHITE);
    }
    text(
        match game.status {
            MineStatus::Ready => "First reveal is safe",
            MineStatus::Playing => "Find every safe square",
            MineStatus::Won => "Field cleared",
            MineStatus::Lost => "A mine was found",
        },
        850.,
        255.,
        18.,
        Color::new(0.63, 0.95, 0.72, 1.),
    );
    panel(
        Rect::new(850., 320., 170., 52.),
        if state.mine_flag_mode {
            Color::new(0.45, 0.20, 0.27, 1.)
        } else {
            Color::new(0.20, 0.13, 0.30, 1.)
        },
    );
    text(
        if state.mine_flag_mode {
            "FLAG MODE"
        } else {
            "REVEAL MODE"
        },
        875.,
        353.,
        16.,
        WHITE,
    );
    panel(
        Rect::new(850., 390., 170., 52.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("RESTART", 892., 423., 16., WHITE);
    text(
        "Tap a square to reveal or flag it.",
        850.,
        500.,
        16.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
    text(
        "Tap a revealed number after marking its mines to chord.",
        850.,
        525.,
        15.,
        Color::new(0.63, 0.58, 0.72, 1.),
    );
}

fn mine_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(20., 20., 180., 50.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if Rect::new(850., 320., 170., 52.).contains(p) {
        return vec![UiAction::MineFlagMode];
    }
    if Rect::new(850., 390., 170., 52.).contains(p) {
        return vec![UiAction::MineRestart];
    }
    for (index, preset) in MinePreset::ALL.iter().enumerate() {
        if Rect::new(820. + index as f32 * 110., 285., 100., 32.).contains(p) {
            return vec![UiAction::MinePreset(*preset)];
        }
    }
    let board = Rect::new(350., 155., 450., 450.);
    if !board.contains(p) {
        return vec![];
    }
    let cell_size = (board.w - 24.) / state.minesweeper.width as f32;
    let column = ((p.x - board.x - 12.) / cell_size) as usize;
    let row = ((p.y - board.y - 12.) / cell_size) as usize;
    if column >= state.minesweeper.width || row >= state.minesweeper.height {
        return vec![];
    }
    let index = row * state.minesweeper.width + column;
    if state.mine_flag_mode {
        vec![UiAction::MineFlag(index)]
    } else if matches!(state.minesweeper.cells[index], Cell::Revealed(_)) {
        vec![UiAction::MineChord(index)]
    } else {
        vec![UiAction::MineReveal(index)]
    }
}
