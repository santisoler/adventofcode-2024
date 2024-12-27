use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, 10092);
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn get_next_position(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
        }
    }

    fn from(character: &char) -> Self {
        match character {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            e => panic!("Inavlid direction character: {}", e),
        }
    }
}

struct Robot {
    x: i32,
    y: i32,
}

impl Robot {
    fn new() -> Self {
        Robot { x: 0, y: 0 }
    }
}

struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn nrows(&self) -> i32 {
        self.map[0].len() as i32
    }

    fn ncols(&self) -> i32 {
        self.map.len() as i32
    }

    fn get(&self, x: i32, y: i32) -> Option<char> {
        if (x < 0) | (x >= self.nrows()) | (y < 0) | (y >= self.ncols()) {
            return None;
        }
        Some(self.map[y as usize][x as usize])
    }

    fn move_robot(&mut self, robot: &mut Robot, direction: &Direction) {
        let (x_next, y_next) = direction.get_next_position(robot.x, robot.y);
        match self.get(x_next, y_next) {
            Some('.') => {
                robot.x = x_next;
                robot.y = y_next
            }
            Some('#') => (), // cannot move into a wall
            Some('O') => {
                if self.move_box(direction, x_next, y_next) {
                    robot.x = x_next;
                    robot.y = y_next
                }
            }
            Some(e) => panic!("Invalid char {} in {}, {}", e, robot.x, robot.y),
            None => panic!("Invalid location: {}, {}", robot.x, robot.y),
        }
    }

    // Try to move a box to a given direction.
    // If the box was moved, return true. Else return false.
    fn move_box(&mut self, direction: &Direction, x: i32, y: i32) -> bool {
        match self.get(x, y) {
            Some('O') => (),
            Some(_) => panic!("Tried to move a tile that doesn't have a box: {} {}", x, y),
            None => panic!("Invalid location: {}, {}", x, y),
        }
        let (x_next, y_next) = direction.get_next_position(x, y);
        match self.get(x_next, y_next) {
            Some('O') => {
                let moved = self.move_box(direction, x_next, y_next);
                // Move box only if the next one moved as well
                if moved {
                    self.map[y as usize][x as usize] = '.';
                    self.map[y_next as usize][x_next as usize] = 'O';
                }
                return moved;
            }
            Some('.') => {
                // Move box
                self.map[y as usize][x as usize] = '.';
                self.map[y_next as usize][x_next as usize] = 'O';
                return true;
            }
            Some('#') => return false,
            Some(_) => panic!("Found invalid character in: {}, {}", x, y),
            None => panic!("Tried to move box outside the map: {}, {}", x, y),
        }
    }

    fn print(&self, robot: &Robot) {
        let mut map = self.map.clone();
        map[robot.y as usize][robot.x as usize] = '@';
        let text = map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", text);
    }

    // Return the sum of the GPS coordinates of every box in the map
    fn get_gps(&self) -> i32 {
        let mut gps = 0;
        for (j, row) in self.map.iter().enumerate() {
            for (i, char) in row.iter().enumerate() {
                if *char == 'O' {
                    gps += (i + 100 * j) as i32;
                }
            }
        }
        gps
    }
}

fn read_file(fname: &str) -> (Map, Robot, Vec<Direction>) {
    let content = fs::read_to_string(fname).unwrap();
    let mut read_map = true;
    let mut map: Vec<Vec<char>> = vec![];
    let mut directions: Vec<Direction> = vec![];
    let mut robot: Robot = Robot::new();
    for (j, line) in content.lines().enumerate() {
        if line.is_empty() {
            read_map = false;
            continue;
        }
        if read_map {
            let row = line
                .chars()
                .enumerate()
                .map(|(i, c)| match c {
                    '@' => {
                        robot.x = i as i32;
                        robot.y = j as i32;
                        '.'
                    }
                    c => c,
                })
                .collect();
            map.push(row)
        } else {
            directions.extend(
                line.chars()
                    .map(|c| Direction::from(&c))
                    .collect::<Vec<Direction>>(),
            )
        }
    }
    (Map { map }, robot, directions)
}

fn solve_part_one(fname: &str) -> i32 {
    let (mut map, mut robot, directions) = read_file(fname);
    println!("Start state");
    map.print(&robot);
    for direction in directions.iter() {
        map.move_robot(&mut robot, direction);
    }
    println!("\nEnd state");
    map.print(&robot);
    map.get_gps()
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
