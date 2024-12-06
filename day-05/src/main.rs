use std::collections::HashMap;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let fname = "data/test_input";
        let result = solve_part1(fname);
        assert_eq!(result, 143);
    }
}

#[derive(Debug)]
pub struct Rules {
    dict: HashMap<i32, Vec<i32>>,
}

impl Rules {
    pub fn new() -> Self {
        let dict: HashMap<i32, Vec<i32>> = HashMap::new();
        Self { dict }
    }

    pub fn add_rule(&mut self, lower: i32, greater: i32) {
        self.dict
            .entry(lower)
            .and_modify(|v| v.push(greater))
            .or_insert(vec![greater]);
    }

    pub fn is_lower(&self, a: i32, b: i32) -> bool {
        match self.dict.get(&a) {
            Some(lower_values) => return lower_values.contains(&b),
            None => match self.dict.get(&b) {
                Some(lower_values) => return !lower_values.contains(&a),
                None => panic!(
                    "Couldn't find a rule to determine ordering of {} and {}",
                    a, b
                ),
            },
        }
    }
}

fn check_update_ordered(update: &Vec<i32>, rules: &Rules) -> bool {
    for values in update.windows(2) {
        let (left, right) = (values[0], values[1]);
        if !rules.is_lower(left, right) {
            return false;
        }
    }
    true
}

fn solve_part1(fname: &str) -> i32 {
    let content = fs::read_to_string(fname).expect("Couldn't read file.");
    let mut lines = content.lines();

    let rules = {
        let mut rules = Rules::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            };
            let mut values = line.split("|");
            let left: i32 = values.next().unwrap().parse().unwrap();
            let right: i32 = values.next().unwrap().parse().unwrap();
            rules.add_rule(left, right);
        }
        rules
    };

    let mut result = 0;
    for line in lines.by_ref() {
        let update: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        if check_update_ordered(&update, &rules) {
            result += update[update.len() / 2];
        };
    }

    result
}

fn main() {
    let fname = "data/input";
    let result = solve_part1(&fname);
    println!("Solution to part 1: {result}");
}
