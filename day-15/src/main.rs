use directions::Direction;
use map::Map;
use robot::Robot;
use std::fs;

mod directions;
mod map;
mod robot;

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
