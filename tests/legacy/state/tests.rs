//! Snapshot application must preserve independent game payloads.

use idle_hands::testing::cards::Card;
use idle_hands::testing::modules::state::*;

#[test]
fn independent_snapshots_restore_all_games_without_overwriting_each_other() {
    let mut source = AppState::default();
    source.games.game.score = 11;
    source.games.minesweeper.seed = 22;
    source.games.sudoku.moves = 33;
    source.games.nonogram.moves = 44;
    source.games.solitaire.moves = 55;
    source.games.freecell.moves = 66;
    source.games.fivefold.roll_number = 2;
    source.games.reversi.turn = 2;
    source.games.lights_out.moves = 17;
    source.games.tic_tac_toe.moves = 19;
    source.games.memory_pairs.moves = 21;
    source.games.sliding_puzzle.moves = 23;
    source.games.mastermind.row = 3;
    source.games.spider.moves = 25;
    source.games.word_search.moves = 26;
    source.games.hangman.moves = 27;
    source.games.connect_four.moves = 28;
    source.games.checkers.moves = 29;
    source.games.peg_solitaire.moves = 30;
    source.games.mahjong_solitaire.moves = 31;
    source.games.snake.moves = 32;
    source.games.breakout.moves = 33;
    source.games.higher_lower.moves = 34;
    source.games.klondike_golf.moves = 35;
    source.games.blackjack.player.push(Card {
        rank: 8,
        suit: 2,
        face_up: true,
    });
    source.games.spider_solitaire.moves = 37;
    source.games.dungeon_sweeper.moves = 39;
    source.games.potion_2048.score = 41;
    source.games.tiny_tower_defence.wave = 5;
    source.games.one_room_roguelike.score = 41;
    source.games.daily_dungeon.score = 43;
    source.games.nim.moves = 44;

    let snapshots = GameId::ALL
        .iter()
        .map(|&game| GameSnapshot::from_state(&source, game))
        .collect::<Vec<_>>();
    let mut restored = AppState::default();
    for snapshot in snapshots {
        snapshot.apply_to(&mut restored);
    }

    assert_eq!(restored.games.game.score, 11);
    assert_eq!(restored.games.minesweeper.seed, 22);
    assert_eq!(restored.games.sudoku.moves, 33);
    assert_eq!(restored.games.nonogram.moves, 44);
    assert_eq!(restored.games.solitaire.moves, 55);
    assert_eq!(restored.games.freecell.moves, 66);
    assert_eq!(restored.games.fivefold.roll_number, 2);
    assert_eq!(restored.games.reversi.turn, 2);
    assert_eq!(restored.games.lights_out.moves, 17);
    assert_eq!(restored.games.tic_tac_toe.moves, 19);
    assert_eq!(restored.games.memory_pairs.moves, 21);
    assert_eq!(restored.games.sliding_puzzle.moves, 23);
    assert_eq!(restored.games.mastermind.row, 3);
    assert_eq!(restored.games.spider.moves, 25);
    assert_eq!(restored.games.word_search.moves, 26);
    assert_eq!(restored.games.hangman.moves, 27);
    assert_eq!(restored.games.connect_four.moves, 28);
    assert_eq!(restored.games.checkers.moves, 29);
    assert_eq!(restored.games.peg_solitaire.moves, 30);
    assert_eq!(restored.games.mahjong_solitaire.moves, 31);
    assert_eq!(restored.games.snake.moves, 32);
    assert_eq!(restored.games.breakout.moves, 33);
    assert_eq!(restored.games.higher_lower.moves, 34);
    assert_eq!(restored.games.klondike_golf.moves, 35);
    assert_eq!(restored.games.blackjack.player.len(), 3);
    assert_eq!(restored.games.spider_solitaire.moves, 37);
    assert_eq!(restored.games.dungeon_sweeper.moves, 39);
    assert_eq!(restored.games.potion_2048.score, 41);
    assert_eq!(restored.games.tiny_tower_defence.wave, 5);
    assert_eq!(restored.games.one_room_roguelike.score, 41);
    assert_eq!(restored.games.daily_dungeon.score, 43);
    assert_eq!(restored.games.nim.moves, 44);
}
