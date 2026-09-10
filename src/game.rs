//! Application lifecycle and input routing.

use crate::card_hints;
use crate::cosmetics;
use crate::domain::Direction;
use crate::game_input::{card_drag_actions, swipe_direction};
use crate::input::{Gesture, PointerTracker};
use crate::sound::SoundBank;
use crate::{
    data::GameData,
    state::{AppState, GameId, Screen},
};
use crate::{minesweeper_ui, nonogram_ui, responsive_landscape_games, responsive_puzzles, ui};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::notifications::NotificationManager;

#[path = "game_arcade_actions.rs"]
mod game_arcade_actions;
#[path = "game_board_actions.rs"]
mod game_board_actions;
#[path = "game_capture.rs"]
mod game_capture;
#[path = "game_capture_depth.rs"]
mod game_capture_depth;
#[path = "game_capture_records.rs"]
mod game_capture_records;
#[path = "game_capture_rules.rs"]
mod game_capture_rules;
#[path = "game_navigation.rs"]
mod game_navigation;
#[path = "game_persistence.rs"]
mod game_persistence;
#[path = "game_progression.rs"]
pub(crate) mod game_progression;
#[path = "game_realtime.rs"]
mod game_realtime;
#[path = "game_render.rs"]
mod game_render;
#[path = "game_restart.rs"]
mod game_restart;

pub struct Game {
    pub data: GameData,
    pub state: AppState,
    assets: AssetManager,
    notifications: NotificationManager,
    pointer: PointerTracker,
    pub(super) touch_was_active: bool,
    pub(super) capture_solitaire_peek: bool,
    sounds: SoundBank,
    transition: f32,
    confirmation_bypass: bool,
    pub(super) save_dirty: bool,
    pub(super) save_timer: f32,
    analytics: crate::analytics::GameAnalytics,
}
impl Game {
    pub async fn new() -> Self {
        macroquad_toolkit::rng::srand(get_time().to_bits());
        let data = GameData::load().expect("Idle Hands embedded data failed to load");
        let mut assets = AssetManager::new();
        let placeholder = Image::gen_image_color(16, 16, crate::theme::SURFACE_DARK);
        assets.set_placeholder_texture_direct(Texture2D::from_image(&placeholder));
        assets.load_texture_configs(&data.texture_manifest).await;
        let state = AppState::new(&data);
        let mut game = Self {
            analytics: crate::analytics::GameAnalytics::disabled(&state),
            state,
            data,
            assets,
            notifications: NotificationManager::new(),
            pointer: PointerTracker::default(),
            touch_was_active: false,
            capture_solitaire_peek: false,
            sounds: SoundBank::load().await,
            transition: 0.,
            confirmation_bypass: false,
            save_dirty: false,
            save_timer: 0.,
        };
        game.initialize_launch_state();
        game.load_autosave();
        game.refresh_daily_challenge();
        game.analytics = crate::analytics::GameAnalytics::new(
            &game.state,
            !macroquad_toolkit::capture::capture_requested("IDLE_HANDS"),
        );
        game
    }

    pub fn update(&mut self, dt: f32) {
        if is_mouse_button_pressed(MouseButton::Left)
            || is_mouse_button_released(MouseButton::Left)
            || !get_keys_pressed().is_empty()
        {
            self.analytics.note_input();
        }
        self.notifications.update(dt);
        self.pointer.tick(dt);
        self.update_card_peek();
        if self.state.reduced_motion {
            self.transition = 0.;
        } else {
            self.transition = (self.transition - dt * 3.5).max(0.);
        }
        self.state.games.minesweeper.tick(dt);
        if self.state.games.minesweeper.status == crate::minesweeper::MineStatus::Won {
            let slot = self.state.games.minesweeper.preset.index();
            let time = self.state.games.minesweeper.elapsed_whole_seconds();
            self.state.mine_records[slot] =
                Some(self.state.mine_records[slot].map_or(time, |best| best.min(time)));
            self.state.records.minesweeper[slot] = self.state.mine_records[slot];
        }
        self.tick_realtime(dt);
        self.tick_elapsed(dt);
        self.tick_autosave(dt);
        if is_mouse_button_pressed(MouseButton::Left) {
            let viewport = ui::viewport();
            self.pointer
                .press(viewport.screen_to_ui_checked(vec2(mouse_position().0, mouse_position().1)));
        }
        if is_mouse_button_released(MouseButton::Left) {
            let viewport = ui::viewport();
            let position =
                viewport.screen_to_ui_checked(vec2(mouse_position().0, mouse_position().1));
            if let Some(gesture) = self.pointer.release(position) {
                match gesture {
                    Gesture::Drag { start, end }
                        if self.state.screen == Screen::Game(GameId::Game2048)
                            && (end - start).length() > 32.0 =>
                    {
                        self.try_move(swipe_direction(end - start))
                    }
                    Gesture::Drag { start, end }
                        if self.state.screen == Screen::Game(GameId::Nonogram) =>
                    {
                        let actions = if ui::is_compact_landscape() {
                            responsive_landscape_games::nonogram_drag_actions(
                                &self.state,
                                start,
                                end,
                            )
                        } else if ui::is_portrait() {
                            responsive_puzzles::nonogram_drag_actions(&self.state, start, end)
                        } else {
                            nonogram_ui::drag_actions(&self.state, start, end)
                        };
                        for action in actions {
                            self.apply(action);
                        }
                    }
                    Gesture::Drag { start, end }
                        if matches!(
                            self.state.screen,
                            Screen::Game(GameId::Solitaire | GameId::FreeCell)
                        ) =>
                    {
                        for action in card_drag_actions(
                            &self.state,
                            start,
                            end,
                            ui::is_portrait(),
                            ui::is_compact_landscape(),
                        ) {
                            self.apply(action);
                        }
                    }
                    Gesture::Tap(position) => {
                        for action in ui::clicks_at(&self.state, position) {
                            self.apply(action);
                        }
                    }
                    Gesture::LongPress(position)
                        if self.state.screen == Screen::Game(GameId::Minesweeper) =>
                    {
                        let actions = if ui::is_compact_landscape() {
                            responsive_landscape_games::minesweeper_long_press(
                                &self.state,
                                position,
                            )
                        } else if ui::is_portrait() {
                            responsive_puzzles::minesweeper_long_press(&self.state, position)
                        } else {
                            minesweeper_ui::long_press(&self.state, position)
                        };
                        for action in actions {
                            self.apply(action);
                        }
                    }
                    Gesture::Drag { start, end } => self.apply_navigation_drag(start, end),
                    Gesture::LongPress(_) => {}
                }
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            self.pointer.cancel();
            self.state.screen = Screen::Cabinet;
            self.state.favorites_view = false;
            self.state.recent_view = false;
            self.state.daily_archive_view = false;
            self.state.achievements_view = false;
            self.state.confirm_restart = false;
            self.state.pending_restart = None;
            self.state.confirm_reset = false;
            self.state.tutorial = None;
            if !self.state.reduced_motion {
                self.transition = 1.;
            }
        }
        if self.state.screen.game() == Some(GameId::Game2048) {
            for (key, direction) in [
                (KeyCode::Up, Direction::Up),
                (KeyCode::Right, Direction::Right),
                (KeyCode::Down, Direction::Down),
                (KeyCode::Left, Direction::Left),
            ] {
                if is_key_pressed(key) {
                    self.try_move(direction);
                }
            }
        }
        self.update_navigation_scroll();
        self.analytics.update(dt, &self.state);
    }

    pub fn end_analytics_session(&mut self) {
        self.analytics.end_session();
    }
    fn apply(&mut self, action: ui::UiAction) {
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
        match action {
            ui::UiAction::Open(index) => self.open_game(index),
            ui::UiAction::ContinueGame => {
                let index = self.state.selected;
                self.open_game(index);
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
            ui::UiAction::CabinetSort => {
                let sort =
                    crate::cabinet_status::CabinetSort::from_index(self.state.cabinet_sort).next();
                self.state.cabinet_sort = sort.index();
                self.state.cabinet_scroll = 0;
                self.notifications
                    .info(format!("Cabinet order: {}", sort.label()));
            }
            ui::UiAction::CabinetScroll(delta) => {
                self.state.cabinet_scroll = self
                    .state
                    .cabinet_scroll
                    .saturating_add_signed(delta as isize)
                    .min(GameId::ALL.len().saturating_sub(1));
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
            ui::UiAction::RecordsFilter(filter) => {
                self.state.records_filter = if crate::records_data::FILTERS.contains(&filter) {
                    filter
                } else {
                    0
                };
                self.state.library_scroll = 0;
            }
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
            ui::UiAction::Favorites => {
                self.state.screen = Screen::Records;
                self.state.library_scroll = 0;
                self.state.favorites_view = true;
                self.state.recent_view = false;
                self.state.daily_archive_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::Recent => {
                self.state.screen = Screen::Records;
                self.state.library_scroll = 0;
                self.state.favorites_view = false;
                self.state.recent_view = true;
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
                self.state.achievement_filter = filter.min(2);
                self.state.library_scroll = 0;
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
            ui::UiAction::ConfirmRestart => {
                if let Some(restart) = self.state.pending_restart.take() {
                    self.state.confirm_restart = false;
                    self.confirmation_bypass = true;
                    self.apply(restart);
                    self.confirmation_bypass = false;
                    return;
                }
                self.state.games.game =
                    crate::state::Game2048::new(self.state.games.game.seed.wrapping_add(1));
                self.state.confirm_restart = false;
            }
            ui::UiAction::Cancel => {
                self.state.confirm_restart = false;
                self.state.pending_restart = None;
            }
            ui::UiAction::ToggleSound => self.state.sound = !self.state.sound,
            ui::UiAction::ToggleMotion => self.state.reduced_motion = !self.state.reduced_motion,
            ui::UiAction::ToggleHighContrast => {
                self.state.high_contrast = !self.state.high_contrast
            }
            ui::UiAction::ToggleLargeText => self.state.large_text = !self.state.large_text,
            ui::UiAction::CycleCardBack => {
                self.state.card_back =
                    cosmetics::next_card_back(self.state.card_back, self.state.stamps)
            }
            ui::UiAction::CycleBoardTheme => {
                self.state.board_theme =
                    cosmetics::next_board_theme(self.state.board_theme, self.state.stamps)
            }
            ui::UiAction::CycleSoundSet => {
                self.state.sound_set =
                    cosmetics::next_sound_set(self.state.sound_set, self.state.stamps)
            }
            ui::UiAction::CycleCabinetDecoration => {
                self.state.cabinet_decoration = cosmetics::next_cabinet_decoration(
                    self.state.cabinet_decoration,
                    self.state.stamps,
                )
            }
            ui::UiAction::ResetData => self.state.confirm_reset = true,
            ui::UiAction::ConfirmResetData => {
                self.state = AppState::new(&self.data);
            }
            ui::UiAction::CancelResetData => self.state.confirm_reset = false,
            _ => unreachable!("board action was already handled"),
        }
        self.finish_action(previous_screen, action);
    }
}

#[cfg(test)]
mod tests;
