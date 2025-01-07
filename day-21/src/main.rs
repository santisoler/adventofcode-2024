use itertools::{repeat_n, Itertools};
use memoize::memoize;
use std::fs;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, 126384)
    }
}

type Position = (i32, i32);

#[derive(Debug)]
enum Keypad {
    Directional,
    Numeric,
}

impl Keypad {
    fn new_from(code: &str) -> Self {
        let is_numeric = {
            let mut is_numeric = true;
            for c in code.chars() {
                if c != 'A' && !c.is_numeric() {
                    is_numeric = false;
                    break;
                }
            }
            is_numeric
        };
        match is_numeric {
            true => Keypad::Numeric,
            false => Keypad::Directional,
        }
    }

    /// Get all possible sequences to get from the start key to the end key.
    ///
    /// These sequences are all valid (visit only valid keys) and all are the shortest paths to get
    /// from the start key to the end key.
    fn get_partial_sequences(&self, start_key: char, end_key: char) -> Vec<String> {
        // Get start and end positions
        let start = self.get_position(start_key);
        let end = self.get_position(end_key);

        // Find all possible sequences
        let nx = end.0 as i32 - start.0 as i32;
        let ny = end.1 as i32 - start.1 as i32;
        let base_sequences = fill_direction_chars(nx, true) + &fill_direction_chars(ny, false);
        let permutations = base_sequences
            .chars()
            .permutations(base_sequences.len())
            .unique();
        let mut sequences: Vec<String> = permutations
            .map(|p| p.iter().collect::<String>())
            .filter(|sequences| self.is_valid(start_key, sequences))
            .collect();
        for sequence in sequences.iter_mut() {
            sequence.push('A')
        }
        sequences
    }

    /// Check whether a sequence is valid for this keypad.
    fn is_valid(&self, start: char, sequences: &str) -> bool {
        let forbidden = match self {
            Keypad::Numeric => (0, 3),
            Keypad::Directional => (0, 0),
        };
        let mut current_position = self.get_position(start);
        for sequence in sequences.chars() {
            current_position = walk(&current_position, sequence);
            if current_position == forbidden {
                return false;
            };
        }
        true
    }

    fn get_position(&self, key: char) -> Position {
        match self {
            Keypad::Numeric => match key {
                '0' => (1, 3),
                'A' => (2, 3),
                '1' => (0, 2),
                '2' => (1, 2),
                '3' => (2, 2),
                '4' => (0, 1),
                '5' => (1, 1),
                '6' => (2, 1),
                '7' => (0, 0),
                '8' => (1, 0),
                '9' => (2, 0),
                _ => panic!("Invalid key {key} for a numeric pad"),
            },
            Keypad::Directional => match key {
                '^' => (1, 0),
                'A' => (2, 0),
                'v' => (1, 1),
                '<' => (0, 1),
                '>' => (2, 1),
                _ => panic!("Invalid key {key} for a directional pad"),
            },
        }
    }
}

/// Generate a String full of one of the direction characters.
fn fill_direction_chars(displacement: i32, horizontal: bool) -> String {
    let direction = match horizontal {
        true => match displacement.signum() {
            -1 => '<',
            1 => '>',
            0 => '@',
            _ => panic!(),
        },
        false => match displacement.signum() {
            -1 => '^',
            1 => 'v',
            0 => '@',
            _ => panic!(),
        },
    };
    repeat_n(direction, displacement.abs() as usize).collect::<String>()
}

fn walk(position: &Position, sequence: char) -> Position {
    match sequence {
        '>' => (position.0 + 1, position.1),
        '<' => (position.0 - 1, position.1),
        '^' => (position.0, position.1 - 1),
        'v' => (position.0, position.1 + 1),
        e => panic!("Invalid sequence: {e}"),
    }
}

/// Prepend a string with a single character.
fn prepend(code: &str, leading: char) -> String {
    String::from(format!("{leading}")) + code
}

/// Get the shortest sequence we need to input in the directional keypad to produce the desired
/// code.
///
/// The `n_keypads` is the total number of keypads operated by a robot, including the numeric one.
/// So, if we have Keypad -> Keypad -> Keypad -> Numpad, where we can type our sequence in the
/// first one, we have a total of 3 keypads controlled by a robot.
///
/// We memoize this function to significatnly speed up the computation: since there will be a lot
/// of repeated sequences that will be passed to this function in the recursion, it's best to cache
/// the results rather than repeating the computation (which is costly specially when `n_keypads`
/// is a high number).
#[memoize]
fn get_shortest_length(code: String, n_keypads: u32) -> u64 {
    // Generate the keypad based on the code: the code will be numeric, so the first keypad should
    // be also numeric.
    let keypad = Keypad::new_from(&code);
    if n_keypads == 0 {
        return code.len() as u64;
    }
    // Prepend an A to the code: all robots start a code in the A key.
    let code = prepend(&code, 'A');
    // Find shortest length recursively, iterating over pair of chars (moving windows).
    let mut shortest_length = 0;
    for (start, end) in code.chars().tuple_windows() {
        // Get all possible sequences to generate that code
        let sequences = keypad.get_partial_sequences(start, end);
        // Get the minimum length of the sequence at the last keymap (recursively)
        let min_length = sequences
            .iter()
            .map(|sequence| get_shortest_length(sequence.to_string(), n_keypads - 1))
            .min()
            .unwrap();
        // Add the minimum length to the running result
        shortest_length += min_length;
    }
    shortest_length
}

/// Compute the complexity of a given code.
/// The complexity of a code its defined as the product of the lenght of the shortest sequence and
/// the numerical part of the code.
fn get_complexity(code: &str, n_keypads: u32) -> u64 {
    let min_length = get_shortest_length(code.to_string(), n_keypads);
    let numeric_part: u64 = code
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();
    numeric_part * min_length
}

fn solve_part_one(fname: &str) -> u64 {
    let content = fs::read_to_string(fname).unwrap();
    let codes = content.lines().collect::<Vec<&str>>();
    let n_keypads = 3;
    let complexities = codes
        .iter()
        .map(|code| get_complexity(code, n_keypads))
        .sum();
    complexities
}

fn solve_part_two(fname: &str) -> u64 {
    let content = fs::read_to_string(fname).unwrap();
    let codes = content.lines().collect::<Vec<&str>>();
    let n_keypads = 26;
    let complexities = codes
        .iter()
        .map(|code| get_complexity(code, n_keypads))
        .sum();
    complexities
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
    let result = solve_part_two(fname);
    println!("Solution to part two: {result}");
}
