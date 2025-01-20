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

    #[test]
    fn test_int_to_bools() {
        let result = int_to_bools(0b1100110);
        assert_eq!(result, vec![false, true, true, false, false, true, true]);
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

fn bools_to_int(bools: &Vec<bool>) -> u64 {
    let mut result = 0;
    for (i, bool) in bools.iter().enumerate() {
        if *bool {
            result += 2u64.pow(i as u32)
        }
    }
    result
}

fn int_to_bools(integer: u64) -> Vec<bool> {
    let mut bools = vec![];
    let mut tmp = integer;
    while tmp != 0 {
        // Get last significant bit with an AND operator
        let bit = match tmp & 1 {
            1 => true,
            0 => false,
            _ => panic!(),
        };
        tmp >>= 1; // shift the integer one bit to the left (remove the last significant one)
        bools.push(bit);
    }
    bools
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
        zetas
    };
    let mut zetas_values: Vec<bool> = vec![];
    for zeta in zetas.iter() {
        let value = get_value(&zeta, &rules, &stack);
        zetas_values.push(value);
    }
    bools_to_int(&zetas_values)
}

fn _get_var(stack: &Stack, starts_with: &str) -> u64 {
    let vars = {
        let mut vars_names: Vec<String> = stack
            .keys()
            .filter(|k| k.starts_with(starts_with))
            .map(|k| k.to_string())
            .collect();
        vars_names.sort();
        let x: Vec<bool> = vars_names.iter().map(|x| *stack.get(x).unwrap()).collect();
        x
    };
    bools_to_int(&vars)
}

fn get_x_and_y(stack: &Stack) -> (u64, u64) {
    (_get_var(stack, "x"), _get_var(stack, "y"))
}

fn solve_part_two(fname: &str) {
    let (rules, stack) = read_file(fname);
    let (x, y) = get_x_and_y(&stack);
    let expected_z = int_to_bools(x + y);
    let (x, y) = (int_to_bools(x), int_to_bools(y));
    for i in 0..x.len() {
        let z_variable = format!("z{:02}", i);
        let z = get_value(&z_variable, &rules, &stack);
        println!(
            "{i}: xi: {}, yi: {}, zi: {}, expected_zi: {}",
            x[i], y[i], z, expected_z[i]
        );
        if z != expected_z[i] {
            println!("{:?}", rules.get(&z_variable));
        }
    }
}

fn main() {
    let fname = "data/input";
    let start = Instant::now();
    let result = solve_part_one(fname);
    let end = Instant::now();
    println!("Solution to part one: {result}");
    println!("Elapsed time: {}s", (end - start).as_secs_f64());

    // let fname = "data/test_input_02";
    let result = solve_part_two(fname);
}
