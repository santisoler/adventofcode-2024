use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_program_1() {
        let (a, b, c) = (0, 0, 9);
        let program = vec![2, 6];
        let mut computer = ThreeBitsComputer::initialize(a, b, c, program);
        let out = computer.run();
        assert!(out.is_empty());
        assert_eq!(computer.b, 1);
    }

    #[test]
    fn test_simple_program_2() {
        let (a, b, c) = (10, 0, 0);
        let program = vec![5, 0, 5, 1, 5, 4];
        let mut computer = ThreeBitsComputer::initialize(a, b, c, program);
        let out = computer.run();
        assert_eq!(out, vec![0, 1, 2]);
    }

    #[test]
    fn test_simple_program_3() {
        let (a, b, c) = (2024, 0, 0);
        let program = vec![0, 1, 5, 4, 3, 0];
        let mut computer = ThreeBitsComputer::initialize(a, b, c, program);
        let out = computer.run();
        assert_eq!(out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.a, 0);
    }

    #[test]
    fn test_simple_program_4() {
        let (a, b, c) = (0, 29, 0);
        let program = vec![1, 7];
        let mut computer = ThreeBitsComputer::initialize(a, b, c, program);
        computer.run();
        assert_eq!(computer.b, 26);
    }

    #[test]
    fn test_simple_program_5() {
        let (a, b, c) = (0, 2024, 43690);
        let program = vec![4, 0];
        let mut computer = ThreeBitsComputer::initialize(a, b, c, program);
        computer.run();
        assert_eq!(computer.b, 44354);
    }

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn test_part_two() {
        let fname = "data/test_input_2";
        let result = solve_part_two(fname);
        assert_eq!(result, 117440);
    }
}

#[derive(Debug, Clone)]
struct ThreeBitsComputer {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
    pointer: usize,
}

impl ThreeBitsComputer {
    fn initialize(a: u64, b: u64, c: u64, program: Vec<u8>) -> Self {
        Self {
            a,
            b,
            c,
            program,
            pointer: 0,
        }
    }

    /// Run a full program
    fn run(&mut self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];
        while self.pointer < self.program.len() {
            match self.run_cycle() {
                Some(o) => output.push(o),
                None => (),
            };
        }
        output
    }

    /// Run one CPU cycle (execute a single instruction)
    fn run_cycle(&mut self) -> Option<u8> {
        let instruction = self.program[self.pointer];
        let operand = self.program[self.pointer + 1];
        self.pointer += 2;
        match instruction {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => return Some(self.out(operand)),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            e => panic!("invalid instruction {e}"),
        }
        None
    }

    /// Run full iteration of the program
    fn run_iteration(&mut self) -> u8 {
        // Force pointer to be at zero at the start of the iteration.
        self.pointer = 0;
        let mut output = 0;
        loop {
            match self.run_cycle() {
                Some(o) => output = o,
                None => (),
            }
            // Finish iteration when the pointer jumpted to zero or when it's outside the program.
            if self.pointer == 0 || self.pointer >= self.program.len() - 1 {
                break;
            }
        }
        output
    }

    fn get_combo_operand(&self, operand: u8) -> u32 {
        match operand {
            0..=3 => operand as u32,
            4 => self.a as u32,
            5 => self.b as u32,
            6 => self.c as u32,
            7 => panic!("Found invalid combo operand 7"),
            e => panic!("Found invalid combo operand '{e}'"),
        }
    }

    /// Division between A and 2.pow(combo operand), truncate result and store in A.
    fn adv(&mut self, operand: u8) {
        let combo_operand = self.get_combo_operand(operand);
        self.a = self.a / 2_u64.pow(combo_operand);
    }

    /// Bitwise XOR between B and literal operand. Result stored in B.
    fn bxl(&mut self, operand: u8) {
        self.b = self.b ^ operand as u64;
    }

    /// Modulo between combo operand and 8. Results stored in B.
    fn bst(&mut self, operand: u8) {
        let combo_operand = self.get_combo_operand(operand);
        self.b = (combo_operand % 8) as u64;
    }

    /// Nothing if a = 0. Else, jump pointer to the position given by the literal operand.
    /// In such case, the pointer doesn't jumps two steps after running this instruction.
    fn jnz(&mut self, operand: u8) {
        if self.a == 0 {
            return ();
        };
        self.pointer = operand as usize;
    }

    /// Bitwise XOR between B and C. Result stored in B.
    fn bxc(&mut self, _operand: u8) {
        self.b = self.b ^ self.c;
    }

    /// Combo operand % 8 and then output the value
    fn out(&mut self, operand: u8) -> u8 {
        let combo_operand = self.get_combo_operand(operand);
        (combo_operand % 8) as u8
    }

    /// Division between A and 2.pow(combo operand), truncate result and store in B.
    fn bdv(&mut self, operand: u8) {
        let combo_operand = self.get_combo_operand(operand);
        self.b = self.a / 2_u64.pow(combo_operand);
    }

    /// Division between A and 2.pow(combo operand), truncate result and store in C.
    fn cdv(&mut self, operand: u8) {
        let combo_operand = self.get_combo_operand(operand);
        self.c = self.a / 2_u64.pow(combo_operand);
    }
}

fn read_file(fname: &str) -> ThreeBitsComputer {
    let content = fs::read_to_string(fname).unwrap();
    let mut lines = content.lines();
    let mut registers = [0, 0, 0];
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        };
        let mut parts = line.split(":");
        let reg = parts
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .to_lowercase();
        let value: u64 = parts.next().unwrap().trim().parse().unwrap();
        match &reg[..] {
            "a" => registers[0] = value,
            "b" => registers[1] = value,
            "c" => registers[1] = value,
            _ => panic!(),
        }
    }
    let program: Vec<u8> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();
    ThreeBitsComputer::initialize(registers[0], registers[1], registers[2], program)
}

fn fmt_output(output: &Vec<u8>) -> String {
    let output: Vec<String> = output.iter().map(|c| format!("{c}")).collect();
    output.join(",")
}

/// Find the value of A that would make the computer to produce the same program.
///
/// This is a recursive function. The `a` argument should be the possible solution of A from the
/// previous step (or zero if it's the first step).
/// The iteration argument should be the index of the iteration that will be tested (use
/// `program.len() - 1` if  this is the first step).
fn solve_for_a(computer: &mut ThreeBitsComputer, a: u64, iteration: i32) -> Option<u64> {
    if iteration < 0 {
        return Some(a);
    }
    let expected = computer.program[iteration as usize];
    for next_bits in 0..=0b111 {
        let a_try = a * 2u64.pow(3 as u32) + next_bits;
        computer.a = a_try;
        let output = computer.run_iteration();
        if output == expected {
            if let Some(s) = solve_for_a(computer, a_try, iteration - 1) {
                return Some(s);
            }
        }
    }
    None
}

fn solve_part_one(fname: &str) -> Vec<u8> {
    let mut computer = read_file(fname);
    computer.run()
}

/// Solve part two of the puzzle.
///
/// Apply a DFS recursive algorithm to construct A three-bits at a time.
///
/// This solution works under the following assumptions:
///
/// * An iteration is a single run of all instructions in the program code sequentially.
///   We assume the program doesn't jump the pointer (besides the regular 2 steps) until the very
///   last instruction.
/// * The last instruction is `3,0`: jump pointer to the begining if A is not zero, halt otherwise.
/// * Each iteration overwrites the values of registers B and C, so no state of B and C should be
///   kept between iterations.
/// * Each iteration "pops" the last three bits of A, and that's the only change applied to the
///   register A. This means that it only runs adv (optcode 0) only once per iteration and always
///   with an operator 3 (`0,3`).
/// * Each iteration prints out a single time.
///
/// Both my personal input and the example provided by AoC satisfy these conditions.
///
/// With these assumptions in mind we can construct the bits of A by sets of three bits. We know
/// that A will have 3N bits, where N is the number of iterations (i.e. number of values in the
/// program code if A is a vaid solution). Since the last three bits of A are popped on each
/// iteration, the iteration n doesn't need to know about the last 3n bits of A. This means that
/// the last iteration (the ones that should print out `3` and leave A equal to zero) will only
/// care about the first three bits of A (since all the other ones were already popped by the
/// previous iterations). Therefore, we can start building A from its first three-bits, trying to
/// find which ones can output the same number as the corresponding value in the program.
/// By iterating over the numbers in the program code in reverse order (start from the last one
/// -`3`-, then with the previous one -`0`- and so on), we can build a value of A that can produce
/// the same code as the program.
///
/// There could be multiple solutions of three-bits for each one of the steps. But that doesn't
/// mean they are all valid. So, we need to be able to discard solutions. Moreover, we need to find
/// the lowest possible solution for A.
/// To solve it, I implemented a DFS algorithm implemented in the `solve_for_a` recursive function.
fn solve_part_two(fname: &str) -> u64 {
    let mut computer = read_file(fname);
    let n_iterations = computer.program.len() - 1;
    // Start solve_for_a with a starting `a = 0` and `iteration` as the last one.
    match solve_for_a(&mut computer, 0, n_iterations as i32) {
        Some(a) => a,
        None => panic!("Couldn't find optimal value for a"),
    }
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {}", fmt_output(&result));
    let result = solve_part_two(fname);
    println!("Solution to part two: {result}");
}
