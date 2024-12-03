use std::fs;
use std::iter;

#[cfg(test)]
mod tests {
    use crate::solve_part1;

    #[test]
    fn test_part1() {
        let fname = "data/test_input";
        let result = solve_part1(fname);
        assert_eq!(result, 2);
    }
}

fn read_file(fname: &str) -> String {
    let content = fs::read_to_string(fname).expect("Couldn't read file");
    return content;
}

#[allow(dead_code)]
fn is_valid_old(report: &Vec<i32>) -> bool {
    // Return true if the report is valid
    let mut prev_diff_sign: i32 = -2;
    for (index, value) in report[..report.len() - 1].iter().enumerate() {
        let diff = report[index + 1] - value;
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if index == 0 {
            prev_diff_sign = diff.signum();
            continue;
        };
        if diff.signum() != prev_diff_sign {
            return false;
        };
    }
    return true;
}

fn is_valid(report: &Vec<i32>) -> bool {
    // Return true if the report is valid

    // Define diffs as an iterator
    let diffs = iter::zip(&report[..report.len() - 1], &report[1..])
        .map(|(value, next)| next - value)
        .into_iter();

    // Iterate over the other diffs
    let mut prev_diff: Option<i32> = None;
    for diff in diffs {
        match diff.abs() {
            1..=3 => (),
            _ => return false,
        };
        match prev_diff {
            Some(v) => {
                if diff.signum() != v.signum() {
                    return false;
                }
            }
            None => (),
        }
        prev_diff = Some(diff);
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

fn main() {
    let fname = "data/input";
    let result = solve_part1(fname);
    println!("Solution to part 1: {result}")
}
