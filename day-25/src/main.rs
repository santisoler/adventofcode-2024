use std::fs;
use std::str::Lines;

const TUMBLER_HEIGHT: u32 = 7;
const COMBINATION_LENGTH: usize = 5;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, 3);
    }
}

#[derive(Debug)]
enum Tumbler {
    Lock {
        combination: [u32; COMBINATION_LENGTH],
    },
    Key {
        combination: [u32; COMBINATION_LENGTH],
    },
}

impl Tumbler {
    fn new_from(block: &Vec<&str>) -> Self {
        let is_lock = block[0].contains("#");
        let target_char = match is_lock {
            true => '#',
            false => '.',
        };
        let mut combination: [u32; COMBINATION_LENGTH] = [0; COMBINATION_LENGTH];
        for (row, line) in block.iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                if char == target_char {
                    if is_lock {
                        combination[col] = row as u32;
                    } else {
                        combination[col] = TUMBLER_HEIGHT - row as u32 - 2;
                    }
                }
            }
        }
        match is_lock {
            true => Tumbler::Lock { combination },
            false => Tumbler::Key { combination },
        }
    }

    fn is_lock_compatible(&self, key: &Tumbler) -> bool {
        if let Tumbler::Key { combination: _ } = self {
            panic!()
        };
        if let Tumbler::Lock { combination: _ } = key {
            panic!()
        };
        let result = match (self, key) {
            (Tumbler::Key { combination: _ }, _) => panic!(),
            (_, Tumbler::Lock { combination: _ }) => panic!(),
            (
                Tumbler::Lock {
                    combination: lock_combination,
                },
                Tumbler::Key {
                    combination: key_combination,
                },
            ) => {
                let sum = add(&lock_combination, key_combination);
                sum.iter().all(|&x| x <= TUMBLER_HEIGHT - 2)
            }
        };
        result
    }
}

fn is_lock(tumbler: &Tumbler) -> bool {
    match tumbler {
        Tumbler::Lock { combination: _ } => true,
        Tumbler::Key { combination: _ } => false,
    }
}

/// Add two arrays element-wise
fn add(a: &[u32; COMBINATION_LENGTH], b: &[u32; COMBINATION_LENGTH]) -> [u32; COMBINATION_LENGTH] {
    let mut result = [0; COMBINATION_LENGTH];
    for i in 0..COMBINATION_LENGTH {
        result[i] = a[i] + b[i]
    }
    result
}

fn parse_block<'a>(lines: &mut Lines<'a>) -> Option<Vec<&'a str>> {
    let mut block = vec![];
    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break,
        };
        if line.is_empty() {
            break;
        };
        block.push(line);
    }
    if block.is_empty() {
        return None;
    }
    Some(block)
}

fn parse_file(fname: &str) -> Vec<Tumbler> {
    let content = fs::read_to_string(fname).unwrap();
    let mut lines = content.lines();
    let mut tumblers = vec![];
    loop {
        let block = match parse_block(&mut lines) {
            Some(block) => block,
            None => break,
        };
        tumblers.push(Tumbler::new_from(&block));
    }
    tumblers
}

fn solve_part_one(fname: &str) -> u32 {
    let tumblers = parse_file(fname);
    let mut n_fits = 0;
    for lock in tumblers.iter().filter(|t| is_lock(t)) {
        for key in tumblers.iter().filter(|t| !is_lock(t)) {
            if lock.is_lock_compatible(key) {
                n_fits += 1;
            }
        }
    }
    n_fits
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
