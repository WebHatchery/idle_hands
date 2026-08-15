//! Application lifecycle and input routing.

use crate::card_hints;
use crate::cosmetics;
use crate::input::{Gesture, PointerTracker};
use crate::sound::{SoundBank, SoundCue};
use crate::{
    data::GameData,
    state::{AppState, CollectionSave, Direction, GameId, GameSnapshot, ProfileSave, Screen},
};
use crate::{
    freecell_ui, minesweeper_ui, nonogram_ui, responsive_cards, responsive_landscape_cards,
    responsive_landscape_games, responsive_puzzles, solitaire_ui, ui,
};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::notifications::{
    NotificationAnchor, NotificationManager, NotificationRenderConfig,
};
use macroquad_toolkit::persistence::{
    load_from_slot_with_migration, save_to_slot_with_version, slot_exists,
};
use serde::de::DeserializeOwned;

#[path = "game_progression.rs"]
mod game_progression;

pub struct Game {
    pub data: GameData,
    pub state: AppState,
    assets: AssetManager,
    notifications: NotificationManager,
    pointer: PointerTracker,
    sounds: SoundBank,
    transition: f32,
}
impl Game {
    pub async fn new() -> Self {
        let data = GameData::load().expect("Idle Hands embedded data failed to load");
        let mut assets = AssetManager::new();
        let placeholder = Image::gen_image_color(16, 16, Color::new(0.25, 0.18, 0.35, 1.0));
        assets.set_placeholder_texture_direct(Texture2D::from_image(&placeholder));
        assets.load_texture_configs(&data.texture_manifest).await;
        let mut game = Self {
            data,
            state: AppState::default(),
            assets,
            notifications: NotificationManager::new(),
            pointer: PointerTracker::default(),
            sounds: SoundBank::load().await,
            transition: 0.,
        };
        game.load_autosave();
        game
    }

    pub fn begin_capture_scene(&mut self, scene: &str) {
        self.state = AppState::default();
        self.notifications = NotificationManager::new();
        let scene = scene
            .strip_prefix("portrait_")
            .or_else(|| scene.strip_prefix("landscape_"))
            .unwrap_or(scene);
        self.state.screen = match scene {
            "2048" | "gameplay" => Screen::Game(GameId::Game2048),
            "tutorial_2048" => Screen::Game(GameId::Game2048),
            "minesweeper" => Screen::Game(GameId::Minesweeper),
            "sudoku" => Screen::Game(GameId::Sudoku),
            "sudoku_accessible" => Screen::Game(GameId::Sudoku),
            "nonogram" => Screen::Game(GameId::Nonogram),
            "nonogram_large" => Screen::Game(GameId::Nonogram),
            "nonogram_accessible" => Screen::Game(GameId::Nonogram),
            "minesweeper_accessible" => Screen::Game(GameId::Minesweeper),
            "solitaire" => Screen::Game(GameId::Solitaire),
            "solitaire_hint" => Screen::Game(GameId::Solitaire),
            "solitaire_selected" => Screen::Game(GameId::Solitaire),
            "freecell" => Screen::Game(GameId::FreeCell),
            "freecell_hint" => Screen::Game(GameId::FreeCell),
            "freecell_selected" => Screen::Game(GameId::FreeCell),
            "fivefold" => Screen::Game(GameId::Yahtzee),
            "reversi" => Screen::Game(GameId::Reversi),
            "lights_out" => Screen::Game(GameId::LightsOut),
            "tic_tac_toe" => Screen::Game(GameId::TicTacToe),
            "memory_pairs" => Screen::Game(GameId::MemoryPairs),
            "help" => Screen::Help,
            "records" => Screen::Records,
            "rules" => Screen::Rules,
            "credits" => Screen::Credits,
            "settings" => Screen::Settings,
            "settings_reset" => Screen::Settings,
            _ => Screen::Cabinet,
        };
        if scene == "settings_reset" {
            self.state.confirm_reset = true;
        }
        if scene == "nonogram_large" {
            self.state.nonogram =
                crate::nonogram::Nonogram::new(crate::nonogram::NonogramPreset::Large);
            self.state.nonogram_zoomed = true;
            self.state.nonogram_focus = (6, 6);
        }
        if scene == "solitaire_selected" {
            self.state.solitaire.select_tableau(0, 0);
        }
        if scene == "freecell_selected" {
            self.state.freecell.select_cascade(0, 0);
        }
        if scene.ends_with("_accessible") {
            self.state.high_contrast = true;
            self.state.large_text = true;
        }
        if scene == "solitaire_hint" {
            self.state.card_hint = Some(card_hints::solitaire(&self.state));
        } else if scene == "freecell_hint" {
            self.state.card_hint = Some(card_hints::freecell(&self.state));
        }
        if scene.starts_with("tutorial_") {
            if let Screen::Game(game) = self.state.screen {
                self.state.tutorial = Some(game);
            }
        }
        self.transition = 0.;
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
                .press(viewport.screen_to_logical(vec2(mouse_position().0, mouse_position().1)));
        }
        if is_mouse_button_released(MouseButton::Left) {
            let viewport = ui::viewport();
            let position = viewport.screen_to_logical(vec2(mouse_position().0, mouse_position().1));
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
                    Gesture::Drag { .. } => {}
                    Gesture::LongPress(_) => {}
                }
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            self.pointer.cancel();
            self.state.screen = Screen::Cabinet;
            self.state.confirm_restart = false;
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
        let _ = dt;
    }
    pub fn draw(&mut self) {
        clear_background(cosmetics::background(self.state.board_theme));
        let viewport = ui::viewport();
        let (layout_width, layout_height) = ui::layout_size();
        set_camera(&Camera2D {
            target: vec2(layout_width / 2., layout_height / 2.),
            zoom: vec2(
                2. * viewport.scale / layout_width,
                2. * viewport.scale / layout_height,
            ),
            ..Default::default()
        });
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
        if !card_hints::is_hint(action) {
            self.state.card_hint = None;
        }
        match action {
            ui::UiAction::Open(index) => {
                self.state.selected = index;
                let id = GameId::ALL[index];
                if matches!(
                    id,
                    GameId::Game2048
                        | GameId::Minesweeper
                        | GameId::Sudoku
                        | GameId::Nonogram
                        | GameId::Solitaire
                        | GameId::FreeCell
                        | GameId::Yahtzee
                        | GameId::Reversi
                        | GameId::LightsOut
                        | GameId::TicTacToe
                        | GameId::MemoryPairs
                ) {
                    self.state.screen = Screen::Game(id);
                    self.state.tutorial = (!matches!(
                        id,
                        GameId::LightsOut | GameId::TicTacToe | GameId::MemoryPairs
                    ) && !self.state.tutorial_seen[id.index()])
                    .then_some(id);
                } else {
                    self.notifications
                        .info(format!("{} is coming soon", id.title()));
                }
            }
            ui::UiAction::Cabinet => {
                self.state.screen = Screen::Cabinet;
                self.state.tutorial = None;
                self.state.confirm_reset = false;
            }
            ui::UiAction::Help => self.state.screen = Screen::Help,
            ui::UiAction::Records => self.state.screen = Screen::Records,
            ui::UiAction::Rules => self.state.screen = Screen::Rules,
            ui::UiAction::Credits => self.state.screen = Screen::Credits,
            ui::UiAction::Settings => self.state.screen = Screen::Settings,
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
            ui::UiAction::NonogramCell(index) => {
                self.state.nonogram.select(index);
                self.state.nonogram.toggle(index);
            }
            ui::UiAction::NonogramMode => {
                self.state.nonogram.toggle_mode();
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
            ui::UiAction::FivefoldNew => {
                self.state.fivefold =
                    crate::fivefold::Fivefold::new(self.state.fivefold.seed.wrapping_add(1));
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
            ui::UiAction::MemoryPairsUndo => {
                self.state.memory_pairs.undo();
            }
            ui::UiAction::MemoryPairsNew => {
                let seed = self.state.memory_pairs.seed.wrapping_add(1);
                self.state.memory_pairs.reset(seed);
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
                self.state.game = crate::state::Game2048::new(self.state.game.seed.wrapping_add(1));
                self.state.confirm_restart = false;
            }
            ui::UiAction::Cancel => self.state.confirm_restart = false,
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
                self.state = AppState::default();
            }
            ui::UiAction::CancelResetData => self.state.confirm_reset = false,
        }
        if self.state.screen != previous_screen {
            self.transition = if self.state.reduced_motion { 0. } else { 1. };
        }
        self.update_records();
        self.save_autosave();
        self.play_feedback(
            if matches!(
                action,
                ui::UiAction::ConfirmRestart | ui::UiAction::ConfirmResetData
            ) {
                SoundCue::Success
            } else {
                SoundCue::Tap
            },
        );
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

fn card_drag_actions(
    state: &crate::state::AppState,
    start: Vec2,
    end: Vec2,
    portrait: bool,
    compact_landscape: bool,
) -> Vec<ui::UiAction> {
    if !matches!(
        state.screen,
        Screen::Game(GameId::Solitaire | GameId::FreeCell)
    ) {
        return Vec::new();
    }
    let actions_at = |point| {
        if portrait {
            match state.screen {
                Screen::Game(GameId::Solitaire) => responsive_cards::solitaire_clicks(state, point),
                Screen::Game(GameId::FreeCell) => responsive_cards::freecell_clicks(state, point),
                _ => Vec::new(),
            }
        } else if compact_landscape {
            match state.screen {
                Screen::Game(GameId::Solitaire) => {
                    responsive_landscape_cards::solitaire_clicks(state, point)
                }
                Screen::Game(GameId::FreeCell) => {
                    responsive_landscape_cards::freecell_clicks(state, point)
                }
                _ => Vec::new(),
            }
        } else {
            match state.screen {
                Screen::Game(GameId::Solitaire) => solitaire_ui::solitaire_clicks(state, point),
                Screen::Game(GameId::FreeCell) => freecell_ui::freecell_clicks(state, point),
                _ => Vec::new(),
            }
        }
    };
    actions_at(start)
        .into_iter()
        .chain(actions_at(end))
        .collect()
}

#[cfg(test)]
mod tests;

fn swipe_direction(delta: Vec2) -> Direction {
    if delta.x.abs() > delta.y.abs() {
        if delta.x > 0.0 {
            Direction::Right
        } else {
            Direction::Left
        }
    } else if delta.y > 0.0 {
        Direction::Down
    } else {
        Direction::Up
    }
}
