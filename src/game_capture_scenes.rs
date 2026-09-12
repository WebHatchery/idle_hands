//! Explicit routing for non-canonical capture scenes.

use crate::state::{GameId, Screen};

pub(super) fn special_screen_for_scene(scene: &str) -> Screen {
    match scene {
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
        "fivefold" | "fivefold_hint" | "fivefold_hint_accessible" => Screen::Game(GameId::Yahtzee),
        "reversi" | "reversi_hint" | "reversi_hint_accessible" => Screen::Game(GameId::Reversi),
        "reversi_accessible" => Screen::Game(GameId::Reversi),
        "lights_out"
        | "lights_out_accessible"
        | "lights_out_hint"
        | "lights_out_hint_accessible"
        | "lights_out_solver" => Screen::Game(GameId::LightsOut),
        "tic_tac_toe" | "tic_tac_toe_hint" => Screen::Game(GameId::TicTacToe),
        "tic_tac_toe_accessible" | "tic_tac_toe_hint_accessible" => Screen::Game(GameId::TicTacToe),
        "memory_pairs"
        | "memory_pairs_accessible"
        | "memory_pairs_hint"
        | "memory_pairs_hint_accessible"
        | "memory_pairs_memory" => Screen::Game(GameId::MemoryPairs),
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
        "hangman" | "hangman_hint" | "hangman_hint_accessible" | "hangman_depth" => {
            Screen::Game(GameId::Hangman)
        }
        "connect_four" | "connect_four_hint" | "connect_four_hint_accessible" => {
            Screen::Game(GameId::ConnectFour)
        }
        "connect_four_accessible" => Screen::Game(GameId::ConnectFour),
        "checkers" | "checkers_hint" | "checkers_hint_accessible" => Screen::Game(GameId::Checkers),
        "checkers_accessible" => Screen::Game(GameId::Checkers),
        "peg_solitaire" | "peg_solitaire_hint" | "peg_solitaire_hint_accessible" => {
            Screen::Game(GameId::PegSolitaire)
        }
        "peg_solitaire_accessible" => Screen::Game(GameId::PegSolitaire),
        "mahjong_solitaire" | "mahjong_solitaire_hint" | "mahjong_solitaire_hint_accessible" => {
            Screen::Game(GameId::MahjongSolitaire)
        }
        "mahjong_solitaire_accessible" => Screen::Game(GameId::MahjongSolitaire),
        "snake" | "snake_hint" | "snake_hint_accessible" | "snake_garden" => {
            Screen::Game(GameId::Snake)
        }
        "snake_accessible" => Screen::Game(GameId::Snake),
        "breakout" | "breakout_hint" | "breakout_hint_accessible" | "breakout_wall_two" => {
            Screen::Game(GameId::Breakout)
        }
        "breakout_accessible" => Screen::Game(GameId::Breakout),
        "higher_lower"
        | "higher_lower_hint"
        | "higher_lower_hint_accessible"
        | "higher_lower_stakes" => Screen::Game(GameId::HigherLower),
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
        "pyramid" | "pyramid_chains" => Screen::Game(GameId::Pyramid),
        "pyramid_accessible" => Screen::Game(GameId::Pyramid),
        "pyramid_hint" | "pyramid_hint_accessible" => Screen::Game(GameId::Pyramid),
        "tri_peaks" | "tri_peaks_accessible" | "tri_peaks_runs" => Screen::Game(GameId::TriPeaks),
        "nim" | "nim_accessible" | "nim_hint" | "nim_hint_accessible" | "nim_tactics" => {
            Screen::Game(GameId::Nim)
        }
        "tri_peaks_hint" | "tri_peaks_hint_accessible" => Screen::Game(GameId::TriPeaks),
        "dungeon_sweeper"
        | "dungeon_sweeper_accessible"
        | "dungeon_sweeper_hint"
        | "dungeon_sweeper_hint_accessible"
        | "dungeon_relics" => Screen::Game(GameId::DungeonSweeper),
        "potion_2048" | "potion_2048_hint" | "potion_2048_hint_accessible" | "potion_catalyst" => {
            Screen::Game(GameId::Potion2048)
        }
        "potion_2048_accessible" => Screen::Game(GameId::Potion2048),
        "tiny_tower_defence" | "tiny_tower_defence_hint" | "tiny_tower_roles" => {
            Screen::Game(GameId::TinyTowerDefence)
        }
        "tiny_tower_defence_accessible" | "tiny_tower_defence_hint_accessible" => {
            Screen::Game(GameId::TinyTowerDefence)
        }
        "one_room_roguelike"
        | "one_room_roguelike_hint"
        | "one_room_roguelike_hint_accessible"
        | "rogue_roles" => Screen::Game(GameId::OneRoomRoguelike),
        "one_room_roguelike_accessible" => Screen::Game(GameId::OneRoomRoguelike),
        "daily_dungeon"
        | "daily_dungeon_hint"
        | "daily_dungeon_hint_accessible"
        | "daily_scouting" => Screen::Game(GameId::DailyDungeon),
        "daily_dungeon_accessible" => Screen::Game(GameId::DailyDungeon),
        "dots_boxes" | "dots_boxes_hint" | "dots_boxes_hint_accessible" => {
            Screen::Game(GameId::DotsBoxes)
        }
        "dots_tactics" => Screen::Game(GameId::DotsBoxes),
        "dots_boxes_accessible" => Screen::Game(GameId::DotsBoxes),
        "sokoban" | "sokoban_hint" | "sokoban_hint_accessible" | "sokoban_deadlock" => {
            Screen::Game(GameId::Sokoban)
        }
        "sokoban_accessible" => Screen::Game(GameId::Sokoban),
        "mancala" | "mancala_hint" | "mancala_hint_accessible" | "mancala_tactics" => {
            Screen::Game(GameId::Mancala)
        }
        "hanoi" | "hanoi_hint" | "hanoi_hint_accessible" | "hanoi_master" => {
            Screen::Game(GameId::Hanoi)
        }
        "number_match" | "number_match_hint" | "number_match_hint_accessible" => {
            Screen::Game(GameId::NumberMatch)
        }
        "number_match_links" => Screen::Game(GameId::NumberMatch),
        "number_match_accessible" => Screen::Game(GameId::NumberMatch),
        "flood_it" | "flood_it_hint" | "flood_it_hint_accessible" => Screen::Game(GameId::FloodIt),
        "flood_surges" => Screen::Game(GameId::FloodIt),
        "flood_it_accessible" => Screen::Game(GameId::FloodIt),
        "color_sort" | "color_sort_hint" | "color_sort_hint_accessible" => {
            Screen::Game(GameId::ColorSort)
        }
        "color_sort_runs" => Screen::Game(GameId::ColorSort),
        "color_sort_accessible" => Screen::Game(GameId::ColorSort),
        "battleship" | "battleship_hint" | "battleship_hint_accessible" => {
            Screen::Game(GameId::Battleship)
        }
        "battleship_sonar" => Screen::Game(GameId::Battleship),
        "battleship_accessible" => Screen::Game(GameId::Battleship),
        "word_grid" | "word_grid_hint" | "word_grid_hint_accessible" => {
            Screen::Game(GameId::WordGrid)
        }
        "word_grid_deduction" => Screen::Game(GameId::WordGrid),
        "word_grid_accessible" => Screen::Game(GameId::WordGrid),
        "word_ladder"
        | "word_ladder_hint"
        | "word_ladder_hint_accessible"
        | "word_ladder_best"
        | "word_ladder_progress"
        | "word_ladder_routes"
        | "word_ladder_confirm" => Screen::Game(GameId::WordLadder),
        "pipe_loop" | "pipe_loop_hint" | "pipe_loop_hint_accessible" => {
            Screen::Game(GameId::PipeLoop)
        }
        "pipe_network" => Screen::Game(GameId::PipeLoop),
        "pipe_loop_accessible" => Screen::Game(GameId::PipeLoop),
        "maze_walk" | "maze_walk_hint" | "maze_walk_hint_accessible" => {
            Screen::Game(GameId::MazeWalk)
        }
        "maze_beacons" => Screen::Game(GameId::MazeWalk),
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
        | "records_cards"
        | "records_arcade"
        | "daily_archive"
        | "daily_archive_accessible"
        | "daily_archive_scrolled"
        | "achievements"
        | "achievements_accessible"
        | "achievements_earned"
        | "achievements_locked"
        | "favorites_browse"
        | "favorites_info"
        | "favorites_all"
        | "recent_browse"
        | "recent_info" => Screen::Records,
        "statistics" | "statistics_accessible" => Screen::Statistics,
        "tutorials" | "tutorials_accessible" => Screen::Tutorials,
        "finder" | "finder_filtered" | "finder_scrolled" => Screen::Finder,
        "profile" | "profile_accessible" => Screen::Profile,
        "drawer_info" | "drawer_info_accessible" => Screen::DrawerInfo(GameId::Solitaire),
        "rules" | "rules_scrolled" | "rules_logic" | "rules_word" => Screen::Rules,
        "credits" => Screen::Credits,
        "settings" | "settings_accessible" | "settings_reset" => Screen::Settings,
        _ => Screen::Cabinet,
    }
}
