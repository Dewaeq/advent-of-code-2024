use aoc_runner_derive::aoc;

struct Program {
    pub register_a: i32,
    pub register_b: i32,
    pub register_c: i32,
    pub commands: Vec<u8>,
    pub output: Vec<i32>,
}

impl Program {
    fn run(&mut self) {
        let mut pointer = 0;

        while pointer < self.commands.len() - 1 {
            let opcode = self.commands[pointer];
            let instruction: Instruction = unsafe { std::mem::transmute(opcode) };
            let operand = instruction.operand(self.commands[pointer + 1], self);

            instruction.execute(operand, self, &mut pointer);

            if instruction != Instruction::Jnz {
                pointer += 2;
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn operand(&self, index: u8, program: &Program) -> i32 {
        match self {
            Instruction::Bxl | Instruction::Jnz => index as _,
            _ => match index {
                x @ 0..=3 => x as _,
                4 => program.register_a,
                5 => program.register_b,
                6 => program.register_c,
                _ => unreachable!(),
            },
        }
    }

    fn execute(self, operand: i32, program: &mut Program, pointer: &mut usize) {
        match self {
            Instruction::Adv => {
                program.register_a = program.register_a / 2i32.pow(operand as _);
            }
            Instruction::Bxl => {
                program.register_b = operand ^ program.register_b;
            }
            Instruction::Bst => {
                program.register_b = operand % 8;
            }
            Instruction::Jnz => {
                if program.register_a != 0 {
                    *pointer = operand as _;
                } else {
                    *pointer += 2;
                }
            }
            Instruction::Bxc => {
                program.register_b = program.register_b ^ program.register_c;
            }
            Instruction::Out => {
                program.output.push(operand % 8);
            }
            Instruction::Bdv => {
                program.register_b = program.register_a / 2i32.pow(operand as _);
            }
            Instruction::Cdv => {
                program.register_c = program.register_a / 2i32.pow(operand as _);
            }
        }
    }
}

fn parse(input: &str) -> Program {
    let mut lines = input.lines();
    let a = lines.next().unwrap()[12..].parse::<i32>().unwrap();
    let c = lines.next().unwrap()[12..].parse::<i32>().unwrap();
    let b = lines.next().unwrap()[12..].parse::<i32>().unwrap();
    lines.next().unwrap();

    let commands = lines.next().unwrap()[9..]
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    Program {
        register_a: a,
        register_b: b,
        register_c: c,
        commands,
        output: vec![],
    }
}

#[aoc(day17, part1)]
fn part1(input: &str) -> String {
    let mut program = parse(input);

    program.run();

    program
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[aoc(day17, part2)]
fn part2(input: &str) -> i32 {
    let mut program = parse(input);
    let b = program.register_b;
    let c = program.register_c;
    let commands = program.commands.clone();

    let target_output = commands.iter().map(|&c| c as i32).collect::<Vec<_>>();
    let mut a = 0;

    loop {
        program.register_a = a;
        program.register_b = b;
        program.register_c = c;
        program.commands = commands.clone();
        program.output.clear();

        program.run();

        if program.output == target_output {
            return a;
        }

        a += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const TEST2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST1), "4,6,3,5,6,3,5,2,1,0")
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST2), 117440)
    }
}
