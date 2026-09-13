use crate::state::{AppState, GameId};

pub(crate) fn configured_label(state: &AppState, game: GameId, id: &str) -> Option<String> {
    state
        .content
        .variants_for(game)
        .iter()
        .find(|entry| entry.id == id)
        .map(|entry| entry.label.clone())
}

pub fn label(state: &AppState, game: GameId) -> String {
    match game {
        GameId::Game2048 => format!("BOARD · {}", state.games.game.board_size.label()),
        GameId::Minesweeper => state.games.minesweeper.preset.label().to_owned(),
        GameId::Sudoku => state.games.sudoku.difficulty.label().to_owned(),
        GameId::Nonogram => format!("NONOGRAM · {}", state.games.nonogram.preset.label()),
        GameId::Solitaire => state.games.solitaire.ruleset.label().to_owned(),
        GameId::FreeCell => state.games.freecell.variant.label().to_owned(),
        GameId::Yahtzee => state.games.fivefold.variant.label().to_owned(),
        GameId::Reversi => format!("REVERSI · {:?}", state.games.reversi.ai_level).to_uppercase(),
        GameId::LightsOut => state.games.lights_out.difficulty.label().to_owned(),
        GameId::TicTacToe => {
            format!("TIC-TAC-TOE · {:?}", state.games.tic_tac_toe.ai_level).to_uppercase()
        }
        GameId::MemoryPairs => state.games.memory_pairs.variant.label().to_owned(),
        GameId::SlidingPuzzle => state.games.sliding_puzzle.variant.label().to_owned(),
        GameId::Mastermind => state.games.mastermind.variant.label().to_owned(),
        GameId::Spider => state.games.spider.mode.label().to_owned(),
        GameId::WordSearch => configured_label(state, game, state.games.word_search.theme.key())
            .unwrap_or_else(|| "WORD SEARCH".to_owned()),
        GameId::Hangman => format!(
            "HANGMAN · {} / {:?}",
            configured_label(state, game, state.games.hangman.category.key())
                .unwrap_or_else(|| state.games.hangman.category.label().to_owned()),
            state.games.hangman.rule
        )
        .to_uppercase(),
        GameId::ConnectFour => {
            format!("CONNECT FOUR · {:?}", state.games.connect_four.ai_level).to_uppercase()
        }
        GameId::Checkers => {
            format!("CHECKERS · {:?}", state.games.checkers.ai_level).to_uppercase()
        }
        GameId::PegSolitaire => state.games.peg_solitaire.variant.label().to_owned(),
        GameId::MahjongSolitaire => state.games.mahjong_solitaire.layout.label().to_owned(),
        GameId::Snake => format!("SNAKE · {:?}", state.games.snake.mode).to_uppercase(),
        GameId::Breakout => format!("WALL RUN · {} / 3", state.games.breakout.level),
        GameId::HigherLower => {
            format!("HIGHER / LOWER · {:?}", state.games.higher_lower.rule).to_uppercase()
        }
        GameId::KlondikeGolf => state.games.klondike_golf.rule.label().to_owned(),
        GameId::Blackjack => state.games.blackjack.rule.label().to_owned(),
        GameId::SpiderSolitaire => state.games.spider_solitaire.rule.label().to_owned(),
        GameId::DungeonSweeper => {
            format!("DUNGEON · {:?}", state.games.dungeon_sweeper.difficulty).to_uppercase()
        }
        GameId::Potion2048 => {
            format!("POTION · {:?}", state.games.potion_2048.difficulty).to_uppercase()
        }
        GameId::TinyTowerDefence => format!(
            "TOWER ROLE · {:?}",
            state.games.tiny_tower_defence.selected_kind
        )
        .to_uppercase(),
        GameId::OneRoomRoguelike => format!(
            "ROOM ROLE · {:?}",
            state.games.one_room_roguelike.hero_class
        )
        .to_uppercase(),
        GameId::DailyDungeon => {
            format!("DAILY · {:?}", state.games.daily_dungeon.rule).to_uppercase()
        }
        GameId::DotsBoxes => {
            format!("DOTS · {:?}", state.games.dots_boxes.difficulty).to_uppercase()
        }
        GameId::Sokoban => format!("WAREHOUSE · ROOM {}", state.games.sokoban.level + 1),
        GameId::Mancala => format!("MANCALA · {:?}", state.games.mancala.variant).to_uppercase(),
        GameId::Hanoi => format!("HANOI · {} DISKS", state.games.hanoi.disks),
        GameId::NumberMatch => {
            format!("NUMBER MATCH · {:?}", state.games.number_match.rule).to_uppercase()
        }
        GameId::FloodIt => {
            format!("FLOOD IT · {:?}", state.games.flood_it.difficulty).to_uppercase()
        }
        GameId::ColorSort => {
            format!("COLOR SORT · {:?}", state.games.color_sort.difficulty).to_uppercase()
        }
        GameId::Battleship => state.games.battleship.fleet.label().to_owned(),
        GameId::WordGrid => format!(
            "WORD GRID · {}",
            configured_label(
                state,
                game,
                if state.games.word_grid.mode == crate::word_grid::WordGridMode::Classic {
                    "default"
                } else {
                    "alternate"
                },
            )
            .unwrap_or_else(|| format!("{:?}", state.games.word_grid.mode))
        )
        .to_uppercase(),
        GameId::WordLadder => format!(
            "WORD LADDER · {}",
            configured_label(
                state,
                game,
                if state.games.word_ladder.mode == crate::word_ladder::LadderMode::Direct {
                    "default"
                } else {
                    "alternate"
                },
            )
            .unwrap_or_else(|| format!("{:?}", state.games.word_ladder.mode))
        )
        .to_uppercase(),
        GameId::PipeLoop => {
            format!("PIPE LOOP · {:?}", state.games.pipe_loop.pattern).to_uppercase()
        }
        GameId::MazeWalk => format!("MAZE WALK · {:?}", state.games.maze_walk.mode).to_uppercase(),
        GameId::MatchThree => {
            format!("MATCH THREE · {:?}", state.games.match_three.difficulty).to_uppercase()
        }
        GameId::Pyramid => state.games.pyramid.draw_rule.label().to_owned(),
        GameId::TriPeaks => format!("TRIPEAKS · {:?}", state.games.tri_peaks.rule).to_uppercase(),
        GameId::Nim => format!("NIM · {:?}", state.games.nim.rule).to_uppercase(),
        GameId::SpaceInvaders => format!("INVADERS · {}", state.games.space_invaders.mode_label()),
        GameId::Asteroids => format!("ASTEROIDS · {}", state.games.asteroids.mode_label()),
        GameId::Frogger => format!("FROGGER · {}", state.games.frogger.mode_label()),
        GameId::MunchMaze => format!("MUNCH MAZE · {}", state.games.munch_maze.mode_label()),
        GameId::BlockStack => format!("BLOCK STACK · {}", state.games.block_stack.mode_label()),
        GameId::TerrainCannon => format!(
            "TERRAIN CANNON · {}",
            state.games.terrain_cannon.mode_label()
        ),
        GameId::FlingFury => format!("FLING FURY · {}", state.games.fling_fury.mode_label()),
        GameId::PaddleDuel => format!("PADDLE DUEL · {}", state.games.paddle_duel.mode_label()),
        GameId::RiddleRoom => format!(
            "RIDDLE ROOM · {}",
            misc_variant(state, game, state.games.riddle_room.seed)
        ),
        GameId::PatternVault => format!(
            "PATTERN VAULT · {}",
            misc_variant(state, game, state.games.pattern_vault.seed)
        ),
        GameId::SumCircuit => format!(
            "SUM CIRCUIT · {}",
            misc_variant(state, game, state.games.sum_circuit.seed)
        ),
        GameId::OrbitOrder => format!(
            "ORBIT ORDER · {}",
            misc_variant(state, game, state.games.orbit_order.seed)
        ),
        GameId::WordForge => format!(
            "WORD FORGE · {}",
            misc_variant(state, game, state.games.word_forge.seed)
        ),
    }
}

fn misc_variant(state: &AppState, game: GameId, seed: u64) -> String {
    let id = if seed.is_multiple_of(2) {
        "alternate"
    } else {
        "default"
    };
    configured_label(state, game, id).unwrap_or_else(|| {
        if id == "alternate" {
            "TWIST".into()
        } else {
            "CLASSIC".into()
        }
    })
}
