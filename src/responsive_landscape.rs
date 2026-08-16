//! Medium landscape layouts for short touch screens.

use crate::{
    cosmetics,
    data::GameData,
    palette_ui,
    state::{AppState, Direction, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

pub const WIDTH: f32 = 844.;
pub const HEIGHT: f32 = 390.;

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

const CABINET_COLUMNS: usize = 6;
const CABINET_VISIBLE_ROWS: usize = 5;
const CABINET_PAGE_SIZE: usize = CABINET_COLUMNS * CABINET_VISIBLE_ROWS;

fn cabinet_rect(index: usize) -> Rect {
    Rect::new(
        8. + (index % CABINET_COLUMNS) as f32 * 139.,
        54. + (index / CABINET_COLUMNS) as f32 * 52.,
        133.,
        46.,
    )
}

pub fn draw_cabinet(state: &AppState, _data: &GameData, loaded: usize) {
    let accent = cosmetics::cabinet_accent(state.cabinet_decoration);
    text("IDLE HANDS", 12., 30., 25., accent);
    text(
        "Tap a title to play  -  FAV circle to star",
        12.,
        45.,
        8.,
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    crate::cabinet_art::draw_header_motif(812., 28., 13., accent);
    crate::cabinet_art::draw_shelves(8., 60., 828., 258., accent);
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
            rect.x + 3.,
            rect.y + 29.,
            7.,
            WHITE,
        );
        if state.cabinet_filter == filter {
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3., WHITE);
        }
    }
    panel(
        Rect::new(560., 2., 130., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("CONTINUE", 570., 29., 8., WHITE);
    text(
        selected.title(),
        570.,
        40.,
        6.,
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
            rect.x + 6.,
            rect.y + 17.,
            if game.title().len() > 13 { 8. } else { 10. },
            Color::new(0.98, 0.82, 0.42, 1.),
        );
        let favorite = state.favorites.get(game.index()).copied().unwrap_or(false);
        if favorite {
            draw_circle(rect.right() - 22., rect.y + 34., 6., accent);
        } else {
            draw_circle_lines(rect.right() - 22., rect.y + 34., 6., 2., accent);
        }
        text(
            cabinet_status(state, *game),
            rect.x + 6.,
            rect.y + 36.,
            9.,
            Color::new(0.98, 0.75, 0.30, 1.),
        );
        draw_circle(
            rect.right() - 22.,
            rect.y + 14.,
            9.,
            cosmetics::cabinet_accent(state.cabinet_decoration),
        );
        text(
            &crate::cabinet_status::drawer_number(*game).to_string(),
            rect.right() - 26.,
            rect.y + 18.,
            8.,
            Color::new(0.08, 0.05, 0.12, 1.),
        );
    }
    for (rect, label) in [
        (Rect::new(370., 2., 82., 44.), "PREV"),
        (Rect::new(462., 2., 82., 44.), "NEXT"),
    ] {
        panel(rect, Color::new(0.18, 0.12, 0.28, 1.));
        text(label, rect.x + 12., rect.y + 28., 9., WHITE);
    }
    if visible_games(state).is_empty() {
        text(
            crate::cabinet_status::empty_filter_message(state.cabinet_filter),
            20.,
            100.,
            11.,
            WHITE,
        );
    }
    panel(
        Rect::new(12., 330., 180., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("FAVORITES", 24., 358., 10., WHITE);
    text(
        &state
            .favorites
            .iter()
            .filter(|favorite| **favorite)
            .count()
            .to_string(),
        130.,
        358.,
        10.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    panel(
        Rect::new(200., 330., 180., 44.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("RECENT", 212., 358., 10., WHITE);
    text(
        &state.recent_games.len().to_string(),
        338.,
        358.,
        10.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    text(
        &format!("{} stamps  -  {} textures", state.stamps, loaded),
        400.,
        354.,
        11.,
        Color::new(0.52, 0.48, 0.64, 1.),
    );
    for (rect, label) in [
        (Rect::new(450., 330., 112., 44.), "HELP"),
        (Rect::new(570., 330., 112., 44.), "RECORDS"),
        (Rect::new(690., 330., 140., 44.), "SETTINGS"),
    ] {
        panel(rect, Color::new(0.12, 0.08, 0.20, 1.));
        text(label, rect.x + 15., rect.y + 29., 11., WHITE);
    }
}

pub fn cabinet_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(560., 2., 130., 44.), p) {
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
    if crate::ui::hit(Rect::new(370., 2., 82., 44.), p) {
        return vec![UiAction::CabinetScroll(-1)];
    }
    if crate::ui::hit(Rect::new(462., 2., 82., 44.), p) {
        return vec![UiAction::CabinetScroll(1)];
    }
    if crate::ui::hit(Rect::new(12., 330., 180., 44.), p) {
        return vec![UiAction::Favorites];
    }
    if crate::ui::hit(Rect::new(200., 330., 180., 44.), p) {
        return vec![UiAction::Recent];
    }
    for (rect, action) in [
        (Rect::new(450., 330., 112., 44.), UiAction::Help),
        (Rect::new(570., 330., 112., 44.), UiAction::Records),
        (Rect::new(690., 330., 140., 44.), UiAction::Settings),
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
        (Rect::new(190., 2., 54., 44.), "ALL", 0),
        (Rect::new(248., 2., 54., 44.), "OPEN", 1),
        (Rect::new(306., 2., 54., 44.), "DONE", 2),
    ]
}
fn visible_games(state: &AppState) -> Vec<GameId> {
    GameId::ALL
        .iter()
        .copied()
        .filter(|game| crate::cabinet_status::matches_filter(state, *game, state.cabinet_filter))
        .collect()
}

fn scrolled_games(state: &AppState) -> Vec<GameId> {
    let games = visible_games(state);
    let rows = games.len().div_ceil(CABINET_COLUMNS);
    let first = state
        .cabinet_scroll
        .min(rows.saturating_sub(CABINET_VISIBLE_ROWS))
        * CABINET_COLUMNS;
    games
        .into_iter()
        .skip(first)
        .take(CABINET_PAGE_SIZE)
        .collect()
}

pub fn draw_2048(state: &AppState) {
    let game = &state.game;
    panel(
        Rect::new(0., 0., 110., 44.),
        Color::new(0.12, 0.08, 0.20, 1.),
    );
    text("‹ CABINET", 12., 29., 13., Color::new(0.78, 0.70, 0.92, 1.));
    text("2048", 12., 58., 27., Color::new(0.98, 0.83, 0.45, 1.));
    text(
        &format!("Score {}  -  Best {}", game.score, game.best),
        120.,
        51.,
        12.,
        WHITE,
    );
    let board = Rect::new(12., 65., 320., 320.);
    panel(board, Color::new(0.10, 0.07, 0.16, 1.));
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
        let rect = Rect::new(
            380. + (index % 2) as f32 * 82.,
            145. + (index / 2) as f32 * 58.,
            74.,
            46.,
        );
        panel(rect, Color::new(0.18, 0.12, 0.28, 1.));
        text(
            ["UP", "LEFT", "DOWN", "RIGHT"][index],
            rect.x + 12.,
            rect.y + 29.,
            11.,
            Color::new(0.98, 0.83, 0.45, 1.),
        );
        let _ = direction;
    }
    panel(
        Rect::new(590., 145., 110., 46.),
        Color::new(0.18, 0.12, 0.28, 1.),
    );
    text("UNDO", 625., 174., 12., WHITE);
    panel(
        Rect::new(715., 145., 115., 46.),
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    text("NEW GAME", 738., 174., 11., WHITE);
    panel(
        Rect::new(590., 205., 110., 46.),
        Color::new(0.18, 0.12, 0.28, 1.),
    );
    text("HINT", 625., 234., 12., WHITE);
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or("Swipe the board or tap a direction."),
        380.,
        285.,
        13.,
        Color::new(0.70, 0.64, 0.78, 1.),
    );
    if state.confirm_restart {
        panel(
            Rect::new(375., 215., 300., 120.),
            Color::new(0.16, 0.09, 0.20, 1.),
        );
        text("Start a new board?", 435., 250., 18., WHITE);
        panel(
            Rect::new(395., 270., 115., 42.),
            Color::new(0.25, 0.16, 0.32, 1.),
        );
        text("CANCEL", 425., 297., 12., WHITE);
        panel(
            Rect::new(535., 270., 115., 42.),
            Color::new(0.45, 0.22, 0.25, 1.),
        );
        text("START", 572., 297., 12., WHITE);
    }
}

pub fn game2048_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 110., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if state.confirm_restart {
        if crate::ui::hit(Rect::new(395., 270., 115., 42.), p) {
            return vec![UiAction::Cancel];
        }
        if crate::ui::hit(Rect::new(535., 270., 115., 42.), p) {
            return vec![UiAction::ConfirmRestart];
        }
        return vec![];
    }
    if crate::ui::hit(Rect::new(590., 145., 110., 46.), p) && state.game.can_undo() {
        return vec![UiAction::Undo];
    }
    if crate::ui::hit(Rect::new(715., 145., 115., 46.), p) {
        return vec![UiAction::Restart];
    }
    if crate::ui::hit(Rect::new(590., 205., 110., 46.), p) {
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
        if Rect::new(
            380. + (index % 2) as f32 * 82.,
            145. + (index / 2) as f32 * 58.,
            74.,
            46.,
        )
        .contains(p)
        {
            return vec![UiAction::Move(*direction)];
        }
    }
    vec![]
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
        _ => "PLAY NOW",
    }
}
