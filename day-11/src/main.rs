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

fn blink_once(stone: u64) -> Stone {
    if stone == 0 {
        return Stone::Single(1);
    };
    let n_digits = count_digits(stone);
    if n_digits % 2 == 0 {
        let left = stone / u64::pow(10, n_digits / 2);
        let right = stone - left * u64::pow(10, n_digits / 2);
        return Stone::Pair(left, right);
    } else {
        return Stone::Single(stone * 2024);
    };
}

fn count_digits(integer: u64) -> u32 {
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

fn count_stones(stone: u64, n_blinks: u64) -> u64 {
    // Count how many stones are generated after `n_blinks`.
    if n_blinks == 0 {
        return 1;
    };
    let result = match blink_once(stone) {
        Stone::Single(s) => count_stones(s, n_blinks - 1),
        Stone::Pair(s1, s2) => [s1, s2]
            .iter()
            .map(|s| count_stones(*s, n_blinks - 1))
            .sum(),
    };
    result
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
    let stones = read_file(fname);
    let n_stones = stones.iter().map(|s| count_stones(*s, 25)).sum();
    n_stones
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
