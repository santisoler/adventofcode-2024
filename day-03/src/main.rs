use regex::Regex;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let fname = "data/test_input";
        let result = solve_part1(fname);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part2() {
        let fname = "data/test_input_2";
        let result = solve_part2(fname);
        assert_eq!(result, 48);
    }

    #[test]
    fn test_parse_and_execute() {
        let result = parse_and_execute("ajlfasdmul(5,4)alksjf");
        assert_eq!(result, 20);
    }

    #[test]
    fn test_parse_and_execute_with_do() {
        let result =
            parse_and_execute_with_do("ajlfasdmul(5,4)alkdon't()sjmul(3,4)lajdon't()ajdmul(3,9)f");
        assert_eq!(result, 20);
    }
}

fn parse_and_execute(code: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let result = {
        let mut result = 0;
        for capture in re.captures_iter(code) {
            let first: i32 = (&capture[1]).parse().expect("Couldn't parse");
            let second: i32 = (&capture[2]).parse().expect("Couldn't parse");
            result += first * second
        }
        result
    };
    result
}

fn parse_and_execute_with_do(line: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|don't\(\)|do\(\)").unwrap();
    let result = {
        let mut result = 0;
        let mut run = true;
        for capture in re.captures_iter(line) {
            if (&capture[0]).eq("do()") {
                run = true;
                continue;
            }
            if (&capture[0]).eq("don't()") {
                run = false;
                continue;
            }
            if run {
                let first: i32 = (&capture[1]).parse().expect("Couldn't parse");
                let second: i32 = (&capture[2]).parse().expect("Couldn't parse");
                result += first * second
            }
        }
        result
    };
    result
}

pub fn solve_part1(fname: &str) -> i32 {
    let content = fs::read_to_string(fname).expect("Couldn't read!");
    let result = parse_and_execute(&content);
    result
}

pub fn solve_part2(fname: &str) -> i32 {
    let content = fs::read_to_string(fname).expect("Couldn't read!");
    let result = parse_and_execute_with_do(&content);
    result
}

fn main() {
    let fname = "data/input";
    let result = solve_part1(fname);
    println!("Solution to part 1: {result}");
    let result = solve_part2(fname);
    println!("Solution to part 2: {result}");
}
