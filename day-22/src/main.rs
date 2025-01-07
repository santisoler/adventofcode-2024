use std::fs;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, 37327623);
    }
}

const PRUNER: u64 = 16777216; // this is eq to 0b100000... (with 24 zeros)

fn next_secret_number(secret_number: u64) -> u64 {
    let mut next_secret_number = ((secret_number * 64) ^ secret_number) % PRUNER;
    next_secret_number = ((next_secret_number / 32) ^ next_secret_number) % PRUNER;
    next_secret_number = ((next_secret_number * 2048) ^ next_secret_number) % PRUNER;
    next_secret_number
}

fn predict_nth_secret_number(secret_number: u64, nth: usize) -> u64 {
    let mut secret_number = secret_number;
    for _ in 0..nth {
        secret_number = next_secret_number(secret_number);
    }
    secret_number
}

fn solve_part_one(fname: &str) -> u64 {
    let content = fs::read_to_string(fname).unwrap();
    let initial_secret_numbers: Vec<u64> = content
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    initial_secret_numbers
        .iter()
        .map(|x| predict_nth_secret_number(*x, 2_000))
        .sum()
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
