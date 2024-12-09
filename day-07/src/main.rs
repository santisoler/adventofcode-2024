use itertools;
use itertools::Itertools;
use std::fs;
use std::iter;

const OPERATORS: [Operator; 2] = [Operator::Sum, Operator::Product];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let fname = "data/test_input";
        let result = solve_part1(fname);
        assert_eq!(result, 3749);
    }
}

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
}

impl Operator {
    fn operate(&self, x: i64, y: i64) -> i64 {
        match self {
            Operator::Sum => x + y,
            Operator::Product => x * y,
        }
    }
}

fn apply_operators(operators: &Vec<&Operator>, values: &Vec<i64>) -> i64 {
    let mut result = values[0];
    for (operator, value) in iter::zip(operators, &values[1..]) {
        result = operator.operate(result, *value);
    }
    result
}

fn is_equation_valid(expected_result: i64, factors: &Vec<i64>) -> bool {
    let combinations =
        itertools::repeat_n(OPERATORS.iter(), factors.len() - 1).multi_cartesian_product();
    for operators in combinations {
        if apply_operators(&operators, &factors) == expected_result {
            return true;
        }
    }
    return false;
}

fn solve_part1(fname: &str) -> i64 {
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

        if is_equation_valid(expected_result, &factors) {
            result += expected_result;
        }
    }
    result
}
fn main() {
    let fname = "data/input";
    let result = solve_part1(fname);
    println!("Solution to part 1: {result}");
}
