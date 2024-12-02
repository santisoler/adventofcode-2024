use std::collections::HashMap;
use std::fs;
use std::iter;

#[cfg(test)]
mod tests {
    use crate::{solve_part1, solve_part2};

    #[test]
    fn test_part1() {
        let fname = "data/test_input";
        let result = solve_part1(fname.to_string());
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2() {
        let fname = "data/test_input";
        let result = solve_part2(fname.to_string());
        assert_eq!(result, 31);
    }
}

fn read_file(fname: String) -> (Vec<i32>, Vec<i32>) {
    let content = fs::read_to_string(fname).expect("Couldn't read file");
    let (left, right) = {
        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();
        for line in content.lines() {
            let line_vec: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().expect("error"))
                .collect();
            left.push(line_vec[0]);
            right.push(line_vec[1]);
        }
        (left, right)
    };
    return (left, right);
}

fn solve_part1(fname: String) -> i32 {
    let (mut left, mut right) = read_file(fname);
    left.sort();
    right.sort();
    let result = iter::zip(left, right).map(|(x, y)| (x - y).abs()).sum();
    return result;
}

fn solve_part2(fname: String) -> i32 {
    let (left, right) = read_file(fname);
    let counts: HashMap<i32, i32> = {
        let mut counts = HashMap::new();
        for element in right {
            counts.entry(element).and_modify(|x| *x += 1).or_insert(1);
        }
        counts
    };
    let result = left
        .iter()
        .map(|x| match counts.get(&x) {
            Some(v) => x * v,
            None => 0,
        })
        .sum();
    result
}

fn main() {
    let fname = "data/input";
    let solution = solve_part1(fname.to_string());
    println!("Solution to part 1: {solution}");
    let solution = solve_part2(fname.to_string());
    println!("Solution to part 2: {solution}");
}
