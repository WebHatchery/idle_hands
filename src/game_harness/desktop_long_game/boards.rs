use crate::battleship::Battleship;
use crate::checkers::Checkers;
use crate::color_sort::ColorSort;
use crate::connect_four::ConnectFour;
use crate::daily_dungeon::{DailyDungeon, DailyHint};
use crate::dots_boxes::DotsBoxes;
use crate::dungeon_sweeper::DungeonSweeper;
use crate::flood_it::FloodIt;
use crate::hanoi::Hanoi;
use crate::higher_lower::HigherLower;
use crate::mahjong_solitaire::MahjongSolitaire;
use crate::mancala::Mancala;
use crate::match_three::MatchThree;
use crate::maze_walk::MazeWalk;
use crate::number_match::NumberMatch;
use crate::one_room_roguelike::{OneRoomRoguelike, RogueHint};
use crate::peg_solitaire::PegSolitaire;
use crate::pipe_loop::PipeLoop;
use crate::reversi::{AiLevel as ReversiAi, Reversi};
use crate::sokoban::Sokoban;
use crate::state::GameId;
use crate::tic_tac_toe::TicTacToe;
use crate::tiny_tower_defence::{TinyTowerDefence, TowerHint};

use super::{assert_serializable, SESSION_SEED};

pub(super) fn run(game: GameId) {
    match game {
        GameId::Reversi => reversi(),
        GameId::TicTacToe => tic_tac_toe(),
        GameId::ConnectFour => connect_four(),
        GameId::DungeonSweeper => dungeon_sweeper(),
        GameId::Checkers => checkers(),
        GameId::PegSolitaire => peg_solitaire(),
        GameId::MahjongSolitaire => mahjong(),
        GameId::HigherLower => higher_lower(),
        GameId::OneRoomRoguelike => one_room(),
        GameId::DailyDungeon => daily_dungeon(),
        GameId::DotsBoxes => dots_boxes(),
        GameId::Sokoban => sokoban(),
        GameId::Mancala => mancala(),
        GameId::Hanoi => hanoi(),
        GameId::NumberMatch => number_match(),
        GameId::FloodIt => flood_it(),
        GameId::ColorSort => color_sort(),
        GameId::Battleship => battleship(),
        GameId::PipeLoop => pipe_loop(),
        GameId::MazeWalk => maze_walk(),
        GameId::MatchThree => match_three(),
        GameId::Nim => nim(),
        GameId::TinyTowerDefence => tiny_tower_defence(),
        _ => panic!("board session was assigned the wrong game"),
    }
}

fn reversi() {
    let mut game = Reversi::new(SESSION_SEED, ReversiAi::Gentle);
    let mut moves = 0;
    for _ in 0..8 {
        for _ in 0..64 {
            if game.turn == 1 {
                if let Some(index) = game.hint_move() {
                    assert!(game.place(index));
                    moves += 1;
                } else if !game.pass() {
                    break;
                }
            } else if game.ai_move() {
                moves += 1;
            } else {
                break;
            }
        }
        game = Reversi::new(SESSION_SEED + moves as u64 + 1, ReversiAi::Gentle);
    }
    assert!(
        moves >= 32,
        "Reversi should keep alternating player and AI turns"
    );
    assert_serializable(&game);
}

fn tic_tac_toe() {
    let mut game = TicTacToe::new(SESSION_SEED);
    let mut moves = 0;
    for round in 0..24 {
        if let Some(index) = game.hint_move() {
            if game.place(index) {
                moves += 1;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        moves >= 16,
        "Tic-Tac-Toe should retain its repeated desktop turn"
    );
    assert_serializable(&game);
}

fn connect_four() {
    let mut game = ConnectFour::new(SESSION_SEED);
    let mut moves = 0;
    for round in 0..12 {
        for _ in 0..16 {
            let Some(column) = game.hint_column() else {
                break;
            };
            if game.drop(column) {
                moves += 1;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        moves >= 24,
        "Connect Four should retain column actions across games"
    );
    assert_serializable(&game);
}

fn dungeon_sweeper() {
    let mut game = DungeonSweeper::new(SESSION_SEED);
    let mut reveals = 0;
    for round in 0..12 {
        for _ in 0..64 {
            let Some(index) = game.hint_cell() else {
                break;
            };
            if game.reveal(index) {
                reveals += 1;
            } else {
                break;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        reveals >= 32,
        "Dungeon Sweeper should retain safe late-board reveals"
    );
    assert_serializable(&game);
}

fn checkers() {
    let mut game = Checkers::new(SESSION_SEED);
    let mut moves = 0;
    for round in 0..8 {
        for _ in 0..32 {
            let Some((from, to)) = game.hint_move() else {
                break;
            };
            assert!(game.tap(from));
            assert!(game.tap(to));
            moves += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(moves >= 16, "Checkers should retain its two-tap move path");
    assert_serializable(&game);
}

fn peg_solitaire() {
    let mut game = PegSolitaire::new(SESSION_SEED);
    let mut moves = 0;
    for round in 0..12 {
        for _ in 0..64 {
            let Some((from, to)) = game.hint_move() else {
                break;
            };
            assert!(game.tap(from));
            assert!(game.tap(to));
            moves += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        moves >= 24,
        "Peg Solitaire should retain its jump targets late in a board"
    );
    assert_serializable(&game);
}

fn mahjong() {
    let mut game = MahjongSolitaire::new(SESSION_SEED);
    let mut pairs = 0;
    for round in 0..12 {
        for _ in 0..24 {
            let Some((first, second)) = game.hint_pair() else {
                break;
            };
            assert!(game.tap(first));
            assert!(game.tap(second));
            pairs += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(pairs >= 8, "Mahjong should retain available-pair targeting");
    assert_serializable(&game);
}

fn higher_lower() {
    let mut game = HigherLower::new(SESSION_SEED);
    let mut guesses = 0;
    for round in 0..24 {
        if let Some(guess) = game.hint_guess() {
            assert!(game.guess(guess));
            guesses += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        guesses >= 16,
        "Higher or Lower should retain its next-card action"
    );
    assert_serializable(&game);
}

fn one_room() {
    let mut game = OneRoomRoguelike::new(SESSION_SEED);
    let mut actions = 0;
    for round in 0..12 {
        for _ in 0..64 {
            let Some(action) = game.hint_action() else {
                break;
            };
            let changed = match action {
                RogueHint::Strike => game.strike(),
                RogueHint::Potion => game.drink_potion(),
                RogueHint::Move(direction) => game.move_in(direction),
            };
            if !changed {
                break;
            }
            actions += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        actions >= 32,
        "One Room Roguelike should retain combat and movement"
    );
    assert_serializable(&game);
}

fn daily_dungeon() {
    let mut game = DailyDungeon::new(SESSION_SEED);
    let mut actions = 0;
    for round in 0..12 {
        for _ in 0..64 {
            let Some(action) = game.hint_action() else {
                break;
            };
            let changed = match action {
                DailyHint::Scout => game.scout(),
                DailyHint::Move(direction) => game.move_in(direction),
            };
            if !changed {
                break;
            }
            actions += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        actions >= 32,
        "Daily Dungeon should retain scouting and movement"
    );
    assert_serializable(&game);
}

fn dots_boxes() {
    let mut game = DotsBoxes::new(SESSION_SEED);
    let mut moves = 0;
    for round in 0..8 {
        for _ in 0..64 {
            let Some(edge) = game.hint_edge() else {
                break;
            };
            assert!(game.play(edge));
            moves += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(moves >= 32, "Dots and Boxes should retain edge targeting");
    assert_serializable(&game);
}

fn sokoban() {
    let mut game = Sokoban::new(SESSION_SEED);
    let mut moves = 0;
    for round in 0..12 {
        for _ in 0..96 {
            let Some(direction) = game.hint_direction() else {
                break;
            };
            if game.move_in(direction) {
                moves += 1;
            } else {
                break;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        moves >= 24,
        "Sokoban should retain its guided movement path"
    );
    assert_serializable(&game);
}

fn mancala() {
    let mut game = Mancala::new(SESSION_SEED);
    let mut moves = 0;
    for round in 0..16 {
        for _ in 0..48 {
            let Some(pit) = game.hint_pit() else {
                break;
            };
            if game.play(pit) {
                moves += 1;
            } else {
                break;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        moves >= 32,
        "Mancala should retain pit selection after many turns"
    );
    assert_serializable(&game);
}

fn hanoi() {
    let mut game = Hanoi::new_with_disks(SESSION_SEED, 7);
    let mut moves = 0;
    for round in 0..8 {
        for _ in 0..128 {
            let Some((source, destination)) = game.hint_move() else {
                break;
            };
            assert!(game.tap_peg(source));
            assert!(game.tap_peg(destination));
            moves += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        moves >= 32,
        "Tower of Hanoi should retain its multi-tap move path"
    );
    assert_serializable(&game);
}

fn number_match() {
    let mut game = NumberMatch::new(SESSION_SEED);
    let mut pairs = 0;
    for round in 0..12 {
        for _ in 0..64 {
            let Some((first, second)) = game.hint_pair() else {
                break;
            };
            assert!(game.tap(first));
            assert!(game.tap(second));
            pairs += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        pairs >= 24,
        "Number Match should retain linked pair targeting"
    );
    assert_serializable(&game);
}

fn flood_it() {
    let mut game = FloodIt::new(SESSION_SEED);
    let mut moves = 0;
    for round in 0..12 {
        for _ in 0..64 {
            let Some(color) = game.hint_color() else {
                break;
            };
            if game.choose(color) {
                moves += 1;
            } else {
                break;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(moves >= 32, "Flood It should retain region-color actions");
    assert_serializable(&game);
}

fn color_sort() {
    let mut game = ColorSort::new(SESSION_SEED);
    let mut moves = 0;
    for round in 0..12 {
        for _ in 0..64 {
            let Some((source, destination)) = game.hint_move() else {
                break;
            };
            assert!(game.tap_tube(source));
            assert!(game.tap_tube(destination));
            moves += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        moves >= 24,
        "Color Sort should retain its source-and-destination path"
    );
    assert_serializable(&game);
}

fn battleship() {
    let mut game = Battleship::new(SESSION_SEED);
    let mut shots = 0;
    for round in 0..8 {
        for _ in 0..64 {
            let Some(cell) = game.hint_cell() else {
                break;
            };
            if game.fire(cell) {
                shots += 1;
            } else {
                break;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(shots >= 32, "Battleship should retain late-board firing");
    assert_serializable(&game);
}

fn pipe_loop() {
    let mut game = PipeLoop::new(SESSION_SEED);
    let mut rotations = 0;
    for round in 0..12 {
        for _ in 0..64 {
            let Some((index, turns)) = game.hint_rotation() else {
                break;
            };
            for _ in 0..turns {
                assert!(game.rotate(index));
                rotations += 1;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        rotations >= 24,
        "Pipe Loop should retain rotation targets late in a puzzle"
    );
    assert_serializable(&game);
}

fn maze_walk() {
    let mut game = MazeWalk::new(SESSION_SEED);
    let mut moves = 0;
    for round in 0..12 {
        for _ in 0..128 {
            let Some(direction) = game.hint_direction() else {
                break;
            };
            if game.step(direction) {
                moves += 1;
            } else {
                break;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        moves >= 32,
        "Maze Walk should retain its route after many steps"
    );
    assert_serializable(&game);
}

fn match_three() {
    let mut game = MatchThree::new(SESSION_SEED);
    let mut swaps = 0;
    for round in 0..12 {
        for _ in 0..64 {
            let Some((first, second)) = game.hint_swap() else {
                break;
            };
            assert!(game.tap(first));
            assert!(game.tap(second));
            swaps += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        swaps >= 8,
        "Match Three should retain swap resolution late in a run"
    );
    assert_serializable(&game);
}

fn nim() {
    let mut game = crate::nim::Nim::new(SESSION_SEED);
    let mut moves = 0;
    for round in 0..24 {
        if let Some((heap, amount)) = game.hint_move() {
            game.select_heap(heap);
            assert!(game.take(amount));
            moves += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(moves >= 16, "Nim should retain heap selection and taking");
    assert_serializable(&game);
}

fn tiny_tower_defence() {
    let mut game = TinyTowerDefence::new(SESSION_SEED);
    let mut actions = 0;
    for round in 0..8 {
        for _ in 0..96 {
            match game.hint_action() {
                Some(TowerHint::Build(index, kind)) => {
                    game.select_kind(kind);
                    assert!(game.build_or_upgrade(index));
                    actions += 1;
                }
                Some(TowerHint::WaveControl) => {
                    assert!(game.start_or_advance());
                    actions += 1;
                }
                None => {
                    if !game.start_or_advance() {
                        break;
                    }
                }
            }
            game.tick(1.0);
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        actions >= 16,
        "Tiny Tower Defence should retain build and wave actions"
    );
    assert_serializable(&game);
}
