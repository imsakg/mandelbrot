use crate::{Error, Viewport};

/// Bounded viewport-only undo history. New moves discard redo entries.
#[derive(Debug)]
pub struct Navigation {
    entries: Vec<Viewport>,
    cursor: usize,
    capacity: usize,
}
impl Navigation {
    pub fn new(initial: Viewport, capacity: usize) -> Result<Self, Error> {
        if !(1..=1000).contains(&capacity) {
            return Err(Error("history capacity must be in 1..=1000".into()));
        }
        Ok(Self {
            entries: vec![initial],
            cursor: 0,
            capacity,
        })
    }
    pub fn current(&self) -> Viewport {
        self.entries[self.cursor]
    }
    pub fn push(&mut self, view: Viewport) {
        if view == self.current() {
            return;
        }
        self.entries.truncate(self.cursor + 1);
        self.entries.push(view);
        if self.entries.len() > self.capacity {
            self.entries.remove(0);
        }
        self.cursor = self.entries.len() - 1;
    }
    pub fn undo(&mut self) -> Option<Viewport> {
        if self.cursor == 0 {
            None
        } else {
            self.cursor -= 1;
            Some(self.current())
        }
    }
    pub fn redo(&mut self) -> Option<Viewport> {
        if self.cursor + 1 == self.entries.len() {
            None
        } else {
            self.cursor += 1;
            Some(self.current())
        }
    }
}
