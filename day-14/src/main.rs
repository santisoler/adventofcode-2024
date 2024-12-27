use std::collections::HashMap;
use std::fmt;
use std::fs;

const TILES_X: usize = 101;
const TILES_Y: usize = 103;
const TARGET_CONSECUTIVE_ROBOTS: i32 = 30;

// Use these to test the test_input file
// const TILES_X: i32 = 11;
// const TILES_Y: i32 = 7;

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    // Return the quadrant of the position
    fn get_quadrant(&self) -> Option<i32> {
        let x_offset = self.x - TILES_X as i32 / 2;
        let y_offset = self.y - TILES_Y as i32 / 2;
        match (x_offset, y_offset) {
            (0, _) => None,
            (_, 0) => None,
            (1.., 1..) => Some(1),
            (..0, 1..) => Some(2),
            (..0, ..0) => Some(3),
            (1.., ..0) => Some(4),
        }
    }
}

struct Robot {
    initial: Position,
    vx: i32,
    vy: i32,
}

impl Robot {
    // // Return the position of the robot after a given time
    fn get_position(&self, time: i32) -> Position {
        Position {
            x: (self.initial.x + self.vx * time).rem_euclid(TILES_X as i32),
            y: (self.initial.y + self.vy * time).rem_euclid(TILES_Y as i32),
        }
    }
}

struct Map {
    map: [[char; TILES_X]; TILES_Y],
}

impl Map {
    fn new() -> Self {
        let map = [['.'; TILES_X]; TILES_Y];
        Map { map }
    }

    fn new_from(positions: &Vec<Position>) -> Self {
        let mut map = Self::new();
        map.fill(positions);
        map
    }

    fn clean(&mut self) {
        for i in 0..TILES_Y {
            for j in 0..TILES_X {
                self.map[i][j] = '.'
            }
        }
    }

    fn fill(&mut self, positions: &Vec<Position>) {
        self.clean();
        for position in positions.iter() {
            self.map[position.y as usize][position.x as usize] = '#';
        }
    }

    // Count the maximum number of consecutive robots in any row
    fn get_max_consecutive_robots(&self) -> i32 {
        let max_consecutive_robots = self
            .map
            .iter()
            .map(|row| Self::_max_consecutive_robots_in_row(row))
            .max()
            .unwrap();
        max_consecutive_robots
    }

    // Count maximum consecutive robots in a single row
    fn _max_consecutive_robots_in_row(row: &[char; TILES_X]) -> i32 {
        let (mut max_consecutive, mut tmp) = (0, 0);
        for char in row {
            match char {
                '#' => tmp += 1,
                _ => {
                    if tmp > max_consecutive {
                        max_consecutive = tmp
                    }
                    tmp = 0;
                }
            }
        }
        // Check by the end of the row
        if tmp > max_consecutive {
            max_consecutive = tmp
        }
        max_consecutive
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = self
            .map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", text)
    }
}

fn read_line(line: &str) -> Robot {
    let mut parts = line.split_whitespace();
    // let p: Vec<&str> = parts.next().unwrap().replace("p=", "").split(",").collect();
    let position: Vec<i32> = parts
        .next()
        .unwrap()
        .replace("p=", "")
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let velocity: Vec<i32> = parts
        .next()
        .unwrap()
        .replace("v=", "")
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    Robot {
        initial: Position {
            x: position[0],
            y: position[1],
        },
        vx: velocity[0],
        vy: velocity[1],
    }
}

fn read_file(fname: &str) -> Vec<Robot> {
    let content = fs::read_to_string(fname).unwrap();
    let robots = content.lines().map(|line| read_line(line)).collect();
    robots
}

fn get_safety_factor(robots: &Vec<Robot>, n_steps: i32) -> i32 {
    let mut robots_per_quadrant: HashMap<i32, i32> = HashMap::new();
    for robot in robots {
        let position = robot.get_position(n_steps);
        if let Some(quadrant) = position.get_quadrant() {
            robots_per_quadrant
                .entry(quadrant)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        };
    }
    let safety_factor = robots_per_quadrant.values().product();
    safety_factor
}

fn solve_part_one(fname: &str) -> i32 {
    let n_steps = 100;
    let robots = read_file(fname);
    get_safety_factor(&robots, n_steps)
}

fn solve_part_two(fname: &str) -> i32 {
    let robots = read_file(fname);
    let mut easter_egg_time = 0;
    for time in 0..100_000 {
        let positions: Vec<Position> = robots.iter().map(|r| r.get_position(time)).collect();
        let map = Map::new_from(&positions);
        let max_consecutive_robots = map.get_max_consecutive_robots();
        if max_consecutive_robots > TARGET_CONSECUTIVE_ROBOTS {
            println!("{map}");
            easter_egg_time = time;
            break;
        }
    }
    easter_egg_time
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
    let result = solve_part_two(fname);
    println!("Solution to part two: {result}");
}
