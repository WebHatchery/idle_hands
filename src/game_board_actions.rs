//! Small action handlers for the touch-first board-game cabinets.

use super::Game;
use crate::{sound::SoundCue, ui::UiAction};

impl Game {
    pub(super) fn apply_board_action(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::HangmanGuess(letter) => {
                self.state.hangman.guess(*letter);
            }
            UiAction::HangmanHint => {
                self.state.card_hint = Some(crate::card_hints::hangman(&self.state));
            }
            UiAction::HangmanNew => {
                let seed = self.state.hangman.seed.wrapping_add(1);
                self.state.hangman.reset(seed);
            }
            UiAction::ConnectFourDrop(column) => self.apply_connect_four_drop(*column),
            UiAction::ConnectFourHint => {
                self.state.card_hint = Some(crate::card_hints::connect_four(&self.state));
                return true;
            }
            UiAction::ConnectFourUndo => self.apply_connect_four_undo(),
            UiAction::ConnectFourNew => self.apply_connect_four_new(),
            UiAction::CheckersTap(square) => self.apply_checkers_tap(*square),
            UiAction::CheckersHint => {
                self.state.card_hint = Some(crate::card_hints::checkers(&self.state));
                return true;
            }
            UiAction::CheckersUndo => self.apply_checkers_undo(),
            UiAction::CheckersNew => self.apply_checkers_new(),
            UiAction::PegSolitaireTap(square) => {
                self.state.peg_solitaire.tap(*square);
            }
            UiAction::PegSolitaireHint => {
                self.state.card_hint = Some(crate::card_hints::peg_solitaire(&self.state));
                return true;
            }
            UiAction::PegSolitaireUndo => {
                self.state.peg_solitaire.undo();
            }
            UiAction::PegSolitaireNew => {
                let seed = self.state.peg_solitaire.seed.wrapping_add(1);
                self.state.peg_solitaire.reset(seed);
            }
            UiAction::MahjongSolitaireTap(index) => {
                self.state.mahjong_solitaire.tap(*index);
            }
            UiAction::MahjongSolitaireHint => {
                self.state.card_hint = Some(crate::card_hints::mahjong_solitaire(&self.state));
                return true;
            }
            UiAction::MahjongSolitaireUndo => {
                self.state.mahjong_solitaire.undo();
            }
            UiAction::MahjongSolitaireNew => {
                let seed = self.state.mahjong_solitaire.seed.wrapping_add(1);
                self.state.mahjong_solitaire.reset(seed);
            }
            UiAction::SnakeStep(direction) => {
                self.state.snake.step(*direction);
            }
            UiAction::SnakeHint => {
                self.state.card_hint = Some(crate::card_hints::snake(&self.state));
                return true;
            }
            UiAction::SnakeUndo => {
                self.state.snake.undo();
            }
            UiAction::SnakeNew => {
                let seed = self.state.snake.seed.wrapping_add(1);
                self.state.snake.reset(seed);
            }
            UiAction::BreakoutStep(movement) => {
                self.state.breakout.step(*movement);
            }
            UiAction::BreakoutHint => {
                self.state.card_hint = Some(crate::card_hints::breakout(&self.state));
                return true;
            }
            UiAction::BreakoutUndo => {
                self.state.breakout.undo();
            }
            UiAction::BreakoutNew => {
                let seed = self.state.breakout.seed.wrapping_add(1);
                self.state.breakout.reset(seed);
            }
            UiAction::HigherLowerGuess(guess) => {
                self.state.higher_lower.guess(*guess);
            }
            UiAction::HigherLowerHint => {
                self.state.card_hint = Some(crate::card_hints::higher_lower(&self.state));
                return true;
            }
            UiAction::HigherLowerUndo => {
                self.state.higher_lower.undo();
            }
            UiAction::HigherLowerNew => {
                let seed = self.state.higher_lower.seed.wrapping_add(1);
                self.state.higher_lower.reset(seed);
            }
            UiAction::KlondikeGolfColumn(column) => {
                self.state.klondike_golf.tap_column(*column);
            }
            UiAction::KlondikeGolfStock => {
                self.state.klondike_golf.draw_stock();
            }
            UiAction::KlondikeGolfHint => {
                self.state.card_hint = Some(crate::card_hints::klondike_golf(&self.state));
            }
            UiAction::KlondikeGolfUndo => {
                self.state.klondike_golf.undo();
            }
            UiAction::KlondikeGolfNew => {
                let seed = self.state.klondike_golf.seed.wrapping_add(1);
                self.state.klondike_golf.reset(seed);
            }
            UiAction::BlackjackHit => {
                self.state.blackjack.hit();
            }
            UiAction::BlackjackStand => {
                self.state.blackjack.stand();
            }
            UiAction::BlackjackHint => {
                self.state.card_hint = Some(crate::card_hints::blackjack(&self.state));
                return true;
            }
            UiAction::BlackjackUndo => {
                self.state.blackjack.undo();
            }
            UiAction::BlackjackNew => {
                let seed = self.state.blackjack.seed.wrapping_add(1);
                self.state.blackjack.reset(seed);
            }
            UiAction::SpiderSolitaireSelect(column, depth) => {
                self.state.spider_solitaire.select_column(*column, *depth);
            }
            UiAction::SpiderSolitaireMove(column) => {
                self.state.spider_solitaire.move_selected(*column);
            }
            UiAction::SpiderSolitaireDeal => {
                self.state.spider_solitaire.deal_stock();
            }
            UiAction::SpiderSolitaireHint => {
                self.state.card_hint = Some(crate::card_hints::spider_solitaire(&self.state));
            }
            UiAction::SpiderSolitaireUndo => {
                self.state.spider_solitaire.undo();
            }
            UiAction::SpiderSolitaireNew => {
                let seed = self.state.spider_solitaire.seed.wrapping_add(1);
                self.state.spider_solitaire.reset(seed);
            }
            UiAction::PyramidTap(index) => {
                self.state.pyramid.tap(*index);
            }
            UiAction::PyramidStock => {
                self.state.pyramid.draw_stock();
            }
            UiAction::PyramidHint => {
                self.state.card_hint = Some(crate::card_hints::pyramid(&self.state));
            }
            UiAction::PyramidUndo => {
                self.state.pyramid.undo();
            }
            UiAction::PyramidNew => {
                let seed = self.state.pyramid.seed.wrapping_add(1);
                self.state.pyramid.reset(seed);
            }
            UiAction::TriPeaksTap(index) => {
                self.state.tri_peaks.tap(*index);
            }
            UiAction::TriPeaksStock => {
                self.state.tri_peaks.draw_stock();
            }
            UiAction::TriPeaksHint => {
                self.state.card_hint = Some(crate::card_hints::tri_peaks(&self.state));
            }
            UiAction::TriPeaksUndo => {
                self.state.tri_peaks.undo();
            }
            UiAction::TriPeaksNew => {
                let seed = self.state.tri_peaks.seed.wrapping_add(1);
                self.state.tri_peaks.reset(seed);
            }
            UiAction::NimSelect(heap) => {
                self.state.nim.select_heap(*heap);
            }
            UiAction::NimTake(amount) => {
                self.state.nim.take(*amount);
            }
            UiAction::NimHint => {
                self.state.card_hint = Some(crate::card_hints::nim(&self.state));
            }
            UiAction::NimUndo => {
                self.state.nim.undo();
            }
            UiAction::NimNew => {
                let seed = self.state.nim.seed.wrapping_add(1);
                self.state.nim.reset(seed);
            }
            UiAction::DungeonCell(index) => {
                if self.state.mine_flag_mode {
                    self.state.dungeon_sweeper.toggle_flag(*index);
                } else {
                    self.state.dungeon_sweeper.reveal(*index);
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
                self.state.dungeon_sweeper.undo();
            }
            UiAction::DungeonNew => {
                let seed = self.state.dungeon_sweeper.seed.wrapping_add(1);
                self.state.dungeon_sweeper.reset(seed);
            }
            UiAction::PotionMove(direction) => {
                self.state.potion_2048.move_in(*direction);
            }
            UiAction::PotionHint => {
                self.state.card_hint = Some(crate::card_hints::potion_2048(&self.state));
                return true;
            }
            UiAction::PotionUndo => {
                self.state.potion_2048.undo();
            }
            UiAction::PotionNew => {
                let seed = self.state.potion_2048.seed.wrapping_add(1);
                self.state.potion_2048.reset(seed);
            }
            UiAction::TowerCell(index) => {
                self.state.tiny_tower_defence.build_or_upgrade(*index);
            }
            UiAction::TowerWave => {
                self.state.tiny_tower_defence.start_or_advance();
            }
            UiAction::TowerHint => {
                self.state.card_hint = Some(crate::card_hints::tiny_tower_defence(&self.state));
                return true;
            }
            UiAction::TowerUndo => {
                self.state.tiny_tower_defence.undo();
            }
            UiAction::TowerNew => {
                let seed = self.state.tiny_tower_defence.seed.wrapping_add(1);
                self.state.tiny_tower_defence.reset(seed);
            }
            UiAction::RogueMove(direction) => {
                self.state.one_room_roguelike.move_in(*direction);
            }
            UiAction::RogueStrike => {
                self.state.one_room_roguelike.strike();
            }
            UiAction::RoguePotion => {
                self.state.one_room_roguelike.drink_potion();
            }
            UiAction::RogueUndo => {
                self.state.one_room_roguelike.undo();
            }
            UiAction::RogueNew => {
                let seed = self.state.one_room_roguelike.seed.wrapping_add(1);
                self.state.one_room_roguelike.reset(seed);
            }
            UiAction::DailyMove(direction) => {
                self.state.daily_dungeon.move_in(*direction);
            }
            UiAction::DailyUndo => {
                self.state.daily_dungeon.undo();
            }
            UiAction::DailyNew => {
                let seed = self.state.daily_dungeon.seed.wrapping_add(1);
                self.state.daily_dungeon.reset(seed);
            }
            UiAction::DotsEdge(edge) => {
                self.state.dots_boxes.play(*edge);
            }
            UiAction::DotsUndo => {
                self.state.dots_boxes.undo();
            }
            UiAction::DotsNew => {
                let seed = self.state.dots_boxes.seed.wrapping_add(1);
                self.state.dots_boxes.reset(seed);
            }
            UiAction::SokobanMove(direction) => {
                self.state.sokoban.move_in(*direction);
            }
            UiAction::SokobanUndo => {
                self.state.sokoban.undo();
            }
            UiAction::SokobanNew => {
                let seed = self.state.sokoban.seed.wrapping_add(1);
                self.state.sokoban.reset(seed);
            }
            UiAction::MancalaPit(pit) => {
                self.state.mancala.play(*pit);
            }
            UiAction::MancalaUndo => {
                self.state.mancala.undo();
            }
            UiAction::MancalaNew => {
                let seed = self.state.mancala.seed.wrapping_add(1);
                self.state.mancala.reset(seed);
            }
            UiAction::HanoiPeg(peg) => {
                self.state.hanoi.tap_peg(*peg);
            }
            UiAction::HanoiUndo => {
                self.state.hanoi.undo();
            }
            UiAction::HanoiNew => {
                let seed = self.state.hanoi.seed.wrapping_add(1);
                self.state.hanoi.reset(seed);
            }
            UiAction::NumberMatchTap(index) => {
                self.state.number_match.tap(*index);
            }
            UiAction::NumberMatchUndo => {
                self.state.number_match.undo();
            }
            UiAction::NumberMatchNew => {
                let seed = self.state.number_match.seed.wrapping_add(1);
                self.state.number_match.reset(seed);
            }
            UiAction::FloodColor(color) => {
                self.state.flood_it.choose(*color);
            }
            UiAction::FloodUndo => {
                self.state.flood_it.undo();
            }
            UiAction::FloodNew => {
                let seed = self.state.flood_it.seed.wrapping_add(1);
                self.state.flood_it.reset(seed);
            }
            UiAction::ColorSortTap(tube) => {
                self.state.color_sort.tap_tube(*tube);
            }
            UiAction::ColorSortUndo => {
                self.state.color_sort.undo();
            }
            UiAction::ColorSortNew => {
                let seed = self.state.color_sort.seed.wrapping_add(1);
                self.state.color_sort.reset(seed);
            }
            UiAction::BattleshipFire(cell) => {
                self.state.battleship.fire(*cell);
            }
            UiAction::BattleshipUndo => {
                self.state.battleship.undo();
            }
            UiAction::BattleshipNew => {
                let seed = self.state.battleship.seed.wrapping_add(1);
                self.state.battleship.reset(seed);
            }
            UiAction::WordGridLetter(letter) => {
                self.state.word_grid.tap_letter(*letter);
            }
            UiAction::WordGridBackspace => {
                self.state.word_grid.backspace();
            }
            UiAction::WordGridSubmit => {
                self.state.word_grid.submit();
            }
            UiAction::WordGridUndo => {
                self.state.word_grid.undo();
            }
            UiAction::WordGridNew => {
                let seed = self.state.word_grid.seed.wrapping_add(1);
                self.state.word_grid.reset(seed);
            }
            UiAction::PipeRotate(index) => {
                self.state.pipe_loop.rotate(*index);
            }
            UiAction::PipeUndo => {
                self.state.pipe_loop.undo();
            }
            UiAction::PipeNew => {
                let seed = self.state.pipe_loop.seed.wrapping_add(1);
                self.state.pipe_loop.reset(seed);
            }
            UiAction::MazeStep(direction) => {
                self.state.maze_walk.step(*direction);
            }
            UiAction::MazeUndo => {
                self.state.maze_walk.undo();
            }
            UiAction::MazeNew => {
                let seed = self.state.maze_walk.seed.wrapping_add(1);
                self.state.maze_walk.reset(seed);
            }
            UiAction::MatchThreeTap(index) => {
                self.state.match_three.tap(*index);
            }
            UiAction::MatchThreeUndo => {
                self.state.match_three.undo();
            }
            UiAction::MatchThreeNew => {
                let seed = self.state.match_three.seed.wrapping_add(1);
                self.state.match_three.reset(seed);
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
        if self.state.screen != previous_screen {
            self.transition = if self.state.reduced_motion { 0. } else { 1. };
        }
        self.update_records();
        self.save_autosave();
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
        self.state.connect_four.drop(column);
    }

    pub(super) fn apply_connect_four_undo(&mut self) {
        self.state.connect_four.undo();
    }

    pub(super) fn apply_connect_four_new(&mut self) {
        let seed = self.state.connect_four.seed.wrapping_add(1);
        self.state.connect_four.reset(seed);
    }

    pub(super) fn apply_checkers_tap(&mut self, square: usize) {
        self.state.checkers.tap(square);
    }

    pub(super) fn apply_checkers_undo(&mut self) {
        self.state.checkers.undo();
    }

    pub(super) fn apply_checkers_new(&mut self) {
        let seed = self.state.checkers.seed.wrapping_add(1);
        self.state.checkers.reset(seed);
    }
}
