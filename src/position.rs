use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct CellPos {
    pub x: usize,
    pub y: usize,
}

impl CellPos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn offset(&self, other: &CellPos) -> (i32, i32) {
        let x = other.x as i32 - self.x as i32;
        let y = other.y as i32 - self.y as i32;
        (x, y)
    }
}

impl Default for CellPos {
    fn default() -> CellPos {
        CellPos {
            x: 0,
            y: 0,
        }
    }
}

impl Add for CellPos {
    type Output = CellPos;

    fn add(self, other: CellPos) -> CellPos {
        CellPos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for CellPos {
    type Output = CellPos;

    fn sub(self, other: CellPos) -> CellPos {
        CellPos {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}
