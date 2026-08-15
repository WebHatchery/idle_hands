//! Application lifecycle and input routing.

use crate::input::{Gesture, PointerTracker};
use crate::{
    data::GameData,
    state::{AppState, CollectionSave, Direction, GameId, Screen},
};
use crate::{nonogram_ui, ui};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::notifications::{
    NotificationAnchor, NotificationManager, NotificationRenderConfig,
};
use macroquad_toolkit::persistence::{
    load_from_slot_with_migration, save_to_slot_with_version, slot_exists,
};

pub struct Game {
    pub data: GameData,
    pub state: AppState,
    assets: AssetManager,
    notifications: NotificationManager,
    pointer: PointerTracker,
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
        };
        game.load_autosave();
        game
    }
    pub fn update(&mut self, dt: f32) {
        self.notifications.update(dt);
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
                        for action in nonogram_ui::drag_actions(&self.state, start, end) {
                            self.apply(action);
                        }
                    }
                    Gesture::Tap(_) => {
                        for action in ui::clicks(&self.state) {
                            self.apply(action);
                        }
                    }
                    Gesture::Drag { .. } => {}
                }
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            self.pointer.cancel();
            self.state.screen = Screen::Cabinet;
            self.state.confirm_restart = false;
            self.state.tutorial = None;
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
        clear_background(Color::new(0.035, 0.028, 0.055, 1.0));
        let viewport = ui::viewport();
        set_camera(&Camera2D {
            target: vec2(ui::LOGICAL_WIDTH / 2., ui::LOGICAL_HEIGHT / 2.),
            zoom: vec2(
                2. * viewport.scale / screen_width(),
                -2. * viewport.scale / screen_height(),
            ),
            ..Default::default()
        });
        ui::draw(&self.state, &self.data, self.assets.len());
        set_default_camera();
        self.notifications
            .draw_with_config(&NotificationRenderConfig {
                anchor: NotificationAnchor::BottomRight,
                ..Default::default()
            });
    }
    fn apply(&mut self, action: ui::UiAction) {
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
                ) {
                    self.state.screen = Screen::Game(id);
                    self.state.tutorial = (!self.state.tutorial_seen[id.index()]).then_some(id);
                } else {
                    self.notifications
                        .info(format!("{} is coming soon", id.title()));
                }
            }
            ui::UiAction::Cabinet => {
                self.state.screen = Screen::Cabinet;
                self.state.tutorial = None;
            }
            ui::UiAction::Help => self.state.screen = Screen::Help,
            ui::UiAction::Records => self.state.screen = Screen::Records,
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
            }
            ui::UiAction::SolitaireStock => {
                self.state.solitaire.draw_stock();
            }
            ui::UiAction::SolitaireTableau(column, depth) => {
                if self.state.solitaire.selected.is_some() {
                    self.state.solitaire.move_to_tableau(column);
                } else {
                    self.state.solitaire.select_tableau(column, depth);
                }
            }
            ui::UiAction::SolitaireWaste => {
                self.state.solitaire.select_waste();
            }
            ui::UiAction::SolitaireFoundation(suit) => {
                self.state.solitaire.move_to_foundation(suit);
            }
            ui::UiAction::SolitaireUndo => {
                self.state.solitaire.undo();
            }
            ui::UiAction::SolitaireNew => {
                self.state.solitaire =
                    crate::solitaire::Solitaire::new(self.state.solitaire.seed.wrapping_add(1));
            }
            ui::UiAction::FreeCellCell(cell) => {
                if self.state.freecell.selected.is_some() {
                    self.state.freecell.move_selected_to_cascade(cell);
                } else {
                    self.state.freecell.select_cell(cell);
                }
            }
            ui::UiAction::FreeCellCascade(cascade, depth) => {
                if self.state.freecell.selected.is_some() {
                    self.state.freecell.move_selected_to_cascade(cascade);
                } else {
                    self.state.freecell.select_cascade(cascade, depth);
                }
            }
            ui::UiAction::FreeCellFoundation(suit) => {
                self.state.freecell.move_selected_to_foundation(suit);
            }
            ui::UiAction::FreeCellUndo => {
                self.state.freecell.undo();
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
                if self.state.reversi.place(index) {
                    self.state.reversi.ai_move();
                }
            }
            ui::UiAction::ReversiPass => {
                if self.state.reversi.pass() {
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
        }
        self.update_records();
        self.save_autosave();
    }
    fn update_records(&mut self) {
        let records = &mut self.state.records;
        records.best_2048 = records.best_2048.max(self.state.game.best);
        let sudoku_index = match self.state.sudoku.difficulty {
            crate::sudoku::SudokuDifficulty::Easy => 0,
            crate::sudoku::SudokuDifficulty::Medium => 1,
            crate::sudoku::SudokuDifficulty::Hard => 2,
        };
        if self.state.sudoku.status == crate::sudoku::SudokuStatus::Won {
            if let Some(moves) = self.state.sudoku.best_moves {
                records.sudoku[sudoku_index] =
                    Some(records.sudoku[sudoku_index].map_or(moves, |best| best.min(moves)));
            }
        }
        let nonogram_index = match self.state.nonogram.preset {
            crate::nonogram::NonogramPreset::Small => 0,
            crate::nonogram::NonogramPreset::Medium => 1,
            crate::nonogram::NonogramPreset::Large => 2,
        };
        if self.state.nonogram.status == crate::nonogram::NonogramStatus::Won {
            if let Some(moves) = self.state.nonogram.best_moves {
                records.nonogram[nonogram_index] =
                    Some(records.nonogram[nonogram_index].map_or(moves, |best| best.min(moves)));
            }
        }
        if self.state.solitaire.status == crate::solitaire::SolitaireStatus::Won {
            records.solitaire_best_moves = Some(
                records
                    .solitaire_best_moves
                    .map_or(self.state.solitaire.moves, |best| {
                        best.min(self.state.solitaire.moves)
                    }),
            );
        }
        if self.state.freecell.status == crate::freecell::FreeCellStatus::Won {
            records.freecell_best_moves = Some(
                records
                    .freecell_best_moves
                    .map_or(self.state.freecell.moves, |best| {
                        best.min(self.state.freecell.moves)
                    }),
            );
        }
        if self.state.fivefold.status == crate::fivefold::FivefoldStatus::Complete {
            records.fivefold_best_total =
                records.fivefold_best_total.max(self.state.fivefold.total());
        }
        if self.state.reversi.status == crate::reversi::ReversiStatus::Won {
            records.reversi_best_score = records
                .reversi_best_score
                .max(self.state.reversi.score(1) as u8);
        }
    }
    fn try_move(&mut self, direction: Direction) {
        if self.state.game.move_in(direction) && self.state.game.won() {
            self.notifications
                .success("2048 reached — keep playing or start a fresh board");
        }
        self.save_autosave();
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
    }

    fn load_autosave(&mut self) {
        if !slot_exists(&self.data.config.game_name, &self.data.config.save_slot) {
            return;
        }
        let loaded: Result<CollectionSave, String> = load_from_slot_with_migration(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &self.data.config.version,
            |_, value| {
                let payload = value.get("data").cloned().unwrap_or(value);
                serde_json::from_value(payload)
                    .map_err(|error| format!("Unsupported collection save: {}", error))
            },
        );
        match loaded {
            Ok(save) => {
                save.apply_to(&mut self.state);
                self.notifications.info("Restored the cabinet autosave");
            }
            Err(error) => self
                .notifications
                .warning(format!("Autosave could not be loaded: {}", error)),
        }
    }
}
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
