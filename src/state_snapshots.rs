//! Independent active-game snapshot storage.

use crate::battleship::Battleship;
use crate::color_sort::ColorSort;
use crate::daily_dungeon::DailyDungeon;
use crate::dots_boxes::DotsBoxes;
use crate::dungeon_sweeper::DungeonSweeper;
use crate::flood_it::FloodIt;
use crate::hanoi::Hanoi;
use crate::mancala::Mancala;
use crate::match_three::MatchThree;
use crate::maze_walk::MazeWalk;
use crate::nim::Nim;
use crate::number_match::NumberMatch;
use crate::one_room_roguelike::OneRoomRoguelike;
use crate::pipe_loop::PipeLoop;
use crate::potion_2048::Potion2048;
use crate::pyramid::Pyramid;
use crate::state::Game2048;
use crate::state::{AppState, GameId};
use crate::tiny_tower_defence::TinyTowerDefence;
use crate::tri_peaks::TriPeaks;
use crate::word_ladder::WordLadder;
use crate::{
    blackjack::Blackjack, breakout::Breakout, checkers::Checkers, connect_four::ConnectFour,
    fivefold::Fivefold, freecell::FreeCell, hangman::Hangman, higher_lower::HigherLower,
    klondike_golf::KlondikeGolf, lights_out::LightsOut, mahjong_solitaire::MahjongSolitaire,
    mastermind::Mastermind, memory_pairs::MemoryPairs, minesweeper::Minesweeper,
    nonogram::Nonogram, peg_solitaire::PegSolitaire, reversi::Reversi,
    sliding_puzzle::SlidingPuzzle, snake::Snake, sokoban::Sokoban, solitaire::Solitaire,
    spider::Spider, spider_solitaire::SpiderSolitaire, sudoku::Sudoku, tic_tac_toe::TicTacToe,
    word_grid::WordGrid, word_search::WordSearch,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameSnapshot {
    Game2048(Game2048),
    Minesweeper(Minesweeper),
    Sudoku(Sudoku),
    Nonogram(Nonogram),
    Solitaire(Solitaire),
    FreeCell(FreeCell),
    Fivefold(Fivefold),
    Reversi(Reversi),
    LightsOut(LightsOut),
    TicTacToe(TicTacToe),
    MemoryPairs(MemoryPairs),
    SlidingPuzzle(SlidingPuzzle),
    Mastermind(Mastermind),
    Spider(Spider),
    WordSearch(WordSearch),
    Hangman(Hangman),
    ConnectFour(ConnectFour),
    Checkers(Checkers),
    PegSolitaire(PegSolitaire),
    MahjongSolitaire(MahjongSolitaire),
    Snake(Snake),
    Breakout(Breakout),
    HigherLower(HigherLower),
    KlondikeGolf(KlondikeGolf),
    Blackjack(Blackjack),
    SpiderSolitaire(SpiderSolitaire),
    DungeonSweeper(DungeonSweeper),
    Potion2048(Potion2048),
    TinyTowerDefence(TinyTowerDefence),
    OneRoomRoguelike(OneRoomRoguelike),
    DailyDungeon(DailyDungeon),
    DotsBoxes(DotsBoxes),
    Sokoban(Sokoban),
    Mancala(Mancala),
    Hanoi(Hanoi),
    NumberMatch(NumberMatch),
    FloodIt(FloodIt),
    ColorSort(ColorSort),
    Battleship(Battleship),
    WordGrid(WordGrid),
    PipeLoop(PipeLoop),
    MazeWalk(MazeWalk),
    MatchThree(MatchThree),
    Pyramid(Pyramid),
    TriPeaks(TriPeaks),
    Nim(Nim),
    WordLadder(WordLadder),
}

impl GameSnapshot {
    pub fn from_state(state: &AppState, game: GameId) -> Self {
        match game {
            GameId::Game2048 => Self::Game2048(state.games.game.clone()),
            GameId::Minesweeper => Self::Minesweeper(state.games.minesweeper.clone()),
            GameId::Sudoku => Self::Sudoku(state.games.sudoku.clone()),
            GameId::Nonogram => Self::Nonogram(state.games.nonogram.clone()),
            GameId::Solitaire => Self::Solitaire(state.games.solitaire.clone()),
            GameId::FreeCell => Self::FreeCell(state.games.freecell.clone()),
            GameId::Yahtzee => Self::Fivefold(state.games.fivefold.clone()),
            GameId::Reversi => Self::Reversi(state.games.reversi.clone()),
            GameId::LightsOut => Self::LightsOut(state.games.lights_out.clone()),
            GameId::TicTacToe => Self::TicTacToe(state.games.tic_tac_toe.clone()),
            GameId::MemoryPairs => Self::MemoryPairs(state.games.memory_pairs.clone()),
            GameId::SlidingPuzzle => Self::SlidingPuzzle(state.games.sliding_puzzle.clone()),
            GameId::Mastermind => Self::Mastermind(state.games.mastermind.clone()),
            GameId::Spider => Self::Spider(state.games.spider.clone()),
            GameId::WordSearch => Self::WordSearch(state.games.word_search.clone()),
            GameId::Hangman => Self::Hangman(state.games.hangman.clone()),
            GameId::ConnectFour => Self::ConnectFour(state.games.connect_four.clone()),
            GameId::Checkers => Self::Checkers(state.games.checkers.clone()),
            GameId::PegSolitaire => Self::PegSolitaire(state.games.peg_solitaire.clone()),
            GameId::MahjongSolitaire => {
                Self::MahjongSolitaire(state.games.mahjong_solitaire.clone())
            }
            GameId::Snake => Self::Snake(state.games.snake.clone()),
            GameId::Breakout => Self::Breakout(state.games.breakout.clone()),
            GameId::HigherLower => Self::HigherLower(state.games.higher_lower.clone()),
            GameId::KlondikeGolf => Self::KlondikeGolf(state.games.klondike_golf.clone()),
            GameId::Blackjack => Self::Blackjack(state.games.blackjack.clone()),
            GameId::SpiderSolitaire => Self::SpiderSolitaire(state.games.spider_solitaire.clone()),
            GameId::DungeonSweeper => Self::DungeonSweeper(state.games.dungeon_sweeper.clone()),
            GameId::Potion2048 => Self::Potion2048(state.games.potion_2048.clone()),
            GameId::TinyTowerDefence => {
                Self::TinyTowerDefence(state.games.tiny_tower_defence.clone())
            }
            GameId::OneRoomRoguelike => {
                Self::OneRoomRoguelike(state.games.one_room_roguelike.clone())
            }
            GameId::DailyDungeon => Self::DailyDungeon(state.games.daily_dungeon.clone()),
            GameId::DotsBoxes => Self::DotsBoxes(state.games.dots_boxes.clone()),
            GameId::Sokoban => Self::Sokoban(state.games.sokoban.clone()),
            GameId::Mancala => Self::Mancala(state.games.mancala.clone()),
            GameId::Hanoi => Self::Hanoi(state.games.hanoi.clone()),
            GameId::NumberMatch => Self::NumberMatch(state.games.number_match.clone()),
            GameId::FloodIt => Self::FloodIt(state.games.flood_it.clone()),
            GameId::ColorSort => Self::ColorSort(state.games.color_sort.clone()),
            GameId::Battleship => Self::Battleship(state.games.battleship.clone()),
            GameId::WordGrid => Self::WordGrid(state.games.word_grid.clone()),
            GameId::PipeLoop => Self::PipeLoop(state.games.pipe_loop.clone()),
            GameId::MazeWalk => Self::MazeWalk(state.games.maze_walk.clone()),
            GameId::MatchThree => Self::MatchThree(state.games.match_three.clone()),
            GameId::Pyramid => Self::Pyramid(state.games.pyramid.clone()),
            GameId::TriPeaks => Self::TriPeaks(state.games.tri_peaks.clone()),
            GameId::Nim => Self::Nim(state.games.nim.clone()),
            GameId::WordLadder => Self::WordLadder(state.games.word_ladder.clone()),
        }
    }

    pub fn apply_to(self, state: &mut AppState) {
        match self {
            Self::Game2048(game) => state.games.game = game,
            Self::Minesweeper(game) => state.games.minesweeper = game,
            Self::Sudoku(game) => state.games.sudoku = game,
            Self::Nonogram(game) => state.games.nonogram = game,
            Self::Solitaire(game) => state.games.solitaire = game,
            Self::FreeCell(game) => state.games.freecell = game,
            Self::Fivefold(game) => state.games.fivefold = game,
            Self::Reversi(game) => state.games.reversi = game,
            Self::LightsOut(game) => state.games.lights_out = game,
            Self::TicTacToe(game) => state.games.tic_tac_toe = game,
            Self::MemoryPairs(game) => state.games.memory_pairs = game,
            Self::SlidingPuzzle(game) => state.games.sliding_puzzle = game,
            Self::Mastermind(game) => state.games.mastermind = game,
            Self::Spider(game) => state.games.spider = game,
            Self::WordSearch(game) => state.games.word_search = game,
            Self::Hangman(game) => state.games.hangman = game,
            Self::ConnectFour(game) => state.games.connect_four = game,
            Self::Checkers(game) => state.games.checkers = game,
            Self::PegSolitaire(game) => state.games.peg_solitaire = game,
            Self::MahjongSolitaire(game) => state.games.mahjong_solitaire = game,
            Self::Snake(game) => state.games.snake = game,
            Self::Breakout(game) => state.games.breakout = game,
            Self::HigherLower(game) => state.games.higher_lower = game,
            Self::KlondikeGolf(game) => state.games.klondike_golf = game,
            Self::Blackjack(game) => state.games.blackjack = game,
            Self::SpiderSolitaire(game) => state.games.spider_solitaire = game,
            Self::DungeonSweeper(game) => state.games.dungeon_sweeper = game,
            Self::Potion2048(game) => state.games.potion_2048 = game,
            Self::TinyTowerDefence(game) => state.games.tiny_tower_defence = game,
            Self::OneRoomRoguelike(game) => state.games.one_room_roguelike = game,
            Self::DailyDungeon(game) => state.games.daily_dungeon = game,
            Self::DotsBoxes(game) => state.games.dots_boxes = game,
            Self::Sokoban(game) => state.games.sokoban = game,
            Self::Mancala(game) => state.games.mancala = game,
            Self::Hanoi(game) => state.games.hanoi = game,
            Self::NumberMatch(game) => state.games.number_match = game,
            Self::FloodIt(game) => state.games.flood_it = game,
            Self::ColorSort(game) => state.games.color_sort = game,
            Self::Battleship(game) => state.games.battleship = game,
            Self::WordGrid(game) => state.games.word_grid = game,
            Self::PipeLoop(game) => state.games.pipe_loop = game,
            Self::MazeWalk(game) => state.games.maze_walk = game,
            Self::MatchThree(game) => state.games.match_three = game,
            Self::Pyramid(game) => state.games.pyramid = game,
            Self::TriPeaks(game) => state.games.tri_peaks = game,
            Self::Nim(game) => state.games.nim = game,
            Self::WordLadder(game) => state.games.word_ladder = game,
        }
    }
}
