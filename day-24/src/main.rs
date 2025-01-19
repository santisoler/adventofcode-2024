use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::time::Instant;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_one_01() {
        let fname = "data/test_input_01";
        let result = solve_part_one(fname);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_one_02() {
        let fname = "data/test_input_02";
        let result = solve_part_one(fname);
        assert_eq!(result, 2024);
    }
}

type Rules = HashMap<String, (LogicGate, String, String)>;
type Stack = HashMap<String, bool>;

#[derive(Debug)]
enum LogicGate {
    AND,
    OR,
    XOR,
}

impl fmt::Display for LogicGate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            LogicGate::AND => "AND",
            LogicGate::OR => "OR",
            LogicGate::XOR => "XOR",
        };
        write!(f, "{}", string)
    }
}

impl LogicGate {
    fn new_from(string: &str) -> Self {
        match string {
            "AND" => LogicGate::AND,
            "OR" => LogicGate::OR,
            "XOR" => LogicGate::XOR,
            _ => panic!("Cannot parse '{string}' as a logic gate"),
        }
    }

    fn get_output(&self, a: bool, b: bool) -> bool {
        match self {
            LogicGate::AND => a & b,
            LogicGate::OR => a | b,
            LogicGate::XOR => a ^ b,
        }
    }
}

fn parse_bool(string: &str) -> bool {
    match string {
        "0" => false,
        "1" => true,
        _ => panic!("Cannot parse '{string}' into bool"),
    }
}

fn bools_to_int(bools: Vec<bool>) -> u64 {
    let mut result = 0;
    for (i, bool) in bools.iter().enumerate() {
        if *bool {
            result += 2u64.pow(i as u32)
        }
    }
    result
}

fn read_file(fname: &str) -> (Rules, Stack) {
    let content = fs::read_to_string(fname).unwrap();
    let mut rules = Rules::new();
    let mut stack = Stack::new();
    let mut lines = content.lines();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        };
        let mut parts = line.split(":");
        let variable = parts.next().unwrap().trim().to_string();
        let value = parse_bool(parts.next().unwrap().trim());
        stack.insert(variable, value);
    }
    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break,
        };
        let mut parts = line.split("->");
        let mut operation = parts.next().unwrap().split_whitespace();
        let arg1 = operation.next().unwrap().trim().to_string();
        let logic_gate = LogicGate::new_from(operation.next().unwrap().trim());
        let arg2 = operation.next().unwrap().trim().to_string();
        let output = parts.next().unwrap().trim().to_string();
        rules.insert(output, (logic_gate, arg1, arg2));
    }
    (rules, stack)
}

fn get_value(variable: &str, rules: &Rules, stack: &Stack) -> bool {
    if stack.contains_key(variable) {
        return stack.get(variable).unwrap().to_owned();
    }
    let (logic_gate, arg1, arg2) = rules.get(variable).unwrap();
    let arg1_value = get_value(arg1, rules, stack);
    let arg2_value = get_value(arg2, rules, stack);
    let output = logic_gate.get_output(arg1_value, arg2_value);
    output
}

fn solve_part_one(fname: &str) -> u64 {
    let (rules, stack) = read_file(fname);
    let zetas = {
        let mut zetas: Vec<String> = rules
            .keys()
            .filter(|k| k.starts_with("z"))
            .map(|k| k.to_string())
            .collect();
        zetas.sort();
        // zetas.reverse();
        zetas
    };
    let mut zetas_values: Vec<bool> = vec![];
    for zeta in zetas.iter() {
        let value = get_value(&zeta, &rules, &stack);
        zetas_values.push(value);
    }
    bools_to_int(zetas_values)
}

fn main() {
    let fname = "data/input";
    let start = Instant::now();
    let result = solve_part_one(fname);
    let end = Instant::now();
    println!("Solution to part one: {result}");
    println!("Elapsed time: {}s", (end - start).as_secs_f64());
}
