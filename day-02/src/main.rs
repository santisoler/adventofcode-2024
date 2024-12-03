use std::fs;

#[cfg(test)]
mod tests {
    use crate::{solve_part1, solve_part2};

    #[test]
    fn test_part1() {
        let fname = "data/test_input";
        let result = solve_part1(fname);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let fname = "data/test_input";
        let result = solve_part2(fname);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2_custom_file() {
        let fname = "data/test_input_2";
        let result = solve_part2(fname);
        assert_eq!(result, 6);
    }
}

fn read_file(fname: &str) -> String {
    let content = fs::read_to_string(fname).expect("Couldn't read file");
    return content;
}

fn is_valid(report: &Vec<i32>) -> bool {
    let mut prev_diff: Option<i32> = None;
    for i in 0..report.len() - 1 {
        if !is_level_valid(report[i], report[i + 1], &mut prev_diff) {
            return false;
        }
    }
    return true;
}

fn is_level_valid(this: i32, next: i32, prev_diff: &mut Option<i32>) -> bool {
    // Check if a single level (step between two values) is valid.
    //
    // Can optionally take a previous difference.
    let diff = next - this;
    if let Some(x) = prev_diff {
        if diff.signum() != x.signum() {
            return false;
        };
    };
    if (diff.abs() < 1) | (diff.abs() > 3) {
        return false;
    };
    *prev_diff = Some(diff);
    return true;
}

fn is_valid_with_tolerance(report: &Vec<i32>) -> bool {
    // Return true if the report is valid.
    //
    // Allow one bad level as tolerance.
    let mut prev_diff: Option<i32> = None;
    for i in 0..report.len() - 1 {
        if !is_level_valid(report[i], report[i + 1], &mut prev_diff) {
            let min_value = if i == 0 { i } else { i - 1 };
            for j in min_value..=i + 1 {
                let mut new_report = report.clone();
                new_report.remove(j);
                if is_valid(&new_report) {
                    return true;
                }
            }
            return false;
        }
    }
    return true;
}

fn solve_part1(fname: &str) -> i32 {
    let content = read_file(fname);
    let mut result = 0;
    for line in content.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Couldn't convert to integer."))
            .collect();
        result += is_valid(&report) as i32;
    }
    return result;
}

fn solve_part2(fname: &str) -> i32 {
    let content = read_file(fname);
    let mut result = 0;
    for line in content.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Couldn't convert to integer."))
            .collect();
        let blah = is_valid_with_tolerance(&report) as i32;
        result += blah;
    }
    return result;
}

fn main() {
    let fname = "data/input";
    let result = solve_part1(fname);
    println!("Solution to part 1: {result}");
    let result = solve_part2(fname);
    println!("Solution to part 2: {result}");
}
