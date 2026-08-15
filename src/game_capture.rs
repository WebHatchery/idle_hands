use super::Game;
use crate::{
    card_hints,
    state::{GameId, Screen},
};
use macroquad_toolkit::notifications::NotificationManager;

impl Game {
    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.state = crate::state::AppState::default();
        self.notifications = NotificationManager::new();
        let scene = scene
            .strip_prefix("portrait_")
            .or_else(|| scene.strip_prefix("landscape_"))
            .unwrap_or(scene);
        self.state.screen = match scene {
            "2048" | "gameplay" | "tutorial_2048" => Screen::Game(GameId::Game2048),
            "minesweeper" | "minesweeper_accessible" => Screen::Game(GameId::Minesweeper),
            "sudoku" | "sudoku_accessible" => Screen::Game(GameId::Sudoku),
            "nonogram" | "nonogram_large" | "nonogram_accessible" => Screen::Game(GameId::Nonogram),
            "solitaire" | "solitaire_hint" | "solitaire_selected" => {
                Screen::Game(GameId::Solitaire)
            }
            "freecell" | "freecell_hint" | "freecell_selected" => Screen::Game(GameId::FreeCell),
            "fivefold" => Screen::Game(GameId::Yahtzee),
            "reversi" => Screen::Game(GameId::Reversi),
            "lights_out" | "lights_out_accessible" => Screen::Game(GameId::LightsOut),
            "tic_tac_toe" => Screen::Game(GameId::TicTacToe),
            "memory_pairs" | "memory_pairs_accessible" => Screen::Game(GameId::MemoryPairs),
            "sliding_puzzle" | "sliding_puzzle_accessible" => Screen::Game(GameId::SlidingPuzzle),
            "mastermind" => Screen::Game(GameId::Mastermind),
            "spider" => Screen::Game(GameId::Spider),
            "word_search" => Screen::Game(GameId::WordSearch),
            "hangman" => Screen::Game(GameId::Hangman),
            "connect_four" => Screen::Game(GameId::ConnectFour),
            "checkers" => Screen::Game(GameId::Checkers),
            "peg_solitaire" => Screen::Game(GameId::PegSolitaire),
            "mahjong_solitaire" => Screen::Game(GameId::MahjongSolitaire),
            "snake" => Screen::Game(GameId::Snake),
            "breakout" => Screen::Game(GameId::Breakout),
            "higher_lower" => Screen::Game(GameId::HigherLower),
            "klondike_golf" => Screen::Game(GameId::KlondikeGolf),
            "blackjack" => Screen::Game(GameId::Blackjack),
            "spider_solitaire" => Screen::Game(GameId::SpiderSolitaire),
            "dungeon_sweeper" | "dungeon_sweeper_accessible" => {
                Screen::Game(GameId::DungeonSweeper)
            }
            "potion_2048" => Screen::Game(GameId::Potion2048),
            "tiny_tower_defence" => Screen::Game(GameId::TinyTowerDefence),
            "one_room_roguelike" => Screen::Game(GameId::OneRoomRoguelike),
            "daily_dungeon" => Screen::Game(GameId::DailyDungeon),
            "dots_boxes" => Screen::Game(GameId::DotsBoxes),
            "dots_boxes_accessible" => Screen::Game(GameId::DotsBoxes),
            "sokoban" => Screen::Game(GameId::Sokoban),
            "sokoban_accessible" => Screen::Game(GameId::Sokoban),
            "mancala" => Screen::Game(GameId::Mancala),
            "hanoi" => Screen::Game(GameId::Hanoi),
            "number_match" => Screen::Game(GameId::NumberMatch),
            "number_match_accessible" => Screen::Game(GameId::NumberMatch),
            "flood_it" => Screen::Game(GameId::FloodIt),
            "flood_it_accessible" => Screen::Game(GameId::FloodIt),
            "color_sort" => Screen::Game(GameId::ColorSort),
            "color_sort_accessible" => Screen::Game(GameId::ColorSort),
            "battleship" => Screen::Game(GameId::Battleship),
            "battleship_accessible" => Screen::Game(GameId::Battleship),
            "word_grid" => Screen::Game(GameId::WordGrid),
            "word_grid_accessible" => Screen::Game(GameId::WordGrid),
            "pipe_loop" => Screen::Game(GameId::PipeLoop),
            "pipe_loop_accessible" => Screen::Game(GameId::PipeLoop),
            "maze_walk" | "maze_walk_accessible" => Screen::Game(GameId::MazeWalk),
            "match_three" | "match_three_accessible" => Screen::Game(GameId::MatchThree),
            "mastermind_accessible" => Screen::Game(GameId::Mastermind),
            "help" => Screen::Help,
            "records" => Screen::Records,
            "rules" => Screen::Rules,
            "credits" => Screen::Credits,
            "settings" | "settings_reset" => Screen::Settings,
            _ => Screen::Cabinet,
        };
        if scene == "settings_reset" {
            self.state.confirm_reset = true;
        }
        if scene == "nonogram_large" {
            self.state.nonogram =
                crate::nonogram::Nonogram::new(crate::nonogram::NonogramPreset::Large);
            self.state.nonogram_zoomed = true;
            self.state.nonogram_focus = (6, 6);
        }
        if scene == "solitaire_selected" {
            self.state.solitaire.select_tableau(0, 0);
        }
        if scene == "freecell_selected" {
            self.state.freecell.select_cascade(0, 0);
        }
        if scene.ends_with("_accessible") {
            self.state.high_contrast = true;
            self.state.large_text = true;
        }
        if scene == "solitaire_hint" {
            self.state.card_hint = Some(card_hints::solitaire(&self.state));
        } else if scene == "freecell_hint" {
            self.state.card_hint = Some(card_hints::freecell(&self.state));
        }
        if scene.starts_with("tutorial_") {
            if let Screen::Game(game) = self.state.screen {
                self.state.tutorial = Some(game);
            }
        }
        self.transition = 0.;
    }
}
