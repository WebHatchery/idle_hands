//! Runtime summary for saves that were preserved after a load failure.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveRecoveryNotice {
    pub damaged_slots: usize,
    pub quarantined_slots: usize,
}

impl SaveRecoveryNotice {
    pub fn new() -> Self {
        Self {
            damaged_slots: 0,
            quarantined_slots: 0,
        }
    }

    pub fn record(&mut self, quarantined: bool) {
        self.damaged_slots += 1;
        if quarantined {
            self.quarantined_slots += 1;
        }
    }

    pub fn summary(&self) -> String {
        if self.damaged_slots == 1 && self.quarantined_slots == 1 {
            "One save was moved aside safely.".into()
        } else if self.quarantined_slots == self.damaged_slots {
            format!("{} saves were moved aside safely.", self.damaged_slots)
        } else if self.damaged_slots == 1 {
            "One save could not be loaded safely.".into()
        } else {
            format!(
                "{} saves need attention before they can load.",
                self.damaged_slots
            )
        }
    }
}

impl Default for SaveRecoveryNotice {
    fn default() -> Self {
        Self::new()
    }
}
