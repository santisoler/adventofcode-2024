use crate::directions::Direction;
use crate::robot::Robot;

pub struct Map {
    pub map: Vec<Vec<char>>,
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
    pub fn get_gps(&self) -> i32 {
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
