use std::fs;
use std::iter;

#[cfg(test)]
mod tests {
    use crate::solve_part1;

    #[test]
    fn test_part1() {
        let fname = "data/test_input";
        let result = solve_part1(fname.to_string());
        assert_eq!(result, 11);
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
    let result = {
        let mut result = 0;
        for (l, r) in iter::zip(left, right) {
            result += (r - l).abs();
        }
        result
    };
    return result;
}

fn main() {
    let fname = "data/input";
    let solution = solve_part1(fname.to_string());
    println!("Solution to part 1: {solution}");
}
