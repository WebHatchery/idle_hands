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
            "2048" | "gameplay" | "tutorial_2048" | "2048_hint" | "2048_hint_accessible" => {
                Screen::Game(GameId::Game2048)
            }
            "minesweeper"
            | "minesweeper_accessible"
            | "minesweeper_hint"
            | "minesweeper_hint_accessible" => Screen::Game(GameId::Minesweeper),
            "sudoku" | "sudoku_accessible" | "sudoku_hint" | "sudoku_hint_accessible" => {
                Screen::Game(GameId::Sudoku)
            }
            "nonogram"
            | "nonogram_large"
            | "nonogram_accessible"
            | "nonogram_hint"
            | "nonogram_hint_accessible" => Screen::Game(GameId::Nonogram),
            "solitaire" | "solitaire_hint" | "solitaire_selected" => {
                Screen::Game(GameId::Solitaire)
            }
            "freecell" | "freecell_hint" | "freecell_selected" => Screen::Game(GameId::FreeCell),
            "fivefold" => Screen::Game(GameId::Yahtzee),
            "reversi" | "reversi_hint" | "reversi_hint_accessible" => Screen::Game(GameId::Reversi),
            "reversi_accessible" => Screen::Game(GameId::Reversi),
            "lights_out"
            | "lights_out_accessible"
            | "lights_out_hint"
            | "lights_out_hint_accessible" => Screen::Game(GameId::LightsOut),
            "tic_tac_toe" | "tic_tac_toe_hint" => Screen::Game(GameId::TicTacToe),
            "tic_tac_toe_accessible" | "tic_tac_toe_hint_accessible" => {
                Screen::Game(GameId::TicTacToe)
            }
            "memory_pairs"
            | "memory_pairs_accessible"
            | "memory_pairs_hint"
            | "memory_pairs_hint_accessible" => Screen::Game(GameId::MemoryPairs),
            "sliding_puzzle"
            | "sliding_puzzle_accessible"
            | "sliding_puzzle_hint"
            | "sliding_puzzle_hint_accessible" => Screen::Game(GameId::SlidingPuzzle),
            "mastermind" | "mastermind_hint" | "mastermind_hint_accessible" => {
                Screen::Game(GameId::Mastermind)
            }
            "spider" => Screen::Game(GameId::Spider),
            "word_search" | "word_search_hint" | "word_search_hint_accessible" => {
                Screen::Game(GameId::WordSearch)
            }
            "hangman" | "hangman_hint" | "hangman_hint_accessible" => Screen::Game(GameId::Hangman),
            "connect_four" | "connect_four_hint" | "connect_four_hint_accessible" => {
                Screen::Game(GameId::ConnectFour)
            }
            "connect_four_accessible" => Screen::Game(GameId::ConnectFour),
            "checkers" | "checkers_hint" | "checkers_hint_accessible" => {
                Screen::Game(GameId::Checkers)
            }
            "checkers_accessible" => Screen::Game(GameId::Checkers),
            "peg_solitaire" | "peg_solitaire_hint" | "peg_solitaire_hint_accessible" => {
                Screen::Game(GameId::PegSolitaire)
            }
            "peg_solitaire_accessible" => Screen::Game(GameId::PegSolitaire),
            "mahjong_solitaire"
            | "mahjong_solitaire_hint"
            | "mahjong_solitaire_hint_accessible" => Screen::Game(GameId::MahjongSolitaire),
            "mahjong_solitaire_accessible" => Screen::Game(GameId::MahjongSolitaire),
            "snake" => Screen::Game(GameId::Snake),
            "snake_accessible" => Screen::Game(GameId::Snake),
            "breakout" => Screen::Game(GameId::Breakout),
            "breakout_accessible" => Screen::Game(GameId::Breakout),
            "higher_lower" => Screen::Game(GameId::HigherLower),
            "higher_lower_accessible" => Screen::Game(GameId::HigherLower),
            "klondike_golf" => Screen::Game(GameId::KlondikeGolf),
            "klondike_golf_accessible" => Screen::Game(GameId::KlondikeGolf),
            "klondike_golf_hint" | "klondike_golf_hint_accessible" => {
                Screen::Game(GameId::KlondikeGolf)
            }
            "blackjack" => Screen::Game(GameId::Blackjack),
            "blackjack_accessible" => Screen::Game(GameId::Blackjack),
            "spider_solitaire" => Screen::Game(GameId::SpiderSolitaire),
            "spider_solitaire_accessible" => Screen::Game(GameId::SpiderSolitaire),
            "spider_solitaire_hint" | "spider_solitaire_hint_accessible" => {
                Screen::Game(GameId::SpiderSolitaire)
            }
            "pyramid" => Screen::Game(GameId::Pyramid),
            "pyramid_accessible" => Screen::Game(GameId::Pyramid),
            "pyramid_hint" | "pyramid_hint_accessible" => Screen::Game(GameId::Pyramid),
            "tri_peaks" | "tri_peaks_accessible" => Screen::Game(GameId::TriPeaks),
            "nim" | "nim_accessible" | "nim_hint" | "nim_hint_accessible" => {
                Screen::Game(GameId::Nim)
            }
            "tri_peaks_hint" | "tri_peaks_hint_accessible" => Screen::Game(GameId::TriPeaks),
            "dungeon_sweeper" | "dungeon_sweeper_accessible" => {
                Screen::Game(GameId::DungeonSweeper)
            }
            "potion_2048" => Screen::Game(GameId::Potion2048),
            "potion_2048_accessible" => Screen::Game(GameId::Potion2048),
            "tiny_tower_defence" => Screen::Game(GameId::TinyTowerDefence),
            "one_room_roguelike" => Screen::Game(GameId::OneRoomRoguelike),
            "one_room_roguelike_accessible" => Screen::Game(GameId::OneRoomRoguelike),
            "daily_dungeon" => Screen::Game(GameId::DailyDungeon),
            "daily_dungeon_accessible" => Screen::Game(GameId::DailyDungeon),
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
        } else if scene == "pyramid_hint" || scene == "pyramid_hint_accessible" {
            self.state.card_hint = Some(card_hints::pyramid(&self.state));
        } else if scene == "tri_peaks_hint" || scene == "tri_peaks_hint_accessible" {
            self.state.card_hint = Some(card_hints::tri_peaks(&self.state));
        } else if scene == "klondike_golf_hint" || scene == "klondike_golf_hint_accessible" {
            self.state.card_hint = Some(card_hints::klondike_golf(&self.state));
        } else if scene == "spider_solitaire_hint" || scene == "spider_solitaire_hint_accessible" {
            self.state.card_hint = Some(card_hints::spider_solitaire(&self.state));
        } else if scene == "nim_hint" || scene == "nim_hint_accessible" {
            self.state.card_hint = Some(card_hints::nim(&self.state));
        } else if scene == "2048_hint" || scene == "2048_hint_accessible" {
            self.state.card_hint = Some(card_hints::game_2048(&self.state));
        } else if scene == "tic_tac_toe_hint" || scene == "tic_tac_toe_hint_accessible" {
            self.state.card_hint = Some(card_hints::tic_tac_toe(&self.state));
        } else if scene == "lights_out_hint" || scene == "lights_out_hint_accessible" {
            self.state.card_hint = Some(card_hints::lights_out(&self.state));
        } else if scene == "memory_pairs_hint" || scene == "memory_pairs_hint_accessible" {
            self.state.card_hint = Some(card_hints::memory_pairs(&self.state));
        } else if scene == "sliding_puzzle_hint" || scene == "sliding_puzzle_hint_accessible" {
            self.state.card_hint = Some(card_hints::sliding_puzzle(&self.state));
        } else if scene == "mastermind_hint" || scene == "mastermind_hint_accessible" {
            self.state.card_hint = Some(card_hints::mastermind(&self.state));
        } else if scene == "sudoku_hint" || scene == "sudoku_hint_accessible" {
            self.state.card_hint = Some(card_hints::sudoku(&self.state));
        } else if scene == "minesweeper_hint" || scene == "minesweeper_hint_accessible" {
            self.state.card_hint = Some(card_hints::minesweeper(&self.state));
        } else if scene == "nonogram_hint" || scene == "nonogram_hint_accessible" {
            self.state.card_hint = Some(card_hints::nonogram(&self.state));
        } else if scene == "word_search_hint" || scene == "word_search_hint_accessible" {
            self.state.card_hint = Some(card_hints::word_search(&self.state));
        } else if scene == "hangman_hint" || scene == "hangman_hint_accessible" {
            self.state.card_hint = Some(card_hints::hangman(&self.state));
        } else if scene == "connect_four_hint" || scene == "connect_four_hint_accessible" {
            self.state.card_hint = Some(card_hints::connect_four(&self.state));
        } else if scene == "checkers_hint" || scene == "checkers_hint_accessible" {
            self.state.card_hint = Some(card_hints::checkers(&self.state));
        } else if scene == "reversi_hint" || scene == "reversi_hint_accessible" {
            self.state.card_hint = Some(card_hints::reversi(&self.state));
        } else if scene == "peg_solitaire_hint" || scene == "peg_solitaire_hint_accessible" {
            self.state.card_hint = Some(card_hints::peg_solitaire(&self.state));
        } else if scene == "mahjong_solitaire_hint" || scene == "mahjong_solitaire_hint_accessible"
        {
            self.state.card_hint = Some(card_hints::mahjong_solitaire(&self.state));
        }
        if scene.starts_with("tutorial_") {
            if let Screen::Game(game) = self.state.screen {
                self.state.tutorial = Some(game);
            }
        }
        self.transition = 0.;
    }
}
