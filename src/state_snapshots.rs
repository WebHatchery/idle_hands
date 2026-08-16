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
            GameId::Game2048 => Self::Game2048(state.game.clone()),
            GameId::Minesweeper => Self::Minesweeper(state.minesweeper.clone()),
            GameId::Sudoku => Self::Sudoku(state.sudoku.clone()),
            GameId::Nonogram => Self::Nonogram(state.nonogram.clone()),
            GameId::Solitaire => Self::Solitaire(state.solitaire.clone()),
            GameId::FreeCell => Self::FreeCell(state.freecell.clone()),
            GameId::Yahtzee => Self::Fivefold(state.fivefold.clone()),
            GameId::Reversi => Self::Reversi(state.reversi.clone()),
            GameId::LightsOut => Self::LightsOut(state.lights_out.clone()),
            GameId::TicTacToe => Self::TicTacToe(state.tic_tac_toe.clone()),
            GameId::MemoryPairs => Self::MemoryPairs(state.memory_pairs.clone()),
            GameId::SlidingPuzzle => Self::SlidingPuzzle(state.sliding_puzzle.clone()),
            GameId::Mastermind => Self::Mastermind(state.mastermind.clone()),
            GameId::Spider => Self::Spider(state.spider.clone()),
            GameId::WordSearch => Self::WordSearch(state.word_search.clone()),
            GameId::Hangman => Self::Hangman(state.hangman.clone()),
            GameId::ConnectFour => Self::ConnectFour(state.connect_four.clone()),
            GameId::Checkers => Self::Checkers(state.checkers.clone()),
            GameId::PegSolitaire => Self::PegSolitaire(state.peg_solitaire.clone()),
            GameId::MahjongSolitaire => Self::MahjongSolitaire(state.mahjong_solitaire.clone()),
            GameId::Snake => Self::Snake(state.snake.clone()),
            GameId::Breakout => Self::Breakout(state.breakout.clone()),
            GameId::HigherLower => Self::HigherLower(state.higher_lower.clone()),
            GameId::KlondikeGolf => Self::KlondikeGolf(state.klondike_golf.clone()),
            GameId::Blackjack => Self::Blackjack(state.blackjack.clone()),
            GameId::SpiderSolitaire => Self::SpiderSolitaire(state.spider_solitaire.clone()),
            GameId::DungeonSweeper => Self::DungeonSweeper(state.dungeon_sweeper.clone()),
            GameId::Potion2048 => Self::Potion2048(state.potion_2048.clone()),
            GameId::TinyTowerDefence => Self::TinyTowerDefence(state.tiny_tower_defence.clone()),
            GameId::OneRoomRoguelike => Self::OneRoomRoguelike(state.one_room_roguelike.clone()),
            GameId::DailyDungeon => Self::DailyDungeon(state.daily_dungeon.clone()),
            GameId::DotsBoxes => Self::DotsBoxes(state.dots_boxes.clone()),
            GameId::Sokoban => Self::Sokoban(state.sokoban.clone()),
            GameId::Mancala => Self::Mancala(state.mancala.clone()),
            GameId::Hanoi => Self::Hanoi(state.hanoi.clone()),
            GameId::NumberMatch => Self::NumberMatch(state.number_match.clone()),
            GameId::FloodIt => Self::FloodIt(state.flood_it.clone()),
            GameId::ColorSort => Self::ColorSort(state.color_sort.clone()),
            GameId::Battleship => Self::Battleship(state.battleship.clone()),
            GameId::WordGrid => Self::WordGrid(state.word_grid.clone()),
            GameId::PipeLoop => Self::PipeLoop(state.pipe_loop.clone()),
            GameId::MazeWalk => Self::MazeWalk(state.maze_walk.clone()),
            GameId::MatchThree => Self::MatchThree(state.match_three.clone()),
            GameId::Pyramid => Self::Pyramid(state.pyramid.clone()),
            GameId::TriPeaks => Self::TriPeaks(state.tri_peaks.clone()),
            GameId::Nim => Self::Nim(state.nim.clone()),
            GameId::WordLadder => Self::WordLadder(state.word_ladder.clone()),
        }
    }

    pub fn apply_to(self, state: &mut AppState) {
        match self {
            Self::Game2048(game) => state.game = game,
            Self::Minesweeper(game) => state.minesweeper = game,
            Self::Sudoku(game) => state.sudoku = game,
            Self::Nonogram(game) => state.nonogram = game,
            Self::Solitaire(game) => state.solitaire = game,
            Self::FreeCell(game) => state.freecell = game,
            Self::Fivefold(game) => state.fivefold = game,
            Self::Reversi(game) => state.reversi = game,
            Self::LightsOut(game) => state.lights_out = game,
            Self::TicTacToe(game) => state.tic_tac_toe = game,
            Self::MemoryPairs(game) => state.memory_pairs = game,
            Self::SlidingPuzzle(game) => state.sliding_puzzle = game,
            Self::Mastermind(game) => state.mastermind = game,
            Self::Spider(game) => state.spider = game,
            Self::WordSearch(game) => state.word_search = game,
            Self::Hangman(game) => state.hangman = game,
            Self::ConnectFour(game) => state.connect_four = game,
            Self::Checkers(game) => state.checkers = game,
            Self::PegSolitaire(game) => state.peg_solitaire = game,
            Self::MahjongSolitaire(game) => state.mahjong_solitaire = game,
            Self::Snake(game) => state.snake = game,
            Self::Breakout(game) => state.breakout = game,
            Self::HigherLower(game) => state.higher_lower = game,
            Self::KlondikeGolf(game) => state.klondike_golf = game,
            Self::Blackjack(game) => state.blackjack = game,
            Self::SpiderSolitaire(game) => state.spider_solitaire = game,
            Self::DungeonSweeper(game) => state.dungeon_sweeper = game,
            Self::Potion2048(game) => state.potion_2048 = game,
            Self::TinyTowerDefence(game) => state.tiny_tower_defence = game,
            Self::OneRoomRoguelike(game) => state.one_room_roguelike = game,
            Self::DailyDungeon(game) => state.daily_dungeon = game,
            Self::DotsBoxes(game) => state.dots_boxes = game,
            Self::Sokoban(game) => state.sokoban = game,
            Self::Mancala(game) => state.mancala = game,
            Self::Hanoi(game) => state.hanoi = game,
            Self::NumberMatch(game) => state.number_match = game,
            Self::FloodIt(game) => state.flood_it = game,
            Self::ColorSort(game) => state.color_sort = game,
            Self::Battleship(game) => state.battleship = game,
            Self::WordGrid(game) => state.word_grid = game,
            Self::PipeLoop(game) => state.pipe_loop = game,
            Self::MazeWalk(game) => state.maze_walk = game,
            Self::MatchThree(game) => state.match_three = game,
            Self::Pyramid(game) => state.pyramid = game,
            Self::TriPeaks(game) => state.tri_peaks = game,
            Self::Nim(game) => state.nim = game,
            Self::WordLadder(game) => state.word_ladder = game,
        }
    }
}
