use aoc_runner_derive::aoc;

struct Program {
    a: u64,
    b: u64,
    c: u64,
    commands: Vec<u8>,
    output: Vec<u8>,
}

impl Program {
    fn run(&mut self) {
        let mut pointer = 0;

        while pointer < self.commands.len() - 1 {
            let opcode = self.commands[pointer];
            let instruction: Instruction = unsafe { std::mem::transmute(opcode) };
            let operand = instruction.operand(self.commands[pointer + 1], self);

            instruction.execute(operand, self, &mut pointer);
        }
    }

    fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.output.clear();
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
    fn operand(&self, index: u8, program: &Program) -> u64 {
        match self {
            Instruction::Bxl | Instruction::Jnz => index as _,
            _ => match index {
                x @ 0..=3 => x as _,
                4 => program.a,
                5 => program.b,
                6 => program.c,
                _ => unreachable!(),
            },
        }
    }

    fn execute(self, operand: u64, program: &mut Program, pointer: &mut usize) {
        match self {
            Instruction::Adv => program.a = program.a / 2u64.pow(operand as _),
            Instruction::Bxl => program.b = operand ^ program.b,
            Instruction::Bst => program.b = operand % 8,
            Instruction::Jnz => {
                if program.a != 0 {
                    *pointer = operand as _;
                } else {
                    *pointer += 2;
                }
            }
            Instruction::Bxc => program.b = program.b ^ program.c,
            Instruction::Out => program.output.push((operand % 8) as _),
            Instruction::Bdv => program.b = program.a / 2u64.pow(operand as _),
            Instruction::Cdv => program.c = program.a / 2u64.pow(operand as _),
        }

        if self != Instruction::Jnz {
            *pointer += 2;
        }
    }
}

fn parse(input: &str) -> Program {
    let mut lines = input.lines();
    let a = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let c = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let b = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    lines.next().unwrap();

    let commands = lines.next().unwrap()[9..]
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    Program {
        a,
        b,
        c,
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
fn part2(input: &str) -> u64 {
    // 2,4,1,5,7,5,0,3,1,6,4,3,5,5,3,0
    //
    // (2 4) => b = a % 8
    // (1 5) => b = b ^ 5
    // (7 5) => c = a // 2**b
    // (0 3) => a = a // 2**3
    // (1 6) => b = b ^ 6
    // (4 3) => b = b ^ c
    // (5 5) => output.push(b % 8)	(*)
    // (3 0) => repeat if a != 0

    // so:
    // b = a % 8
    // b = b ^ 5
    // c = a >> b
    // a = a >> 3
    // b = b ^ 6
    // b = b ^ c
    // out(b % 8)
    // repeat if a != 0
    //
    // which simplifies to:
    // while a != 0 {
    //     out(f(a) % 8)
    //     a = a >> 3
    // }
    //
    // so, the plan is to iteratively find each 3 bit chunk of A

    let mut program = parse(input);
    let target_output = program.commands.clone();
    let n = target_output.len();

    let mut parts = vec![0];

    for i in (0..n).rev() {
        let mut new_parts = vec![];

        for part in &parts {
            for next_part in 0..8 {
                let a = (part << 3) + next_part;
                program.a = a;
                program.run();

                if program.output == target_output[i..] {
                    new_parts.push(a);
                }

                program.reset();
            }
        }

        parts = new_parts;
    }

    *parts.iter().min().unwrap()
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
