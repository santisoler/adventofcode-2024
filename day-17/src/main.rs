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
}

#[derive(Debug)]
struct ThreeBitsComputer {
    a: u32,
    b: u32,
    c: u32,
    program: Vec<u8>,
    pointer: usize,
}

impl ThreeBitsComputer {
    fn initialize(a: u32, b: u32, c: u32, program: Vec<u8>) -> Self {
        Self {
            a,
            b,
            c,
            program,
            pointer: 0,
        }
    }

    fn run(&mut self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];
        while self.pointer < self.program.len() {
            let instruction = self.program[self.pointer];
            let operand = self.program[self.pointer + 1];
            self.pointer += 2;
            match instruction {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => output.push(self.out(operand)),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                e => panic!("invalid instruction {e}"),
            }
        }
        output
    }

    fn get_combo_operand(&self, operand: u8) -> u32 {
        match operand {
            0..=3 => operand as u32,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Found invalid combo operand 7"),
            e => panic!("Found invalid combo operand '{e}'"),
        }
    }

    // Division between A and 2.pow(combo operand), truncate result and store in A.
    fn adv(&mut self, operand: u8) {
        let combo_operand = self.get_combo_operand(operand);
        self.a = self.a / 2_u32.pow(combo_operand);
    }

    // Bitwise XOR between B and literal operand. Result stored in B.
    fn bxl(&mut self, operand: u8) {
        self.b = self.b ^ operand as u32;
    }

    // Modulo between combo operand and 8. Results stored in B.
    fn bst(&mut self, operand: u8) {
        let combo_operand = self.get_combo_operand(operand);
        self.b = combo_operand % 8;
    }

    // Nothing if a = 0. Else, jump pointer to the position given by the literal operand.
    // In such case, the pointer doesn't jumps two steps after running this instruction.
    fn jnz(&mut self, operand: u8) {
        if self.a == 0 {
            return ();
        };
        self.pointer = operand as usize;
    }

    // Bitwise XOR between B and C. Result stored in B.
    fn bxc(&mut self, _operand: u8) {
        self.b = self.b ^ self.c;
    }

    // Combo operand % 8 and then output the value
    fn out(&mut self, operand: u8) -> u8 {
        let combo_operand = self.get_combo_operand(operand);
        (combo_operand % 8) as u8
    }

    // Division between A and 2.pow(combo operand), truncate result and store in B.
    fn bdv(&mut self, operand: u8) {
        let combo_operand = self.get_combo_operand(operand);
        self.b = self.a / 2_u32.pow(combo_operand);
    }

    // Division between A and 2.pow(combo operand), truncate result and store in C.
    fn cdv(&mut self, operand: u8) {
        let combo_operand = self.get_combo_operand(operand);
        self.c = self.a / 2_u32.pow(combo_operand);
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
        let value: u32 = parts.next().unwrap().trim().parse().unwrap();
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

fn solve_part_one(fname: &str) -> Vec<u8> {
    let mut computer = read_file(fname);
    println!("{:?}", computer);
    computer.run()
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {}", fmt_output(&result));
}
