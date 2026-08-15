//! Compact portrait cabinet and 2048 layouts.

use crate::{
    cosmetics,
    data::GameData,
    palette_ui,
    state::{AppState, Direction, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

pub const WIDTH: f32 = 360.;
pub const HEIGHT: f32 = 780.;

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
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}

pub fn cabinet_rect(index: usize) -> Rect {
    Rect::new(
        6. + (index % 3) as f32 * 118.,
        94. + (index / 3) as f32 * 59.,
        112.,
        53.,
    )
}

pub fn draw_cabinet(state: &AppState, _data: &GameData, loaded: usize) {
    let accent = cosmetics::cabinet_accent(state.cabinet_decoration);
    text("IDLE HANDS", 18., 55., 30., accent);
    crate::cabinet_art::draw_header_motif(332., 52., 13., accent);
    crate::cabinet_art::draw_shelves(8., 100., 344., 520., accent);
    text(
        "Quiet games for a small screen",
        18.,
        78.,
        14.,
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    for (index, game) in GameId::ALL.iter().enumerate() {
        let rect = cabinet_rect(index);
        panel(rect, Color::new(0.17, 0.12, 0.27, 1.));
        text(
            game.title(),
            rect.x + 10.,
            rect.y + 15.,
            10.,
            Color::new(0.98, 0.82, 0.42, 1.),
        );
        text(
            cabinet_status(state, *game),
            rect.x + 10.,
            rect.y + 31.,
            8.,
            Color::new(0.98, 0.75, 0.30, 1.),
        );
        text(
            game.subtitle(),
            rect.x + 10.,
            rect.y + 47.,
            7.,
            Color::new(0.69, 0.65, 0.78, 1.),
        );
        draw_circle(
            rect.right() - 20.,
            rect.y + 12.,
            8.,
            cosmetics::cabinet_accent(state.cabinet_decoration),
        );
        text(
            &(index + 1).to_string(),
            rect.right() - 22.,
            rect.y + 16.,
            8.,
            Color::new(0.08, 0.05, 0.12, 1.),
        );
    }
    for (rect, label) in [
        (Rect::new(8., 665., 108., 40.), "HELP"),
        (Rect::new(126., 665., 108., 40.), "RECORDS"),
        (Rect::new(244., 665., 108., 40.), "SETTINGS"),
    ] {
        panel(rect, Color::new(0.12, 0.08, 0.20, 1.));
        text(label, rect.x + 12., rect.y + 26., 11., WHITE);
    }
    text(
        &format!("{} stamps  •  {} textures", state.stamps, loaded),
        18.,
        640.,
        12.,
        Color::new(0.52, 0.48, 0.64, 1.),
    );
}

pub fn cabinet_clicks(p: Vec2) -> Vec<UiAction> {
    for index in 0..GameId::ALL.len() {
        if cabinet_rect(index).contains(p) {
            return vec![UiAction::Open(index)];
        }
    }
    for (rect, action) in [
        (Rect::new(8., 665., 108., 40.), UiAction::Help),
        (Rect::new(126., 665., 108., 40.), UiAction::Records),
        (Rect::new(244., 665., 108., 40.), UiAction::Settings),
    ] {
        if rect.contains(p) {
            return vec![action];
        }
    }
    vec![]
}

pub fn draw_2048(state: &AppState) {
    let game = &state.game;
    text("‹ CABINET", 16., 35., 15., Color::new(0.78, 0.70, 0.92, 1.));
    text("2048", 16., 82., 38., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        &format!("Score {}  •  Best {}", game.score, game.best),
        18.,
        108.,
        14.,
        WHITE,
    );
    let board = Rect::new(20., 130., 320., 320.);
    panel(board, crate::accessibility::board_fill(state.high_contrast));
    for index in 0..16 {
        let rect = Rect::new(
            board.x + 8. + (index % 4) as f32 * 78.,
            board.y + 8. + (index / 4) as f32 * 78.,
            72.,
            72.,
        );
        let value = game.cells[index];
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            palette_ui::tile_color(value, state.board_theme),
        );
        if value > 0 {
            let label = value.to_string();
            let size = if value < 100 { 25. } else { 19. };
            let width = measure_text(&label, None, size as u16, 1.).width;
            text(
                &label,
                rect.x + (rect.w - width) / 2.,
                rect.y + 45.,
                size,
                WHITE,
            );
        }
    }
    for (index, direction) in [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ]
    .iter()
    .enumerate()
    {
        let rect = Rect::new(20. + index as f32 * 82., 475., 74., 46.);
        panel(rect, Color::new(0.18, 0.12, 0.28, 1.));
        text(
            ["↑", "←", "↓", "→"][index],
            rect.x + 27.,
            rect.y + 32.,
            24.,
            Color::new(0.98, 0.83, 0.45, 1.),
        );
        let _ = direction;
    }
    panel(
        Rect::new(20., 545., 150., 46.),
        Color::new(0.18, 0.12, 0.28, 1.),
    );
    text("UNDO", 70., 575., 15., WHITE);
    panel(
        Rect::new(190., 545., 150., 46.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("NEW GAME", 220., 575., 14., WHITE);
    text(
        "Swipe the board or tap an arrow.",
        42.,
        635.,
        14.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    if state.confirm_restart {
        panel(
            Rect::new(25., 265., 310., 145.),
            Color::new(0.16, 0.09, 0.20, 1.),
        );
        text("Start a new board?", 58., 305., 20., WHITE);
        panel(
            Rect::new(45., 335., 120., 42.),
            Color::new(0.25, 0.16, 0.32, 1.),
        );
        text("CANCEL", 76., 362., 14., WHITE);
        panel(
            Rect::new(195., 335., 120., 42.),
            Color::new(0.45, 0.22, 0.25, 1.),
        );
        text("START", 235., 362., 14., WHITE);
    }
}

pub fn game2048_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(10., 10., 110., 38.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if state.confirm_restart {
        if Rect::new(45., 335., 120., 42.).contains(p) {
            return vec![UiAction::Cancel];
        }
        if Rect::new(195., 335., 120., 42.).contains(p) {
            return vec![UiAction::ConfirmRestart];
        }
        return vec![];
    }
    if Rect::new(20., 545., 150., 46.).contains(p) && state.game.can_undo() {
        return vec![UiAction::Undo];
    }
    if Rect::new(190., 545., 150., 46.).contains(p) {
        return vec![UiAction::Restart];
    }
    for (index, direction) in [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ]
    .iter()
    .enumerate()
    {
        if Rect::new(20. + index as f32 * 82., 475., 74., 46.).contains(p) {
            return vec![UiAction::Move(*direction)];
        }
    }
    vec![]
}

pub fn draw_settings(state: &AppState) {
    panel(
        Rect::new(8., 30., 344., 700.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text("SETTINGS", 22., 82., 30., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        &format!("Profile: {}", state.profile_name),
        22.,
        120.,
        15.,
        WHITE,
    );
    for (y, label) in [
        (
            165.,
            format!("Card back: {}", cosmetics::card_back_name(state.card_back)),
        ),
        (
            215.,
            format!(
                "Board theme: {}",
                cosmetics::board_theme_name(state.board_theme)
            ),
        ),
        (
            265.,
            format!("Sound set: {}", cosmetics::sound_set_name(state.sound_set)),
        ),
        (
            315.,
            format!(
                "Decoration: {}",
                cosmetics::cabinet_decoration_name(state.cabinet_decoration)
            ),
        ),
    ] {
        panel(
            Rect::new(22., y - 28., 316., 42.),
            Color::new(0.16, 0.11, 0.24, 1.),
        );
        text(&label, 34., y, 14., WHITE);
    }
    panel(
        Rect::new(22., 360., 150., 42.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!("Sound: {}", if state.sound { "On" } else { "Off" }),
        35.,
        387.,
        13.,
        WHITE,
    );
    panel(
        Rect::new(186., 360., 152., 42.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!(
            "Motion: {}",
            if state.reduced_motion {
                "Reduced"
            } else {
                "Full"
            }
        ),
        198.,
        387.,
        13.,
        WHITE,
    );
    panel(
        Rect::new(22., 465., 150., 42.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!(
            "Contrast: {}",
            if state.high_contrast { "On" } else { "Off" }
        ),
        35.,
        492.,
        12.,
        WHITE,
    );
    panel(
        Rect::new(186., 465., 152., 42.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!(
            "Text: {}",
            if state.large_text { "Large" } else { "Normal" }
        ),
        200.,
        492.,
        12.,
        WHITE,
    );
    text(
        "Tap a row to cycle unlocked cosmetics.",
        22.,
        445.,
        13.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    for (rect, label) in [
        (Rect::new(22., 665., 76., 42.), "BACK"),
        (Rect::new(108., 665., 76., 42.), "SAVE"),
        (Rect::new(194., 665., 76., 42.), "LOAD"),
        (Rect::new(280., 665., 58., 42.), "RESET"),
    ] {
        panel(rect, Color::new(0.20, 0.13, 0.30, 1.));
        let width = measure_text(label, None, 11, 1.).width;
        text(
            label,
            rect.x + (rect.w - width) / 2.,
            rect.y + 27.,
            11.,
            WHITE,
        );
    }
    if state.confirm_reset {
        panel(
            Rect::new(20., 485., 320., 145.),
            Color::new(0.16, 0.08, 0.16, 0.99),
        );
        text("Reset the cabinet?", 42., 520., 20., WHITE);
        text(
            "This removes saves and records.",
            42.,
            548.,
            13.,
            Color::new(0.78, 0.73, 0.86, 1.),
        );
        panel(
            Rect::new(42., 570., 120., 40.),
            Color::new(0.22, 0.18, 0.35, 1.),
        );
        text("CANCEL", 75., 596., 12., WHITE);
        panel(
            Rect::new(198., 570., 120., 40.),
            Color::new(0.45, 0.20, 0.24, 1.),
        );
        text("RESET", 238., 596., 12., WHITE);
    }
}

pub fn settings_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if state.confirm_reset {
        if Rect::new(42., 570., 120., 40.).contains(p) {
            return vec![UiAction::CancelResetData];
        }
        if Rect::new(198., 570., 120., 40.).contains(p) {
            return vec![UiAction::ConfirmResetData];
        }
        return vec![];
    }
    if Rect::new(22., 665., 76., 42.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    if Rect::new(108., 665., 76., 42.).contains(p) {
        return vec![UiAction::Save];
    }
    if Rect::new(194., 665., 76., 42.).contains(p) {
        return vec![UiAction::Load];
    }
    if Rect::new(280., 665., 58., 42.).contains(p) {
        return vec![UiAction::ResetData];
    }
    for (rect, action) in [
        (Rect::new(22., 137., 316., 42.), UiAction::CycleCardBack),
        (Rect::new(22., 187., 316., 42.), UiAction::CycleBoardTheme),
        (Rect::new(22., 237., 316., 42.), UiAction::CycleSoundSet),
        (
            Rect::new(22., 287., 316., 42.),
            UiAction::CycleCabinetDecoration,
        ),
        (Rect::new(22., 360., 150., 42.), UiAction::ToggleSound),
        (Rect::new(186., 360., 152., 42.), UiAction::ToggleMotion),
        (
            Rect::new(22., 465., 150., 42.),
            UiAction::ToggleHighContrast,
        ),
        (Rect::new(186., 465., 152., 42.), UiAction::ToggleLargeText),
    ] {
        if rect.contains(p) {
            return vec![action];
        }
    }
    vec![]
}

pub fn draw_sudoku(state: &AppState) {
    let game = &state.sudoku;
    text("‹ CABINET", 10., 30., 14., Color::new(0.78, 0.70, 0.92, 1.));
    text("SUDOKU", 12., 78., 34., Color::new(0.98, 0.83, 0.45, 1.));
    for (index, difficulty) in crate::sudoku::SudokuDifficulty::ALL.iter().enumerate() {
        let rect = Rect::new(148. + index as f32 * 68., 48., 62., 28.);
        panel(
            rect,
            if *difficulty == game.difficulty {
                Color::new(0.45, 0.25, 0.42, 1.)
            } else {
                Color::new(0.16, 0.11, 0.24, 1.)
            },
        );
        text(difficulty.label(), rect.x + 8., rect.y + 19., 10., WHITE);
    }
    let board = Rect::new(10., 100., 340., 340.);
    panel(board, crate::accessibility::board_fill(state.high_contrast));
    let cell = 36.8;
    for index in 0..81 {
        let row = index / 9;
        let col = index % 9;
        let rect = Rect::new(
            board.x + 4. + col as f32 * cell,
            board.y + 4. + row as f32 * cell,
            cell - 1.,
            cell - 1.,
        );
        let selected = game.selected == Some(index);
        let conflict = game
            .selected
            .map(|selected| game.conflicts(selected).contains(&index))
            .unwrap_or(false);
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if conflict {
                Color::new(0.35, 0.13, 0.20, 1.)
            } else if selected {
                Color::new(0.30, 0.22, 0.42, 1.)
            } else {
                Color::new(0.15, 0.11, 0.23, 1.)
            },
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if col % 3 == 0 || row % 3 == 0 { 2. } else { 1. },
            crate::accessibility::grid_line(state.high_contrast),
        );
        if game.values[index] != 0 {
            text(
                &game.values[index].to_string(),
                rect.x + 12.,
                rect.y + 26.,
                crate::accessibility::text_size(21., state.large_text),
                if game.is_given(index) {
                    WHITE
                } else {
                    Color::new(0.98, 0.72, 0.38, 1.)
                },
            );
        }
    }
    text(
        if state.sudoku_note_mode {
            "PENCIL"
        } else {
            "NUMBER PAD"
        },
        12.,
        465.,
        13.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    for number in 1..=9 {
        let col = (number - 1) % 3;
        let row = (number - 1) / 3;
        let rect = Rect::new(12. + col as f32 * 114., 480. + row as f32 * 52., 104., 44.);
        panel(rect, Color::new(0.20, 0.13, 0.30, 1.));
        text(&number.to_string(), rect.x + 46., rect.y + 29., 20., WHITE);
    }
    panel(
        Rect::new(12., 650., 104., 42.),
        Color::new(0.45, 0.25, 0.42, 1.),
    );
    text("PENCIL", 35., 677., 13., WHITE);
    panel(
        Rect::new(128., 650., 104., 42.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("ERASE", 158., 677., 13., WHITE);
    panel(
        Rect::new(244., 650., 104., 42.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("UNDO", 276., 677., 13., WHITE);
    text(
        &format!(
            "Moves {}  •  Best {}",
            game.moves,
            game.best_moves.map_or("—".into(), |v| v.to_string())
        ),
        12.,
        730.,
        12.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
}

pub fn sudoku_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if Rect::new(0., 0., 110., 42.).contains(p) {
        return vec![UiAction::Cabinet];
    }
    let board = Rect::new(10., 100., 340., 340.);
    if board.contains(p) {
        let col = ((p.x - board.x - 4.) / 36.8).floor() as usize;
        let row = ((p.y - board.y - 4.) / 36.8).floor() as usize;
        if col < 9 && row < 9 {
            return vec![UiAction::SudokuCell(row * 9 + col)];
        }
    }
    for (index, difficulty) in crate::sudoku::SudokuDifficulty::ALL.iter().enumerate() {
        if Rect::new(148. + index as f32 * 68., 48., 62., 28.).contains(p) {
            return vec![UiAction::SudokuDifficulty(*difficulty)];
        }
    }
    for number in 1..=9 {
        let col = (number - 1) % 3;
        let row = (number - 1) / 3;
        if Rect::new(12. + col as f32 * 114., 480. + row as f32 * 52., 104., 44.).contains(p) {
            return vec![UiAction::SudokuNumber(number as u8)];
        }
    }
    if Rect::new(12., 650., 104., 42.).contains(p) {
        return vec![UiAction::SudokuNoteMode];
    }
    if Rect::new(128., 650., 104., 42.).contains(p) {
        return vec![UiAction::SudokuErase];
    }
    if Rect::new(244., 650., 104., 42.).contains(p) {
        return vec![UiAction::SudokuUndo];
    }
    let _ = state;
    vec![]
}

pub fn tutorial_clicks(p: Vec2) -> Vec<UiAction> {
    if Rect::new(105., 535., 150., 50.).contains(p) {
        vec![UiAction::TutorialContinue]
    } else {
        vec![]
    }
}

pub fn draw_tutorial(game: GameId) {
    panel(
        Rect::new(15., 145., 330., 440.),
        Color::new(0.07, 0.045, 0.13, 0.98),
    );
    text(
        "HOW TO PLAY",
        35.,
        205.,
        25.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    text(game.title(), 35., 245., 21., WHITE);
    text(
        "Use the visible controls",
        35.,
        300.,
        16.,
        Color::new(0.78, 0.73, 0.86, 1.),
    );
    text(
        "and tap CONTINUE below",
        35.,
        335.,
        16.,
        Color::new(0.78, 0.73, 0.86, 1.),
    );
    panel(
        Rect::new(105., 535., 150., 50.),
        Color::new(0.25, 0.45, 0.34, 1.),
    );
    text("CONTINUE", 140., 567., 15., WHITE);
}

pub fn draw_replay_button() {
    panel(
        Rect::new(245., 10., 105., 38.),
        Color::new(0.16, 0.11, 0.25, 0.96),
    );
    text("TUTORIAL", 258., 34., 11., WHITE);
}

pub fn replay_clicks(p: Vec2) -> bool {
    Rect::new(245., 10., 105., 38.).contains(p)
}

fn cabinet_status(state: &AppState, game: GameId) -> &'static str {
    match game {
        GameId::Game2048 if state.records.best_2048 >= 2048 => "COMPLETE",
        GameId::Minesweeper if state.records.minesweeper.iter().any(Option::is_some) => "COMPLETE",
        GameId::Sudoku if state.records.sudoku.iter().any(Option::is_some) => "COMPLETE",
        GameId::Nonogram if state.records.nonogram.iter().any(Option::is_some) => "COMPLETE",
        GameId::Solitaire if state.records.solitaire_best_moves.is_some() => "COMPLETE",
        GameId::FreeCell if state.records.freecell_best_moves.is_some() => "COMPLETE",
        GameId::Yahtzee if state.records.fivefold_best_total > 0 => "COMPLETE",
        GameId::Reversi if state.records.reversi_best_score > 0 => "COMPLETE",
        GameId::LightsOut if state.records.lights_out_best_moves.is_some() => "COMPLETE",
        GameId::TicTacToe if state.records.tic_tac_toe_best_moves.is_some() => "COMPLETE",
        GameId::MemoryPairs if state.records.memory_pairs_best_moves.is_some() => "COMPLETE",
        GameId::SlidingPuzzle if state.records.sliding_puzzle_best_moves.is_some() => "COMPLETE",
        GameId::Mastermind if state.records.mastermind_best_rows.is_some() => "COMPLETE",
        GameId::Spider if state.records.spider_best_moves.is_some() => "COMPLETE",
        GameId::WordSearch if state.records.word_search_best_moves.is_some() => "COMPLETE",
        GameId::Hangman if state.records.hangman_best_moves.is_some() => "COMPLETE",
        GameId::ConnectFour if state.records.connect_four_best_moves.is_some() => "COMPLETE",
        GameId::Checkers if state.records.checkers_best_moves.is_some() => "COMPLETE",
        GameId::PegSolitaire if state.records.peg_solitaire_best_moves.is_some() => "COMPLETE",
        GameId::MahjongSolitaire if state.records.mahjong_solitaire_best_moves.is_some() => {
            "COMPLETE"
        }
        GameId::Snake if state.records.snake_best_score.is_some() => "COMPLETE",
        GameId::Breakout if state.records.breakout_best_score.is_some() => "COMPLETE",
        GameId::HigherLower if state.records.higher_lower_best_score.is_some() => "COMPLETE",
        GameId::KlondikeGolf if state.records.klondike_golf_best_moves.is_some() => "COMPLETE",
        GameId::Blackjack if state.records.blackjack_best_wins.is_some() => "COMPLETE",
        GameId::SpiderSolitaire if state.records.spider_solitaire_best_moves.is_some() => {
            "COMPLETE"
        }
        GameId::DungeonSweeper if state.records.dungeon_sweeper_best_moves.is_some() => "COMPLETE",
        _ => "PLAY NOW",
    }
}
