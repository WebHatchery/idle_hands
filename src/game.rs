//! Application lifecycle and input routing.

use crate::card_hints;
use crate::cosmetics;
use crate::game_input::{card_drag_actions, swipe_direction};
use crate::input::{Gesture, PointerTracker};
use crate::sound::SoundBank;
use crate::{
    data::GameData,
    state::{AppState, CollectionSave, Direction, GameId, GameSnapshot, ProfileSave, Screen},
};
use crate::{minesweeper_ui, nonogram_ui, responsive_landscape_games, responsive_puzzles, ui};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::notifications::{
    NotificationAnchor, NotificationManager, NotificationRenderConfig,
};
use macroquad_toolkit::persistence::{
    load_from_slot_with_migration, save_to_slot_with_version, slot_exists,
};
use serde::de::DeserializeOwned;

#[path = "game_board_actions.rs"]
mod game_board_actions;
#[path = "game_capture.rs"]
mod game_capture;
#[path = "game_navigation.rs"]
mod game_navigation;
#[path = "game_progression.rs"]
mod game_progression;
#[path = "game_restart.rs"]
mod game_restart;

pub struct Game {
    pub data: GameData,
    pub state: AppState,
    assets: AssetManager,
    notifications: NotificationManager,
    pointer: PointerTracker,
    sounds: SoundBank,
    transition: f32,
    confirmation_bypass: bool,
}
impl Game {
    pub async fn new() -> Self {
        let data = GameData::load().expect("Idle Hands embedded data failed to load");
        let mut assets = AssetManager::new();
        let placeholder = Image::gen_image_color(16, 16, crate::theme::SURFACE_DARK);
        assets.set_placeholder_texture_direct(Texture2D::from_image(&placeholder));
        assets.load_texture_configs(&data.texture_manifest).await;
        let mut game = Self {
            state: AppState::new(&data),
            data,
            assets,
            notifications: NotificationManager::new(),
            pointer: PointerTracker::default(),
            sounds: SoundBank::load().await,
            transition: 0.,
            confirmation_bypass: false,
        };
        game.state.color_sort.reset(get_time().to_bits());
        game.load_autosave();
        game
    }

    pub fn update(&mut self, dt: f32) {
        self.notifications.update(dt);
        self.pointer.tick(dt);
        if self.state.reduced_motion {
            self.transition = 0.;
        } else {
            self.transition = (self.transition - dt * 3.5).max(0.);
        }
        self.state.minesweeper.tick(dt);
        if self.state.minesweeper.status == crate::minesweeper::MineStatus::Won {
            let slot = self.state.minesweeper.preset.index();
            let time = self.state.minesweeper.elapsed_whole_seconds();
            self.state.mine_records[slot] =
                Some(self.state.mine_records[slot].map_or(time, |best| best.min(time)));
            self.state.records.minesweeper[slot] = self.state.mine_records[slot];
        }
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
                    Gesture::Tap(_) => {
                        for action in ui::clicks(&self.state) {
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
            self.state.achievements_view = false;
            self.state.confirm_restart = false;
            self.state.pending_restart = None;
            self.state.confirm_reset = false;
            self.state.tutorial = None;
            if !self.state.reduced_motion {
                self.transition = 1.;
            }
        }
        if self.state.screen == Screen::Game(GameId::Game2048) {
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
        let _ = dt;
    }
    pub fn draw(&mut self) {
        clear_background(cosmetics::background(self.state.board_theme));
        let viewport = ui::viewport();
        let (layout_width, layout_height) = ui::layout_size();
        viewport.begin();
        ui::draw(&self.state, &self.data, self.assets.len());
        if self.transition > 0. {
            draw_rectangle(
                0.,
                0.,
                layout_width,
                layout_height,
                Color::new(0.02, 0.015, 0.035, self.transition),
            );
        }
        set_default_camera();
        self.notifications
            .draw_with_config(&NotificationRenderConfig {
                anchor: NotificationAnchor::BottomRight,
                ..Default::default()
            });
    }
    fn apply(&mut self, action: ui::UiAction) {
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
        if self.apply_board_action(&action) {
            self.finish_action(previous_screen, action);
            return;
        }
        match action {
            ui::UiAction::Open(index) => self.open_game(index),
            ui::UiAction::ContinueGame => {
                let index = self.state.selected;
                self.open_game(index);
            }
            ui::UiAction::ToggleFavorite(index) => {
                if let Some(favorite) = self.state.favorites.get_mut(index) {
                    *favorite = !*favorite;
                }
            }
            ui::UiAction::CabinetFilter(filter) => {
                self.state.cabinet_filter = filter.min(9);
                self.state.cabinet_scroll = 0;
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
                    .min(GameId::ALL.len().saturating_sub(1));
            }
            ui::UiAction::Cabinet => {
                self.state.screen = Screen::Cabinet;
                self.state.favorites_view = false;
                self.state.recent_view = false;
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
                self.state.achievements_view = false;
            }
            ui::UiAction::Records => {
                self.state.screen = Screen::Records;
                self.state.library_scroll = 0;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::Favorites => {
                self.state.screen = Screen::Records;
                self.state.library_scroll = 0;
                self.state.favorites_view = true;
                self.state.recent_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::Recent => {
                self.state.screen = Screen::Records;
                self.state.library_scroll = 0;
                self.state.favorites_view = false;
                self.state.recent_view = true;
                self.state.achievements_view = false;
            }
            ui::UiAction::Achievements => {
                self.state.screen = Screen::Records;
                self.state.library_scroll = 0;
                self.state.favorites_view = false;
                self.state.recent_view = false;
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
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::Credits => {
                self.state.screen = Screen::Credits;
                self.state.favorites_view = false;
                self.state.recent_view = false;
                self.state.achievements_view = false;
            }
            ui::UiAction::Settings => {
                self.state.screen = Screen::Settings;
                self.state.favorites_view = false;
                self.state.recent_view = false;
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
            ui::UiAction::Save => self.save_autosave(),
            ui::UiAction::Load => self.load_autosave(),
            ui::UiAction::Game2048Hint => {
                self.state.card_hint = Some(card_hints::game_2048(&self.state));
            }
            ui::UiAction::Move(direction) => self.try_move(direction),
            ui::UiAction::MineReveal(index) => {
                self.state.minesweeper.reveal(index);
            }
            ui::UiAction::MineFlag(index) => {
                self.state.minesweeper.toggle_flag(index);
            }
            ui::UiAction::MineFlagMode => {
                self.state.mine_flag_mode = !self.state.mine_flag_mode;
            }
            ui::UiAction::MineHint => {
                self.state.card_hint = Some(card_hints::minesweeper(&self.state));
            }
            ui::UiAction::MinePreset(preset) => {
                let seed = self.state.minesweeper.seed.wrapping_add(1);
                self.state.minesweeper = if preset == crate::minesweeper::MinePreset::Custom {
                    crate::minesweeper::Minesweeper::custom(12, 12, 20, seed)
                } else {
                    crate::minesweeper::Minesweeper::new(preset, seed)
                };
                self.state.mine_flag_mode = false;
            }
            ui::UiAction::SudokuCell(index) => {
                self.state.sudoku.select(index);
            }
            ui::UiAction::SudokuNumber(value) => {
                if let Some(index) = self.state.sudoku.selected {
                    if self.state.sudoku_note_mode {
                        self.state.sudoku.toggle_note(index, value);
                    } else {
                        self.state.sudoku.place(index, value);
                    }
                }
            }
            ui::UiAction::SudokuErase => {
                if let Some(index) = self.state.sudoku.selected {
                    self.state.sudoku.erase(index);
                }
            }
            ui::UiAction::SudokuNoteMode => {
                self.state.sudoku_note_mode = !self.state.sudoku_note_mode;
            }
            ui::UiAction::SudokuDifficulty(difficulty) => {
                self.state.sudoku = crate::sudoku::Sudoku::with_difficulty(difficulty);
                self.state.sudoku_note_mode = false;
            }
            ui::UiAction::SudokuUndo => {
                self.state.sudoku.undo();
            }
            ui::UiAction::SudokuHint => {
                self.state.card_hint = Some(card_hints::sudoku(&self.state));
            }
            ui::UiAction::NonogramCell(index) => {
                self.state.nonogram.select(index);
                self.state.nonogram.toggle(index);
            }
            ui::UiAction::NonogramMode => {
                self.state.nonogram.toggle_mode();
            }
            ui::UiAction::NonogramHint => {
                self.state.card_hint = Some(card_hints::nonogram(&self.state));
            }
            ui::UiAction::NonogramUndo => {
                self.state.nonogram.undo();
            }
            ui::UiAction::NonogramPreset(preset) => {
                self.state.nonogram = crate::nonogram::Nonogram::new(preset);
                self.state.nonogram_zoomed = false;
                self.state.nonogram_focus = (0, 0);
            }
            ui::UiAction::NonogramZoom => {
                self.state.nonogram_zoomed = !self.state.nonogram_zoomed;
                self.state.nonogram_focus = crate::nonogram::focus_origin(
                    self.state.nonogram.size,
                    self.state.nonogram_zoomed,
                    self.state.nonogram_focus,
                );
            }
            ui::UiAction::NonogramPan(dx, dy) => {
                let (x, y) = self.state.nonogram_focus;
                let next = (
                    x.saturating_add_signed(dx as isize),
                    y.saturating_add_signed(dy as isize),
                );
                self.state.nonogram_focus = crate::nonogram::focus_origin(
                    self.state.nonogram.size,
                    self.state.nonogram_zoomed,
                    next,
                );
            }
            ui::UiAction::SolitaireStock => {
                self.state.solitaire.draw_stock();
            }
            ui::UiAction::SolitaireTableau(column, depth) => {
                if self.state.solitaire.selected.is_some() {
                    if !self.state.solitaire.move_to_tableau(column) {
                        self.notifications
                            .warning("That tableau does not accept this card");
                    }
                } else {
                    self.state.solitaire.select_tableau(column, depth);
                }
            }
            ui::UiAction::SolitaireWaste => {
                self.state.solitaire.select_waste();
            }
            ui::UiAction::SolitaireFoundation(suit) => {
                if !self.state.solitaire.move_to_foundation(suit) {
                    self.notifications
                        .warning("That card cannot go to this foundation yet");
                }
            }
            ui::UiAction::SolitaireUndo => {
                self.state.solitaire.undo();
            }
            ui::UiAction::SolitaireHint => {
                self.state.card_hint = Some(card_hints::solitaire(&self.state));
            }
            ui::UiAction::SolitaireNew => {
                self.state.solitaire =
                    crate::solitaire::Solitaire::new(self.state.solitaire.seed.wrapping_add(1));
            }
            ui::UiAction::FreeCellCell(cell) => {
                if self.state.freecell.selected.is_some() {
                    if !self.state.freecell.move_selected_to_cascade(cell) {
                        self.notifications
                            .warning("That stack cannot move to this cascade");
                    }
                } else {
                    self.state.freecell.select_cell(cell);
                }
            }
            ui::UiAction::FreeCellCascade(cascade, depth) => {
                if self.state.freecell.selected.is_some() {
                    if !self.state.freecell.move_selected_to_cascade(cascade) {
                        self.notifications
                            .warning("That stack cannot move to this cascade");
                    }
                } else {
                    self.state.freecell.select_cascade(cascade, depth);
                }
            }
            ui::UiAction::FreeCellFoundation(suit) => {
                if !self.state.freecell.move_selected_to_foundation(suit) {
                    self.notifications
                        .warning("That card cannot go to this foundation yet");
                }
            }
            ui::UiAction::FreeCellUndo => {
                self.state.freecell.undo();
            }
            ui::UiAction::FreeCellHint => {
                self.state.card_hint = Some(card_hints::freecell(&self.state));
            }
            ui::UiAction::FreeCellNew => {
                self.state.freecell =
                    crate::freecell::FreeCell::new(self.state.freecell.seed.wrapping_add(1));
            }
            ui::UiAction::FivefoldRoll => {
                self.state.fivefold.roll();
            }
            ui::UiAction::FivefoldHold(index) => {
                self.state.fivefold.toggle_hold(index);
            }
            ui::UiAction::FivefoldCategory(category) => {
                self.state.fivefold.choose_category(category);
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
                self.state.fivefold =
                    crate::fivefold::Fivefold::new(self.state.fivefold.seed.wrapping_add(1));
                self.state.fivefold_score_page = 0;
            }
            ui::UiAction::ReversiPlace(index) => {
                if self.state.reversi.ai_level == crate::reversi::AiLevel::TwoPlayer {
                    self.state.reversi.place_current(index);
                } else if self.state.reversi.place(index) {
                    self.state.reversi.ai_move();
                }
            }
            ui::UiAction::ReversiPass => {
                if self.state.reversi.pass()
                    && self.state.reversi.ai_level != crate::reversi::AiLevel::TwoPlayer
                {
                    self.state.reversi.ai_move();
                }
            }
            ui::UiAction::ReversiHint => {
                self.state.card_hint = Some(card_hints::reversi(&self.state));
            }
            ui::UiAction::ReversiNew => {
                self.state.reversi = crate::reversi::Reversi::new(
                    self.state.reversi.seed.wrapping_add(1),
                    self.state.reversi.ai_level,
                );
            }
            ui::UiAction::ReversiLevel(level) => {
                self.state.reversi.ai_level = level;
            }
            ui::UiAction::LightsOutPress(index) => {
                self.state.lights_out.press(index);
            }
            ui::UiAction::LightsOutHint => {
                self.state.card_hint = Some(card_hints::lights_out(&self.state));
            }
            ui::UiAction::LightsOutUndo => {
                self.state.lights_out.undo();
            }
            ui::UiAction::LightsOutNew => {
                let seed = self.state.lights_out.seed.wrapping_add(1);
                self.state.lights_out.reset(seed);
            }
            ui::UiAction::TicTacToePress(index) => {
                self.state.tic_tac_toe.place(index);
            }
            ui::UiAction::TicTacToeHint => {
                self.state.card_hint = Some(card_hints::tic_tac_toe(&self.state));
            }
            ui::UiAction::TicTacToeUndo => {
                self.state.tic_tac_toe.undo();
            }
            ui::UiAction::TicTacToeNew => {
                let seed = self.state.tic_tac_toe.seed.wrapping_add(1);
                self.state.tic_tac_toe.reset(seed);
            }
            ui::UiAction::MemoryPairsSelect(index) => {
                self.state.memory_pairs.select(index);
            }
            ui::UiAction::MemoryPairsHint => {
                self.state.card_hint = Some(card_hints::memory_pairs(&self.state));
            }
            ui::UiAction::MemoryPairsUndo => {
                self.state.memory_pairs.undo();
            }
            ui::UiAction::MemoryPairsNew => {
                let seed = self.state.memory_pairs.seed.wrapping_add(1);
                self.state.memory_pairs.reset(seed);
            }
            ui::UiAction::SlidingPuzzleMove(index) => {
                self.state.sliding_puzzle.move_tile(index);
            }
            ui::UiAction::SlidingPuzzleHint => {
                self.state.card_hint = Some(card_hints::sliding_puzzle(&self.state));
            }
            ui::UiAction::SlidingPuzzleUndo => {
                self.state.sliding_puzzle.undo();
            }
            ui::UiAction::SlidingPuzzleNew => {
                let seed = self.state.sliding_puzzle.seed.wrapping_add(1);
                self.state.sliding_puzzle.reset(seed);
            }
            ui::UiAction::MastermindPick(color) => {
                self.state.mastermind.pick(color);
            }
            ui::UiAction::MastermindHint => {
                self.state.card_hint = Some(card_hints::mastermind(&self.state));
            }
            ui::UiAction::MastermindSubmit => {
                self.state.mastermind.submit();
            }
            ui::UiAction::MastermindClear => {
                self.state.mastermind.clear();
            }
            ui::UiAction::MastermindUndo => {
                self.state.mastermind.undo();
            }
            ui::UiAction::MastermindNew => {
                let seed = self.state.mastermind.seed.wrapping_add(1);
                self.state.mastermind.reset(seed);
            }
            ui::UiAction::SpiderSelect(column, depth) => {
                self.state.spider.select_column(column, depth);
            }
            ui::UiAction::SpiderMove(column) => {
                self.state.spider.move_selected(column);
            }
            ui::UiAction::SpiderDeal => {
                self.state.spider.deal_stock();
            }
            ui::UiAction::SpiderHint => {
                self.state.card_hint = Some(card_hints::spider(&self.state));
            }
            ui::UiAction::SpiderUndo => {
                self.state.spider.undo();
            }
            ui::UiAction::SpiderNew => {
                let seed = self.state.spider.seed.wrapping_add(1);
                self.state.spider.reset(seed);
            }
            ui::UiAction::WordSearchCell(index) => {
                self.state.word_search.select(index);
            }
            ui::UiAction::WordSearchClear => {
                self.state.word_search.clear();
            }
            ui::UiAction::WordSearchHint => {
                self.state.card_hint = Some(card_hints::word_search(&self.state));
            }
            ui::UiAction::WordSearchNew => {
                let seed = self.state.word_search.seed.wrapping_add(1);
                self.state.word_search.reset(seed);
            }
            ui::UiAction::MineChord(index) => {
                self.state.minesweeper.chord(index);
            }
            ui::UiAction::MineRestart => {
                self.state.minesweeper = crate::minesweeper::Minesweeper::beginner(
                    self.state.minesweeper.seed.wrapping_add(1),
                );
            }
            ui::UiAction::Undo => {
                if self.state.game.undo() {
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
                self.state.game = crate::state::Game2048::new(self.state.game.seed.wrapping_add(1));
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

    fn save_autosave(&mut self) {
        let save = CollectionSave::from_state(&self.state, &self.data.config.version);
        if let Err(error) = save_to_slot_with_version(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &save,
            &self.data.config.version,
        ) {
            self.notifications
                .warning(format!("Autosave failed: {}", error));
        }
        let profile_slot = format!("{}_profile", self.data.config.save_slot);
        if let Err(error) = save_to_slot_with_version(
            &self.data.config.game_name,
            &profile_slot,
            &ProfileSave::from_state(&self.state, &self.data.config.version),
            &self.data.config.version,
        ) {
            self.notifications
                .warning(format!("Profile save failed: {}", error));
        }
        if let Screen::Game(game) = self.state.screen {
            let game_slot = format!("{}_{}", self.data.config.save_slot, game.save_key());
            if let Err(error) = save_to_slot_with_version(
                &self.data.config.game_name,
                &game_slot,
                &GameSnapshot::from_state(&self.state, game),
                &self.data.config.version,
            ) {
                self.notifications
                    .warning(format!("{} save failed: {}", game.title(), error));
            }
        }
    }

    fn load_autosave(&mut self) {
        let mut restored = false;
        let collection_slot = self.data.config.save_slot.clone();
        if slot_exists(&self.data.config.game_name, &collection_slot) {
            match self.load_slot::<CollectionSave>(&collection_slot) {
                Ok(save) => {
                    save.apply_to(&mut self.state);
                    restored = true;
                }
                Err(error) => self
                    .notifications
                    .warning(format!("Autosave could not be loaded: {}", error)),
            }
        }
        let profile_slot = format!("{}_profile", collection_slot);
        if slot_exists(&self.data.config.game_name, &profile_slot) {
            if let Ok(profile) = self.load_slot::<ProfileSave>(&profile_slot) {
                profile.apply_to(&mut self.state);
                restored = true;
            }
        }
        for game in GameId::ALL {
            let game_slot = format!("{}_{}", collection_slot, game.save_key());
            if slot_exists(&self.data.config.game_name, &game_slot) {
                if let Ok(snapshot) = self.load_slot::<GameSnapshot>(&game_slot) {
                    snapshot.apply_to(&mut self.state);
                    restored = true;
                }
            }
        }
        if restored {
            self.notifications.info("Restored the cabinet autosave");
        }
    }

    fn load_slot<T: DeserializeOwned>(&self, slot: &str) -> Result<T, String> {
        load_from_slot_with_migration(
            &self.data.config.game_name,
            slot,
            &self.data.config.version,
            |_, value| {
                let payload = value.get("data").cloned().unwrap_or(value);
                serde_json::from_value(payload)
                    .map_err(|error| format!("Unsupported save: {}", error))
            },
        )
    }
}

#[cfg(test)]
mod tests;
