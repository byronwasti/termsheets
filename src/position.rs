use std::ops::{Add, Sub};

pub struct CellPos {
    pub x: usize,
    pub y: usize,
}

impl CellPos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
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
