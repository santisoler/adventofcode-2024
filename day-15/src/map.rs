use crate::Direction;
use crate::Robot;

pub struct Map {
    pub map: Vec<Vec<char>>,
    nrows: i32,
    ncols: i32,
}

impl Map {
    // Create new MapLarge
    pub fn new_from(map: Vec<Vec<char>>) -> Self {
        let nrows = map[0].len() as i32;
        let ncols = map.len() as i32;
        Self { map, nrows, ncols }
    }

    fn get(&self, x: i32, y: i32) -> Option<char> {
        if (x < 0) | (x >= self.nrows) | (y < 0) | (y >= self.ncols) {
            return None;
        }
        Some(self.map[y as usize][x as usize])
    }

    fn write(&mut self, x: i32, y: i32, value: char) {
        if (x < 0) | (x >= self.nrows) | (y < 0) | (y >= self.ncols) {
            panic!("trying to write outside the map")
        }
        self.map[y as usize][x as usize] = value;
    }

    pub fn move_robot(&mut self, robot: &mut Robot, direction: &Direction) {
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

    // Move robot in map with large boxes
    pub fn move_robot_large(&mut self, robot: &mut Robot, direction: &Direction) {
        let (x_next, y_next) = direction.get_next_position(robot.x, robot.y);
        match self.get(x_next, y_next) {
            Some('.') => {
                robot.x = x_next;
                robot.y = y_next
            }
            Some('#') => (), // cannot move into a wall
            Some('[') | Some(']') => {
                if self.move_large_box(direction, x_next, y_next) {
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

    // Return true if a given large box can be moved
    pub fn can_move_large_box(&self, direction: &Direction, x: i32, y: i32) -> bool {
        let box_side = match self.get(x, y) {
            Some(c) => c,
            None => panic!("Invalid location: {}, {}", x, y),
        };
        return match (box_side, direction) {
            ('[', Direction::Right) => {
                let x_next = x + 2;
                match self.get(x_next, y) {
                    Some('#') => false,
                    Some('.') => true,
                    Some('[') => self.can_move_large_box(direction, x_next, y),
                    Some(e) => panic!("Invalid tile {} at {}, {}", e, x_next, y),
                    None => panic!("Invalid location: {}, {}", x, y),
                }
            }
            (']', Direction::Left) => {
                let x_next = x - 2;
                match self.get(x_next, y) {
                    Some('#') => false,
                    Some('.') => true,
                    Some(']') => self.can_move_large_box(direction, x_next, y),
                    Some(e) => panic!("Invalid tile {} at {}, {}", e, x_next, y),
                    None => panic!("Invalid location: {}, {}", x, y),
                }
            }
            ('[', Direction::Up) | ('[', Direction::Down) => {
                let (_, y_next) = direction.get_next_position(x, y);
                let left = match self.get(x, y_next) {
                    Some(c) => c,
                    None => panic!(""),
                };
                let right = match self.get(x + 1, y_next) {
                    Some(c) => c,
                    None => panic!(""),
                };
                match (left, right) {
                    ('.', '.') => true,
                    ('#', _) | (_, '#') => false,
                    ('[', ']') => self.can_move_large_box(direction, x, y_next),
                    (']', '[') => {
                        let can_move_left = self.can_move_large_box(direction, x, y_next);
                        let can_move_right = self.can_move_large_box(direction, x + 1, y_next);
                        can_move_left && can_move_right
                    }
                    (']', '.') => self.can_move_large_box(direction, x, y_next),
                    ('.', '[') => self.can_move_large_box(direction, x + 1, y_next),
                    (l, r) => panic!("invalid {}, {}", l, r),
                }
            }
            (']', Direction::Up) | (']', Direction::Down) => {
                self.can_move_large_box(direction, x - 1, y)
            }
            (_, _) => {
                panic!(
                    "Cannot move tile {} in ({}, {}) to the {:?}",
                    box_side, x, y, direction
                )
            }
        };
    }

    // Move a large box to the given direction if possible.
    // Return true if the box was moved, else false.
    fn move_large_box(&mut self, direction: &Direction, x: i32, y: i32) -> bool {
        // Check if the box can be moved, return false in that case
        if !self.can_move_large_box(direction, x, y) {
            return false;
        }
        // Move the box and their neigbors
        let box_side = match self.get(x, y) {
            Some(c) => c,
            None => panic!("Invalid location: {}, {}", x, y),
        };
        match (box_side, direction) {
            ('[', Direction::Right) => {
                if let Some('[') = self.get(x + 2, y) {
                    self.move_large_box(direction, x + 2, y);
                }
                self.write(x, y, '.');
                self.write(x + 1, y, '[');
                self.write(x + 2, y, ']');
            }
            (']', Direction::Left) => {
                if let Some(']') = self.get(x - 2, y) {
                    self.move_large_box(direction, x - 2, y);
                }
                self.write(x, y, '.');
                self.write(x - 1, y, ']');
                self.write(x - 2, y, '[');
            }
            ('[', Direction::Up) | ('[', Direction::Down) => {
                // todo
                let (_, y_next) = direction.get_next_position(x, y);
                let left = match self.get(x, y_next) {
                    Some(c) => c,
                    None => panic!(""),
                };
                let right = match self.get(x + 1, y_next) {
                    Some(c) => c,
                    None => panic!(""),
                };
                match (left, right) {
                    ('.', '[') => {
                        self.move_large_box(direction, x + 1, y_next);
                    }
                    (']', '.') => {
                        self.move_large_box(direction, x, y_next);
                    }
                    ('[', ']') => {
                        self.move_large_box(direction, x, y_next);
                    }
                    (']', '[') => {
                        self.move_large_box(direction, x, y_next);
                        self.move_large_box(direction, x + 1, y_next);
                    }
                    (_, _) => (),
                }
                self.write(x, y, '.');
                self.write(x + 1, y, '.');
                self.write(x, y_next, '[');
                self.write(x + 1, y_next, ']');
            }
            (']', Direction::Up) | (']', Direction::Down) => {
                self.move_large_box(direction, x - 1, y);
            }
            (_, _) => {
                panic!(
                    "Cannot move tile {} in ({}, {}) to the {:?}",
                    box_side, x, y, direction
                )
            }
        };
        true
    }

    pub fn print(&self, robot: &Robot) {
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
    pub fn get_gps(&self, large: bool) -> i32 {
        let target = match large {
            true => '[',
            false => 'O',
        };
        let mut gps = 0;
        for (j, row) in self.map.iter().enumerate() {
            for (i, char) in row.iter().enumerate() {
                if *char == target {
                    gps += (i + 100 * j) as i32;
                }
            }
        }
        gps
    }
}
