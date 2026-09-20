use super::super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_game_action_group_6(&mut self, action: &UiAction) -> bool {
        match action {
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
                self.state.tower_inspection = Some(*index);
            }
            UiAction::TowerInspect(index) => {
                self.state.tower_inspection = Some(*index);
                self.state.card_hint = None;
            }
            UiAction::TowerBuild(index) => {
                let (built, message) = {
                    let game = &mut self.state.games.tiny_tower_defence;
                    let built = game.build_or_upgrade(*index);
                    let message = (!built).then(|| tower_failure_message(game, *index));
                    (built, message)
                };
                if built {
                    self.state.tower_inspection = None;
                    self.state.card_hint = None;
                } else {
                    self.state.card_hint = message;
                }
            }
            UiAction::TowerCancelInspection => {
                self.state.tower_inspection = None;
                self.state.card_hint = None;
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
            _ => return false,
        }
        true
    }
}

fn tower_failure_message(
    game: &crate::tiny_tower_defence::TinyTowerDefence,
    index: usize,
) -> String {
    match game.cell_availability(index) {
        crate::tiny_tower_defence::TowerCellAvailability::Blocked => {
            "That lane is reserved for entry or the base".to_owned()
        }
        crate::tiny_tower_defence::TowerCellAvailability::MaxLevel { .. } => {
            "That tower is already at maximum level".to_owned()
        }
        crate::tiny_tower_defence::TowerCellAvailability::Build { .. }
        | crate::tiny_tower_defence::TowerCellAvailability::Upgrade { .. }
            if game.phase != crate::tiny_tower_defence::TowerPhase::Build =>
        {
            "Pause the wave before changing towers".to_owned()
        }
        crate::tiny_tower_defence::TowerCellAvailability::Build { cost, .. }
        | crate::tiny_tower_defence::TowerCellAvailability::Upgrade { cost, .. } => {
            format!("Need {cost} gold, but only {} remains", game.gold)
        }
    }
}
