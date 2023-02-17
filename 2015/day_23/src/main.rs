#![warn(clippy::all, clippy::pedantic)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<47>(data));
    println!("Part 2: {}", part_two::<47>(data));
}

fn part_one<const N: usize>(data: &str) -> usize {
    let mut program: Program<N> = Program::new(data, 0);
    program.run();
    program.b
}

fn part_two<const N: usize>(data: &str) -> usize {
    let mut program: Program<N> = Program::new(data, 1);
    program.run();
    program.b
}

#[derive(Debug)]
struct Program<const N: usize> {
    instructions: [Instruction; N],
    instruction: usize,
    a: usize,
    b: usize,
}

impl<const N: usize> Program<N> {
    fn new(data: &str, a: usize) -> Self {
        let mut lines = data.lines();
        let instructions = std::array::from_fn(|_| Instruction::from_str(lines.next().unwrap()));
        Self {
            instructions,
            instruction: 0,
            a,
            b: 0,
        }
    }

    fn run(&mut self) {
        while !self.is_terminated() {
            self.step();
        }
    }

    fn step(&mut self) {
        match self.instructions[self.instruction] {
            Instruction::Half(Register::A) => self.a /= 2,
            Instruction::Half(Register::B) => self.b /= 2,
            Instruction::Triple(Register::A) => self.a *= 3,
            Instruction::Triple(Register::B) => self.b *= 3,
            Instruction::Increment(Register::A) => self.a += 1,
            Instruction::Increment(Register::B) => self.b += 1,
            Instruction::Jump((dir, num)) => self.jump(dir, num),
            Instruction::JumpEven((reg, dir, num)) => {
                if self.val(reg) % 2 == 0 {
                    self.jump(dir, num);
                }
            }
            Instruction::JumpOne((reg, dir, num)) => {
                if self.val(reg) == 1 {
                    self.jump(dir, num);
                }
            }
        }
        self.instruction += 1;
    }

    fn jump(&mut self, dir: JumpDirection, offset: usize) {
        match dir {
            JumpDirection::Forwards => self.instruction += offset - 1,
            JumpDirection::Backwards => self.instruction -= offset + 1,
        }
    }

    fn val(&self, reg: Register) -> usize {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
        }
    }

    fn is_terminated(&self) -> bool {
        self.instruction >= N
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump((JumpDirection, usize)),
    JumpEven((Register, JumpDirection, usize)),
    JumpOne((Register, JumpDirection, usize)),
}

impl Instruction {
    fn from_str(line: &str) -> Self {
        let (instr, rest) = line.split_once(' ').unwrap();
        match instr {
            "hlf" => Self::Half(Register::from_str(rest)),
            "tpl" => Self::Triple(Register::from_str(rest)),
            "inc" => Self::Increment(Register::from_str(rest)),
            "jmp" => Self::Jump(Self::get_jump(rest)),
            "jie" => Self::JumpEven(Self::get_cond_jump(rest)),
            "jio" => Self::JumpOne(Self::get_cond_jump(rest)),
            _ => unreachable!(),
        }
    }

    fn get_cond_jump(jump_str: &str) -> (Register, JumpDirection, usize) {
        let (register, jump) = jump_str.split_once(", ").unwrap();
        let (dir, num) = Self::get_jump(jump);
        (Register::from_str(register), dir, num)
    }

    fn get_jump(jump_str: &str) -> (JumpDirection, usize) {
        let (dir, num) = jump_str.split_at(1);
        match dir {
            "+" => (JumpDirection::Forwards, num.parse().unwrap()),
            "-" => (JumpDirection::Backwards, num.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Register {
    A,
    B,
}

impl Register {
    fn from_str(line: &str) -> Self {
        match line {
            "a" => Register::A,
            "b" => Register::B,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum JumpDirection {
    Forwards,
    Backwards,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(2, part_one::<4>(data));
    }
}
