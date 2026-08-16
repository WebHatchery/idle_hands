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

const CABINET_COLUMNS: usize = 2;
const CABINET_VISIBLE_ROWS: usize = 6;
const CABINET_PAGE_SIZE: usize = CABINET_COLUMNS * CABINET_VISIBLE_ROWS;

pub fn cabinet_rect(index: usize) -> Rect {
    Rect::new(
        6. + (index % CABINET_COLUMNS) as f32 * 174.,
        112. + (index / CABINET_COLUMNS) as f32 * 78.,
        168.,
        72.,
    )
}

pub fn draw_cabinet(state: &AppState, _data: &GameData, loaded: usize) {
    let accent = cosmetics::cabinet_accent(state.cabinet_decoration);
    text("IDLE HANDS", 10., 32., 24., accent);
    crate::cabinet_art::draw_header_motif(336., 30., 11., accent);
    crate::cabinet_art::draw_shelves(8., 118., 344., 460., accent);
    text(
        "Tap a title to play  -  FAV circle to star",
        8.,
        105.,
        10.,
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    let selected = GameId::ALL[state.selected.min(GameId::ALL.len() - 1)];
    for (rect, _, filter) in filter_buttons() {
        panel(
            rect,
            if state.cabinet_filter == filter {
                Color::new(0.35, 0.22, 0.42, 1.)
            } else {
                Color::new(0.20, 0.13, 0.30, 1.)
            },
        );
        text(
            &crate::cabinet_status::filter_label(state, filter),
            rect.x + 5.,
            rect.y + 29.,
            8.,
            WHITE,
        );
        if state.cabinet_filter == filter {
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3., WHITE);
        }
    }
    panel(
        Rect::new(184., 6., 140., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("CONTINUE", 196., 25., 11., WHITE);
    text(
        selected.title(),
        196.,
        42.,
        10.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    for (index, game) in scrolled_games(state).iter().enumerate() {
        let rect = cabinet_rect(index);
        panel(rect, Color::new(0.17, 0.12, 0.27, 1.));
        draw_line(
            rect.right() - 44.,
            rect.y,
            rect.right() - 44.,
            rect.bottom(),
            2.,
            Color::new(0.45, 0.38, 0.65, 0.65),
        );
        text(
            game.title(),
            rect.x + 8.,
            rect.y + 22.,
            if matches!(
                game,
                GameId::TinyTowerDefence
                    | GameId::OneRoomRoguelike
                    | GameId::DailyDungeon
                    | GameId::DotsBoxes
                    | GameId::Sokoban
                    | GameId::Mancala
                    | GameId::Hanoi
                    | GameId::NumberMatch
                    | GameId::FloodIt
                    | GameId::ColorSort
                    | GameId::Battleship
                    | GameId::WordGrid
                    | GameId::PipeLoop
                    | GameId::MazeWalk
                    | GameId::MatchThree
                    | GameId::Pyramid
                    | GameId::TriPeaks
                    | GameId::Nim
            ) {
                11.
            } else {
                13.
            },
            Color::new(0.98, 0.82, 0.42, 1.),
        );
        let favorite = state.favorites.get(game.index()).copied().unwrap_or(false);
        if favorite {
            draw_circle(rect.right() - 22., rect.y + 52., 7., accent);
        } else {
            draw_circle_lines(rect.right() - 22., rect.y + 52., 7., 2., accent);
        }
        text(
            cabinet_status(state, *game),
            rect.x + 8.,
            rect.y + 61.,
            10.,
            Color::new(0.98, 0.75, 0.30, 1.),
        );
        text(
            game.subtitle(),
            rect.x + 8.,
            rect.y + 43.,
            9.,
            Color::new(0.69, 0.65, 0.78, 1.),
        );
        draw_circle(
            rect.right() - 22.,
            rect.y + 19.,
            11.,
            cosmetics::cabinet_accent(state.cabinet_decoration),
        );
        text(
            &crate::cabinet_status::drawer_number(*game).to_string(),
            rect.right() - 26.,
            rect.y + 23.,
            9.,
            Color::new(0.08, 0.05, 0.12, 1.),
        );
    }
    if visible_games(state).is_empty() {
        text(
            crate::cabinet_status::empty_filter_message(state.cabinet_filter),
            18.,
            160.,
            14.,
            WHITE,
        );
    }
    for (rect, label) in [
        (Rect::new(6., 586., 100., 44.), "PREV"),
        (Rect::new(254., 586., 100., 44.), "NEXT"),
    ] {
        panel(rect, Color::new(0.18, 0.12, 0.28, 1.));
        text(label, rect.x + 18., rect.y + 28., 11., WHITE);
    }
    let (first, total) = cabinet_range(state);
    text(
        &format!(
            "{}-{} OF {}",
            first + usize::from(total > 0),
            (first + CABINET_PAGE_SIZE).min(total),
            total
        ),
        126.,
        614.,
        11.,
        Color::new(0.78, 0.73, 0.86, 1.),
    );
    for (rect, label) in [
        (Rect::new(6., 714., 110., 44.), "HELP"),
        (Rect::new(125., 714., 110., 44.), "RECORDS"),
        (Rect::new(244., 714., 110., 44.), "SETTINGS"),
    ] {
        panel(rect, Color::new(0.12, 0.08, 0.20, 1.));
        text(label, rect.x + 12., rect.y + 29., 11., WHITE);
    }
    panel(
        Rect::new(6., 650., 110., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("FAVORITES", 14., 678., 10., WHITE);
    text(
        &state
            .favorites
            .iter()
            .filter(|favorite| **favorite)
            .count()
            .to_string(),
        94.,
        678.,
        9.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    panel(
        Rect::new(125., 650., 110., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("RECENT", 137., 678., 10., WHITE);
    text(
        &state.recent_games.len().to_string(),
        213.,
        678.,
        9.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    text(
        &format!("{} stamps  -  {} textures", state.stamps, loaded),
        244.,
        672.,
        9.,
        Color::new(0.52, 0.48, 0.64, 1.),
    );
}

pub fn cabinet_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(184., 6., 140., 44.), p) {
        return vec![UiAction::ContinueGame];
    }
    for (index, game) in scrolled_games(state).iter().enumerate() {
        if cabinet_favorite_rect(index).contains(p) {
            return vec![UiAction::ToggleFavorite(game.index())];
        }
        if cabinet_open_rect(index).contains(p) {
            return vec![UiAction::Open(game.index())];
        }
    }
    for (rect, _, filter) in filter_buttons() {
        if rect.contains(p) {
            return vec![UiAction::CabinetFilter(filter)];
        }
    }
    if crate::ui::hit(Rect::new(6., 586., 100., 44.), p) {
        return vec![UiAction::CabinetScroll(-1)];
    }
    if crate::ui::hit(Rect::new(254., 586., 100., 44.), p) {
        return vec![UiAction::CabinetScroll(1)];
    }
    if crate::ui::hit(Rect::new(6., 650., 110., 44.), p) {
        return vec![UiAction::Favorites];
    }
    if crate::ui::hit(Rect::new(125., 650., 110., 44.), p) {
        return vec![UiAction::Recent];
    }
    for (rect, action) in [
        (Rect::new(6., 714., 110., 44.), UiAction::Help),
        (Rect::new(125., 714., 110., 44.), UiAction::Records),
        (Rect::new(244., 714., 110., 44.), UiAction::Settings),
    ] {
        if rect.contains(p) {
            return vec![action];
        }
    }
    vec![]
}

fn cabinet_favorite_rect(index: usize) -> Rect {
    let rect = cabinet_rect(index);
    Rect::new(rect.right() - 44., rect.y, 44., rect.h)
}

fn cabinet_open_rect(index: usize) -> Rect {
    let rect = cabinet_rect(index);
    Rect::new(rect.x, rect.y, rect.w - 44., rect.h)
}

fn filter_buttons() -> [(Rect, &'static str, u8); 3] {
    [
        (Rect::new(6., 52., 54., 44.), "ALL", 0),
        (Rect::new(66., 52., 54., 44.), "OPEN", 1),
        (Rect::new(126., 52., 54., 44.), "DONE", 2),
    ]
}
fn visible_games(state: &AppState) -> Vec<GameId> {
    GameId::ALL
        .iter()
        .copied()
        .filter(|game| crate::cabinet_status::matches_filter(state, *game, state.cabinet_filter))
        .collect()
}

fn cabinet_range(state: &AppState) -> (usize, usize) {
    let total = visible_games(state).len();
    let rows = total.div_ceil(CABINET_COLUMNS);
    let max_scroll = rows.saturating_sub(CABINET_VISIBLE_ROWS);
    (
        state.cabinet_scroll.min(max_scroll) * CABINET_COLUMNS,
        total,
    )
}

fn scrolled_games(state: &AppState) -> Vec<GameId> {
    let games = visible_games(state);
    let (first, _) = cabinet_range(state);
    games
        .into_iter()
        .skip(first)
        .take(CABINET_PAGE_SIZE)
        .collect()
}

pub fn draw_2048(state: &AppState) {
    let game = &state.game;
    panel(
        Rect::new(0., 0., 120., 48.),
        Color::new(0.12, 0.08, 0.20, 1.),
    );
    text("‹ CABINET", 16., 35., 15., Color::new(0.78, 0.70, 0.92, 1.));
    text("2048", 16., 82., 38., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        &format!("Score {}  -  Best {}", game.score, game.best),
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
            ["UP", "LEFT", "DOWN", "RIGHT"][index],
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
    panel(
        Rect::new(20., 600., 150., 46.),
        Color::new(0.18, 0.12, 0.28, 1.),
    );
    text("HINT", 70., 630., 15., WHITE);
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or("Swipe the board or tap an arrow."),
        18.,
        685.,
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
            Rect::new(45., 335., 120., 44.),
            Color::new(0.25, 0.16, 0.32, 1.),
        );
        text("CANCEL", 76., 364., 14., WHITE);
        panel(
            Rect::new(195., 335., 120., 44.),
            Color::new(0.45, 0.22, 0.25, 1.),
        );
        text("START", 235., 364., 14., WHITE);
    }
}

pub fn game2048_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 120., 48.), p) {
        return vec![UiAction::Cabinet];
    }
    if state.confirm_restart {
        if crate::ui::hit(Rect::new(45., 335., 120., 44.), p) {
            return vec![UiAction::Cancel];
        }
        if crate::ui::hit(Rect::new(195., 335., 120., 44.), p) {
            return vec![UiAction::ConfirmRestart];
        }
        return vec![];
    }
    if crate::ui::hit(Rect::new(20., 545., 150., 46.), p) && state.game.can_undo() {
        return vec![UiAction::Undo];
    }
    if crate::ui::hit(Rect::new(190., 545., 150., 46.), p) {
        return vec![UiAction::Restart];
    }
    if crate::ui::hit(Rect::new(20., 600., 150., 46.), p) {
        return vec![UiAction::Game2048Hint];
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
        if crate::ui::hit(Rect::new(20. + index as f32 * 82., 475., 74., 46.), p) {
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
            Rect::new(22., y - 28., 316., 44.),
            Color::new(0.16, 0.11, 0.24, 1.),
        );
        text(&label, 34., y, 14., WHITE);
    }
    panel(
        Rect::new(22., 360., 150., 44.),
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
        Rect::new(186., 360., 152., 44.),
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
        Rect::new(22., 465., 150., 44.),
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
        Rect::new(186., 465., 152., 44.),
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
        (Rect::new(22., 665., 76., 44.), "BACK"),
        (Rect::new(108., 665., 76., 44.), "SAVE"),
        (Rect::new(194., 665., 76., 44.), "LOAD"),
        (Rect::new(280., 665., 58., 44.), "RESET"),
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
            Rect::new(42., 568., 120., 44.),
            Color::new(0.22, 0.18, 0.35, 1.),
        );
        text("CANCEL", 75., 596., 12., WHITE);
        panel(
            Rect::new(198., 568., 120., 44.),
            Color::new(0.45, 0.20, 0.24, 1.),
        );
        text("RESET", 238., 596., 12., WHITE);
    }
}

pub fn settings_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if state.confirm_reset {
        if crate::ui::hit(Rect::new(42., 568., 120., 44.), p) {
            return vec![UiAction::CancelResetData];
        }
        if crate::ui::hit(Rect::new(198., 568., 120., 44.), p) {
            return vec![UiAction::ConfirmResetData];
        }
        return vec![];
    }
    if crate::ui::hit(Rect::new(22., 665., 76., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(Rect::new(108., 665., 76., 44.), p) {
        return vec![UiAction::Save];
    }
    if crate::ui::hit(Rect::new(194., 665., 76., 44.), p) {
        return vec![UiAction::Load];
    }
    if crate::ui::hit(Rect::new(280., 665., 58., 44.), p) {
        return vec![UiAction::ResetData];
    }
    for (rect, action) in [
        (Rect::new(22., 137., 316., 44.), UiAction::CycleCardBack),
        (Rect::new(22., 187., 316., 44.), UiAction::CycleBoardTheme),
        (Rect::new(22., 237., 316., 44.), UiAction::CycleSoundSet),
        (
            Rect::new(22., 287., 316., 44.),
            UiAction::CycleCabinetDecoration,
        ),
        (Rect::new(22., 360., 150., 44.), UiAction::ToggleSound),
        (Rect::new(186., 360., 152., 44.), UiAction::ToggleMotion),
        (
            Rect::new(22., 465., 150., 44.),
            UiAction::ToggleHighContrast,
        ),
        (Rect::new(186., 465., 152., 44.), UiAction::ToggleLargeText),
    ] {
        if rect.contains(p) {
            return vec![action];
        }
    }
    vec![]
}

pub fn tutorial_clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(105., 535., 150., 50.), p) {
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
        Rect::new(245., 10., 105., 44.),
        Color::new(0.16, 0.11, 0.25, 0.96),
    );
    text("TUTORIAL", 258., 38., 11., WHITE);
}

pub fn replay_clicks(p: Vec2) -> bool {
    crate::ui::hit(Rect::new(245., 10., 105., 44.), p)
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
        GameId::Potion2048 if state.records.potion_2048_best_score.is_some() => "COMPLETE",
        GameId::TinyTowerDefence if state.records.tiny_tower_defence_best_wave.is_some() => {
            "COMPLETE"
        }
        GameId::OneRoomRoguelike if state.records.one_room_roguelike_best_score.is_some() => {
            "COMPLETE"
        }
        GameId::DailyDungeon if state.records.daily_dungeon_best_score.is_some() => "COMPLETE",
        GameId::DotsBoxes if state.records.dots_boxes_best_score.is_some() => "COMPLETE",
        GameId::Sokoban if state.records.sokoban_best_moves.is_some() => "COMPLETE",
        GameId::Mancala if state.records.mancala_best_score.is_some() => "COMPLETE",
        GameId::Hanoi if state.records.hanoi_best_moves.is_some() => "COMPLETE",
        GameId::NumberMatch if state.records.number_match_best_moves.is_some() => "COMPLETE",
        GameId::FloodIt if state.records.flood_it_best_moves.is_some() => "COMPLETE",
        GameId::ColorSort if state.records.color_sort_best_moves.is_some() => "COMPLETE",
        GameId::Battleship if state.records.battleship_best_moves.is_some() => "COMPLETE",
        GameId::WordGrid if state.records.word_grid_best_moves.is_some() => "COMPLETE",
        GameId::PipeLoop if state.records.pipe_loop_best_moves.is_some() => "COMPLETE",
        GameId::MazeWalk if state.records.maze_walk_best_moves.is_some() => "COMPLETE",
        GameId::MatchThree if state.records.match_three_best_score.is_some() => "COMPLETE",
        GameId::Pyramid if state.records.pyramid_best_moves.is_some() => "COMPLETE",
        GameId::TriPeaks if state.records.tri_peaks_best_moves.is_some() => "COMPLETE",
        GameId::Nim if state.records.nim_best_moves.is_some() => "COMPLETE",
        _ => "PLAY NOW",
    }
}
