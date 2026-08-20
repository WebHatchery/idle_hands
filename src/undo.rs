//! Bounded in-memory undo history shared by deterministic game rules.

const DEFAULT_CAPACITY: usize = 32;

#[derive(Debug, Clone)]
pub struct UndoStack<T> {
    entries: Vec<T>,
    capacity: usize,
}

impl<T> Default for UndoStack<T> {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }
}

impl<T> UndoStack<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::new(),
            capacity: capacity.max(1),
        }
    }

    pub fn push(&mut self, entry: T) {
        if self.entries.len() == self.capacity {
            self.entries.remove(0);
        }
        self.entries.push(entry);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.entries.pop()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests;
