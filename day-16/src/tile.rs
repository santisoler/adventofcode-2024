use std::cmp::Ordering;

pub enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    // Return displacement on x and y in order to make one step towards the current direction.
    pub fn get_displacement(&self) -> (i32, i32) {
        match self {
            Orientation::North => (0, -1),
            Orientation::South => (0, 1),
            Orientation::East => (1, 0),
            Orientation::West => (-1, 0),
        }
    }

    // Count how many turns are needed to orient in the other direction.
    pub fn count_turns(&self, other: &Orientation) -> u32 {
        let (n1, n2) = (self.as_int(), other.as_int());
        let n_turns = (n1 - n2).abs() as u32;
        if n_turns == 3 {
            return 1;
        }
        n_turns
    }

    fn as_int(&self) -> i32 {
        match self {
            Orientation::North => 0,
            Orientation::East => 1,
            Orientation::South => 2,
            Orientation::West => 3,
        }
    }
}

pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub score: u32,
    pub orientation: Orientation,
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
