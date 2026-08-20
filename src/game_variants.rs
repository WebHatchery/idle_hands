//! Shared rule-card metadata and deterministic variant rotation.

use crate::{
    data::GameData,
    state::{AppState, GameId},
};

fn next<T: Copy + PartialEq>(all: &[T], current: T) -> T {
    let index = all.iter().position(|item| *item == current).unwrap_or(0);
    all[(index + 1) % all.len()]
}

pub fn label(state: &AppState, game: GameId) -> String {
    match game {
        GameId::Game2048 => format!("BOARD · {}", state.game.board_size.label()),
        GameId::Minesweeper => state.minesweeper.preset.label().to_owned(),
        GameId::Sudoku => state.sudoku.difficulty.label().to_owned(),
        GameId::Nonogram => format!("NONOGRAM · {}", state.nonogram.preset.label()),
        GameId::Solitaire => state.solitaire.ruleset.label().to_owned(),
        GameId::FreeCell => state.freecell.variant.label().to_owned(),
        GameId::Yahtzee => state.fivefold.variant.label().to_owned(),
        GameId::Reversi => format!("REVERSI · {:?}", state.reversi.ai_level).to_uppercase(),
        GameId::LightsOut => state.lights_out.difficulty.label().to_owned(),
        GameId::TicTacToe => {
            format!("TIC-TAC-TOE · {:?}", state.tic_tac_toe.ai_level).to_uppercase()
        }
        GameId::MemoryPairs => state.memory_pairs.variant.label().to_owned(),
        GameId::SlidingPuzzle => state.sliding_puzzle.variant.label().to_owned(),
        GameId::Mastermind => state.mastermind.variant.label().to_owned(),
        GameId::Spider => state.spider.mode.label().to_owned(),
        GameId::WordSearch => state.word_search.theme.label().to_owned(),
        GameId::Hangman => format!(
            "HANGMAN · {:?} / {:?}",
            state.hangman.category, state.hangman.rule
        )
        .to_uppercase(),
        GameId::ConnectFour => {
            format!("CONNECT FOUR · {:?}", state.connect_four.ai_level).to_uppercase()
        }
        GameId::Checkers => format!("CHECKERS · {:?}", state.checkers.ai_level).to_uppercase(),
        GameId::PegSolitaire => state.peg_solitaire.variant.label().to_owned(),
        GameId::MahjongSolitaire => state.mahjong_solitaire.layout.label().to_owned(),
        GameId::Snake => format!("SNAKE · {:?}", state.snake.mode).to_uppercase(),
        GameId::Breakout => format!("WALL RUN · {} / 3", state.breakout.level),
        GameId::HigherLower => {
            format!("HIGHER / LOWER · {:?}", state.higher_lower.rule).to_uppercase()
        }
        GameId::KlondikeGolf => state.klondike_golf.rule.label().to_owned(),
        GameId::Blackjack => state.blackjack.rule.label().to_owned(),
        GameId::SpiderSolitaire => state.spider_solitaire.rule.label().to_owned(),
        GameId::DungeonSweeper => {
            format!("DUNGEON · {:?}", state.dungeon_sweeper.difficulty).to_uppercase()
        }
        GameId::Potion2048 => format!("POTION · {:?}", state.potion_2048.difficulty).to_uppercase(),
        GameId::TinyTowerDefence => {
            format!("TOWER ROLE · {:?}", state.tiny_tower_defence.selected_kind).to_uppercase()
        }
        GameId::OneRoomRoguelike => {
            format!("ROOM ROLE · {:?}", state.one_room_roguelike.hero_class).to_uppercase()
        }
        GameId::DailyDungeon => format!("DAILY · {:?}", state.daily_dungeon.rule).to_uppercase(),
        GameId::DotsBoxes => format!("DOTS · {:?}", state.dots_boxes.difficulty).to_uppercase(),
        GameId::Sokoban => format!("WAREHOUSE · ROOM {}", state.sokoban.level + 1),
        GameId::Mancala => format!("MANCALA · {:?}", state.mancala.variant).to_uppercase(),
        GameId::Hanoi => format!("HANOI · {} DISKS", state.hanoi.disks),
        GameId::NumberMatch => {
            format!("NUMBER MATCH · {:?}", state.number_match.rule).to_uppercase()
        }
        GameId::FloodIt => format!("FLOOD IT · {:?}", state.flood_it.difficulty).to_uppercase(),
        GameId::ColorSort => {
            format!("COLOR SORT · {:?}", state.color_sort.difficulty).to_uppercase()
        }
        GameId::Battleship => state.battleship.fleet.label().to_owned(),
        GameId::WordGrid => format!("WORD GRID · {:?}", state.word_grid.mode).to_uppercase(),
        GameId::WordLadder => format!("WORD LADDER · {:?}", state.word_ladder.mode).to_uppercase(),
        GameId::PipeLoop => format!("PIPE LOOP · {:?}", state.pipe_loop.pattern).to_uppercase(),
        GameId::MazeWalk => format!("MAZE WALK · {:?}", state.maze_walk.mode).to_uppercase(),
        GameId::MatchThree => {
            format!("MATCH THREE · {:?}", state.match_three.difficulty).to_uppercase()
        }
        GameId::Pyramid => state.pyramid.draw_rule.label().to_owned(),
        GameId::TriPeaks => format!("TRIPEAKS · {:?}", state.tri_peaks.rule).to_uppercase(),
        GameId::Nim => format!("NIM · {:?}", state.nim.rule).to_uppercase(),
    }
}

pub fn cycle(state: &mut AppState, data: &GameData, game: GameId) {
    let seed = |value: u64| value.wrapping_add(1);
    match game {
        GameId::Game2048 => {
            let size = next(&crate::game_2048::Game2048Size::ALL, state.game.board_size);
            state.game = crate::state::Game2048::new_with_size(seed(state.game.seed), size);
        }
        GameId::Minesweeper => {
            let preset = next(
                &crate::minesweeper::MinePreset::ALL,
                state.minesweeper.preset,
            );
            state.minesweeper =
                crate::minesweeper::Minesweeper::new(preset, seed(state.minesweeper.seed));
        }
        GameId::Sudoku => {
            let difficulty = next(
                &crate::sudoku::SudokuDifficulty::ALL,
                state.sudoku.difficulty,
            );
            state.sudoku = crate::sudoku::Sudoku::with_difficulty(difficulty);
        }
        GameId::Nonogram => {
            let preset = next(&crate::nonogram::NonogramPreset::ALL, state.nonogram.preset);
            state.nonogram = crate::nonogram::Nonogram::new_with_variant(
                preset,
                state.nonogram.variant.wrapping_add(1),
            );
        }
        GameId::Solitaire => {
            let ruleset = next(
                &crate::solitaire::SolitaireRuleset::ALL,
                state.solitaire.ruleset,
            );
            state.solitaire =
                crate::solitaire::Solitaire::with_ruleset(seed(state.solitaire.seed), ruleset);
        }
        GameId::FreeCell => {
            let variant = next(
                &crate::freecell::FreeCellVariant::ALL,
                state.freecell.variant,
            );
            state.freecell =
                crate::freecell::FreeCell::new_with_variant(seed(state.freecell.seed), variant);
        }
        GameId::Yahtzee => {
            let variant = next(
                &crate::fivefold::FivefoldVariant::ALL,
                state.fivefold.variant,
            );
            state.fivefold =
                crate::fivefold::Fivefold::new_with_variant(seed(state.fivefold.seed), variant);
        }
        GameId::Reversi => {
            let all = [
                crate::reversi::AiLevel::Gentle,
                crate::reversi::AiLevel::Sharp,
                crate::reversi::AiLevel::TwoPlayer,
            ];
            state.reversi = crate::reversi::Reversi::new(
                seed(state.reversi.seed),
                next(&all, state.reversi.ai_level),
            );
        }
        GameId::LightsOut => {
            let all = [
                crate::lights_out::LightsDifficulty::Classic,
                crate::lights_out::LightsDifficulty::Dense,
            ];
            let difficulty = next(&all, state.lights_out.difficulty);
            state.lights_out = crate::lights_out::LightsOut::new_with_difficulty(
                seed(state.lights_out.seed),
                difficulty,
            );
        }
        GameId::TicTacToe => {
            let all = [
                crate::tic_tac_toe::AiLevel::Gentle,
                crate::tic_tac_toe::AiLevel::Sharp,
                crate::tic_tac_toe::AiLevel::Expert,
            ];
            let mut game_state = crate::tic_tac_toe::TicTacToe::new(seed(state.tic_tac_toe.seed));
            game_state.set_ai_level(next(&all, state.tic_tac_toe.ai_level));
            state.tic_tac_toe = game_state;
        }
        GameId::MemoryPairs => {
            let variant = next(
                &crate::memory_pairs::MemoryVariant::ALL,
                state.memory_pairs.variant,
            );
            state.memory_pairs = crate::memory_pairs::MemoryPairs::new_with_variant(
                seed(state.memory_pairs.seed),
                variant,
            );
        }
        GameId::SlidingPuzzle => {
            let variant = next(
                &crate::sliding_puzzle::SlidingVariant::ALL,
                state.sliding_puzzle.variant,
            );
            state.sliding_puzzle = crate::sliding_puzzle::SlidingPuzzle::new_with_variant(
                seed(state.sliding_puzzle.seed),
                variant,
            );
        }
        GameId::Mastermind => {
            let variant = next(
                &crate::mastermind::MastermindVariant::ALL,
                state.mastermind.variant,
            );
            state.mastermind = crate::mastermind::Mastermind::new_with_variant(
                seed(state.mastermind.seed),
                variant,
            );
        }
        GameId::Hangman => {
            let category = state.hangman.category.next();
            let rule = match state.hangman.rule {
                crate::hangman::HangmanRule::Classic => crate::hangman::HangmanRule::Rapid,
                crate::hangman::HangmanRule::Rapid => crate::hangman::HangmanRule::Classic,
            };
            state.hangman =
                crate::hangman::Hangman::new_with_options(seed(state.hangman.seed), category, rule);
        }
        GameId::Spider => {
            let mode = next(&crate::spider::SpiderMode::ALL, state.spider.mode);
            state.spider = crate::spider::Spider::new_with_mode(seed(state.spider.seed), mode);
        }
        GameId::WordSearch => {
            let theme = next(
                &crate::word_search::WordSearchTheme::ALL,
                state.word_search.theme,
            );
            state.word_search =
                crate::word_search::WordSearch::new_with_theme(seed(state.word_search.seed), theme);
        }
        GameId::ConnectFour => {
            let all = [
                crate::connect_four::AiLevel::Gentle,
                crate::connect_four::AiLevel::Sharp,
                crate::connect_four::AiLevel::Expert,
            ];
            let mut game_state =
                crate::connect_four::ConnectFour::new(seed(state.connect_four.seed));
            game_state.set_ai_level(next(&all, state.connect_four.ai_level));
            state.connect_four = game_state;
        }
        GameId::Checkers => {
            let all = [
                crate::checkers::AiLevel::Gentle,
                crate::checkers::AiLevel::Sharp,
                crate::checkers::AiLevel::Expert,
            ];
            let mut game_state = crate::checkers::Checkers::new(seed(state.checkers.seed));
            game_state.set_ai_level(next(&all, state.checkers.ai_level));
            state.checkers = game_state;
        }
        GameId::PegSolitaire => {
            let variant = next(
                &crate::peg_solitaire::PegVariant::ALL,
                state.peg_solitaire.variant,
            );
            state.peg_solitaire = crate::peg_solitaire::PegSolitaire::new_with_variant(
                seed(state.peg_solitaire.seed),
                variant,
            );
        }
        GameId::MahjongSolitaire => {
            let layout = next(
                &crate::mahjong_solitaire::MahjongLayout::ALL,
                state.mahjong_solitaire.layout,
            );
            state.mahjong_solitaire = crate::mahjong_solitaire::MahjongSolitaire::new_with_layout(
                seed(state.mahjong_solitaire.seed),
                layout,
            );
        }
        GameId::Snake => {
            let all = [
                crate::snake::SnakeMode::Classic,
                crate::snake::SnakeMode::Wrap,
                crate::snake::SnakeMode::Garden,
            ];
            let mode = next(&all, state.snake.mode);
            state.snake = crate::snake::Snake::new_with_mode(seed(state.snake.seed), mode);
        }
        GameId::Breakout => {
            let level = (state.breakout.level % crate::breakout::Breakout::target_level()) + 1;
            state.breakout =
                crate::breakout::Breakout::new_with_level(seed(state.breakout.seed), level);
        }
        GameId::HigherLower => {
            let all = [
                crate::higher_lower::HigherLowerRule::Friendly,
                crate::higher_lower::HigherLowerRule::House,
            ];
            let rule = next(&all, state.higher_lower.rule);
            let mut game_state =
                crate::higher_lower::HigherLower::new(seed(state.higher_lower.seed));
            game_state.set_rule(rule, seed(state.higher_lower.seed));
            state.higher_lower = game_state;
        }
        GameId::KlondikeGolf => {
            let rule = next(
                &crate::klondike_golf::GolfRule::ALL,
                state.klondike_golf.rule,
            );
            state.klondike_golf = crate::klondike_golf::KlondikeGolf::new_with_rule(
                seed(state.klondike_golf.seed),
                rule,
            );
        }
        GameId::Blackjack => {
            let rule = next(&crate::blackjack::BlackjackRule::ALL, state.blackjack.rule);
            state.blackjack =
                crate::blackjack::Blackjack::new_with_rule(seed(state.blackjack.seed), rule);
        }
        GameId::SpiderSolitaire => {
            let rule = next(
                &crate::spider_solitaire::SpiderRule::ALL,
                state.spider_solitaire.rule,
            );
            state.spider_solitaire = crate::spider_solitaire::SpiderSolitaire::new_with_rule(
                seed(state.spider_solitaire.seed),
                rule,
            );
        }
        GameId::DungeonSweeper => {
            let difficulty = next(
                &crate::dungeon_sweeper::DungeonDifficulty::ALL,
                state.dungeon_sweeper.difficulty,
            );
            state.dungeon_sweeper = crate::dungeon_sweeper::DungeonSweeper::new_with_difficulty(
                seed(state.dungeon_sweeper.seed),
                difficulty,
            );
        }
        GameId::Potion2048 => {
            let difficulty = next(
                &crate::potion_2048::PotionDifficulty::ALL,
                state.potion_2048.difficulty,
            );
            state.potion_2048 = crate::potion_2048::Potion2048::new_with_difficulty(
                seed(state.potion_2048.seed),
                difficulty,
            );
        }
        GameId::TinyTowerDefence => {
            let kind = next(
                &crate::tiny_tower_defence::TowerKind::ALL,
                state.tiny_tower_defence.selected_kind,
            );
            let mut tower = crate::tiny_tower_defence::TinyTowerDefence::new(seed(
                state.tiny_tower_defence.seed,
            ));
            tower.select_kind(kind);
            state.tiny_tower_defence = tower;
        }
        GameId::OneRoomRoguelike => {
            let class = next(
                &crate::one_room_roguelike::HeroClass::ALL,
                state.one_room_roguelike.hero_class,
            );
            state.one_room_roguelike = crate::one_room_roguelike::OneRoomRoguelike::new_with_class(
                seed(state.one_room_roguelike.seed),
                class,
            );
        }
        GameId::DailyDungeon => {
            let rule = next(
                &crate::daily_dungeon::DailyRule::ALL,
                state.daily_dungeon.rule,
            );
            state.daily_dungeon = crate::daily_dungeon::DailyDungeon::new_with_rule(
                seed(state.daily_dungeon.seed),
                rule,
            );
        }
        GameId::DotsBoxes => {
            let difficulty = next(
                &crate::dots_boxes::DotsDifficulty::ALL,
                state.dots_boxes.difficulty,
            );
            state.dots_boxes = crate::dots_boxes::DotsBoxes::new_with_config(
                seed(state.dots_boxes.seed),
                difficulty,
                &data.puzzles.dots_boxes,
            );
        }
        GameId::Sokoban => {
            let level = (state.sokoban.level + 1) % crate::sokoban::LEVEL_COUNT;
            state.sokoban =
                crate::sokoban::Sokoban::new_with_level(seed(state.sokoban.seed), level);
        }
        GameId::Battleship => {
            let fleet = next(
                &crate::battleship::BattleshipFleet::ALL,
                state.battleship.fleet,
            );
            state.battleship =
                crate::battleship::Battleship::new_with_fleet(seed(state.battleship.seed), fleet);
        }
        GameId::Hanoi => {
            let disks = next(&[3u8, 5, 7], state.hanoi.disks);
            state.hanoi = crate::hanoi::Hanoi::new_with_disks(seed(state.hanoi.seed), disks);
        }
        GameId::Mancala => {
            let all = [
                crate::mancala::MancalaVariant::Quick,
                crate::mancala::MancalaVariant::Classic,
                crate::mancala::MancalaVariant::Grand,
            ];
            state
                .mancala
                .set_variant(next(&all, state.mancala.variant), seed(state.mancala.seed));
        }
        GameId::NumberMatch => {
            let all = [
                crate::number_match::LinkRule::Neighbors,
                crate::number_match::LinkRule::Lines,
                crate::number_match::LinkRule::Diagonals,
            ];
            state.number_match.set_rule(
                next(&all, state.number_match.rule),
                seed(state.number_match.seed),
            );
        }
        GameId::FloodIt => {
            let difficulty = next(
                &crate::flood_it::FloodDifficulty::ALL,
                state.flood_it.difficulty,
            );
            state.flood_it = crate::flood_it::FloodIt::new_with_config(
                seed(state.flood_it.seed),
                difficulty,
                &data.puzzles.flood_it,
            );
        }
        GameId::ColorSort => {
            let difficulty = next(
                &crate::color_sort::ColorSortDifficulty::ALL,
                state.color_sort.difficulty,
            );
            state.color_sort = crate::color_sort::ColorSort::new_with_config(
                seed(state.color_sort.seed),
                difficulty,
                &data.puzzles.color_sort,
            );
        }
        GameId::WordGrid => {
            let all = [
                crate::word_grid::WordGridMode::Classic,
                crate::word_grid::WordGridMode::Hard,
            ];
            state.word_grid = crate::word_grid::WordGrid::new_with_mode(
                seed(state.word_grid.seed),
                next(&all, state.word_grid.mode),
            );
        }
        GameId::WordLadder => {
            let all = [
                crate::word_ladder::LadderMode::Direct,
                crate::word_ladder::LadderMode::Scenic,
            ];
            state.word_ladder = crate::word_ladder::WordLadder::new_with_mode(
                seed(state.word_ladder.seed),
                next(&all, state.word_ladder.mode),
            );
        }
        GameId::PipeLoop => {
            let all = [
                crate::pipe_loop::PipePattern::Serpent,
                crate::pipe_loop::PipePattern::Trunk,
            ];
            state.pipe_loop.set_pattern(
                next(&all, state.pipe_loop.pattern),
                seed(state.pipe_loop.seed),
            );
        }
        GameId::MazeWalk => {
            let all = [
                crate::maze_walk::MazeMode::Explorer,
                crate::maze_walk::MazeMode::Fog,
            ];
            state
                .maze_walk
                .set_mode(next(&all, state.maze_walk.mode), seed(state.maze_walk.seed));
        }
        GameId::MatchThree => {
            let difficulty = next(
                &crate::match_three::MatchThreeDifficulty::ALL,
                state.match_three.difficulty,
            );
            state.match_three = crate::match_three::MatchThree::new_with_config(
                seed(state.match_three.seed),
                difficulty,
                &data.puzzles.match_three,
            );
        }
        GameId::Pyramid => {
            let all = [
                crate::pyramid::PyramidDraw::One,
                crate::pyramid::PyramidDraw::Three,
            ];
            state.pyramid.set_draw_rule(
                next(&all, state.pyramid.draw_rule),
                seed(state.pyramid.seed),
            );
        }
        GameId::TriPeaks => {
            let all = [
                crate::tri_peaks::TriPeaksRule::Strict,
                crate::tri_peaks::TriPeaksRule::Wrap,
            ];
            state
                .tri_peaks
                .set_rule(next(&all, state.tri_peaks.rule), seed(state.tri_peaks.seed));
        }
        GameId::Nim => {
            let all = [crate::nim::NimRule::Normal, crate::nim::NimRule::Misere];
            state
                .nim
                .set_rule(next(&all, state.nim.rule), seed(state.nim.seed));
        }
    }
}

#[cfg(test)]
mod tests;
