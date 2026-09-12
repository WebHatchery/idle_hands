//! Shared rule-card metadata and deterministic variant rotation.

use crate::{
    data::GameData,
    state::{AppState, GameId},
};

fn next<T: Copy + PartialEq>(all: &[T], current: T) -> T {
    let index = all.iter().position(|item| *item == current).unwrap_or(0);
    all[(index + 1) % all.len()]
}

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

pub fn cycle(state: &mut AppState, data: &GameData, game: GameId) {
    let seed = |value: u64| value.wrapping_add(1);
    match game {
        GameId::Game2048 => {
            let size = next(
                &crate::game_2048::Game2048Size::ALL,
                state.games.game.board_size,
            );
            state.games.game =
                crate::state::Game2048::new_with_size(seed(state.games.game.seed), size);
        }
        GameId::Minesweeper => {
            let preset = next(
                &crate::minesweeper::MinePreset::ALL,
                state.games.minesweeper.preset,
            );
            state.games.minesweeper =
                crate::minesweeper::Minesweeper::new(preset, seed(state.games.minesweeper.seed));
        }
        GameId::Sudoku => {
            let difficulty = next(
                &crate::sudoku::SudokuDifficulty::ALL,
                state.games.sudoku.difficulty,
            );
            state.games.sudoku = crate::sudoku::Sudoku::with_difficulty(difficulty);
        }
        GameId::Nonogram => {
            let preset = next(
                &crate::nonogram::NonogramPreset::ALL,
                state.games.nonogram.preset,
            );
            state.games.nonogram = crate::nonogram::Nonogram::new_with_variant(
                preset,
                state.games.nonogram.variant.wrapping_add(1),
            );
        }
        GameId::Solitaire => {
            let ruleset = next(
                &crate::solitaire::SolitaireRuleset::ALL,
                state.games.solitaire.ruleset,
            );
            state.games.solitaire = crate::solitaire::Solitaire::with_ruleset(
                seed(state.games.solitaire.seed),
                ruleset,
            );
        }
        GameId::FreeCell => {
            let variant = next(
                &crate::freecell::FreeCellVariant::ALL,
                state.games.freecell.variant,
            );
            state.games.freecell = crate::freecell::FreeCell::new_with_variant(
                seed(state.games.freecell.seed),
                variant,
            );
        }
        GameId::Yahtzee => {
            let variant = next(
                &crate::fivefold::FivefoldVariant::ALL,
                state.games.fivefold.variant,
            );
            state.games.fivefold = crate::fivefold::Fivefold::new_with_variant(
                seed(state.games.fivefold.seed),
                variant,
            );
        }
        GameId::Reversi => {
            let all = [
                crate::reversi::AiLevel::Gentle,
                crate::reversi::AiLevel::Sharp,
                crate::reversi::AiLevel::TwoPlayer,
            ];
            state.games.reversi = crate::reversi::Reversi::new(
                seed(state.games.reversi.seed),
                next(&all, state.games.reversi.ai_level),
            );
        }
        GameId::LightsOut => {
            let all = [
                crate::lights_out::LightsDifficulty::Classic,
                crate::lights_out::LightsDifficulty::Dense,
            ];
            let difficulty = next(&all, state.games.lights_out.difficulty);
            state.games.lights_out = crate::lights_out::LightsOut::new_with_difficulty(
                seed(state.games.lights_out.seed),
                difficulty,
            );
        }
        GameId::TicTacToe => {
            let all = [
                crate::tic_tac_toe::AiLevel::Gentle,
                crate::tic_tac_toe::AiLevel::Sharp,
                crate::tic_tac_toe::AiLevel::Expert,
            ];
            let mut game_state =
                crate::tic_tac_toe::TicTacToe::new(seed(state.games.tic_tac_toe.seed));
            game_state.set_ai_level(next(&all, state.games.tic_tac_toe.ai_level));
            state.games.tic_tac_toe = game_state;
        }
        GameId::MemoryPairs => {
            let variant = next(
                &crate::memory_pairs::MemoryVariant::ALL,
                state.games.memory_pairs.variant,
            );
            state.games.memory_pairs = crate::memory_pairs::MemoryPairs::new_with_variant(
                seed(state.games.memory_pairs.seed),
                variant,
            );
        }
        GameId::SlidingPuzzle => {
            let variant = next(
                &crate::sliding_puzzle::SlidingVariant::ALL,
                state.games.sliding_puzzle.variant,
            );
            state.games.sliding_puzzle = crate::sliding_puzzle::SlidingPuzzle::new_with_variant(
                seed(state.games.sliding_puzzle.seed),
                variant,
            );
        }
        GameId::Mastermind => {
            let variant = next(
                &crate::mastermind::MastermindVariant::ALL,
                state.games.mastermind.variant,
            );
            state.games.mastermind = crate::mastermind::Mastermind::new_with_variant(
                seed(state.games.mastermind.seed),
                variant,
            );
        }
        GameId::Hangman => {
            let category = state.games.hangman.category.next();
            let rule = match state.games.hangman.rule {
                crate::hangman::HangmanRule::Classic => crate::hangman::HangmanRule::Rapid,
                crate::hangman::HangmanRule::Rapid => crate::hangman::HangmanRule::Classic,
            };
            state.games.hangman = crate::hangman::Hangman::new_with_options_config_and_balance(
                seed(state.games.hangman.seed),
                category,
                rule,
                &data.content.words.hangman,
                &data.content.balance.word_games,
            );
        }
        GameId::Spider => {
            let mode = next(&crate::spider::SpiderMode::ALL, state.games.spider.mode);
            state.games.spider =
                crate::spider::Spider::new_with_mode(seed(state.games.spider.seed), mode);
        }
        GameId::WordSearch => {
            let theme = next(
                &crate::word_search::WordSearchTheme::ALL,
                state.games.word_search.theme,
            );
            state.games.word_search = crate::word_search::WordSearch::new_with_theme_config(
                seed(state.games.word_search.seed),
                theme,
                &data.content.words.word_search,
            );
        }
        GameId::ConnectFour => {
            let all = [
                crate::connect_four::AiLevel::Gentle,
                crate::connect_four::AiLevel::Sharp,
                crate::connect_four::AiLevel::Expert,
            ];
            let mut game_state =
                crate::connect_four::ConnectFour::new(seed(state.games.connect_four.seed));
            game_state.set_ai_level(next(&all, state.games.connect_four.ai_level));
            state.games.connect_four = game_state;
        }
        GameId::Checkers => {
            let all = [
                crate::checkers::AiLevel::Gentle,
                crate::checkers::AiLevel::Sharp,
                crate::checkers::AiLevel::Expert,
            ];
            let mut game_state = crate::checkers::Checkers::new(seed(state.games.checkers.seed));
            game_state.set_ai_level(next(&all, state.games.checkers.ai_level));
            state.games.checkers = game_state;
        }
        GameId::PegSolitaire => {
            let variant = next(
                &crate::peg_solitaire::PegVariant::ALL,
                state.games.peg_solitaire.variant,
            );
            state.games.peg_solitaire = crate::peg_solitaire::PegSolitaire::new_with_variant(
                seed(state.games.peg_solitaire.seed),
                variant,
            );
        }
        GameId::MahjongSolitaire => {
            let layout = next(
                &crate::mahjong_solitaire::MahjongLayout::ALL,
                state.games.mahjong_solitaire.layout,
            );
            state.games.mahjong_solitaire =
                crate::mahjong_solitaire::MahjongSolitaire::new_with_layout(
                    seed(state.games.mahjong_solitaire.seed),
                    layout,
                );
        }
        GameId::Snake => {
            let all = [
                crate::snake::SnakeMode::Classic,
                crate::snake::SnakeMode::Wrap,
                crate::snake::SnakeMode::Garden,
            ];
            let mode = next(&all, state.games.snake.mode);
            state.games.snake =
                crate::snake::Snake::new_with_mode(seed(state.games.snake.seed), mode);
        }
        GameId::Breakout => {
            let level = (state.games.breakout.level % state.games.breakout.target_level) + 1;
            state.games.breakout =
                crate::breakout::Breakout::new_with_level(seed(state.games.breakout.seed), level);
        }
        GameId::HigherLower => {
            let all = [
                crate::higher_lower::HigherLowerRule::Friendly,
                crate::higher_lower::HigherLowerRule::House,
            ];
            let rule = next(&all, state.games.higher_lower.rule);
            let mut game_state =
                crate::higher_lower::HigherLower::new(seed(state.games.higher_lower.seed));
            game_state.set_rule(rule, seed(state.games.higher_lower.seed));
            state.games.higher_lower = game_state;
        }
        GameId::KlondikeGolf => {
            let rule = next(
                &crate::klondike_golf::GolfRule::ALL,
                state.games.klondike_golf.rule,
            );
            state.games.klondike_golf = crate::klondike_golf::KlondikeGolf::new_with_rule(
                seed(state.games.klondike_golf.seed),
                rule,
            );
        }
        GameId::Blackjack => {
            let rule = next(
                &crate::blackjack::BlackjackRule::ALL,
                state.games.blackjack.rule,
            );
            state.games.blackjack =
                crate::blackjack::Blackjack::new_with_rule(seed(state.games.blackjack.seed), rule);
        }
        GameId::SpiderSolitaire => {
            let rule = next(
                &crate::spider_solitaire::SpiderRule::ALL,
                state.games.spider_solitaire.rule,
            );
            state.games.spider_solitaire = crate::spider_solitaire::SpiderSolitaire::new_with_rule(
                seed(state.games.spider_solitaire.seed),
                rule,
            );
        }
        GameId::DungeonSweeper => {
            let difficulty = next(
                &crate::dungeon_sweeper::DungeonDifficulty::ALL,
                state.games.dungeon_sweeper.difficulty,
            );
            state.games.dungeon_sweeper =
                crate::dungeon_sweeper::DungeonSweeper::new_with_difficulty(
                    seed(state.games.dungeon_sweeper.seed),
                    difficulty,
                );
        }
        GameId::Potion2048 => {
            let difficulty = next(
                &crate::potion_2048::PotionDifficulty::ALL,
                state.games.potion_2048.difficulty,
            );
            state.games.potion_2048 = crate::potion_2048::Potion2048::new_with_difficulty(
                seed(state.games.potion_2048.seed),
                difficulty,
            );
        }
        GameId::TinyTowerDefence => {
            let kind = next(
                &crate::tiny_tower_defence::TowerKind::ALL,
                state.games.tiny_tower_defence.selected_kind,
            );
            let mut tower = crate::tiny_tower_defence::TinyTowerDefence::new(seed(
                state.games.tiny_tower_defence.seed,
            ));
            tower.select_kind(kind);
            state.games.tiny_tower_defence = tower;
        }
        GameId::OneRoomRoguelike => {
            let class = next(
                &crate::one_room_roguelike::HeroClass::ALL,
                state.games.one_room_roguelike.hero_class,
            );
            state.games.one_room_roguelike =
                crate::one_room_roguelike::OneRoomRoguelike::new_with_class(
                    seed(state.games.one_room_roguelike.seed),
                    class,
                );
        }
        GameId::DailyDungeon => {
            let rule = next(
                &crate::daily_dungeon::DailyRule::ALL,
                state.games.daily_dungeon.rule,
            );
            state.games.daily_dungeon = if state.games.daily_dungeon.day_key == 0 {
                crate::daily_dungeon::DailyDungeon::new_with_rule(
                    seed(state.games.daily_dungeon.seed),
                    rule,
                )
            } else {
                crate::daily_dungeon::DailyDungeon::new_for_day_with_rule(
                    state.games.daily_dungeon.day_key,
                    rule,
                )
            };
        }
        GameId::DotsBoxes => {
            let difficulty = next(
                &crate::dots_boxes::DotsDifficulty::ALL,
                state.games.dots_boxes.difficulty,
            );
            state.games.dots_boxes = crate::dots_boxes::DotsBoxes::new_with_config(
                seed(state.games.dots_boxes.seed),
                difficulty,
                &data.puzzles.dots_boxes,
            );
        }
        GameId::Sokoban => {
            let level = (state.games.sokoban.level + 1) % crate::sokoban::LEVEL_COUNT;
            state.games.sokoban =
                crate::sokoban::Sokoban::new_with_level(seed(state.games.sokoban.seed), level);
        }
        GameId::Battleship => {
            let fleet = next(
                &crate::battleship::BattleshipFleet::ALL,
                state.games.battleship.fleet,
            );
            state.games.battleship = crate::battleship::Battleship::new_with_fleet(
                seed(state.games.battleship.seed),
                fleet,
            );
        }
        GameId::Hanoi => {
            let disks = next(&[3u8, 5, 7], state.games.hanoi.disks);
            state.games.hanoi =
                crate::hanoi::Hanoi::new_with_disks(seed(state.games.hanoi.seed), disks);
        }
        GameId::Mancala => {
            let all = [
                crate::mancala::MancalaVariant::Quick,
                crate::mancala::MancalaVariant::Classic,
                crate::mancala::MancalaVariant::Grand,
            ];
            state.games.mancala.set_variant(
                next(&all, state.games.mancala.variant),
                seed(state.games.mancala.seed),
            );
        }
        GameId::NumberMatch => {
            let all = [
                crate::number_match::LinkRule::Neighbors,
                crate::number_match::LinkRule::Lines,
                crate::number_match::LinkRule::Diagonals,
            ];
            state.games.number_match.set_rule(
                next(&all, state.games.number_match.rule),
                seed(state.games.number_match.seed),
            );
        }
        GameId::FloodIt => {
            let difficulty = next(
                &crate::flood_it::FloodDifficulty::ALL,
                state.games.flood_it.difficulty,
            );
            state.games.flood_it = crate::flood_it::FloodIt::new_with_config(
                seed(state.games.flood_it.seed),
                difficulty,
                &data.puzzles.flood_it,
            );
        }
        GameId::ColorSort => {
            let difficulty = next(
                &crate::color_sort::ColorSortDifficulty::ALL,
                state.games.color_sort.difficulty,
            );
            state.games.color_sort = crate::color_sort::ColorSort::new_with_config(
                seed(state.games.color_sort.seed),
                difficulty,
                &data.puzzles.color_sort,
            );
        }
        GameId::WordGrid => {
            let all = [
                crate::word_grid::WordGridMode::Classic,
                crate::word_grid::WordGridMode::Hard,
            ];
            state.games.word_grid = crate::word_grid::WordGrid::new_with_mode_config(
                seed(state.games.word_grid.seed),
                next(&all, state.games.word_grid.mode),
                &data.content.words.word_grid,
                data.content.balance.word_games.word_grid_max_guesses,
            );
        }
        GameId::WordLadder => {
            let all = [
                crate::word_ladder::LadderMode::Direct,
                crate::word_ladder::LadderMode::Scenic,
            ];
            state.games.word_ladder = crate::word_ladder::WordLadder::new_with_mode_config(
                seed(state.games.word_ladder.seed),
                next(&all, state.games.word_ladder.mode),
                &data.content.words.word_ladder.dictionary,
                &data.content.words.word_ladder.puzzles,
            );
        }
        GameId::PipeLoop => {
            let all = [
                crate::pipe_loop::PipePattern::Serpent,
                crate::pipe_loop::PipePattern::Trunk,
            ];
            state.games.pipe_loop.set_pattern(
                next(&all, state.games.pipe_loop.pattern),
                seed(state.games.pipe_loop.seed),
            );
        }
        GameId::MazeWalk => {
            let all = [
                crate::maze_walk::MazeMode::Explorer,
                crate::maze_walk::MazeMode::Fog,
            ];
            state.games.maze_walk.set_mode(
                next(&all, state.games.maze_walk.mode),
                seed(state.games.maze_walk.seed),
            );
        }
        GameId::MatchThree => {
            let difficulty = next(
                &crate::match_three::MatchThreeDifficulty::ALL,
                state.games.match_three.difficulty,
            );
            state.games.match_three = crate::match_three::MatchThree::new_with_config(
                seed(state.games.match_three.seed),
                difficulty,
                &data.puzzles.match_three,
            );
        }
        GameId::Pyramid => {
            let all = [
                crate::pyramid::PyramidDraw::One,
                crate::pyramid::PyramidDraw::Three,
            ];
            state.games.pyramid.set_draw_rule(
                next(&all, state.games.pyramid.draw_rule),
                seed(state.games.pyramid.seed),
            );
        }
        GameId::TriPeaks => {
            let all = [
                crate::tri_peaks::TriPeaksRule::Strict,
                crate::tri_peaks::TriPeaksRule::Wrap,
            ];
            state.games.tri_peaks.set_rule(
                next(&all, state.games.tri_peaks.rule),
                seed(state.games.tri_peaks.seed),
            );
        }
        GameId::Nim => {
            let all = [crate::nim::NimRule::Normal, crate::nim::NimRule::Misere];
            state
                .games
                .nim
                .set_rule(next(&all, state.games.nim.rule), seed(state.games.nim.seed));
        }
        GameId::SpaceInvaders => state.games.space_invaders.cycle_mode(),
        GameId::Asteroids => state.games.asteroids.cycle_mode(),
        GameId::Frogger => state.games.frogger.cycle_mode(),
        GameId::MunchMaze => state.games.munch_maze.cycle_mode(),
        GameId::BlockStack => state.games.block_stack.cycle_mode(),
        GameId::TerrainCannon => state.games.terrain_cannon.cycle_mode(),
        GameId::FlingFury => state.games.fling_fury.cycle_mode(),
        GameId::PaddleDuel => state.games.paddle_duel.cycle_mode(),
        GameId::RiddleRoom => {
            let seed = state.games.riddle_room.seed;
            state.games.riddle_room.reset(seed.wrapping_add(1));
        }
        GameId::PatternVault => {
            let seed = state.games.pattern_vault.seed;
            state.games.pattern_vault.reset(seed.wrapping_add(1));
        }
        GameId::SumCircuit => {
            let seed = state.games.sum_circuit.seed;
            state.games.sum_circuit.reset(seed.wrapping_add(1));
        }
        GameId::OrbitOrder => {
            let seed = state.games.orbit_order.seed;
            state.games.orbit_order.reset(seed.wrapping_add(1));
        }
        GameId::WordForge => {
            let seed = state.games.word_forge.seed;
            state.games.word_forge.reset(seed.wrapping_add(1));
        }
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

#[cfg(test)]
mod tests;
