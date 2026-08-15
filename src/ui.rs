//! Touch-first cabinet and 2048 presentation.

use crate::blackjack_ui;
use crate::breakout_ui;
use crate::checkers_ui;
use crate::connect_four_ui;
use crate::cosmetics;
use crate::daily_dungeon_ui;
use crate::dots_boxes_ui;
use crate::dungeon_sweeper_ui;
use crate::fivefold_ui;
use crate::flood_it_ui;
use crate::freecell_ui;
use crate::hangman_ui;
use crate::hanoi_ui;
use crate::higher_lower_ui;
use crate::input::Viewport;
use crate::klondike_golf_ui;
use crate::library_ui;
use crate::lights_out_ui;
use crate::mahjong_solitaire_ui;
use crate::mancala_ui;
use crate::mastermind_ui;
use crate::memory_pairs_ui;
use crate::minesweeper_ui;
use crate::nonogram_ui;
use crate::number_match_ui;
use crate::one_room_roguelike_ui;
use crate::palette_ui;
use crate::peg_solitaire_ui;
use crate::potion_2048_ui;
use crate::records_ui;
use crate::responsive_cards;
use crate::responsive_landscape;
use crate::responsive_landscape_cards;
use crate::responsive_landscape_games;
use crate::responsive_landscape_library;
use crate::responsive_library;
use crate::responsive_puzzles;
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
use crate::tutorial_ui;
pub use crate::ui_action::UiAction;
use crate::word_search_ui;
use crate::{
    data::GameData,
    state::{AppState, Direction, GameId, Screen},
};
use macroquad::prelude::*;
pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;
pub fn viewport() -> Viewport {
    let (width, height) = layout_size();
    Viewport::new(screen_width(), screen_height(), width, height)
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
        .screen_to_logical(vec2(mouse_position().0, mouse_position().1))
        .unwrap_or(vec2(-1000., -1000.))
}
pub fn clicks(state: &AppState) -> Vec<UiAction> {
    actions_at(state, mouse())
}

pub fn actions_at(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if state.tutorial.is_some() {
        if is_compact_landscape() {
            return responsive_landscape::tutorial_clicks(p);
        }
        if is_portrait() {
            return responsive_ui::tutorial_clicks(p);
        }
        return tutorial_ui::clicks(p);
    }
    if matches!(state.screen, Screen::Game(_))
        && ((is_compact_landscape() && responsive_landscape::replay_clicks(p))
            || (is_portrait() && responsive_ui::replay_clicks(p))
            || (!is_portrait() && tutorial_ui::REPLAY_RECT.contains(p)))
    {
        return vec![UiAction::ReplayTutorial];
    }
    match state.screen {
        Screen::Cabinet if is_compact_landscape() => responsive_landscape::cabinet_clicks(p),
        Screen::Cabinet if is_portrait() => responsive_ui::cabinet_clicks(p),
        Screen::Cabinet => {
            let mut out = vec![];
            for i in 0..GameId::ALL.len() {
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
        Screen::Game(GameId::Sudoku) if is_portrait() => responsive_ui::sudoku_clicks(state, p),
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
            responsive_cards::fivefold_clicks(state, p)
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
        Screen::Game(GameId::Mastermind) => mastermind_ui::clicks(state, p),
        Screen::Help => {
            if is_compact_landscape() {
                responsive_landscape_library::help_clicks(p)
            } else if is_portrait() {
                responsive_library::help_clicks(p)
            } else if Rect::new(1030., 635., 180., 48.).contains(p) {
                vec![UiAction::Cabinet]
            } else if Rect::new(600., 635., 180., 48.).contains(p) {
                vec![UiAction::Rules]
            } else if Rect::new(800., 635., 180., 48.).contains(p) {
                vec![UiAction::Credits]
            } else {
                vec![]
            }
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
    match state.screen {
        Screen::Cabinet if is_compact_landscape() => {
            responsive_landscape::draw_cabinet(state, data, loaded_assets)
        }
        Screen::Cabinet if is_portrait() => responsive_ui::draw_cabinet(state, data, loaded_assets),
        Screen::Cabinet => draw_cabinet(state, data, loaded_assets),
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
        Screen::Game(GameId::Sudoku) if is_portrait() => responsive_ui::draw_sudoku(state),
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
        Screen::Game(GameId::Yahtzee) if is_portrait() => responsive_cards::draw_fivefold(state),
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
        Screen::Game(GameId::Mastermind) => mastermind_ui::draw(state),
        Screen::Help if is_compact_landscape() => responsive_landscape_library::draw_help(),
        Screen::Help if is_portrait() => responsive_library::draw_help(),
        Screen::Help => draw_help(),
        Screen::Records if is_compact_landscape() => {
            responsive_landscape_library::draw_records(state)
        }
        Screen::Records if is_portrait() => responsive_library::draw_records(state),
        Screen::Records => records_ui::draw_records(state),
        Screen::Rules if is_compact_landscape() => responsive_landscape_library::draw_rules(),
        Screen::Rules if is_portrait() => responsive_library::draw_rules(),
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
            responsive_landscape::draw_tutorial(game);
        } else if is_portrait() {
            responsive_ui::draw_tutorial(game);
        } else {
            tutorial_ui::draw_overlay(game);
        }
    } else if matches!(state.screen, Screen::Game(game) if !matches!(game, GameId::LightsOut | GameId::TicTacToe | GameId::MemoryPairs | GameId::SlidingPuzzle | GameId::Mastermind | GameId::Spider | GameId::WordSearch | GameId::Hangman | GameId::ConnectFour | GameId::Checkers | GameId::PegSolitaire | GameId::MahjongSolitaire | GameId::Snake | GameId::Breakout | GameId::HigherLower | GameId::KlondikeGolf))
    {
        if is_compact_landscape() {
            responsive_landscape::draw_replay_button();
        } else if is_portrait() {
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
    let accent = cosmetics::cabinet_accent(state.cabinet_decoration);
    text("IDLE HANDS", 46., 70., 48., accent);
    crate::cabinet_art::draw_header_motif(1160., 108., 24., accent);
    crate::cabinet_art::draw_shelves(48., 165., 1184., 480., accent);
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
    for i in 0..GameId::ALL.len() {
        let r = cabinet_rect(i);
        let active = crate::cabinet_status::is_active(GameId::ALL[i]);
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
            r.y + 28.,
            if matches!(
                GameId::ALL[i],
                GameId::TinyTowerDefence
                    | GameId::OneRoomRoguelike
                    | GameId::DailyDungeon
                    | GameId::DotsBoxes
                    | GameId::Sokoban
                    | GameId::Mancala
                    | GameId::Hanoi
                    | GameId::NumberMatch
                    | GameId::FloodIt
            ) {
                10.
            } else {
                18.
            },
            if active {
                Color::new(0.98, 0.82, 0.42, 1.)
            } else {
                WHITE
            },
        );
        let status = crate::cabinet_status::status(state, GameId::ALL[i]);
        text(
            status,
            r.x + 18.,
            r.y + 50.,
            11.,
            crate::cabinet_status::color(status),
        );
        text(
            GameId::ALL[i].subtitle(),
            r.x + 18.,
            r.y + 72.,
            11.,
            Color::new(0.69, 0.65, 0.78, 1.),
        );
        draw_circle(
            r.right() - 34.,
            r.y + 25.,
            12.,
            if active {
                Color::new(0.85, 0.55, 0.28, 1.)
            } else {
                Color::new(0.22, 0.18, 0.31, 1.)
            },
        );
        text(
            &format!("{}", i + 1),
            r.right() - 37.,
            r.y + 29.,
            11.,
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
    let col = i % 7;
    let row = i / 7;
    Rect::new(48. + col as f32 * 170., 155. + row as f32 * 100., 160., 80.)
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
