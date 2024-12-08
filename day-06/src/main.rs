use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let fname = "data/test_input";
        let result = solve_part1(fname);
        assert_eq!(result, 41);
    }
}

#[derive(Debug)]
enum Location {
    Obstacle,
    Empty,
    Visited,
}

#[derive(Debug)]
enum Orientation {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Location>>,
}

impl Map {
    fn new() -> Self {
        Self { map: vec![] }
    }

    fn n_rows(&self) -> usize {
        return self.map.len();
    }

    fn n_cols(&self) -> usize {
        return self.map[0].len();
    }
}
#[derive(Debug)]
struct Guard {
    row: i32,
    column: i32,
    orientation: Orientation,
}

impl Guard {
    fn is_outside(&self, map: &Map) -> bool {
        match self.orientation {
            Orientation::Up => {
                if self.row == 0 {
                    return true;
                }
            }
            Orientation::Down => {
                if self.row == map.n_rows() as i32 - 1 {
                    return true;
                }
            }
            Orientation::Left => {
                if self.column == 0 {
                    return true;
                }
            }
            Orientation::Right => {
                if self.column == map.n_cols() as i32 - 1 {
                    return true;
                }
            }
        }
        return false;
    }

    fn rotate(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        };
    }

    fn step_forward(&mut self, map: &mut Map) -> bool {
        if self.is_outside(&map) {
            return false;
        }
        let (next_row, next_col) = match self.orientation {
            Orientation::Up => (self.row as usize - 1, self.column as usize),
            Orientation::Down => (self.row as usize + 1, self.column as usize),
            Orientation::Left => (self.row as usize, self.column as usize - 1),
            Orientation::Right => (self.row as usize, self.column as usize + 1),
        };
        let next_location = &map.map[next_row][next_col];
        if let Location::Obstacle = next_location {
            return false;
        } else {
            map.map[next_row][next_col] = Location::Visited;
            (self.row, self.column) = (next_row as i32, next_col as i32);
            return true;
        };
    }
}

fn parse_file(fname: &str) -> (Map, Guard) {
    let content = fs::read_to_string(fname).expect("Couldn't read");
    let mut map = Map::new();
    let mut guard = Guard {
        row: -1,
        column: -1,
        orientation: Orientation::Up,
    };
    for (i, line) in content.lines().enumerate() {
        let mut row = vec![];
        for (j, character) in line.chars().enumerate() {
            match character {
                '.' => {
                    row.push(Location::Empty);
                }
                '#' => {
                    row.push(Location::Obstacle);
                }
                '^' => {
                    row.push(Location::Visited);
                    (guard.row, guard.column) = (i as i32, j as i32);
                    guard.orientation = Orientation::Up;
                }
                '>' => {
                    row.push(Location::Visited);
                    (guard.row, guard.column) = (i as i32, j as i32);
                    guard.orientation = Orientation::Right;
                }
                '<' => {
                    row.push(Location::Visited);
                    (guard.row, guard.column) = (i as i32, j as i32);
                    guard.orientation = Orientation::Left;
                }
                'v' => {
                    row.push(Location::Visited);
                    (guard.row, guard.column) = (i as i32, j as i32);
                    guard.orientation = Orientation::Down;
                }
                _ => {
                    panic!("Invalid character {character}")
                }
            }
        }
        map.map.push(row);
    }
    (map, guard)
}

fn solve_part1(fname: &str) -> i32 {
    let (mut map, mut guard) = parse_file(fname);
    loop {
        if guard.is_outside(&map) {
            break;
        }
        if !guard.step_forward(&mut map) {
            guard.rotate()
        };
    }
    let visited = map
        .map
        .iter()
        .flatten()
        .filter(|location| {
            if let Location::Visited = location {
                true
            } else {
                false
            }
        })
        .count();
    visited as i32
}

fn main() {
    let fname = "data/input";
    let result = solve_part1(fname);
    println!("Solution to part 1: {result}")
}
