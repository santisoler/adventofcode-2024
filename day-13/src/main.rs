use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const TOKENS_A: i32 = 3;
const TOKENS_B: i32 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, 480);
    }
}

struct Button {
    x: i32,
    y: i32,
}

struct Prize {
    x: i32,
    y: i32,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_button_line(line: String) -> Button {
    let units: Vec<i32> = line
        .split(":")
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.trim().split("+").last().unwrap().parse().unwrap())
        .collect();
    Button {
        x: units[0],
        y: units[1],
    }
}

fn read_prize_line(line: String) -> Prize {
    let units: Vec<i32> = line
        .split(":")
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.trim().split("=").last().unwrap().parse().unwrap())
        .collect();
    Prize {
        x: units[0],
        y: units[1],
    }
}

// Count the minimum number of tokens needed to get the prize
fn count_tokens(a_button: Button, b_button: Button, prize: Prize) -> Option<i32> {
    // Solve the linear equations system to get the number of presses of each button needed to get
    // the prize.
    let det = a_button.x * b_button.y - b_button.x * a_button.y;
    let mut a_presses = prize.x * b_button.y - b_button.x * prize.y;
    let mut b_presses = -prize.x * a_button.y + a_button.x * prize.y;
    // Check if it's possible to get the prize on that machine
    if (a_presses % det != 0) | (b_presses % det != 0) {
        return None;
    }
    a_presses /= det;
    b_presses /= det;
    // If we need to press buttons negative times, the machine doesn't have a solution
    if (a_presses < 0) | (b_presses < 0) {
        return None;
    }
    // Limit the amount of times we can press each button
    if (a_presses > 100) | (b_presses > 100) {
        return None;
    }
    let tokens = TOKENS_A * a_presses + TOKENS_B * b_presses;
    Some(tokens)
}

fn solve_part_one(fname: &str) -> i32 {
    let mut lines = match read_lines(fname) {
        Ok(lines) => lines.flatten(),
        Err(e) => panic!("{e}"),
    };
    let mut result = 0;
    loop {
        let a_button = match lines.next() {
            Some(line) => read_button_line(line),
            None => break,
        };
        let b_button = match lines.next() {
            Some(line) => read_button_line(line),
            None => break,
        };
        let prize = match lines.next() {
            Some(line) => read_prize_line(line),
            None => break,
        };
        if let Some(tokens) = count_tokens(a_button, b_button, prize) {
            result += tokens
        };
        if let None = lines.next() {
            break;
        }
    }
    result
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
