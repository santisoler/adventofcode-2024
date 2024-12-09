use itertools;
use itertools::Itertools;
use std::fs;
use std::iter;
use std::time::Instant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let fname = "data/test_input";
        let result = solve_part1(fname);
        assert_eq!(result, 3749);
    }
    #[test]
    fn test_part2() {
        let fname = "data/test_input";
        let result = solve_part2(fname);
        assert_eq!(result, 11387);
    }
}

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
    Concat,
}

impl Operator {
    fn operate(&self, x: i64, y: i64) -> i64 {
        match self {
            Operator::Sum => x + y,
            Operator::Product => x * y,
            Operator::Concat => concat(x, y),
        }
    }
}

fn concat(a: i64, b: i64) -> i64 {
    a * 10i64.pow(b.ilog10() + 1) + b
}

fn is_equation_valid(
    expected_result: i64,
    factors: &Vec<i64>,
    operator_types: &Vec<Operator>,
) -> bool {
    let combinations =
        itertools::repeat_n(operator_types.iter(), factors.len() - 1).multi_cartesian_product();
    for operators in combinations {
        // Apply operators
        let result = {
            let mut result = factors[0];
            for (operator, value) in iter::zip(operators, &factors[1..]) {
                result = operator.operate(result, *value);
                // Stop the iterations if the result is already larger than the expected one
                if result > expected_result {
                    break;
                }
            }
            result
        };

        if result == expected_result {
            return true;
        }
    }
    return false;
}

fn solve_part1(fname: &str) -> i64 {
    let operator_types = vec![Operator::Sum, Operator::Product];

    let content = fs::read_to_string(fname).expect("Couldn't read");
    let mut result = 0;
    for line in content.lines() {
        let mut equation = line.split(":");
        let expected_result: i64 = equation.next().unwrap().parse().unwrap();
        let factors: Vec<i64> = equation
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if is_equation_valid(expected_result, &factors, &operator_types) {
            result += expected_result;
        }
    }
    result
}

fn solve_part2(fname: &str) -> i64 {
    let operator_types = vec![Operator::Sum, Operator::Product, Operator::Concat];

    let content = fs::read_to_string(fname).expect("Couldn't read");
    let mut result = 0;
    for line in content.lines() {
        let mut equation = line.split(":");
        let expected_result: i64 = equation.next().unwrap().parse().unwrap();
        let factors: Vec<i64> = equation
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if is_equation_valid(expected_result, &factors, &operator_types) {
            result += expected_result;
        }
    }
    result
}

fn main() {
    let fname = "data/input";
    let result = solve_part1(fname);
    println!("Solution to part 1: {result}");
    let start = Instant::now();
    let result = solve_part2(fname);
    let end = Instant::now();
    println!("Solution to part 2: {result}");
    println!("Elapsed time: {}s", (end - start).as_secs_f64());
}
