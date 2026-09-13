use crate::state::GameId;

pub(crate) fn round_is_complete(state: &crate::state::AppState, game: GameId) -> bool {
    match game {
        GameId::Game2048 => state.games.game.won(),
        GameId::Minesweeper => {
            state.games.minesweeper.status == crate::minesweeper::MineStatus::Won
        }
        GameId::Sudoku => state.games.sudoku.status == crate::sudoku::SudokuStatus::Won,
        GameId::Nonogram => state.games.nonogram.status == crate::nonogram::NonogramStatus::Won,
        GameId::Solitaire => state.games.solitaire.status == crate::solitaire::SolitaireStatus::Won,
        GameId::FreeCell => state.games.freecell.status == crate::freecell::FreeCellStatus::Won,
        GameId::Yahtzee => state.games.fivefold.status == crate::fivefold::FivefoldStatus::Complete,
        GameId::Reversi => state.games.reversi.status == crate::reversi::ReversiStatus::Won,
        GameId::LightsOut => {
            state.games.lights_out.status == crate::lights_out::LightsOutStatus::Won
        }
        GameId::TicTacToe => {
            state.games.tic_tac_toe.status
                == crate::tic_tac_toe::TicTacToeStatus::Won(crate::tic_tac_toe::Mark::X)
        }
        GameId::MemoryPairs => {
            state.games.memory_pairs.status == crate::memory_pairs::MemoryStatus::Won
        }
        GameId::SlidingPuzzle => {
            state.games.sliding_puzzle.status == crate::sliding_puzzle::SlidingStatus::Won
        }
        GameId::Mastermind => {
            state.games.mastermind.status == crate::mastermind::MastermindStatus::Won
        }
        GameId::Spider => state.games.spider.status == crate::spider::SpiderStatus::Won,
        GameId::WordSearch => {
            state.games.word_search.status == crate::word_search::WordSearchStatus::Won
        }
        GameId::Hangman => state.games.hangman.status == crate::hangman::HangmanStatus::Won,
        GameId::ConnectFour => {
            state.games.connect_four.status
                == crate::connect_four::ConnectFourStatus::Won(crate::connect_four::Disc::Red)
        }
        GameId::Checkers => {
            state.games.checkers.status
                == crate::checkers::CheckersStatus::Won(crate::checkers::Side::Red)
        }
        GameId::PegSolitaire => {
            state.games.peg_solitaire.status == crate::peg_solitaire::PegSolitaireStatus::Won
        }
        GameId::MahjongSolitaire => {
            state.games.mahjong_solitaire.status == crate::mahjong_solitaire::MahjongStatus::Won
        }
        GameId::HigherLower => {
            state.games.higher_lower.status == crate::higher_lower::HigherLowerStatus::Won
        }
        GameId::KlondikeGolf => {
            state.games.klondike_golf.status == crate::klondike_golf::GolfStatus::Won
        }
        GameId::Blackjack => state.games.blackjack.status == crate::blackjack::BlackjackStatus::Won,
        GameId::SpiderSolitaire => {
            state.games.spider_solitaire.status
                == crate::spider_solitaire::SpiderSolitaireStatus::Won
        }
        GameId::DungeonSweeper => {
            state.games.dungeon_sweeper.status == crate::dungeon_sweeper::DungeonStatus::Won
        }
        GameId::Potion2048 => state.games.potion_2048.won(),
        GameId::OneRoomRoguelike => state.games.one_room_roguelike.won(),
        GameId::DailyDungeon => state.games.daily_dungeon.won(),
        GameId::DotsBoxes => state.games.dots_boxes.won(),
        GameId::Sokoban => state.games.sokoban.won(),
        GameId::Mancala => state.games.mancala.won(),
        GameId::Hanoi => state.games.hanoi.won(),
        GameId::NumberMatch => state.games.number_match.won(),
        GameId::FloodIt => state.games.flood_it.won(),
        GameId::ColorSort => state.games.color_sort.won(),
        GameId::Battleship => state.games.battleship.won(),
        GameId::WordGrid => state.games.word_grid.won(),
        GameId::PipeLoop => state.games.pipe_loop.won(),
        GameId::MazeWalk => state.games.maze_walk.won(),
        GameId::MatchThree => state.games.match_three.won(),
        GameId::Pyramid => state.games.pyramid.status == crate::pyramid::PyramidStatus::Won,
        GameId::TriPeaks => state.games.tri_peaks.status == crate::tri_peaks::TriPeaksStatus::Won,
        GameId::Nim => state.games.nim.won(),
        GameId::WordLadder => {
            state.games.word_ladder.phase == crate::word_ladder::WordLadderPhase::Won
        }
        GameId::RiddleRoom => state.games.riddle_room.won(),
        GameId::PatternVault => state.games.pattern_vault.won(),
        GameId::SumCircuit => state.games.sum_circuit.won(),
        GameId::OrbitOrder => state.games.orbit_order.won(),
        GameId::WordForge => state.games.word_forge.won(),
        GameId::Snake
        | GameId::Breakout
        | GameId::TinyTowerDefence
        | GameId::SpaceInvaders
        | GameId::Asteroids
        | GameId::Frogger => false,
        GameId::MunchMaze
        | GameId::BlockStack
        | GameId::TerrainCannon
        | GameId::FlingFury
        | GameId::PaddleDuel => false,
    }
}
