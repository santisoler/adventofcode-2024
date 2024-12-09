use itertools::Itertools;
use std::{collections::HashMap, fs};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let fname = "data/test_input";
        let result = solve_part1(fname);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_2() {
        let fname = "data/test_input";
        let result = solve_part2(fname);
        assert_eq!(result, 34);
    }
}

#[derive(Debug, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn is_inside(&self, nrows: i32, ncols: i32) -> bool {
        if (self.x < 0) | (self.y < 0) {
            return false;
        };
        if (self.x >= ncols) | (self.y >= nrows) {
            return false;
        };
        return true;
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn get_antinodes(antenna_a: &Position, antenna_b: &Position) -> (Position, Position) {
    let x_diff = antenna_b.x - antenna_a.x;
    let y_diff = antenna_b.y - antenna_a.y;
    let antinode_1 = Position {
        x: antenna_a.x - x_diff,
        y: antenna_a.y - y_diff,
    };
    let antinode_2 = Position {
        x: antenna_b.x + x_diff,
        y: antenna_b.y + y_diff,
    };
    return (antinode_1, antinode_2);
}

fn get_all_antinodes(
    antenna_a: &Position,
    antenna_b: &Position,
    nrows: i32,
    ncols: i32,
) -> Vec<Position> {
    // Return positions of all antinodes, including the resonant harmonics
    let x_diff = antenna_b.x - antenna_a.x;
    let y_diff = antenna_b.y - antenna_a.y;

    let mut antinodes = vec![];
    let mut i = 0;
    loop {
        let antinode = Position {
            x: antenna_a.x - i * x_diff,
            y: antenna_a.y - i * y_diff,
        };
        if !antinode.is_inside(nrows, ncols) {
            break;
        }
        antinodes.push(antinode);
        i += 1;
    }
    let mut i = 0;
    loop {
        let antinode = Position {
            x: antenna_b.x + i * x_diff,
            y: antenna_b.y + i * y_diff,
        };
        if !antinode.is_inside(nrows, ncols) {
            break;
        }
        antinodes.push(antinode);
        i += 1;
    }
    return antinodes;
}

fn read_antennas(fname: &str) -> (HashMap<char, Vec<Position>>, i32) {
    let content = fs::read_to_string(fname).expect("Couldn't read");
    let ncols = content.lines().nth(0).unwrap().len() as i32;
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    for (row, line) in content.lines().enumerate() {
        for (col, character) in line.chars().enumerate() {
            match character {
                '.' => (),
                _ => {
                    antennas
                        .entry(character)
                        .and_modify(|p| {
                            p.push(Position {
                                x: row as i32,
                                y: col as i32,
                            })
                        })
                        .or_insert(vec![Position {
                            x: row as i32,
                            y: col as i32,
                        }]);
                }
            }
        }
    }
    (antennas, ncols)
}

fn solve_part1(fname: &str) -> i32 {
    let (antennas, ncols) = read_antennas(fname);
    let nrows = ncols; // assume a square

    let mut antinodes: Vec<Position> = vec![];
    for (_, antenna_locations) in antennas.iter() {
        for pair in antenna_locations.iter().combinations(2) {
            let (antinode_1, antinode_2) = get_antinodes(pair[0], &pair[1]);
            if antinode_1.is_inside(nrows, ncols) {
                antinodes.push(antinode_1);
            }
            if antinode_2.is_inside(nrows, ncols) {
                antinodes.push(antinode_2);
            }
        }
    }
    antinodes.iter().unique().count() as i32
}

fn solve_part2(fname: &str) -> i32 {
    let (antennas, ncols) = read_antennas(fname);
    let nrows = ncols; // assume a square

    let mut antinodes: Vec<Position> = vec![];
    for (_, antenna_locations) in antennas.iter() {
        for pair in antenna_locations.iter().combinations(2) {
            antinodes.extend(get_all_antinodes(pair[0], &pair[1], nrows, ncols))
        }
    }
    antinodes.iter().unique().count() as i32
}

fn main() {
    let fname = "data/input";
    let result = solve_part1(fname);
    println!("Solution to part 1: {result}");
    let result = solve_part2(fname);
    println!("Solution to part 2: {result}");
}
