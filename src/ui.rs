//! Touch-first cabinet and 2048 presentation.

use crate::achievements_ui;
use crate::battleship_ui;
use crate::blackjack_ui;
use crate::breakout_ui;
use crate::cabinet_ui;
use crate::checkers_ui;
use crate::color_sort_ui;
use crate::connect_four_ui;
use crate::daily_dungeon_ui;
use crate::dots_boxes_ui;
use crate::dungeon_sweeper_ui;
use crate::favorites_ui;
use crate::fivefold_ui;
use crate::flood_it_ui;
use crate::freecell_ui;
use crate::hangman_ui;
use crate::hanoi_ui;
use crate::higher_lower_ui;
use crate::klondike_golf_ui;
use crate::library_ui;
use crate::lights_out_ui;
use crate::mahjong_solitaire_ui;
use crate::mancala_ui;
use crate::mastermind_ui;
use crate::match_three_ui;
use crate::maze_walk_ui;
use crate::memory_pairs_ui;
use crate::minesweeper_ui;
use crate::mobile_tutorial_ui;
use crate::nim_ui;
use crate::nonogram_ui;
use crate::number_match_ui;
use crate::one_room_roguelike_ui;
use crate::palette_ui;
use crate::peg_solitaire_ui;
use crate::pipe_loop_ui;
use crate::potion_2048_ui;
use crate::pyramid_ui;
use crate::records_ui;
use crate::responsive_cabinet;
use crate::responsive_cards;
use crate::responsive_fivefold;
use crate::responsive_landscape;
use crate::responsive_landscape_cabinet;
use crate::responsive_landscape_cards;
use crate::responsive_landscape_games;
use crate::responsive_landscape_library;
use crate::responsive_library;
use crate::responsive_puzzles;
use crate::responsive_sudoku;
use crate::responsive_ui;
use crate::reversi_ui;
use crate::settings_ui;
use crate::sliding_puzzle_ui;
use crate::snake_ui;
use crate::sokoban_ui;
use crate::solitaire_ui;
use crate::spider_solitaire_ui;
use crate::spider_ui;
use crate::sudoku_ui;
use crate::tic_tac_toe_ui;
use crate::tiny_tower_defence_ui;
use crate::tri_peaks_ui;
use crate::tutorial_ui;
pub use crate::ui_action::UiAction;
use crate::word_grid_ui;
use crate::word_ladder_ui;
use crate::word_search_ui;
#[path = "restart_modal.rs"]
mod restart_modal;
use crate::{
    data::GameData,
    state::{AppState, Direction, GameId, Screen},
};
use macroquad::prelude::*;
use macroquad_toolkit::ui::{
    end_frame_neighbours, note_neighbour, touch_area_for_scale, VirtualUi,
};

pub fn draw_rounded_panel(rect: Rect, radius: f32, fill: Color, border: Color) {
    const CORNER_SEGMENTS: usize = 6;

    let radius = radius.clamp(0., rect.w.min(rect.h) * 0.5);
    if radius == 0. {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., border);
        return;
    }

    let mut edge = Vec::with_capacity(CORNER_SEGMENTS * 4 + 1);
    for (center, start) in [
        (vec2(rect.x + radius, rect.y + radius), std::f32::consts::PI),
        (
            vec2(rect.right() - radius, rect.y + radius),
            std::f32::consts::PI * 1.5,
        ),
        (vec2(rect.right() - radius, rect.bottom() - radius), 0.),
        (
            vec2(rect.x + radius, rect.bottom() - radius),
            std::f32::consts::FRAC_PI_2,
        ),
    ] {
        for step in 0..=CORNER_SEGMENTS {
            let angle = start + std::f32::consts::FRAC_PI_2 * step as f32 / CORNER_SEGMENTS as f32;
            edge.push(center + vec2(angle.cos(), angle.sin()) * radius);
        }
    }

    let center = rect.center();
    for index in 0..edge.len() {
        draw_triangle(center, edge[index], edge[(index + 1) % edge.len()], fill);
        draw_line(
            edge[index].x,
            edge[index].y,
            edge[(index + 1) % edge.len()].x,
            edge[(index + 1) % edge.len()].y,
            2.,
            border,
        );
    }
}
use std::cell::Cell;
pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;
thread_local! {
    static TOUCH_SCALE: Cell<f32> = const { Cell::new(1.0) };
}
pub fn viewport() -> VirtualUi {
    let (width, height) = layout_size();
    VirtualUi::new(width, height)
}
pub fn layout_size() -> (f32, f32) {
    if is_compact_landscape() {
        (responsive_landscape::WIDTH, responsive_landscape::HEIGHT)
    } else if is_portrait() {
        (responsive_ui::WIDTH, responsive_ui::HEIGHT)
    } else {
        (LOGICAL_WIDTH, LOGICAL_HEIGHT)
    }
}
pub fn is_portrait() -> bool {
    screen_height() > screen_width() * 1.15
}
pub fn is_compact_landscape() -> bool {
    screen_width() <= 900. && screen_width() > screen_height() * 1.15
}
pub fn mouse() -> Vec2 {
    viewport()
        .screen_to_ui_checked(vec2(mouse_position().0, mouse_position().1))
        .unwrap_or(vec2(-1000., -1000.))
}
pub fn physical_touch_rect(rect: Rect, scale: f32) -> Rect {
    touch_area_for_scale(rect, scale)
}
pub fn readable_text_size_for_scale(base: f32, scale: f32) -> f32 {
    let physical_floor = if base < 10. { 9. } else { 11. };
    if scale.is_finite() && scale > 0. {
        base.max(physical_floor / scale)
    } else {
        base
    }
}
pub fn readable_text_size(base: f32) -> f32 {
    TOUCH_SCALE.with(|scale| readable_text_size_for_scale(base, scale.get()))
}
pub fn draw_text(
    value: impl AsRef<str>,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
) -> TextDimensions {
    macroquad_toolkit::ui::draw_ui_text(
        value.as_ref(),
        x,
        y,
        readable_text_size(size),
        crate::theme::text_color(color),
    )
}
pub fn measure_text(
    value: impl AsRef<str>,
    font: Option<&Font>,
    size: u16,
    scale: f32,
) -> TextDimensions {
    let readable_size = readable_text_size(size as f32).round() as u16;
    macroquad_toolkit::ui::measure_ui_text(value.as_ref(), font, readable_size, scale)
}
pub fn hit(rect: Rect, point: Vec2) -> bool {
    let scale = TOUCH_SCALE.with(Cell::get);
    let area = physical_touch_rect(rect, scale);
    note_neighbour(rect);
    area.contains(point)
}
pub fn clicks(state: &AppState) -> Vec<UiAction> {
    TOUCH_SCALE.with(|scale| scale.set(viewport().scale));
    actions_at(state, mouse())
}

pub fn actions_at(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if state.tutorial.is_some() {
        if is_compact_landscape() {
            return mobile_tutorial_ui::tutorial_clicks(p, true);
        }
        if is_portrait() {
            return mobile_tutorial_ui::tutorial_clicks(p, false);
        }
        return tutorial_ui::clicks(p);
    }
    if matches!(state.screen, Screen::Game(_))
        && ((is_compact_landscape() && mobile_tutorial_ui::replay_clicks(p, true))
            || (is_portrait() && mobile_tutorial_ui::replay_clicks(p, false))
            || (!is_portrait() && hit(tutorial_ui::REPLAY_RECT, p)))
    {
        return vec![UiAction::ReplayTutorial];
    }
    if state.confirm_restart && state.pending_restart.is_some() {
        return restart_modal::clicks(p);
    }
    match state.screen {
        Screen::Cabinet if is_compact_landscape() => responsive_landscape_cabinet::clicks(state, p),
        Screen::Cabinet if is_portrait() => responsive_cabinet::clicks(state, p),
        Screen::Cabinet => cabinet_ui::clicks(state, p),
        Screen::Game(GameId::Game2048) if is_compact_landscape() => {
            responsive_landscape::game2048_clicks(state, p)
        }
        Screen::Game(GameId::Game2048) if is_portrait() => responsive_ui::game2048_clicks(state, p),
        Screen::Game(GameId::Game2048) => game_clicks(state, p),
        Screen::Game(GameId::Minesweeper) if is_compact_landscape() => {
            responsive_landscape_games::minesweeper_clicks(state, p)
        }
        Screen::Game(GameId::Minesweeper) if is_portrait() => {
            responsive_puzzles::minesweeper_clicks(state, p)
        }
        Screen::Game(GameId::Minesweeper) => minesweeper_ui::clicks(state, p),
        Screen::Game(GameId::Sudoku) if is_compact_landscape() => {
            responsive_landscape_games::sudoku_clicks(state, p)
        }
        Screen::Game(GameId::Sudoku) if is_portrait() => responsive_sudoku::clicks(state, p),
        Screen::Game(GameId::Sudoku) => sudoku_ui::sudoku_clicks(state, p),
        Screen::Game(GameId::Nonogram) if is_compact_landscape() => {
            responsive_landscape_games::nonogram_clicks(state, p)
        }
        Screen::Game(GameId::Nonogram) if is_portrait() => {
            responsive_puzzles::nonogram_clicks(state, p)
        }
        Screen::Game(GameId::Nonogram) => nonogram_ui::nonogram_clicks(state, p),
        Screen::Game(GameId::Solitaire) if is_compact_landscape() => {
            responsive_landscape_cards::solitaire_clicks(state, p)
        }
        Screen::Game(GameId::Solitaire) if is_portrait() => {
            responsive_cards::solitaire_clicks(state, p)
        }
        Screen::Game(GameId::Solitaire) => solitaire_ui::solitaire_clicks(state, p),
        Screen::Game(GameId::FreeCell) if is_compact_landscape() => {
            responsive_landscape_cards::freecell_clicks(state, p)
        }
        Screen::Game(GameId::FreeCell) if is_portrait() => {
            responsive_cards::freecell_clicks(state, p)
        }
        Screen::Game(GameId::FreeCell) => freecell_ui::freecell_clicks(state, p),
        Screen::Game(GameId::Yahtzee) if is_compact_landscape() => {
            responsive_landscape_cards::fivefold_clicks(state, p)
        }
        Screen::Game(GameId::Yahtzee) if is_portrait() => {
            responsive_fivefold::fivefold_clicks(state, p)
        }
        Screen::Game(GameId::Yahtzee) => fivefold_ui::fivefold_clicks(state, p),
        Screen::Game(GameId::Reversi) if is_compact_landscape() => {
            responsive_landscape_games::reversi_clicks(state, p)
        }
        Screen::Game(GameId::Reversi) if is_portrait() => {
            responsive_cards::reversi_clicks(state, p)
        }
        Screen::Game(GameId::Reversi) => reversi_ui::reversi_clicks(state, p),
        Screen::Game(GameId::LightsOut) => lights_out_ui::clicks(state, p),
        Screen::Game(GameId::TicTacToe) => tic_tac_toe_ui::clicks(state, p),
        Screen::Game(GameId::MemoryPairs) => memory_pairs_ui::clicks(state, p),
        Screen::Game(GameId::SlidingPuzzle) => sliding_puzzle_ui::clicks(state, p),
        Screen::Game(GameId::Spider) => spider_ui::clicks(state, p),
        Screen::Game(GameId::WordSearch) => word_search_ui::clicks(state, p),
        Screen::Game(GameId::Hangman) => hangman_ui::clicks(state, p),
        Screen::Game(GameId::ConnectFour) => connect_four_ui::clicks(state, p),
        Screen::Game(GameId::Checkers) => checkers_ui::clicks(state, p),
        Screen::Game(GameId::PegSolitaire) => peg_solitaire_ui::clicks(state, p),
        Screen::Game(GameId::MahjongSolitaire) => mahjong_solitaire_ui::clicks(state, p),
        Screen::Game(GameId::Snake) => snake_ui::clicks(state, p),
        Screen::Game(GameId::Breakout) => breakout_ui::clicks(state, p),
        Screen::Game(GameId::HigherLower) => higher_lower_ui::clicks(state, p),
        Screen::Game(GameId::KlondikeGolf) => klondike_golf_ui::clicks(state, p),
        Screen::Game(GameId::Blackjack) => blackjack_ui::clicks(state, p),
        Screen::Game(GameId::SpiderSolitaire) => spider_solitaire_ui::clicks(state, p),
        Screen::Game(GameId::Pyramid) => pyramid_ui::clicks(state, p),
        Screen::Game(GameId::TriPeaks) => tri_peaks_ui::clicks(state, p),
        Screen::Game(GameId::Nim) => nim_ui::clicks(state, p),
        Screen::Game(GameId::DungeonSweeper) => dungeon_sweeper_ui::clicks(state, p),
        Screen::Game(GameId::Potion2048) => potion_2048_ui::clicks(state, p),
        Screen::Game(GameId::TinyTowerDefence) => tiny_tower_defence_ui::clicks(state, p),
        Screen::Game(GameId::OneRoomRoguelike) => one_room_roguelike_ui::clicks(state, p),
        Screen::Game(GameId::DailyDungeon) => daily_dungeon_ui::clicks(state, p),
        Screen::Game(GameId::DotsBoxes) => dots_boxes_ui::clicks(state, p),
        Screen::Game(GameId::Sokoban) => sokoban_ui::clicks(state, p),
        Screen::Game(GameId::Mancala) => mancala_ui::clicks(state, p),
        Screen::Game(GameId::Hanoi) => hanoi_ui::clicks(state, p),
        Screen::Game(GameId::NumberMatch) => number_match_ui::clicks(state, p),
        Screen::Game(GameId::FloodIt) => flood_it_ui::clicks(state, p),
        Screen::Game(GameId::ColorSort) => color_sort_ui::clicks(state, p),
        Screen::Game(GameId::Battleship) => battleship_ui::clicks(state, p),
        Screen::Game(GameId::WordGrid) => word_grid_ui::clicks(state, p),
        Screen::Game(GameId::WordLadder) => word_ladder_ui::clicks(state, p),
        Screen::Game(GameId::PipeLoop) => pipe_loop_ui::clicks(state, p),
        Screen::Game(GameId::MazeWalk) => maze_walk_ui::clicks(state, p),
        Screen::Game(GameId::MatchThree) => match_three_ui::clicks(state, p),
        Screen::Game(GameId::Mastermind) => mastermind_ui::clicks(state, p),
        Screen::Help => {
            if is_compact_landscape() {
                responsive_landscape_library::help_clicks(p)
            } else if is_portrait() {
                responsive_library::help_clicks(p)
            } else if hit(Rect::new(1030., 635., 180., 48.), p) {
                vec![UiAction::Cabinet]
            } else if hit(Rect::new(600., 635., 180., 48.), p) {
                vec![UiAction::Rules]
            } else if hit(Rect::new(800., 635., 180., 48.), p) {
                vec![UiAction::Credits]
            } else {
                vec![]
            }
        }
        Screen::Records if state.achievements_view => achievements_ui::clicks(p),
        Screen::Records if state.favorites_view || state.recent_view => {
            favorites_ui::clicks(state, p)
        }
        Screen::Records if is_compact_landscape() => {
            responsive_landscape_library::records_clicks(p)
        }
        Screen::Records if is_portrait() => responsive_library::records_clicks(p),
        Screen::Records => records_ui::records_clicks(p),
        Screen::Rules if is_compact_landscape() => responsive_landscape_library::rules_clicks(p),
        Screen::Rules if is_portrait() => responsive_library::rules_clicks(p),
        Screen::Rules => library_ui::rules_clicks(p),
        Screen::Credits if is_compact_landscape() => {
            responsive_landscape_library::credits_clicks(p)
        }
        Screen::Credits if is_portrait() => responsive_library::credits_clicks(p),
        Screen::Credits => library_ui::credits_clicks(p),
        Screen::Settings if is_compact_landscape() => {
            responsive_landscape_library::settings_clicks(state, p)
        }
        Screen::Settings if is_portrait() => responsive_ui::settings_clicks(state, p),
        Screen::Settings => settings_ui::settings_clicks(state, p),
    }
}
pub fn draw(state: &AppState, data: &GameData, loaded_assets: usize) {
    TOUCH_SCALE.with(|scale| scale.set(viewport().scale));
    match state.screen {
        Screen::Cabinet if is_compact_landscape() => {
            responsive_landscape_cabinet::draw(state, data, loaded_assets)
        }
        Screen::Cabinet if is_portrait() => responsive_cabinet::draw(state, data, loaded_assets),
        Screen::Cabinet => cabinet_ui::draw(state, data, loaded_assets),
        Screen::Game(GameId::Game2048) if is_compact_landscape() => {
            responsive_landscape::draw_2048(state)
        }
        Screen::Game(GameId::Game2048) if is_portrait() => responsive_ui::draw_2048(state),
        Screen::Game(GameId::Game2048) => draw_2048(state),
        Screen::Game(GameId::Minesweeper) if is_compact_landscape() => {
            responsive_landscape_games::draw_minesweeper(state)
        }
        Screen::Game(GameId::Minesweeper) if is_portrait() => {
            responsive_puzzles::draw_minesweeper(state)
        }
        Screen::Game(GameId::Minesweeper) => minesweeper_ui::draw(state),
        Screen::Game(GameId::Sudoku) if is_compact_landscape() => {
            responsive_landscape_games::draw_sudoku(state)
        }
        Screen::Game(GameId::Sudoku) if is_portrait() => responsive_sudoku::draw(state),
        Screen::Game(GameId::Sudoku) => sudoku_ui::draw_sudoku(state),
        Screen::Game(GameId::Nonogram) if is_compact_landscape() => {
            responsive_landscape_games::draw_nonogram(state)
        }
        Screen::Game(GameId::Nonogram) if is_portrait() => responsive_puzzles::draw_nonogram(state),
        Screen::Game(GameId::Nonogram) => nonogram_ui::draw_nonogram(state),
        Screen::Game(GameId::Solitaire) if is_compact_landscape() => {
            responsive_landscape_cards::draw_solitaire(state)
        }
        Screen::Game(GameId::Solitaire) if is_portrait() => responsive_cards::draw_solitaire(state),
        Screen::Game(GameId::Solitaire) => solitaire_ui::draw_solitaire(state),
        Screen::Game(GameId::FreeCell) if is_compact_landscape() => {
            responsive_landscape_cards::draw_freecell(state)
        }
        Screen::Game(GameId::FreeCell) if is_portrait() => responsive_cards::draw_freecell(state),
        Screen::Game(GameId::FreeCell) => freecell_ui::draw_freecell(state),
        Screen::Game(GameId::Yahtzee) if is_compact_landscape() => {
            responsive_landscape_cards::draw_fivefold(state)
        }
        Screen::Game(GameId::Yahtzee) if is_portrait() => responsive_fivefold::draw_fivefold(state),
        Screen::Game(GameId::Yahtzee) => fivefold_ui::draw_fivefold(state),
        Screen::Game(GameId::Reversi) if is_compact_landscape() => {
            responsive_landscape_games::draw_reversi(state)
        }
        Screen::Game(GameId::Reversi) if is_portrait() => responsive_cards::draw_reversi(state),
        Screen::Game(GameId::Reversi) => reversi_ui::draw_reversi(state),
        Screen::Game(GameId::LightsOut) => lights_out_ui::draw(state),
        Screen::Game(GameId::TicTacToe) => tic_tac_toe_ui::draw(state),
        Screen::Game(GameId::MemoryPairs) => memory_pairs_ui::draw(state),
        Screen::Game(GameId::SlidingPuzzle) => sliding_puzzle_ui::draw(state),
        Screen::Game(GameId::Spider) => spider_ui::draw(state),
        Screen::Game(GameId::WordSearch) => word_search_ui::draw(state),
        Screen::Game(GameId::Hangman) => hangman_ui::draw(state),
        Screen::Game(GameId::ConnectFour) => connect_four_ui::draw(state),
        Screen::Game(GameId::Checkers) => checkers_ui::draw(state),
        Screen::Game(GameId::PegSolitaire) => peg_solitaire_ui::draw(state),
        Screen::Game(GameId::MahjongSolitaire) => mahjong_solitaire_ui::draw(state),
        Screen::Game(GameId::Snake) => snake_ui::draw(state),
        Screen::Game(GameId::Breakout) => breakout_ui::draw(state),
        Screen::Game(GameId::HigherLower) => higher_lower_ui::draw(state),
        Screen::Game(GameId::KlondikeGolf) => klondike_golf_ui::draw(state),
        Screen::Game(GameId::Blackjack) => blackjack_ui::draw(state),
        Screen::Game(GameId::SpiderSolitaire) => spider_solitaire_ui::draw(state),
        Screen::Game(GameId::Pyramid) => pyramid_ui::draw(state),
        Screen::Game(GameId::TriPeaks) => tri_peaks_ui::draw(state),
        Screen::Game(GameId::Nim) => nim_ui::draw(state),
        Screen::Game(GameId::DungeonSweeper) => dungeon_sweeper_ui::draw(state),
        Screen::Game(GameId::Potion2048) => potion_2048_ui::draw(state),
        Screen::Game(GameId::TinyTowerDefence) => tiny_tower_defence_ui::draw(state),
        Screen::Game(GameId::OneRoomRoguelike) => one_room_roguelike_ui::draw(state),
        Screen::Game(GameId::DailyDungeon) => daily_dungeon_ui::draw(state),
        Screen::Game(GameId::DotsBoxes) => dots_boxes_ui::draw(state),
        Screen::Game(GameId::Sokoban) => sokoban_ui::draw(state),
        Screen::Game(GameId::Mancala) => mancala_ui::draw(state),
        Screen::Game(GameId::Hanoi) => hanoi_ui::draw(state),
        Screen::Game(GameId::NumberMatch) => number_match_ui::draw(state),
        Screen::Game(GameId::FloodIt) => flood_it_ui::draw(state),
        Screen::Game(GameId::ColorSort) => color_sort_ui::draw(state),
        Screen::Game(GameId::Battleship) => battleship_ui::draw(state),
        Screen::Game(GameId::WordGrid) => word_grid_ui::draw(state),
        Screen::Game(GameId::WordLadder) => word_ladder_ui::draw(state),
        Screen::Game(GameId::PipeLoop) => pipe_loop_ui::draw(state),
        Screen::Game(GameId::MazeWalk) => maze_walk_ui::draw(state),
        Screen::Game(GameId::MatchThree) => match_three_ui::draw(state),
        Screen::Game(GameId::Mastermind) => mastermind_ui::draw(state),
        Screen::Help if is_compact_landscape() => responsive_landscape_library::draw_help(),
        Screen::Help if is_portrait() => responsive_library::draw_help(),
        Screen::Help => draw_help(),
        Screen::Records if state.achievements_view => achievements_ui::draw(state),
        Screen::Records if state.favorites_view || state.recent_view => favorites_ui::draw(state),
        Screen::Records if is_compact_landscape() => {
            responsive_landscape_library::draw_records(state)
        }
        Screen::Records if is_portrait() => responsive_library::draw_records(state),
        Screen::Records => records_ui::draw_records(state),
        Screen::Rules if is_compact_landscape() => responsive_landscape_library::draw_rules(state),
        Screen::Rules if is_portrait() => responsive_library::draw_rules(state),
        Screen::Rules => library_ui::draw_rules(),
        Screen::Credits if is_compact_landscape() => responsive_landscape_library::draw_credits(),
        Screen::Credits if is_portrait() => responsive_library::draw_credits(),
        Screen::Credits => library_ui::draw_credits(),
        Screen::Settings if is_compact_landscape() => {
            responsive_landscape_library::draw_settings(state)
        }
        Screen::Settings if is_portrait() => responsive_ui::draw_settings(state),
        Screen::Settings => settings_ui::draw_settings(state),
    }
    if let Some(game) = state.tutorial {
        if is_compact_landscape() {
            mobile_tutorial_ui::draw_tutorial(game, true);
        } else if is_portrait() {
            mobile_tutorial_ui::draw_tutorial(game, false);
        } else {
            tutorial_ui::draw_overlay(game);
        }
    } else if matches!(state.screen, Screen::Game(_)) {
        if is_compact_landscape() {
            mobile_tutorial_ui::draw_replay_button(true);
        } else if is_portrait() {
            mobile_tutorial_ui::draw_replay_button(false);
        } else {
            tutorial_ui::draw_replay_button();
        }
    }
    if state.confirm_restart && state.pending_restart.is_some() {
        restart_modal::draw(state);
    }
    // Prime neighbour-aware target growth from the visible action map. Input
    // is handled before drawing, so the next frame can expand small controls
    // without allowing adjacent targets to claim the same physical point.
    let _ = actions_at(state, vec2(-10_000., -10_000.));
    end_frame_neighbours();
}
fn text(s: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(s, x, y, crate::ui::readable_text_size(size), color);
}
fn panel(r: Rect, fill: Color) {
    draw_rectangle(r.x, r.y, r.w, r.h, crate::theme::drawer_surface(fill));
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 2., crate::theme::BORDER)
}
fn draw_2048(state: &AppState) {
    let g = &state.game;
    text("‹ CABINET", 40., 55., 20., Color::new(0.78, 0.70, 0.92, 1.));
    text("2048", 40., 105., 52., crate::theme::BRASS);
    text(
        "Slide, merge, breathe",
        44.,
        132.,
        18.,
        crate::theme::SECONDARY,
    );
    score_box(Rect::new(830., 68., 120., 66.), "SCORE", g.score);
    score_box(Rect::new(965., 68., 120., 66.), "BEST", g.best);
    panel(
        Rect::new(830., 160., 360., 380.),
        crate::theme::GAME_PANEL,
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
            let tw = crate::ui::measure_text(&label, None, fs as u16, 1.0).width;
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
        crate::theme::SECONDARY,
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
        panel(r, crate::theme::SURFACE_DARK);
        text(label, r.x + 28., r.y + 33., 26., crate::theme::BRASS);
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
fn game_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    let mut out = vec![];
    if hit(Rect::new(20., 20., 180., 50.), p) {
        out.push(UiAction::Cabinet)
    }
    if hit(Rect::new(400., 390., 140., 48.), p) && state.game.can_undo() {
        out.push(UiAction::Undo)
    }
    if hit(Rect::new(560., 390., 140., 48.), p) {
        out.push(UiAction::Restart)
    }
    if hit(Rect::new(400., 450., 140., 48.), p) {
        out.push(UiAction::Game2048Hint)
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
fn draw_help() {
    panel(
        Rect::new(120., 80., 1040., 560.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("HOW TO PLAY", 170., 145., 42., crate::theme::BRASS);
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
    panel(Rect::new(600., 635., 180., 48.), crate::theme::SURFACE);
    text("RULES", 660., 666., 18., WHITE);
    panel(Rect::new(800., 635., 180., 48.), crate::theme::SURFACE);
    text("CREDITS", 850., 666., 18., WHITE);
    panel(Rect::new(1030., 635., 180., 48.), crate::theme::MOSS_DARK);
    text("BACK", 1090., 666., 18., WHITE)
}
