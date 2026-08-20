//! Small action handlers for the touch-first board-game cabinets.

use super::Game;
use crate::{sound::SoundCue, ui::UiAction};
use macroquad::prelude::get_time;

fn fresh_seed(previous: u64) -> u64 {
    get_time().to_bits() ^ previous.rotate_left(17)
}

impl Game {
    pub(super) fn apply_game_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::CycleGameVariant => {
                if let crate::state::Screen::Game(game) = self.state.screen {
                    crate::game_variants::cycle(&mut self.state, &self.data, game);
                }
            }
            UiAction::HangmanGuess(letter) => {
                self.state.games.hangman.guess(*letter);
            }
            UiAction::HangmanHint => {
                self.state.card_hint = Some(crate::card_hints::hangman(&self.state));
            }
            UiAction::HangmanNew => {
                let seed = self.state.games.hangman.seed.wrapping_add(1);
                self.state.games.hangman.reset(seed);
            }
            UiAction::HangmanReveal => {
                self.state.games.hangman.reveal();
            }
            UiAction::HangmanUndo => {
                self.state.games.hangman.undo();
            }
            UiAction::HangmanCategory(category) => {
                let seed = self.state.games.hangman.seed.wrapping_add(1);
                self.state.games.hangman.set_category(*category, seed);
            }
            UiAction::HangmanRule(rule) => {
                let seed = self.state.games.hangman.seed.wrapping_add(1);
                self.state.games.hangman.set_rule(*rule, seed);
            }
            UiAction::LightsOutGuide => {
                self.state.games.lights_out.toggle_guide();
            }
            UiAction::LightsOutDifficulty(difficulty) => {
                let seed = self.state.games.lights_out.seed.wrapping_add(1);
                self.state
                    .games
                    .lights_out
                    .set_difficulty(*difficulty, seed);
            }
            UiAction::MemoryPairsPeek => {
                self.state.games.memory_pairs.peek();
            }
            UiAction::ConnectFourDrop(column) => self.apply_connect_four_drop(*column),
            UiAction::ConnectFourHint => {
                self.state.card_hint = Some(crate::card_hints::connect_four(&self.state));
                return true;
            }
            UiAction::ConnectFourUndo => self.apply_connect_four_undo(),
            UiAction::ConnectFourNew => self.apply_connect_four_new(),
            UiAction::ConnectFourLevel(level) => self.state.games.connect_four.set_ai_level(*level),
            UiAction::CheckersTap(square) => self.apply_checkers_tap(*square),
            UiAction::CheckersHint => {
                self.state.card_hint = Some(crate::card_hints::checkers(&self.state));
                return true;
            }
            UiAction::CheckersUndo => self.apply_checkers_undo(),
            UiAction::CheckersNew => self.apply_checkers_new(),
            UiAction::CheckersLevel(level) => self.state.games.checkers.set_ai_level(*level),
            UiAction::PegSolitaireTap(square) => {
                self.state.games.peg_solitaire.tap(*square);
            }
            UiAction::PegSolitaireHint => {
                self.state.card_hint = Some(crate::card_hints::peg_solitaire(&self.state));
                return true;
            }
            UiAction::PegSolitaireUndo => {
                self.state.games.peg_solitaire.undo();
            }
            UiAction::PegSolitaireNew => {
                let seed = self.state.games.peg_solitaire.seed.wrapping_add(1);
                self.state.games.peg_solitaire.reset(seed);
            }
            UiAction::MahjongSolitaireTap(index) => {
                self.state.games.mahjong_solitaire.tap(*index);
            }
            UiAction::MahjongSolitaireHint => {
                self.state.card_hint = Some(crate::card_hints::mahjong_solitaire(&self.state));
                return true;
            }
            UiAction::MahjongSolitaireUndo => {
                self.state.games.mahjong_solitaire.undo();
            }
            UiAction::MahjongSolitaireNew => {
                let seed = self.state.games.mahjong_solitaire.seed.wrapping_add(1);
                self.state.games.mahjong_solitaire.reset(seed);
            }
            UiAction::SnakeStep(direction) => {
                self.state.games.snake.set_direction(*direction);
            }
            UiAction::SnakeMode(mode) => {
                let seed = self.state.games.snake.seed.wrapping_add(1);
                self.state.games.snake = crate::snake::Snake::new_with_mode(seed, *mode);
            }
            UiAction::SnakePause => {
                self.state.games.snake.toggle_pause();
            }
            UiAction::SnakeHint => {
                self.state.card_hint = Some(crate::card_hints::snake(&self.state));
                return true;
            }
            UiAction::SnakeUndo => {
                self.state.games.snake.undo();
            }
            UiAction::SnakeNew => {
                let seed = self.state.games.snake.seed.wrapping_add(1);
                self.state.games.snake.reset(seed);
            }
            UiAction::BreakoutStep(movement) => {
                self.state.games.breakout.set_control(*movement);
            }
            UiAction::BreakoutPause => {
                self.state.games.breakout.toggle_pause();
            }
            UiAction::BreakoutHint => {
                self.state.card_hint = Some(crate::card_hints::breakout(&self.state));
                return true;
            }
            UiAction::BreakoutUndo => {
                self.state.games.breakout.undo();
            }
            UiAction::BreakoutNew => {
                let seed = self.state.games.breakout.seed.wrapping_add(1);
                self.state.games.breakout.reset(seed);
            }
            UiAction::HigherLowerGuess(guess) => {
                self.state.games.higher_lower.guess(*guess);
            }
            UiAction::HigherLowerHint => {
                self.state.card_hint = Some(crate::card_hints::higher_lower(&self.state));
                return true;
            }
            UiAction::HigherLowerUndo => {
                self.state.games.higher_lower.undo();
            }
            UiAction::HigherLowerNew => {
                let seed = self.state.games.higher_lower.seed.wrapping_add(1);
                self.state.games.higher_lower.reset(seed);
            }
            UiAction::HigherLowerCashOut => {
                self.state.games.higher_lower.cash_out();
            }
            UiAction::HigherLowerRule(rule) => {
                let seed = self.state.games.higher_lower.seed.wrapping_add(1);
                self.state.games.higher_lower.set_rule(*rule, seed);
            }
            UiAction::KlondikeGolfColumn(column) => {
                self.state.games.klondike_golf.tap_column(*column);
            }
            UiAction::KlondikeGolfStock => {
                self.state.games.klondike_golf.draw_stock();
            }
            UiAction::KlondikeGolfHint => {
                self.state.card_hint = Some(crate::card_hints::klondike_golf(&self.state));
            }
            UiAction::KlondikeGolfUndo => {
                self.state.games.klondike_golf.undo();
            }
            UiAction::KlondikeGolfNew => {
                let seed = self.state.games.klondike_golf.seed.wrapping_add(1);
                self.state.games.klondike_golf.reset(seed);
            }
            UiAction::BlackjackHit => {
                self.state.games.blackjack.hit();
            }
            UiAction::BlackjackStand => {
                self.state.games.blackjack.stand();
            }
            UiAction::BlackjackHint => {
                self.state.card_hint = Some(crate::card_hints::blackjack(&self.state));
                return true;
            }
            UiAction::BlackjackUndo => {
                self.state.games.blackjack.undo();
            }
            UiAction::BlackjackNew => {
                let seed = self.state.games.blackjack.seed.wrapping_add(1);
                self.state.games.blackjack.reset(seed);
            }
            UiAction::SpiderSolitaireSelect(column, depth) => {
                self.state
                    .games
                    .spider_solitaire
                    .select_column(*column, *depth);
            }
            UiAction::SpiderSolitaireMove(column) => {
                self.state.games.spider_solitaire.move_selected(*column);
            }
            UiAction::SpiderSolitaireDeal => {
                self.state.games.spider_solitaire.deal_stock();
            }
            UiAction::SpiderSolitaireHint => {
                self.state.card_hint = Some(crate::card_hints::spider_solitaire(&self.state));
            }
            UiAction::SpiderSolitaireUndo => {
                self.state.games.spider_solitaire.undo();
            }
            UiAction::SpiderSolitaireNew => {
                let seed = self.state.games.spider_solitaire.seed.wrapping_add(1);
                self.state.games.spider_solitaire.reset(seed);
            }
            UiAction::PyramidTap(index) => {
                self.state.games.pyramid.tap(*index);
            }
            UiAction::PyramidStock => {
                self.state.games.pyramid.draw_stock();
            }
            UiAction::PyramidHint => {
                self.state.card_hint = Some(crate::card_hints::pyramid(&self.state));
            }
            UiAction::PyramidUndo => {
                self.state.games.pyramid.undo();
            }
            UiAction::PyramidNew => {
                let seed = self.state.games.pyramid.seed.wrapping_add(1);
                self.state.games.pyramid.reset(seed);
            }
            UiAction::PyramidDrawRule(rule) => {
                let seed = self.state.games.pyramid.seed.wrapping_add(1);
                self.state.games.pyramid.set_draw_rule(*rule, seed);
            }
            UiAction::TriPeaksTap(index) => {
                self.state.games.tri_peaks.tap(*index);
            }
            UiAction::TriPeaksStock => {
                self.state.games.tri_peaks.draw_stock();
            }
            UiAction::TriPeaksHint => {
                self.state.card_hint = Some(crate::card_hints::tri_peaks(&self.state));
            }
            UiAction::TriPeaksUndo => {
                self.state.games.tri_peaks.undo();
            }
            UiAction::TriPeaksNew => {
                let seed = self.state.games.tri_peaks.seed.wrapping_add(1);
                self.state.games.tri_peaks.reset(seed);
            }
            UiAction::TriPeaksRule(rule) => {
                let seed = self.state.games.tri_peaks.seed.wrapping_add(1);
                self.state.games.tri_peaks.set_rule(*rule, seed);
            }
            UiAction::TriPeaksBridge => {
                self.state.games.tri_peaks.toggle_bridge();
            }
            UiAction::NimSelect(heap) => {
                self.state.games.nim.select_heap(*heap);
            }
            UiAction::NimTake(amount) => {
                self.state.games.nim.take(*amount);
            }
            UiAction::NimHint => {
                self.state.card_hint = Some(crate::card_hints::nim(&self.state));
            }
            UiAction::NimUndo => {
                self.state.games.nim.undo();
            }
            UiAction::NimNew => {
                let seed = self.state.games.nim.seed.wrapping_add(1);
                self.state.games.nim.reset(seed);
            }
            UiAction::NimRule(rule) => {
                let seed = self.state.games.nim.seed.wrapping_add(1);
                self.state.games.nim.set_rule(*rule, seed);
            }
            UiAction::DungeonCell(index) => {
                if self.state.mine_flag_mode {
                    self.state.games.dungeon_sweeper.toggle_flag(*index);
                } else {
                    self.state.games.dungeon_sweeper.reveal(*index);
                }
            }
            UiAction::DungeonToggleFlag => {
                self.state.mine_flag_mode = !self.state.mine_flag_mode;
            }
            UiAction::DungeonHint => {
                self.state.card_hint = Some(crate::card_hints::dungeon_sweeper(&self.state));
                return true;
            }
            UiAction::DungeonUndo => {
                self.state.games.dungeon_sweeper.undo();
            }
            UiAction::DungeonNew => {
                let seed = self.state.games.dungeon_sweeper.seed.wrapping_add(1);
                self.state.games.dungeon_sweeper.reset(seed);
            }
            UiAction::DungeonDifficulty(difficulty) => {
                let seed = self.state.games.dungeon_sweeper.seed.wrapping_add(1);
                self.state.games.dungeon_sweeper =
                    crate::dungeon_sweeper::DungeonSweeper::new_with_difficulty(seed, *difficulty);
            }
            UiAction::PotionMove(direction) => {
                self.state.games.potion_2048.move_in(*direction);
            }
            UiAction::PotionHint => {
                self.state.card_hint = Some(crate::card_hints::potion_2048(&self.state));
                return true;
            }
            UiAction::PotionUndo => {
                self.state.games.potion_2048.undo();
            }
            UiAction::PotionNew => {
                let seed = self.state.games.potion_2048.seed.wrapping_add(1);
                self.state.games.potion_2048.reset(seed);
            }
            UiAction::PotionDifficulty(difficulty) => {
                let seed = self.state.games.potion_2048.seed.wrapping_add(1);
                self.state.games.potion_2048 =
                    crate::potion_2048::Potion2048::new_with_difficulty(seed, *difficulty);
            }
            UiAction::TowerCell(index) => {
                self.state.games.tiny_tower_defence.build_or_upgrade(*index);
            }
            UiAction::TowerSelectKind(kind) => {
                self.state.games.tiny_tower_defence.select_kind(*kind);
            }
            UiAction::TowerWave => {
                self.state.games.tiny_tower_defence.start_or_toggle_pause();
            }
            UiAction::TowerHint => {
                self.state.card_hint = Some(crate::card_hints::tiny_tower_defence(&self.state));
                return true;
            }
            UiAction::TowerUndo => {
                self.state.games.tiny_tower_defence.undo();
            }
            UiAction::TowerNew => {
                let seed = self.state.games.tiny_tower_defence.seed.wrapping_add(1);
                self.state.games.tiny_tower_defence.reset(seed);
            }
            UiAction::RogueMove(direction) => {
                self.state.games.one_room_roguelike.move_in(*direction);
            }
            UiAction::RogueStrike => {
                self.state.games.one_room_roguelike.strike();
            }
            UiAction::RoguePotion => {
                self.state.games.one_room_roguelike.drink_potion();
            }
            UiAction::RogueHint => {
                self.state.card_hint = Some(crate::card_hints::one_room_roguelike(&self.state));
                return true;
            }
            UiAction::RogueUndo => {
                self.state.games.one_room_roguelike.undo();
            }
            UiAction::RogueNew => {
                let seed = self.state.games.one_room_roguelike.seed.wrapping_add(1);
                self.state.games.one_room_roguelike.reset(seed);
            }
            UiAction::RogueClass(hero_class) => {
                let seed = self.state.games.one_room_roguelike.seed.wrapping_add(1);
                self.state.games.one_room_roguelike =
                    crate::one_room_roguelike::OneRoomRoguelike::new_with_class(seed, *hero_class);
            }
            UiAction::DailyMove(direction) => {
                self.state.games.daily_dungeon.move_in(*direction);
            }
            UiAction::DailyScout => {
                self.state.games.daily_dungeon.scout();
            }
            UiAction::DailyHint => {
                self.state.card_hint = Some(crate::card_hints::daily_dungeon(&self.state));
                return true;
            }
            UiAction::DailyUndo => {
                self.state.games.daily_dungeon.undo();
            }
            UiAction::DailyNew => {
                let seed = self.state.games.daily_dungeon.seed.wrapping_add(1);
                self.state.games.daily_dungeon.reset(seed);
            }
            UiAction::DotsEdge(edge) => {
                self.state.games.dots_boxes.play(*edge);
            }
            UiAction::DotsHint => {
                self.state.card_hint = Some(crate::card_hints::dots_boxes(&self.state));
                return true;
            }
            UiAction::DotsUndo => {
                self.state.games.dots_boxes.undo();
            }
            UiAction::DotsNew => {
                let seed = self.state.games.dots_boxes.seed.wrapping_add(1);
                self.state.games.dots_boxes.reset(seed);
            }
            UiAction::DotsDifficulty(difficulty) => {
                let seed = self.state.games.dots_boxes.seed.wrapping_add(1);
                self.state.games.dots_boxes = crate::dots_boxes::DotsBoxes::new_with_config(
                    seed,
                    *difficulty,
                    &self.data.puzzles.dots_boxes,
                );
            }
            UiAction::SokobanMove(direction) => {
                self.state.games.sokoban.move_in(*direction);
            }
            UiAction::SokobanHint => {
                self.state.card_hint = Some(crate::card_hints::sokoban(&self.state));
                return true;
            }
            UiAction::SokobanUndo => {
                self.state.games.sokoban.undo();
            }
            UiAction::SokobanRestart => {
                let seed = self.state.games.sokoban.seed;
                self.state.games.sokoban.reset(seed);
            }
            UiAction::SokobanNew => {
                let seed = self.state.games.sokoban.seed.wrapping_add(1);
                self.state.games.sokoban.reset_next(seed);
            }
            UiAction::MancalaPit(pit) => {
                self.state.games.mancala.play(*pit);
            }
            UiAction::MancalaHint => {
                self.state.card_hint = Some(crate::card_hints::mancala(&self.state));
                return true;
            }
            UiAction::MancalaUndo => {
                self.state.games.mancala.undo();
            }
            UiAction::MancalaNew => {
                let seed = self.state.games.mancala.seed.wrapping_add(1);
                self.state.games.mancala.reset(seed);
            }
            UiAction::MancalaLevel(level) => self.state.games.mancala.set_ai_level(*level),
            UiAction::MancalaVariant(variant) => {
                let seed = self.state.games.mancala.seed.wrapping_add(1);
                self.state.games.mancala.set_variant(*variant, seed);
            }
            UiAction::HanoiPeg(peg) => {
                self.state.games.hanoi.tap_peg(*peg);
            }
            UiAction::HanoiHint => {
                self.state.card_hint = Some(crate::card_hints::hanoi(&self.state));
                return true;
            }
            UiAction::HanoiUndo => {
                self.state.games.hanoi.undo();
            }
            UiAction::HanoiNew => {
                let seed = self.state.games.hanoi.seed.wrapping_add(1);
                self.state.games.hanoi.reset(seed);
            }
            UiAction::HanoiDisks(disks) => {
                let seed = self.state.games.hanoi.seed.wrapping_add(1);
                self.state.games.hanoi.set_disks(*disks, seed);
            }
            UiAction::NumberMatchTap(index) => {
                self.state.games.number_match.tap(*index);
            }
            UiAction::NumberMatchHint => {
                self.state.card_hint = Some(crate::card_hints::number_match(&self.state));
                return true;
            }
            UiAction::NumberMatchUndo => {
                self.state.games.number_match.undo();
            }
            UiAction::NumberMatchNew => {
                let seed = self.state.games.number_match.seed.wrapping_add(1);
                self.state.games.number_match.reset(seed);
            }
            UiAction::NumberMatchRemix => {
                self.state.games.number_match.remix();
            }
            UiAction::NumberMatchRule(rule) => {
                let seed = self.state.games.number_match.seed.wrapping_add(1);
                self.state.games.number_match.set_rule(*rule, seed);
            }
            UiAction::FloodColor(color) => {
                self.state.games.flood_it.choose(*color);
            }
            UiAction::FloodHint => {
                self.state.card_hint = Some(crate::card_hints::flood_it(&self.state));
                return true;
            }
            UiAction::FloodUndo => {
                self.state.games.flood_it.undo();
            }
            UiAction::FloodNew => {
                let seed = self.state.games.flood_it.seed.wrapping_add(1);
                self.state.games.flood_it.reset(seed);
            }
            UiAction::FloodSurge => {
                self.state.games.flood_it.use_surge();
            }
            UiAction::FloodDifficulty(difficulty) => {
                let seed = self.state.games.flood_it.seed.wrapping_add(1);
                self.state.games.flood_it = crate::flood_it::FloodIt::new_with_config(
                    seed,
                    *difficulty,
                    &self.data.puzzles.flood_it,
                );
            }
            UiAction::ColorSortTap(tube) => {
                self.state.games.color_sort.tap_tube(*tube);
            }
            UiAction::ColorSortHint => {
                self.state.card_hint = Some(crate::card_hints::color_sort(&self.state));
                return true;
            }
            UiAction::ColorSortUndo => {
                self.state.games.color_sort.undo();
            }
            UiAction::ColorSortNew => {
                let seed = fresh_seed(self.state.games.color_sort.seed);
                self.state.games.color_sort.reset(seed);
            }
            UiAction::ColorSortDifficulty(difficulty) => {
                let seed = fresh_seed(self.state.games.color_sort.seed);
                self.state.games.color_sort = crate::color_sort::ColorSort::new_with_config(
                    seed,
                    *difficulty,
                    &self.data.puzzles.color_sort,
                );
            }
            UiAction::BattleshipFire(cell) => {
                self.state.games.battleship.fire(*cell);
            }
            UiAction::BattleshipSonar => {
                self.state.games.battleship.toggle_sonar();
            }
            UiAction::BattleshipHint => {
                self.state.card_hint = Some(crate::card_hints::battleship(&self.state));
                return true;
            }
            UiAction::BattleshipUndo => {
                self.state.games.battleship.undo();
            }
            UiAction::BattleshipNew => {
                let seed = self.state.games.battleship.seed.wrapping_add(1);
                self.state.games.battleship.reset(seed);
            }
            UiAction::WordGridLetter(letter) => {
                self.state.games.word_grid.tap_letter(*letter);
            }
            UiAction::WordGridBackspace => {
                self.state.games.word_grid.backspace();
            }
            UiAction::WordGridSubmit => {
                self.state.games.word_grid.submit();
            }
            UiAction::WordGridHint => {
                self.state.card_hint = Some(crate::card_hints::word_grid(&self.state));
                return true;
            }
            UiAction::WordGridUndo => {
                self.state.games.word_grid.undo();
            }
            UiAction::WordGridNew => {
                let seed = self.state.games.word_grid.seed.wrapping_add(1);
                self.state.games.word_grid.reset(seed);
            }
            UiAction::WordGridMode(mode) => {
                let seed = self.state.games.word_grid.seed.wrapping_add(1);
                self.state.games.word_grid.set_mode(*mode, seed);
            }
            UiAction::WordLadderLetter(letter) => {
                self.state.games.word_ladder.tap_letter(*letter);
            }
            UiAction::WordLadderBackspace => {
                self.state.games.word_ladder.backspace();
            }
            UiAction::WordLadderSubmit => {
                self.state.games.word_ladder.submit();
            }
            UiAction::WordLadderHint => {
                self.state.card_hint = Some(crate::card_hints::word_ladder(&self.state));
                return true;
            }
            UiAction::WordLadderUndo => {
                self.state.games.word_ladder.undo();
            }
            UiAction::WordLadderNew => {
                let seed = self.state.games.word_ladder.seed.wrapping_add(1);
                self.state.games.word_ladder.reset(seed);
            }
            UiAction::WordLadderMode(mode) => {
                let seed = self.state.games.word_ladder.seed.wrapping_add(1);
                self.state.games.word_ladder.set_mode(*mode, seed);
            }
            UiAction::PipeRotate(index) => {
                self.state.games.pipe_loop.rotate(*index);
            }
            UiAction::PipeHint => {
                self.state.card_hint = Some(crate::card_hints::pipe_loop(&self.state));
                return true;
            }
            UiAction::PipeUndo => {
                self.state.games.pipe_loop.undo();
            }
            UiAction::PipeNew => {
                let seed = self.state.games.pipe_loop.seed.wrapping_add(1);
                self.state.games.pipe_loop.reset(seed);
            }
            UiAction::PipePattern(pattern) => {
                let seed = self.state.games.pipe_loop.seed.wrapping_add(1);
                self.state.games.pipe_loop.set_pattern(*pattern, seed);
            }
            UiAction::MazeStep(direction) => {
                self.state.games.maze_walk.step(*direction);
            }
            UiAction::MazeHint => {
                self.state.card_hint = Some(crate::card_hints::maze_walk(&self.state));
                return true;
            }
            UiAction::MazeUndo => {
                self.state.games.maze_walk.undo();
            }
            UiAction::MazeNew => {
                let seed = self.state.games.maze_walk.seed.wrapping_add(1);
                self.state.games.maze_walk.reset(seed);
            }
            UiAction::MazeMode(mode) => {
                let seed = self.state.games.maze_walk.seed.wrapping_add(1);
                self.state.games.maze_walk.set_mode(*mode, seed);
            }
            UiAction::MatchThreeTap(index) => {
                self.state.games.match_three.tap(*index);
            }
            UiAction::MatchThreeHint => {
                self.state.card_hint = Some(crate::card_hints::match_three(&self.state));
                return true;
            }
            UiAction::MatchThreeUndo => {
                self.state.games.match_three.undo();
            }
            UiAction::MatchThreeNew => {
                let seed = self.state.games.match_three.seed.wrapping_add(1);
                self.state.games.match_three.reset(seed);
            }
            UiAction::MatchThreeDifficulty(difficulty) => {
                let seed = self.state.games.match_three.seed.wrapping_add(1);
                self.state.games.match_three = crate::match_three::MatchThree::new_with_config(
                    seed,
                    *difficulty,
                    &self.data.puzzles.match_three,
                );
            }
            _ => return false,
        }
        true
    }

    pub(super) fn finish_action(
        &mut self,
        previous_screen: crate::state::Screen,
        action: UiAction,
    ) {
        if action.starts_new_round() {
            self.reset_elapsed();
        }
        if self.state.screen != previous_screen {
            self.transition = if self.state.reduced_motion { 0. } else { 1. };
        }
        self.update_records();
        self.request_autosave();
        self.play_feedback(
            if matches!(
                action,
                UiAction::ConfirmRestart | UiAction::ConfirmResetData
            ) {
                SoundCue::Success
            } else {
                SoundCue::Tap
            },
        );
    }

    pub(super) fn apply_connect_four_drop(&mut self, column: usize) {
        self.state.games.connect_four.drop(column);
    }

    pub(super) fn apply_connect_four_undo(&mut self) {
        self.state.games.connect_four.undo();
    }

    pub(super) fn apply_connect_four_new(&mut self) {
        let seed = self.state.games.connect_four.seed.wrapping_add(1);
        self.state.games.connect_four.reset(seed);
    }

    pub(super) fn apply_checkers_tap(&mut self, square: usize) {
        self.state.games.checkers.tap(square);
    }

    pub(super) fn apply_checkers_undo(&mut self) {
        self.state.games.checkers.undo();
    }

    pub(super) fn apply_checkers_new(&mut self) {
        let seed = self.state.games.checkers.seed.wrapping_add(1);
        self.state.games.checkers.reset(seed);
    }
}
