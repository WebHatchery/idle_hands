//! Shared logical viewport and cancellable pointer gesture normalization.

use crate::state::{AppState, Screen};
use macroquad::prelude::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gesture {
    Tap(Vec2),
    Drag { start: Vec2, end: Vec2 },
    LongPress(Vec2),
}

pub const LONG_PRESS_SECONDS: f32 = 0.55;
pub const DRAG_DISTANCE: f32 = 16.0;

pub fn should_cancel_for_touch_count(touch_count: usize) -> bool {
    touch_count > 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerLayer {
    Board,
    Tutorial,
    RestartConfirmation,
    NoticeLog,
    ResetConfirmation,
    LifecyclePause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointerScope {
    pub screen: Screen,
    pub layer: PointerLayer,
}

impl PointerScope {
    pub fn from_state(state: &AppState) -> Self {
        let layer = if state.lifecycle_paused {
            PointerLayer::LifecyclePause
        } else if state.tutorial.is_some() {
            PointerLayer::Tutorial
        } else if state.confirm_restart && state.pending_restart.is_some() {
            PointerLayer::RestartConfirmation
        } else if state.notice_log_view {
            PointerLayer::NoticeLog
        } else if state.confirm_reset {
            PointerLayer::ResetConfirmation
        } else {
            PointerLayer::Board
        };
        Self {
            screen: state.screen,
            layer,
        }
    }
}

#[derive(Debug, Default)]
pub struct PointerTracker {
    pub start: Option<Vec2>,
    pub scope: Option<PointerScope>,
    pub cancelled: bool,
    pub elapsed: f32,
}

impl PointerTracker {
    pub fn press(&mut self, position: Option<Vec2>, scope: PointerScope) {
        self.start = position;
        self.scope = position.map(|_| scope);
        self.cancelled = position.is_none();
        self.elapsed = 0.;
    }
    pub fn tick(&mut self, dt: f32) {
        if self.start.is_some() && !self.cancelled {
            self.elapsed += dt.max(0.);
        }
    }
    pub fn cancel(&mut self) {
        self.start = None;
        self.scope = None;
        self.cancelled = true;
    }
    pub fn sync_scope(&mut self, scope: PointerScope) -> bool {
        if self.start.is_some() && self.scope != Some(scope) {
            self.cancel();
            true
        } else {
            false
        }
    }
    pub fn release(&mut self, position: Option<Vec2>, scope: PointerScope) -> Option<Gesture> {
        let start = self.start.take()?;
        let captured_scope = self.scope.take();
        if self.cancelled || captured_scope != Some(scope) {
            return None;
        }
        let end = position?;
        if (end - start).length() > DRAG_DISTANCE {
            Some(Gesture::Drag { start, end })
        } else if self.elapsed >= LONG_PRESS_SECONDS {
            Some(Gesture::LongPress(end))
        } else {
            Some(Gesture::Tap(end))
        }
    }
}
