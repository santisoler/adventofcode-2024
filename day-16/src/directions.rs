pub enum Direction {
    Forward,
    Right,
    Left,
}

impl Direction {
    pub fn as_int(&self) -> i32 {
        match self {
            Direction::Forward => 0,
            Direction::Right => 1,
            Direction::Left => -1,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    pub fn rotate(&self, direction: &Direction) -> Self {
        self.from_int(self.as_int() + direction.as_int())
    }

    pub fn as_int(&self) -> i32 {
        match self {
            Orientation::North => 0,
            Orientation::East => 1,
            Orientation::South => 2,
            Orientation::West => 3,
        }
    }

    fn from_int(&self, integer: i32) -> Self {
        match integer.rem_euclid(4) {
            0 => Orientation::North,
            1 => Orientation::East,
            2 => Orientation::South,
            3 => Orientation::West,
            _ => panic!("Invalid integer {integer}"),
        }
    }
}
