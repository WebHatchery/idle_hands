//! Application lifecycle hooks for the game host.

use super::Game;

impl Game {
    pub(super) fn resume_lifecycle(&mut self) {
        self.state.lifecycle_paused = false;
        self.notifications.info("The cabinet is ready again");
    }

    pub fn end_analytics_session(&mut self) {
        self.analytics.end_session();
    }

    pub fn note_frame_gap(&mut self, frame_seconds: f32) {
        if !crate::lifecycle::should_pause_game(
            frame_seconds,
            self.state.screen.is_game(),
            self.state.tutorial.is_some(),
            self.state.confirm_restart,
            self.state.confirm_reset,
            self.state.lifecycle_paused,
        ) {
            return;
        }
        self.pointer.cancel();
        self.state.lifecycle_paused = true;
        self.notifications
            .info("Paused safely while the cabinet was away");
        self.flush_autosave();
    }
}
