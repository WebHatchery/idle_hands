//! Shared logical viewport and cancellable pointer gesture normalization.

use macroquad::prelude::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gesture {
    Tap(Vec2),
    Drag { start: Vec2, end: Vec2 },
    LongPress(Vec2),
}

pub const LONG_PRESS_SECONDS: f32 = 0.55;

#[derive(Debug, Default)]
pub struct PointerTracker {
    start: Option<Vec2>,
    cancelled: bool,
    elapsed: f32,
}

impl PointerTracker {
    pub fn press(&mut self, position: Option<Vec2>) {
        self.start = position;
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
        self.cancelled = true;
    }
    pub fn release(&mut self, position: Option<Vec2>) -> Option<Gesture> {
        let start = self.start.take()?;
        if self.cancelled {
            return None;
        }
        let end = position?;
        if (end - start).length() > 16. {
            Some(Gesture::Drag { start, end })
        } else if self.elapsed >= LONG_PRESS_SECONDS {
            Some(Gesture::LongPress(end))
        } else {
            Some(Gesture::Tap(end))
        }
    }
}

#[cfg(test)]
mod tests;
