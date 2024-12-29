use std::cmp::Ordering;

use crate::directions::{Direction, Orientation};

pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub score: u32,
    pub orientation: Orientation,
}

impl Tile {
    pub fn get_neighbor(&self, direction: &Direction) -> (usize, usize, Orientation) {
        let orientation = self.orientation.rotate(&direction);
        let (x, y) = match orientation {
            Orientation::East => (self.x + 1, self.y),
            Orientation::West => (self.x - 1, self.y),
            Orientation::North => (self.x, self.y - 1),
            Orientation::South => (self.x, self.y + 1),
        };
        (x, y, orientation)
    }
}

// Need to implement the Ord trait for Tile so it can be used in the BinaryHeap.
impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Tile {}
