use super::Game;
use crate::{
    card_hints,
    state::{GameId, Screen},
};
use macroquad_toolkit::notifications::NotificationManager;

impl Game {
    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.state = crate::state::AppState::new(&self.data);
        self.capture_solitaire_peek = false;
        self.notifications = NotificationManager::new();
        let scene = scene
            .strip_prefix("portrait_")
            .or_else(|| scene.strip_prefix("landscape_"))
            .unwrap_or(scene);
        let tutorial = scene.starts_with("tutorial_");
        let scene = scene.strip_prefix("tutorial_").unwrap_or(scene);
        self.state.screen = match scene {
            "2048" | "2048_confirm" | "gameplay" | "2048_hint" | "2048_hint_accessible" => {
                Screen::Game(GameId::Game2048)
            }
            "minesweeper"
            | "minesweeper_accessible"
            | "minesweeper_hint"
            | "minesweeper_hint_accessible"
            | "minesweeper_confirm" => Screen::Game(GameId::Minesweeper),
            "sudoku" | "sudoku_accessible" | "sudoku_hint" | "sudoku_hint_accessible" => {
                Screen::Game(GameId::Sudoku)
            }
            "nonogram"
            | "nonogram_large"
            | "nonogram_accessible"
            | "nonogram_hint"
            | "nonogram_hint_accessible" => Screen::Game(GameId::Nonogram),
            "solitaire" | "solitaire_hint" | "solitaire_selected" | "solitaire_peek" => {
                Screen::Game(GameId::Solitaire)
            }
            "freecell" | "freecell_hint" | "freecell_selected" => Screen::Game(GameId::FreeCell),
            "fivefold" | "fivefold_hint" | "fivefold_hint_accessible" => {
                Screen::Game(GameId::Yahtzee)
            }
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
            "spider" | "spider_hint" | "spider_hint_accessible" => Screen::Game(GameId::Spider),
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
            "snake" | "snake_hint" | "snake_hint_accessible" => Screen::Game(GameId::Snake),
            "snake_accessible" => Screen::Game(GameId::Snake),
            "breakout" | "breakout_hint" | "breakout_hint_accessible" | "breakout_wall_two" => {
                Screen::Game(GameId::Breakout)
            }
            "breakout_accessible" => Screen::Game(GameId::Breakout),
            "higher_lower" | "higher_lower_hint" | "higher_lower_hint_accessible" => {
                Screen::Game(GameId::HigherLower)
            }
            "higher_lower_accessible" => Screen::Game(GameId::HigherLower),
            "klondike_golf" => Screen::Game(GameId::KlondikeGolf),
            "klondike_golf_accessible" => Screen::Game(GameId::KlondikeGolf),
            "klondike_golf_hint" | "klondike_golf_hint_accessible" => {
                Screen::Game(GameId::KlondikeGolf)
            }
            "blackjack" | "blackjack_hint" | "blackjack_hint_accessible" => {
                Screen::Game(GameId::Blackjack)
            }
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
            "dungeon_sweeper"
            | "dungeon_sweeper_accessible"
            | "dungeon_sweeper_hint"
            | "dungeon_sweeper_hint_accessible" => Screen::Game(GameId::DungeonSweeper),
            "potion_2048" | "potion_2048_hint" | "potion_2048_hint_accessible" => {
                Screen::Game(GameId::Potion2048)
            }
            "potion_2048_accessible" => Screen::Game(GameId::Potion2048),
            "tiny_tower_defence" | "tiny_tower_defence_hint" => {
                Screen::Game(GameId::TinyTowerDefence)
            }
            "tiny_tower_defence_accessible" | "tiny_tower_defence_hint_accessible" => {
                Screen::Game(GameId::TinyTowerDefence)
            }
            "one_room_roguelike"
            | "one_room_roguelike_hint"
            | "one_room_roguelike_hint_accessible" => Screen::Game(GameId::OneRoomRoguelike),
            "one_room_roguelike_accessible" => Screen::Game(GameId::OneRoomRoguelike),
            "daily_dungeon" | "daily_dungeon_hint" | "daily_dungeon_hint_accessible" => {
                Screen::Game(GameId::DailyDungeon)
            }
            "daily_dungeon_accessible" => Screen::Game(GameId::DailyDungeon),
            "dots_boxes" | "dots_boxes_hint" | "dots_boxes_hint_accessible" => {
                Screen::Game(GameId::DotsBoxes)
            }
            "dots_boxes_accessible" => Screen::Game(GameId::DotsBoxes),
            "sokoban" | "sokoban_hint" | "sokoban_hint_accessible" => Screen::Game(GameId::Sokoban),
            "sokoban_accessible" => Screen::Game(GameId::Sokoban),
            "mancala" | "mancala_hint" | "mancala_hint_accessible" => Screen::Game(GameId::Mancala),
            "hanoi" | "hanoi_hint" | "hanoi_hint_accessible" => Screen::Game(GameId::Hanoi),
            "number_match" | "number_match_hint" | "number_match_hint_accessible" => {
                Screen::Game(GameId::NumberMatch)
            }
            "number_match_accessible" => Screen::Game(GameId::NumberMatch),
            "flood_it" | "flood_it_hint" | "flood_it_hint_accessible" => {
                Screen::Game(GameId::FloodIt)
            }
            "flood_it_accessible" => Screen::Game(GameId::FloodIt),
            "color_sort" | "color_sort_hint" | "color_sort_hint_accessible" => {
                Screen::Game(GameId::ColorSort)
            }
            "color_sort_accessible" => Screen::Game(GameId::ColorSort),
            "battleship" | "battleship_hint" | "battleship_hint_accessible" => {
                Screen::Game(GameId::Battleship)
            }
            "battleship_accessible" => Screen::Game(GameId::Battleship),
            "word_grid" | "word_grid_hint" | "word_grid_hint_accessible" => {
                Screen::Game(GameId::WordGrid)
            }
            "word_grid_accessible" => Screen::Game(GameId::WordGrid),
            "word_ladder"
            | "word_ladder_hint"
            | "word_ladder_hint_accessible"
            | "word_ladder_best"
            | "word_ladder_progress"
            | "word_ladder_confirm" => Screen::Game(GameId::WordLadder),
            "pipe_loop" | "pipe_loop_hint" | "pipe_loop_hint_accessible" => {
                Screen::Game(GameId::PipeLoop)
            }
            "pipe_loop_accessible" => Screen::Game(GameId::PipeLoop),
            "maze_walk" | "maze_walk_hint" | "maze_walk_hint_accessible" => {
                Screen::Game(GameId::MazeWalk)
            }
            "match_three"
            | "match_three_accessible"
            | "match_three_confirm"
            | "match_three_hint"
            | "match_three_hint_accessible"
            | "match_three_specials" => Screen::Game(GameId::MatchThree),
            "mastermind_accessible" => Screen::Game(GameId::Mastermind),
            "help" => Screen::Help,
            "records"
            | "records_scrolled"
            | "records_accessible"
            | "records_word_ladder"
            | "records_progress"
            | "achievements"
            | "achievements_accessible"
            | "achievements_earned"
            | "achievements_locked"
            | "favorites_browse"
            | "favorites_all"
            | "recent_browse" => Screen::Records,
            "rules" | "rules_scrolled" => Screen::Rules,
            "credits" => Screen::Credits,
            "settings" | "settings_accessible" | "settings_reset" => Screen::Settings,
            _ => Screen::Cabinet,
        };
        if scene == "settings_reset" {
            self.state.confirm_reset = true;
        }
        if scene == "cabinet_scrolled" {
            self.state.cabinet_filter = 9;
            self.state.cabinet_scroll = 6;
        }
        if matches!(scene, "records_scrolled" | "rules_scrolled") {
            self.state.library_scroll = 12;
        }
        if scene == "2048_confirm" {
            self.state.confirm_restart = true;
            self.state.pending_restart = Some(crate::ui::UiAction::Restart);
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
        if scene == "breakout_wall_two" {
            let game = &mut self.state.breakout;
            game.level = 2;
            game.lives = 2;
            game.score = 64;
            game.paused = true;
            game.serve_ready = true;
            game.bricks = vec![false; 64];
            game.brick_health = vec![0; 64];
            for index in 0_usize..64 {
                let row = index / 16;
                let column = index % 16;
                if !(row + column).is_multiple_of(5) {
                    let health = if row == 0 || column.is_multiple_of(4) {
                        2
                    } else {
                        1
                    };
                    game.bricks[index] = true;
                    game.brick_health[index] = health;
                }
            }
        }
        if scene == "solitaire_peek" {
            self.capture_solitaire_peek = true;
            self.state.solitaire.tableau[0] = (1..=13)
                .map(|rank| crate::cards::Card {
                    rank,
                    suit: 3,
                    face_up: true,
                })
                .collect();
            self.state.solitaire_peek = Some(crate::solitaire::CardSource::Tableau(0, 6));
        }
        if scene == "cabinet_favorites" {
            for index in [0, GameId::Spider.index(), GameId::Nim.index()] {
                self.state.favorites[index] = true;
            }
        }
        if scene == "cabinet_recent" {
            self.state.recent_games = vec![
                GameId::WordLadder,
                GameId::Spider,
                GameId::Minesweeper,
                GameId::Solitaire,
            ];
        }
        if matches!(scene, "cabinet_open" | "cabinet_done" | "cabinet_empty") {
            self.state.records.best_2048 = 2048;
            self.state.records.solitaire_best_moves = Some(42);
            self.state.cabinet_filter = if scene == "cabinet_empty" {
                self.state.records = crate::state::CollectionRecords::default();
                2
            } else if scene == "cabinet_done" {
                2
            } else {
                1
            };
        }
        if scene == "match_three_confirm" {
            self.state.confirm_restart = true;
            self.state.pending_restart = Some(crate::ui::UiAction::MatchThreeNew);
        }
        if scene == "match_three_specials" {
            use crate::match_three::MatchThreeSpecial;
            let game = &mut self.state.match_three;
            game.specials = vec![MatchThreeSpecial::None; game.cells.len()];
            for (index, special) in [
                (16, MatchThreeSpecial::Row),
                (24, MatchThreeSpecial::Burst),
                (32, MatchThreeSpecial::Column),
            ] {
                game.specials[index] = special;
            }
            game.score = 90;
            game.moves = 6;
            game.last_cascade = 3;
            game.best_cascade = 3;
        }
        if scene == "minesweeper_confirm" {
            self.state.confirm_restart = true;
            self.state.pending_restart = Some(crate::ui::UiAction::MineRestart);
        }
        if scene == "word_ladder_best" {
            self.state.records.word_ladder_best_moves = Some(5);
        }
        if scene == "word_ladder_progress" {
            self.state.word_ladder.reset(0);
            for letter in b"PLATE" {
                self.state.word_ladder.tap_letter(letter - b'A');
            }
            self.state.word_ladder.submit();
        }
        if scene == "word_ladder_confirm" {
            self.state.confirm_restart = true;
            self.state.pending_restart = Some(crate::ui::UiAction::WordLadderNew);
        }
        if scene == "records_word_ladder" {
            self.state.records.word_ladder_best_moves = Some(5);
        }
        if scene == "records_progress" {
            self.state.records.best_2048 = 2048;
            self.state.records.solitaire_best_moves = Some(42);
            self.state.records.word_ladder_best_moves = Some(5);
            let _ = crate::progression::sync(
                &mut self.state.achievements,
                &mut self.state.stamps,
                &self.state.records,
            );
        }
        if matches!(
            scene,
            "achievements"
                | "achievements_accessible"
                | "achievements_earned"
                | "achievements_locked"
        ) {
            self.state.records.best_2048 = 2048;
            self.state.records.solitaire_best_moves = Some(42);
            self.state.records.word_ladder_best_moves = Some(5);
            let _ = crate::progression::sync(
                &mut self.state.achievements,
                &mut self.state.stamps,
                &self.state.records,
            );
            self.state.achievements_view = true;
            self.state.achievement_filter = if scene == "achievements_earned" {
                1
            } else if scene == "achievements_locked" {
                2
            } else {
                0
            };
        }
        if scene == "favorites_browse" {
            for index in [
                0,
                GameId::Spider.index(),
                GameId::Nim.index(),
                GameId::WordLadder.index(),
            ] {
                self.state.favorites[index] = true;
            }
            self.state.favorites_view = true;
        }
        if scene == "favorites_all" {
            self.state.favorites.fill(true);
            self.state.favorites_view = true;
        }
        if scene == "recent_browse" {
            self.state.recent_games = vec![
                GameId::WordLadder,
                GameId::Spider,
                GameId::Minesweeper,
                GameId::Solitaire,
                GameId::Game2048,
            ];
            self.state.recent_view = true;
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
        } else if scene == "snake_hint" || scene == "snake_hint_accessible" {
            self.state.card_hint = Some(card_hints::snake(&self.state));
        } else if scene == "breakout_hint" || scene == "breakout_hint_accessible" {
            self.state.card_hint = Some(card_hints::breakout(&self.state));
        } else if scene == "higher_lower_hint" || scene == "higher_lower_hint_accessible" {
            self.state.card_hint = Some(card_hints::higher_lower(&self.state));
        } else if scene == "blackjack_hint" || scene == "blackjack_hint_accessible" {
            self.state.card_hint = Some(card_hints::blackjack(&self.state));
        } else if scene == "dungeon_sweeper_hint" || scene == "dungeon_sweeper_hint_accessible" {
            self.state.card_hint = Some(card_hints::dungeon_sweeper(&self.state));
        } else if scene == "potion_2048_hint" || scene == "potion_2048_hint_accessible" {
            self.state.card_hint = Some(card_hints::potion_2048(&self.state));
        } else if scene == "tiny_tower_defence_hint"
            || scene == "tiny_tower_defence_hint_accessible"
        {
            self.state.card_hint = Some(card_hints::tiny_tower_defence(&self.state));
        } else if scene == "one_room_roguelike_hint"
            || scene == "one_room_roguelike_hint_accessible"
        {
            self.state.card_hint = Some(card_hints::one_room_roguelike(&self.state));
        } else if scene == "daily_dungeon_hint" || scene == "daily_dungeon_hint_accessible" {
            self.state.card_hint = Some(card_hints::daily_dungeon(&self.state));
        } else if scene == "dots_boxes_hint" || scene == "dots_boxes_hint_accessible" {
            self.state.card_hint = Some(card_hints::dots_boxes(&self.state));
        } else if scene == "sokoban_hint" || scene == "sokoban_hint_accessible" {
            self.state.card_hint = Some(card_hints::sokoban(&self.state));
        } else if scene == "mancala_hint" || scene == "mancala_hint_accessible" {
            self.state.card_hint = Some(card_hints::mancala(&self.state));
        } else if scene == "hanoi_hint" || scene == "hanoi_hint_accessible" {
            self.state.card_hint = Some(card_hints::hanoi(&self.state));
        } else if scene == "number_match_hint" || scene == "number_match_hint_accessible" {
            self.state.card_hint = Some(card_hints::number_match(&self.state));
        } else if scene == "flood_it_hint" || scene == "flood_it_hint_accessible" {
            self.state.card_hint = Some(card_hints::flood_it(&self.state));
        } else if scene == "color_sort_hint" || scene == "color_sort_hint_accessible" {
            self.state.card_hint = Some(card_hints::color_sort(&self.state));
        } else if scene == "battleship_hint" || scene == "battleship_hint_accessible" {
            self.state.card_hint = Some(card_hints::battleship(&self.state));
        } else if scene == "word_grid_hint" || scene == "word_grid_hint_accessible" {
            self.state.card_hint = Some(card_hints::word_grid(&self.state));
        } else if scene == "word_ladder_hint" || scene == "word_ladder_hint_accessible" {
            self.state.card_hint = Some(card_hints::word_ladder(&self.state));
        } else if scene == "pipe_loop_hint" || scene == "pipe_loop_hint_accessible" {
            self.state.card_hint = Some(card_hints::pipe_loop(&self.state));
        } else if scene == "maze_walk_hint" || scene == "maze_walk_hint_accessible" {
            self.state.card_hint = Some(card_hints::maze_walk(&self.state));
        } else if scene == "fivefold_hint" || scene == "fivefold_hint_accessible" {
            self.state.card_hint = Some(card_hints::fivefold(&self.state));
        } else if scene == "spider_hint" || scene == "spider_hint_accessible" {
            self.state.card_hint = Some(card_hints::spider(&self.state));
        } else if scene == "match_three_hint" || scene == "match_three_hint_accessible" {
            self.state.card_hint = Some(card_hints::match_three(&self.state));
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
        if tutorial {
            if let Screen::Game(game) = self.state.screen {
                self.state.tutorial = Some(game);
            }
        }
        self.transition = 0.;
    }
}
