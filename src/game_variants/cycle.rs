use super::next;
use crate::{
    data::GameData,
    state::{AppState, GameId},
};

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
