use super::super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_game_action_group_7(&mut self, action: &UiAction) -> bool {
        match action {
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
                let day = crate::daily_challenge::current_day();
                self.state.games.daily_dungeon.reset(day);
            }
            _ => return false,
        }
        true
    }
}
