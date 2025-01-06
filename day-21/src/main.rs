use itertools::{repeat_n, Itertools};
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

    /// Get all possible sequences to type a full code.
    ///
    /// These sequences are all valid (visit only valid keys) and all are the shortest paths to get
    /// to enter the code. We assume that the initial possition is at `A`, regardless of the type
    /// of keypad.
    fn get_full_sequences(&self, code: &str) -> Vec<String> {
        let mut sequences = self.get_partial_sequences('A', code.chars().next().unwrap());
        for (start, end) in code.chars().tuple_windows() {
            let next_sequences = self.get_partial_sequences(start, end);
            let mut sequences_tmp = vec![];
            for mov in sequences.iter() {
                for next in next_sequences.iter() {
                    sequences_tmp.push(mov.clone() + next)
                }
            }
            sequences = sequences_tmp;
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

fn get_sequences(start: char, end: char, keypads: &[Keypad]) -> Vec<String> {
    // Get first inner keypad
    let keypad = &keypads[0];
    // Handle when we found the innermost keypad
    if keypads.len() == 1 {
        return keypad.get_partial_sequences(start, end);
    };
    let mut sequences = vec![];
    for inner_sequences in get_sequences(start, end, &keypads[1..]) {
        sequences.extend(keypad.get_full_sequences(&inner_sequences))
    }
    sequences
}

/// Compute the length of the shortest sequence by splitting the code in pairs.
fn get_length_of_shortest_sequence(code: &str, keypads: &[Keypad]) -> u32 {
    let mut length = 0 as u32;
    // Any sequence starts with the initial position in A, so first we need to
    // move from A to the first numeric key in the code.
    let sequences = get_sequences('A', code.chars().next().unwrap(), keypads);
    length += sequences.iter().map(|s| s.len() as u32).min().unwrap();
    // Do the same for any pair of characters in the code (as moving windows)
    for (start, end) in code.chars().tuple_windows() {
        let sequences = get_sequences(start, end, keypads);
        length += sequences.iter().map(|s| s.len() as u32).min().unwrap();
    }
    length
}

/// Compute the complexity of a given code.
/// The complexity of a code its defined as the product of the lenght of the shortest sequence and
/// the numerical part of the code.
fn get_complexity(code: &str, keypads: &[Keypad]) -> u32 {
    let min_length = get_length_of_shortest_sequence(code, keypads);
    let numeric_part: u32 = code
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();
    numeric_part * min_length
}

fn solve_part_one(fname: &str) -> u32 {
    let content = fs::read_to_string(fname).unwrap();
    let codes = content.lines().collect::<Vec<&str>>();
    let keypads = [Keypad::Directional, Keypad::Directional, Keypad::Numeric];
    let complexities = codes
        .iter()
        .map(|code| get_complexity(code, &keypads))
        .sum();
    complexities
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
