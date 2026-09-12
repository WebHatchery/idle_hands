//! Central action dispatcher for the cabinet runtime.

use super::{game_restart, Game};
use crate::card_hints;
use crate::{
    state::{AppState, Screen},
    ui,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShellActionResult {
    Continue,
    Stop,
    Unhandled,
}

impl Game {
    pub(super) fn apply(&mut self, action: ui::UiAction) {
        self.state.games.solitaire_peek = None;
        self.state.games.spider_solitaire_peek = None;
        let previous_screen = self.state.screen;
        if !self.confirmation_bypass
            && game_restart::requires_new_confirmation(action)
            && self.state.pending_restart.is_none()
        {
            self.state.pending_restart = Some(action);
            self.state.confirm_restart = true;
            return;
        }
        if !card_hints::is_hint(action) {
            self.state.card_hint = None;
        }
        if !crate::game_actions::is_shell(action) && self.apply_game_action(&action) {
            self.finish_action(previous_screen, action);
            return;
        }
        if self.apply_browse_action(action) {
            self.finish_action(previous_screen, action);
            return;
        }
        match self.apply_shell_action(action) {
            ShellActionResult::Continue => self.finish_action(previous_screen, action),
            ShellActionResult::Stop => {}
            ShellActionResult::Unhandled => unreachable!("shell action was not handled"),
        }
    }

    fn apply_shell_action(&mut self, action: ui::UiAction) -> ShellActionResult {
        match self.apply_shell_group_1(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_2(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_3(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_4(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_5(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_6(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_7(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_8(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_9(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_10(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        ShellActionResult::Unhandled
    }

    fn apply_shell_group_1(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::Open(index) => self.open_game(index),
            ui::UiAction::ContinueGame => {
                self.open_game(crate::continue_data::preferred_game(&self.state).index());
            }
            ui::UiAction::ToggleFavorite(index) => self.toggle_favorite(index),
            ui::UiAction::ClearRecent => {
                self.state.recent_games.clear();
                self.state.library_scroll = 0;
                self.notifications.info("Recent shelf cleared");
            }
            ui::UiAction::CabinetFilter(filter) => {
                self.state.cabinet_filter = filter.min(9);
                self.state.cabinet_scroll = 0;
            }
            ui::UiAction::FinderFilter(filter) => {
                self.state.cabinet_filter = crate::finder_data::normalize_filter(filter);
                self.state.library_scroll = 0;
            }
            ui::UiAction::CabinetSort => {
                let sort =
                    crate::cabinet_status::CabinetSort::from_index(self.state.cabinet_sort).next();
                self.state.cabinet_sort = sort.index();
                self.state.cabinet_scroll = 0;
                self.notifications
                    .info(format!("Cabinet order: {}", sort.label()));
            }
            ui::UiAction::CabinetScroll(delta) => {
                let page_size = crate::cabinet_data::page_size_for_layout(
                    crate::ui::is_portrait(),
                    crate::ui::is_compact_landscape(),
                );
                let limit = crate::cabinet_data::scroll_limit(&self.state, page_size);
                self.state.cabinet_scroll = self
                    .state
                    .cabinet_scroll
                    .saturating_add_signed(delta as isize)
                    .min(limit);
            }
            ui::UiAction::LibraryScroll(delta) => {
                self.state.library_scroll = self
                    .state
                    .library_scroll
                    .saturating_add_signed(delta as isize)
                    .min(self.library_scroll_limit());
            }
            ui::UiAction::DailyArchiveScroll(delta) => {
                let page_size = crate::daily_archive_ui::page_size();
                self.state.daily_archive_scroll = self
                    .state
                    .daily_archive_scroll
                    .saturating_add_signed(delta as isize)
                    .min(
                        self.state
                            .records
                            .daily_results
                            .len()
                            .saturating_sub(page_size),
                    );
            }
            ui::UiAction::DailyArchiveOpen(day) => self.open_archived_daily_day(day),
            ui::UiAction::RecordsFilter(filter) => {
                self.state.records_filter = if crate::records_data::FILTERS.contains(&filter) {
                    filter
                } else {
                    0
                };
                self.state.library_scroll = 0;
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }

    fn apply_shell_group_2(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::RulesFilter(filter) => {
                self.state.rules_filter = crate::rules_data::normalize_filter(filter);
                self.state.library_scroll = 0;
            }
            ui::UiAction::Cabinet => {
                self.state.screen = Screen::Cabinet;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
                self.state.tutorial = None;
                self.state.confirm_reset = false;
                self.state.confirm_restart = false;
                self.state.pending_restart = None;
            }
            ui::UiAction::Help => {
                self.state.screen = Screen::Help;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::Records => {
                self.state.screen = Screen::Records;
                self.state.library_scroll = 0;
                self.state.records_filter = 0;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::DailyArchive => {
                self.state.screen = Screen::Records;
                self.state.library_scroll = 0;
                self.state.daily_archive_scroll = 0;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = true;
                self.state.achievements_view = false;
            }
            ui::UiAction::Achievements => {
                self.state.screen = Screen::Records;
                self.state.library_scroll = 0;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = true;
                self.state.achievement_filter = 0;
            }
            ui::UiAction::AchievementFilter(filter) => {
                self.set_achievement_filter(filter);
            }
            ui::UiAction::Rules => {
                self.state.screen = Screen::Rules;
                self.state.library_scroll = 0;
                self.state.rules_filter = 0;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::Credits => {
                self.state.screen = Screen::Credits;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::Settings => {
                self.state.screen = Screen::Settings;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::TutorialContinue => {
                if let Some(game) = self.state.tutorial {
                    self.state.tutorial_seen[game.index()] = true;
                    self.state.tutorial = None;
                }
            }
            ui::UiAction::ReplayTutorial => {
                if let Screen::Game(game) = self.state.screen {
                    self.state.tutorial = Some(game);
                }
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }

    fn apply_shell_group_3(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::Save => self.flush_autosave(),
            ui::UiAction::Load => {
                self.load_autosave();
                self.analytics.sync_progress(&self.state);
            }
            ui::UiAction::Game2048Hint => {
                self.state.card_hint = Some(card_hints::game_2048(&self.state));
            }
            ui::UiAction::Game2048Size(board_size) => {
                self.state.games.game = crate::state::Game2048::new_with_size(
                    self.state.games.game.seed.wrapping_add(1),
                    board_size,
                );
            }
            ui::UiAction::Move(direction) => self.try_move(direction),
            ui::UiAction::MineReveal(index) => {
                self.state.games.minesweeper.reveal(index);
            }
            ui::UiAction::MineFlag(index) => {
                self.state.games.minesweeper.toggle_flag(index);
            }
            ui::UiAction::MineFlagMode => {
                self.state.mine_flag_mode = !self.state.mine_flag_mode;
            }
            ui::UiAction::MineHint => {
                self.state.card_hint = Some(card_hints::minesweeper(&self.state));
            }
            ui::UiAction::MinePreset(preset) => {
                let seed = self.state.games.minesweeper.seed.wrapping_add(1);
                self.state.games.minesweeper = if preset == crate::minesweeper::MinePreset::Custom {
                    crate::minesweeper::Minesweeper::custom(12, 12, 20, seed)
                } else {
                    crate::minesweeper::Minesweeper::new(preset, seed)
                };
                self.state.mine_flag_mode = false;
            }
            ui::UiAction::SudokuCell(index) => {
                self.state.games.sudoku.select(index);
            }
            ui::UiAction::SudokuNumber(value) => {
                if let Some(index) = self.state.games.sudoku.selected {
                    if self.state.sudoku_note_mode {
                        self.state.games.sudoku.toggle_note(index, value);
                    } else {
                        self.state.games.sudoku.place(index, value);
                    }
                }
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }

    fn apply_shell_group_4(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::SudokuErase => {
                if let Some(index) = self.state.games.sudoku.selected {
                    self.state.games.sudoku.erase(index);
                }
            }
            ui::UiAction::SudokuNoteMode => {
                self.state.sudoku_note_mode = !self.state.sudoku_note_mode;
            }
            ui::UiAction::SudokuDifficulty(difficulty) => {
                self.state.games.sudoku = crate::sudoku::Sudoku::with_difficulty(difficulty);
                self.state.sudoku_note_mode = false;
            }
            ui::UiAction::SudokuUndo => {
                self.state.games.sudoku.undo();
            }
            ui::UiAction::SudokuHint => {
                self.state.card_hint = Some(card_hints::sudoku(&self.state));
            }
            ui::UiAction::NonogramCell(index) => {
                self.state.games.nonogram.select(index);
                self.state.games.nonogram.toggle(index);
            }
            ui::UiAction::NonogramMode => {
                self.state.games.nonogram.toggle_mode();
            }
            ui::UiAction::NonogramHint => {
                self.state.card_hint = Some(card_hints::nonogram(&self.state));
            }
            ui::UiAction::NonogramUndo => {
                self.state.games.nonogram.undo();
            }
            ui::UiAction::NonogramPreset(preset) => {
                let variant = self.state.games.nonogram.variant.wrapping_add(1);
                self.state.games.nonogram =
                    crate::nonogram::Nonogram::new_with_variant(preset, variant);
                self.state.games.nonogram_zoomed = false;
                self.state.games.nonogram_focus = (0, 0);
            }
            ui::UiAction::NonogramZoom => {
                self.state.games.nonogram_zoomed = !self.state.games.nonogram_zoomed;
                self.state.games.nonogram_focus = crate::nonogram::focus_origin(
                    self.state.games.nonogram.size,
                    self.state.games.nonogram_zoomed,
                    self.state.games.nonogram_focus,
                );
            }
            ui::UiAction::NonogramPan(dx, dy) => {
                let (x, y) = self.state.games.nonogram_focus;
                let next = (
                    x.saturating_add_signed(dx as isize),
                    y.saturating_add_signed(dy as isize),
                );
                self.state.games.nonogram_focus = crate::nonogram::focus_origin(
                    self.state.games.nonogram.size,
                    self.state.games.nonogram_zoomed,
                    next,
                );
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }

    fn apply_shell_group_5(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::SolitaireStock => {
                self.state.games.solitaire.draw_stock();
            }
            ui::UiAction::SolitaireTableau(column, depth) => {
                let had_selection = self.state.games.solitaire.selected.is_some();
                if !self.state.games.solitaire.tap_tableau(column, depth) && had_selection {
                    self.notifications
                        .warning("That tableau does not accept this card");
                }
            }
            ui::UiAction::SolitaireWaste => {
                self.state.games.solitaire.tap_waste();
            }
            ui::UiAction::SolitaireFoundation(suit) => {
                if !self.state.games.solitaire.move_to_foundation(suit) {
                    self.notifications
                        .warning("That card cannot go to this foundation yet");
                }
            }
            ui::UiAction::SolitaireUndo => {
                self.state.games.solitaire.undo();
            }
            ui::UiAction::SolitaireHint => {
                self.state.card_hint = Some(card_hints::solitaire(&self.state));
            }
            ui::UiAction::SolitaireNew => {
                self.state.games.solitaire = crate::solitaire::Solitaire::new(
                    self.state.games.solitaire.seed.wrapping_add(1),
                );
            }
            ui::UiAction::FreeCellCell(cell) => {
                self.state.games.freecell.tap_cell(cell);
            }
            ui::UiAction::FreeCellCascade(cascade, depth) => {
                let had_selection = self.state.games.freecell.selected.is_some();
                if !self.state.games.freecell.tap_cascade(cascade, depth) && had_selection {
                    self.notifications
                        .warning("That stack cannot move to this cascade");
                }
            }
            ui::UiAction::FreeCellFoundation(suit) => {
                if !self.state.games.freecell.move_selected_to_foundation(suit) {
                    self.notifications
                        .warning("That card cannot go to this foundation yet");
                }
            }
            ui::UiAction::FreeCellUndo => {
                self.state.games.freecell.undo();
            }
            ui::UiAction::FreeCellHint => {
                self.state.card_hint = Some(card_hints::freecell(&self.state));
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }

    fn apply_shell_group_6(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::FreeCellNew => {
                self.state.games.freecell =
                    crate::freecell::FreeCell::new(self.state.games.freecell.seed.wrapping_add(1));
            }
            ui::UiAction::FivefoldRoll => {
                self.state.games.fivefold.roll();
            }
            ui::UiAction::FivefoldHold(index) => {
                self.state.games.fivefold.toggle_hold(index);
            }
            ui::UiAction::FivefoldCategory(category) => {
                self.state.games.fivefold.choose_category(category);
            }
            ui::UiAction::FivefoldScorePage(delta) => {
                self.state.fivefold_score_page = self
                    .state
                    .fivefold_score_page
                    .saturating_add_signed(delta as isize)
                    .min(2);
            }
            ui::UiAction::FivefoldHint => {
                self.state.card_hint = Some(card_hints::fivefold(&self.state));
            }
            ui::UiAction::FivefoldNew => {
                self.state.games.fivefold =
                    crate::fivefold::Fivefold::new(self.state.games.fivefold.seed.wrapping_add(1));
                self.state.fivefold_score_page = 0;
            }
            ui::UiAction::ReversiPlace(index) => {
                if self.state.games.reversi.ai_level == crate::reversi::AiLevel::TwoPlayer {
                    self.state.games.reversi.place_current(index);
                } else if self.state.games.reversi.place(index) {
                    self.state.games.reversi.ai_move();
                }
            }
            ui::UiAction::ReversiPass => {
                if self.state.games.reversi.pass()
                    && self.state.games.reversi.ai_level != crate::reversi::AiLevel::TwoPlayer
                {
                    self.state.games.reversi.ai_move();
                }
            }
            ui::UiAction::ReversiHint => {
                self.state.card_hint = Some(card_hints::reversi(&self.state));
            }
            ui::UiAction::ReversiNew => {
                self.state.games.reversi = crate::reversi::Reversi::new(
                    self.state.games.reversi.seed.wrapping_add(1),
                    self.state.games.reversi.ai_level,
                );
            }
            ui::UiAction::ReversiLevel(level) => {
                self.state.games.reversi.ai_level = level;
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }

    fn apply_shell_group_7(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::LightsOutPress(index) => {
                self.state.games.lights_out.press(index);
            }
            ui::UiAction::LightsOutHint => {
                self.state.card_hint = Some(card_hints::lights_out(&self.state));
            }
            ui::UiAction::LightsOutUndo => {
                self.state.games.lights_out.undo();
            }
            ui::UiAction::LightsOutNew => {
                let seed = self.state.games.lights_out.seed.wrapping_add(1);
                self.state.games.lights_out.reset(seed);
            }
            ui::UiAction::TicTacToePress(index) => {
                self.state.games.tic_tac_toe.place(index);
            }
            ui::UiAction::TicTacToeHint => {
                self.state.card_hint = Some(card_hints::tic_tac_toe(&self.state));
            }
            ui::UiAction::TicTacToeUndo => {
                self.state.games.tic_tac_toe.undo();
            }
            ui::UiAction::TicTacToeNew => {
                let seed = self.state.games.tic_tac_toe.seed.wrapping_add(1);
                self.state.games.tic_tac_toe.reset(seed);
            }
            ui::UiAction::TicTacToeLevel(level) => {
                self.state.games.tic_tac_toe.set_ai_level(level);
            }
            ui::UiAction::MemoryPairsSelect(index) => {
                self.state.games.memory_pairs.select(index);
            }
            ui::UiAction::MemoryPairsHint => {
                self.state.card_hint = Some(card_hints::memory_pairs(&self.state));
            }
            ui::UiAction::MemoryPairsUndo => {
                self.state.games.memory_pairs.undo();
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }

    fn apply_shell_group_8(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::MemoryPairsNew => {
                let seed = self.state.games.memory_pairs.seed.wrapping_add(1);
                self.state.games.memory_pairs.reset(seed);
            }
            ui::UiAction::SlidingPuzzleMove(index) => {
                self.state.games.sliding_puzzle.move_tile(index);
            }
            ui::UiAction::SlidingPuzzleHint => {
                self.state.card_hint = Some(card_hints::sliding_puzzle(&self.state));
            }
            ui::UiAction::SlidingPuzzleUndo => {
                self.state.games.sliding_puzzle.undo();
            }
            ui::UiAction::SlidingPuzzleNew => {
                let seed = self.state.games.sliding_puzzle.seed.wrapping_add(1);
                self.state.games.sliding_puzzle.reset(seed);
            }
            ui::UiAction::MastermindPick(color) => {
                self.state.games.mastermind.pick(color);
            }
            ui::UiAction::MastermindHint => {
                self.state.card_hint = Some(card_hints::mastermind(&self.state));
            }
            ui::UiAction::MastermindSubmit => {
                self.state.games.mastermind.submit();
            }
            ui::UiAction::MastermindClear => {
                self.state.games.mastermind.clear();
            }
            ui::UiAction::MastermindUndo => {
                self.state.games.mastermind.undo();
            }
            ui::UiAction::MastermindNew => {
                let seed = self.state.games.mastermind.seed.wrapping_add(1);
                self.state.games.mastermind.reset(seed);
            }
            ui::UiAction::SpiderSelect(column, depth) => {
                self.state.games.spider.tap_column(column, depth);
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }

    fn apply_shell_group_9(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::SpiderDeal => {
                self.state.games.spider.deal_stock();
            }
            ui::UiAction::SpiderHint => {
                self.state.card_hint = Some(card_hints::spider(&self.state));
            }
            ui::UiAction::SpiderUndo => {
                self.state.games.spider.undo();
            }
            ui::UiAction::SpiderNew => {
                let seed = self.state.games.spider.seed.wrapping_add(1);
                self.state.games.spider.reset(seed);
            }
            ui::UiAction::WordSearchCell(index) => {
                self.state.games.word_search.select(index);
            }
            ui::UiAction::WordSearchClear => {
                self.state.games.word_search.clear();
            }
            ui::UiAction::WordSearchHint => {
                self.state.card_hint = Some(card_hints::word_search(&self.state));
            }
            ui::UiAction::WordSearchNew => {
                let seed = self.state.games.word_search.seed.wrapping_add(1);
                self.state.games.word_search.reset(seed);
            }
            ui::UiAction::MineChord(index) => {
                self.state.games.minesweeper.chord(index);
            }
            ui::UiAction::MineRestart => {
                self.state.games.minesweeper = crate::minesweeper::Minesweeper::beginner(
                    self.state.games.minesweeper.seed.wrapping_add(1),
                );
            }
            ui::UiAction::Undo => {
                if self.state.games.game.undo() {
                    self.notifications.info("One move undone");
                }
            }
            ui::UiAction::Restart => self.state.confirm_restart = true,
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }

    fn apply_shell_group_10(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::ConfirmRestart => {
                if let Some(restart) = self.state.pending_restart.take() {
                    self.state.confirm_restart = false;
                    self.confirmation_bypass = true;
                    self.apply(restart);
                    self.confirmation_bypass = false;
                    return ShellActionResult::Stop;
                }
                self.state.games.game =
                    crate::state::Game2048::new(self.state.games.game.seed.wrapping_add(1));
                self.state.confirm_restart = false;
            }
            ui::UiAction::Cancel => {
                self.state.confirm_restart = false;
                self.state.pending_restart = None;
            }
            ui::UiAction::ResumeLifecycle => self.resume_lifecycle(),
            ui::UiAction::DismissSaveRecovery => self.dismiss_save_recovery(),
            ui::UiAction::ToggleNoticeLog => self.toggle_notice_log(),
            ui::UiAction::ToggleSound
            | ui::UiAction::ToggleMotion
            | ui::UiAction::ToggleHighContrast
            | ui::UiAction::ToggleLargeText
            | ui::UiAction::CycleCardBack
            | ui::UiAction::CycleBoardTheme
            | ui::UiAction::CycleSoundSet
            | ui::UiAction::CycleCabinetDecoration
            | ui::UiAction::SetProfileName(_) => self.apply_settings_action(action),
            ui::UiAction::ResetData => self.state.confirm_reset = true,
            ui::UiAction::ConfirmResetData => {
                self.state = AppState::new(&self.data);
            }
            ui::UiAction::CancelResetData => self.state.confirm_reset = false,
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }
}
