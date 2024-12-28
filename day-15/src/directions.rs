use std::fmt;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn get_next_position(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
        }
    }

    pub fn from(character: &char) -> Self {
        match character {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            e => panic!("Inavlid direction character: {}", e),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Direction::Up => "^",
            Direction::Down => "v",
            Direction::Right => ">",
            Direction::Left => "<",
        };
        write!(f, "{}", string)
    }
}
