use std::fs;

#[cfg(test)]
mod tests {
    // use crate::{solve_part1, solve_part2};
    use crate::parse_line;
    use crate::solve_part1;

    #[test]
    fn test_part1() {
        let fname = "data/test_input";
        let result = solve_part1(fname);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("alkjmul(3,2)"), 6);
        assert_eq!(parse_line("xmul(2,4)%"), 8);
        assert_eq!(parse_line("xmul(2,4)%mul(5,6)"), 8 + 30);
        assert_eq!(parse_line("do_not_mul(5,5)3"), 25);
        assert_eq!(parse_line("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)3"), 8 + 25);
        assert_eq!(
            parse_line("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            2 * 4 + 5 * 5 + 11 * 8 + 8 * 5
        );
    }
}

fn find_argument(string: &str, last_char: char) -> Option<usize> {
    // Find numeric argument until a given last_char is encountered
    for (i, c) in string.chars().enumerate() {
        if c.is_numeric() {
            continue;
        }
        if c == last_char {
            return Some(i);
        } else {
            return None;
        }
    }
    return None;
}

fn find_arguments(string: &str) -> Option<(usize, usize)> {
    let comma_index: usize = match find_argument(&string, ',') {
        Some(comma_index) => comma_index,
        None => return None,
    };
    let end_index: usize = match find_argument(&string[comma_index + 1..], ')') {
        Some(end_index) => end_index + comma_index + 1,
        None => return None,
    };
    return Some((comma_index, end_index));
}

fn parse_line(string: &str) -> i32 {
    let mut string = string;
    let mut result = 0;
    loop {
        match string.find("mul(") {
            Some(index) => string = &string[index + 4..],
            None => break,
        };
        let (comma, end) = match find_arguments(&string) {
            Some(indices) => indices,
            None => continue,
        };
        let first: i32 = string[..comma].parse().unwrap();
        let second: i32 = string[comma + 1..end].parse().unwrap();
        result += first * second;
        string = &string[end + 1..];
    }
    result
}

fn solve_part1(fname: &str) -> i32 {
    let content = fs::read_to_string(fname).expect("Couldn't read!");
    let mut result = 0;
    for line in content.lines() {
        result += parse_line(&line);
    }
    return result;
}

fn main() {
    let fname = "data/input";
    let result = solve_part1(fname);
    println!("Solution to part 1: {result}");
    // let result = solve_part2(fname);
    // println!("Solution to part 2: {result}");
}
