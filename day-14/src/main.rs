use std::collections::HashMap;
use std::fs;

const TILES_X: i32 = 101;
const TILES_Y: i32 = 103;

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
        let x_offset = self.x - TILES_X / 2;
        let y_offset = self.y - TILES_Y / 2;
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
            x: (self.initial.x + self.vx * time).rem_euclid(TILES_X),
            y: (self.initial.y + self.vy * time).rem_euclid(TILES_Y),
        }
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

fn solve_part_one(fname: &str) -> i32 {
    let n_steps = 100;
    let robots = read_file(fname);
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

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
