//! Touch-first cabinet and 2048 presentation.

use crate::cosmetics;
use crate::fivefold_ui;
use crate::freecell_ui;
use crate::grid::GridLayout;
use crate::input::Viewport;
use crate::library_ui;
use crate::minesweeper_ui;
use crate::nonogram_ui;
use crate::palette_ui;
use crate::records_ui;
use crate::responsive_puzzles;
use crate::responsive_ui;
use crate::reversi_ui;
use crate::settings_ui;
use crate::solitaire_ui;
use crate::sudoku_ui;
use crate::tutorial_ui;
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
    Rules,
    Credits,
    ResetData,
    ConfirmResetData,
    CancelResetData,
    TutorialContinue,
    ReplayTutorial,
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
    CycleCardBack,
    CycleBoardTheme,
    CycleSoundSet,
    CycleCabinetDecoration,
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
    let (width, height) = layout_size();
    Viewport::new(screen_width(), screen_height(), width, height)
}
pub fn layout_size() -> (f32, f32) {
    if is_portrait() {
        (responsive_ui::WIDTH, responsive_ui::HEIGHT)
    } else {
        (LOGICAL_WIDTH, LOGICAL_HEIGHT)
    }
}
pub fn is_portrait() -> bool {
    screen_height() > screen_width() * 1.15
}
pub fn mouse() -> Vec2 {
    viewport()
        .screen_to_logical(vec2(mouse_position().0, mouse_position().1))
        .unwrap_or(vec2(-1000., -1000.))
}
pub fn clicks(state: &AppState) -> Vec<UiAction> {
    let p = mouse();
    if state.tutorial.is_some() {
        if is_portrait() {
            return responsive_ui::tutorial_clicks(p);
        }
        return tutorial_ui::clicks(p);
    }
    if matches!(state.screen, Screen::Game(_)) {
        if (is_portrait() && responsive_ui::replay_clicks(p))
            || (!is_portrait() && tutorial_ui::REPLAY_RECT.contains(p))
        {
            return vec![UiAction::ReplayTutorial];
        }
    }
    match state.screen {
        Screen::Cabinet if is_portrait() => responsive_ui::cabinet_clicks(p),
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
        Screen::Game(GameId::Game2048) if is_portrait() => responsive_ui::game2048_clicks(state, p),
        Screen::Game(GameId::Game2048) => game_clicks(state, p),
        Screen::Game(GameId::Minesweeper) if is_portrait() => {
            responsive_puzzles::minesweeper_clicks(state, p)
        }
        Screen::Game(GameId::Minesweeper) => minesweeper_ui::clicks(state, p),
        Screen::Game(GameId::Sudoku) if is_portrait() => responsive_ui::sudoku_clicks(state, p),
        Screen::Game(GameId::Sudoku) => sudoku_ui::sudoku_clicks(state, p),
        Screen::Game(GameId::Nonogram) if is_portrait() => {
            responsive_puzzles::nonogram_clicks(state, p)
        }
        Screen::Game(GameId::Nonogram) => nonogram_ui::nonogram_clicks(state, p),
        Screen::Game(GameId::Solitaire) => solitaire_ui::solitaire_clicks(state, p),
        Screen::Game(GameId::FreeCell) => freecell_ui::freecell_clicks(state, p),
        Screen::Game(GameId::Yahtzee) => fivefold_ui::fivefold_clicks(state, p),
        Screen::Game(GameId::Reversi) => reversi_ui::reversi_clicks(state, p),
        Screen::Help => {
            if Rect::new(1030., 635., 180., 48.).contains(p) {
                vec![UiAction::Cabinet]
            } else if Rect::new(600., 635., 180., 48.).contains(p) {
                vec![UiAction::Rules]
            } else if Rect::new(800., 635., 180., 48.).contains(p) {
                vec![UiAction::Credits]
            } else {
                vec![]
            }
        }
        Screen::Records => records_ui::records_clicks(p),
        Screen::Rules => library_ui::rules_clicks(p),
        Screen::Credits => library_ui::credits_clicks(p),
        Screen::Settings if is_portrait() => responsive_ui::settings_clicks(state, p),
        Screen::Settings => settings_ui::settings_clicks(state, p),
    }
}
pub fn draw(state: &AppState, data: &GameData, loaded_assets: usize) {
    match state.screen {
        Screen::Cabinet if is_portrait() => responsive_ui::draw_cabinet(state, data, loaded_assets),
        Screen::Cabinet => draw_cabinet(state, data, loaded_assets),
        Screen::Game(GameId::Game2048) if is_portrait() => responsive_ui::draw_2048(state),
        Screen::Game(GameId::Game2048) => draw_2048(state),
        Screen::Game(GameId::Minesweeper) if is_portrait() => {
            responsive_puzzles::draw_minesweeper(state)
        }
        Screen::Game(GameId::Minesweeper) => minesweeper_ui::draw(state),
        Screen::Game(GameId::Sudoku) if is_portrait() => responsive_ui::draw_sudoku(state),
        Screen::Game(GameId::Sudoku) => sudoku_ui::draw_sudoku(state),
        Screen::Game(GameId::Nonogram) if is_portrait() => responsive_puzzles::draw_nonogram(state),
        Screen::Game(GameId::Nonogram) => nonogram_ui::draw_nonogram(state),
        Screen::Game(GameId::Solitaire) => solitaire_ui::draw_solitaire(state),
        Screen::Game(GameId::FreeCell) => freecell_ui::draw_freecell(state),
        Screen::Game(GameId::Yahtzee) => fivefold_ui::draw_fivefold(state),
        Screen::Game(GameId::Reversi) => reversi_ui::draw_reversi(state),
        Screen::Help => draw_help(),
        Screen::Records => records_ui::draw_records(state),
        Screen::Rules => library_ui::draw_rules(),
        Screen::Credits => library_ui::draw_credits(),
        Screen::Settings if is_portrait() => responsive_ui::draw_settings(state),
        Screen::Settings => settings_ui::draw_settings(state),
    }
    if let Some(game) = state.tutorial {
        if is_portrait() {
            responsive_ui::draw_tutorial(game);
        } else {
            tutorial_ui::draw_overlay(game);
        }
    } else if matches!(state.screen, Screen::Game(_)) {
        if is_portrait() {
            responsive_ui::draw_replay_button();
        } else {
            tutorial_ui::draw_replay_button();
        }
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
        cosmetics::cabinet_accent(state.cabinet_decoration),
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
            "{}  •  {} stamps  •  {} games waiting at the cabinet",
            state.profile_name,
            state.stamps,
            GameId::ALL.len()
        ),
        48.,
        130.,
        18.,
        Color::new(0.60, 0.56, 0.72, 1.),
    );
    text(
        &format!(
            "{}  •  {}",
            cosmetics::cabinet_decoration_name(state.cabinet_decoration),
            cosmetics::board_theme_name(state.board_theme)
        ),
        990.,
        686.,
        15.,
        cosmetics::cabinet_accent(state.cabinet_decoration),
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
        let status = cabinet_status(state, GameId::ALL[i]);
        text(
            status,
            r.x + 18.,
            r.y + 70.,
            14.,
            cabinet_status_color(status),
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
fn cabinet_status(state: &AppState, game: GameId) -> &'static str {
    let complete = match game {
        GameId::Game2048 => state.records.best_2048 >= 2048,
        GameId::Minesweeper => state.records.minesweeper.iter().any(Option::is_some),
        GameId::Sudoku => state.records.sudoku.iter().any(Option::is_some),
        GameId::Nonogram => state.records.nonogram.iter().any(Option::is_some),
        GameId::Solitaire => state.records.solitaire_best_moves.is_some(),
        GameId::FreeCell => state.records.freecell_best_moves.is_some(),
        GameId::Yahtzee => state.records.fivefold_best_total > 0,
        GameId::Reversi => state.records.reversi_best_score > 0,
    };
    if complete {
        "COMPLETE"
    } else if cabinet_has_progress(state, game) {
        "IN PROGRESS"
    } else {
        "PLAY NOW"
    }
}
fn cabinet_has_progress(state: &AppState, game: GameId) -> bool {
    match game {
        GameId::Game2048 => state.game.score > 0 || state.game.best > 0,
        GameId::Minesweeper => state.minesweeper.status != MineStatus::Ready,
        GameId::Sudoku => state.sudoku.moves > 0,
        GameId::Nonogram => state.nonogram.moves > 0,
        GameId::Solitaire => state.solitaire.moves > 0,
        GameId::FreeCell => state.freecell.moves > 0,
        GameId::Yahtzee => state.fivefold.roll_number > 0,
        GameId::Reversi => state.reversi.moves > 0,
    }
}
fn cabinet_status_color(status: &str) -> Color {
    match status {
        "COMPLETE" => Color::new(0.55, 1., 0.72, 1.),
        "IN PROGRESS" => Color::new(0.98, 0.75, 0.30, 1.),
        _ => Color::new(0.98, 0.75, 0.30, 1.),
    }
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
        Rect::new(600., 635., 180., 48.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("RULES", 660., 666., 18., WHITE);
    panel(
        Rect::new(800., 635., 180., 48.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("CREDITS", 850., 666., 18., WHITE);
    panel(
        Rect::new(1030., 635., 180., 48.),
        Color::new(0.25, 0.16, 0.32, 1.),
    );
    text("BACK", 1090., 666., 18., WHITE)
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
    let grid = GridLayout::new(
        Rect::new(board.x + 12., board.y + 12., board.w - 24., board.h - 24.),
        game.width,
        game.height,
    );
    let cell_size = grid.cell_width;
    for index in 0..game.cells.len() {
        let cell = grid.cell_rect(index).unwrap();
        let rect = Rect::new(cell.x, cell.y, cell.w - 2., cell.h - 2.);
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
    let grid = GridLayout::new(
        Rect::new(board.x + 12., board.y + 12., board.w - 24., board.h - 24.),
        state.minesweeper.width,
        state.minesweeper.height,
    );
    let Some(index) = grid.index_at(p) else {
        return vec![];
    };
    if state.mine_flag_mode {
        vec![UiAction::MineFlag(index)]
    } else if matches!(state.minesweeper.cells[index], Cell::Revealed(_)) {
        vec![UiAction::MineChord(index)]
    } else {
        vec![UiAction::MineReveal(index)]
    }
}
