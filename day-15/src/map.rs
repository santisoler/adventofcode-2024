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

    // Try to move a large box to a given direction.
    // If the box was moved, return true. Else return false.
    pub fn move_large_box(&mut self, direction: &Direction, x: i32, y: i32) -> bool {
        let box_side = match self.get(x, y) {
            Some(c) => c,
            None => panic!("Invalid location: {}, {}", x, y),
        };
        match (box_side, direction) {
            ('[', Direction::Right) | (']', Direction::Left) => {
                let (x_next, y_next) = direction.get_next_position(x, y);
                let moved = self.move_large_box(direction, x_next, y_next);
                if moved {
                    self.write(x, y, '.');
                    self.write(x_next, y_next, box_side);
                }
                return moved;
            }
            (']', Direction::Right) | ('[', Direction::Left) => {
                let (x_next, y_next) = direction.get_next_position(x, y);
                match self.get(x_next, y_next) {
                    Some('#') => return false,
                    Some('[') | Some(']') => {
                        let moved = self.move_large_box(direction, x_next, y_next);
                        if moved {
                            self.write(x_next, y_next, box_side);
                        }
                        return moved;
                    }
                    Some('.') => {
                        self.write(x_next, y_next, box_side);
                        return true;
                    }
                    Some(e) => panic!("invalid char: {}", e),
                    None => panic!("invalid location: {}, {}", x_next, y_next),
                }
            }
            (']', Direction::Up) | (']', Direction::Down) => {
                // let the next branch decide the vertical movement of the box
                return self.move_large_box(direction, x - 1, y);
            }
            ('[', Direction::Up) | ('[', Direction::Down) => {
                let (x_left, y_left) = direction.get_next_position(x, y);
                let (x_right, y_right) = direction.get_next_position(x + 1, y);
                let left = match self.get(x_left, y_left) {
                    Some(c) => c,
                    None => panic!("invalid location: {}, {}", x_left, y_left),
                };
                let right = match self.get(x_right, y_right) {
                    Some(c) => c,
                    None => panic!("invalid location: {}, {}", x_right, y_right),
                };
                match (left, right) {
                    ('#', _) | (_, '#') => return false,
                    ('[', ']') => {
                        let moved = self.move_large_box(direction, x_left, y_left);
                        if moved {
                            self.write(x, y, '.');
                            self.write(x + 1, y, '.');
                            self.write(x_left, y_left, '[');
                            self.write(x_right, y_right, ']');
                        }
                        return moved;
                    }
                    (']', '[') => {
                        let moved_left = self.move_large_box(direction, x_left, y_left);
                        let moved_right = self.move_large_box(direction, x_right, y_right);
                        let moved = moved_left && moved_right;
                        if moved {
                            self.write(x, y, '.');
                            self.write(x + 1, y, '.');
                            self.write(x_left, y_left, '[');
                            self.write(x_right, y_right, ']');
                        }
                        return moved;
                    }
                    (']', '.') => {
                        let moved_left = self.move_large_box(direction, x_left, y_left);
                        if moved_left {
                            self.write(x, y, '.');
                            self.write(x + 1, y, '.');
                            self.write(x_left, y_left, '[');
                            self.write(x_right, y_right, ']');
                        }
                        return moved_left;
                    }
                    ('.', '[') => {
                        let moved_right = self.move_large_box(direction, x_right, y_right);
                        if moved_right {
                            self.write(x, y, '.');
                            self.write(x + 1, y, '.');
                            self.write(x_left, y_left, '[');
                            self.write(x_right, y_right, ']');
                        }
                        return moved_right;
                    }
                    ('.', '.') => {
                        self.write(x, y, '.');
                        self.write(x + 1, y, '.');
                        self.write(x_left, y_left, '[');
                        self.write(x_right, y_right, ']');
                        return true;
                    }
                    (l, r) => panic!("invalid chars: {}, {}", l, r),
                }
            }
            (_, _) => panic!(
                "Tried to move a tile that doesn't have a box: {} {} {}",
                x, y, box_side
            ),
        }
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
