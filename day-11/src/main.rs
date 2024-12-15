use std::collections::HashMap;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, 55312);
    }
}

enum Stone {
    Single(u64),
    Pair(u64, u64),
}

struct Counts {
    counts: HashMap<u64, u64>,
}

impl Counts {
    fn new(stones: Vec<u64>) -> Self {
        let mut counts: HashMap<u64, u64> = HashMap::new();
        for stone in stones {
            counts.entry(stone).and_modify(|c| *c += 1).or_insert(1);
        }
        return Self { counts };
    }

    fn blink(&mut self) {
        let mut counts: HashMap<u64, u64> = HashMap::new();
        for (stone, quantity) in self.counts.iter() {
            match blink_stone(*stone) {
                Stone::Single(new_stone) => {
                    counts
                        .entry(new_stone)
                        .and_modify(|c| *c += quantity)
                        .or_insert(*quantity);
                }
                Stone::Pair(new_stone_1, new_stone_2) => {
                    counts
                        .entry(new_stone_1)
                        .and_modify(|c| *c += quantity)
                        .or_insert(*quantity);
                    counts
                        .entry(new_stone_2)
                        .and_modify(|c| *c += quantity)
                        .or_insert(*quantity);
                }
            }
        }
        self.counts = counts;
    }

    fn count_stones(&self) -> u64 {
        self.counts.values().sum()
    }
}

fn count_digits(integer: u64) -> u64 {
    if integer == 0 {
        return 1;
    };
    let mut n_digits = 1;
    let mut tmp = integer;
    loop {
        tmp /= 10;
        if tmp == 0 {
            break;
        }
        n_digits += 1;
    }
    return n_digits;
}

fn blink_stone(stone: u64) -> Stone {
    // Blink a single stone
    if stone == 0 {
        return Stone::Single(1);
    };
    let n_digits = count_digits(stone);
    if n_digits % 2 == 0 {
        let left = stone / u64::pow(10, n_digits as u32 / 2);
        let right = stone - left * u64::pow(10, n_digits as u32 / 2);
        return Stone::Pair(left, right);
    } else {
        return Stone::Single(stone * 2024);
    };
}

fn read_file(fname: &str) -> Vec<u64> {
    let content = fs::read_to_string(fname).expect("Couldn't read");
    let stones = content
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    stones
}

fn solve_part_one(fname: &str) -> u64 {
    let n_blinks = 25;
    let stones = read_file(fname);
    let mut counts = Counts::new(stones);
    for _ in 0..n_blinks {
        counts.blink()
    }
    counts.count_stones()
}

fn solve_part_two(fname: &str) -> u64 {
    let n_blinks = 75;
    let stones = read_file(fname);
    let mut counts = Counts::new(stones);
    for _ in 0..n_blinks {
        counts.blink()
    }
    counts.count_stones()
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
    let result = solve_part_two(fname);
    println!("Solution to part two: {result}");
}
