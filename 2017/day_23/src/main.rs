#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::str::FromStr;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> u32 {
    let mut p = Program::new(data);
    p.run();
    p.muls
}

fn part_two(data: &str) -> usize {
    // Extract the interesting numbers from the program
    let p = Program::new(data);
    let mut b = 1;
    let mut c = 1;
    let mut inc = 1;
    if let Instruction::Set((Register::B, Operand::Val(v))) = p.instructions[0] {
        b = v;
    }
    if let Instruction::Mul((Register::B, Operand::Val(v))) = p.instructions[4] {
        b *= v;
    }
    if let Instruction::Sub((Register::B, Operand::Val(v))) = p.instructions[5] {
        b -= v;
    }
    if let Instruction::Sub((Register::C, Operand::Val(v))) = p.instructions[7] {
        c = b - v;
    }
    if let Instruction::Sub((Register::B, Operand::Val(v))) = p.instructions[30] {
        inc = -v;
    }
    // Cast numbers into the right types
    let b = u32::try_from(b).unwrap();
    let c = u32::try_from(c).unwrap();
    let inc = usize::try_from(inc).unwrap();

    (b..=c).step_by(inc).filter(|&v| !is_prime(v)).count()
}

const fn is_prime(num: u32) -> bool {
    let mut test = 2;
    while (num / test) >= test {
        if num % test == 0 {
            return false;
        }
        test += 1;
    }
    true
}

const INSTRUCTIONS_SIZE: usize = 31;

#[derive(Debug)]
struct Program {
    instructions: [Instruction; INSTRUCTIONS_SIZE],
    registers: [i64; 8],
    instruction_pointer: usize,
    muls: u32,
}

impl Program {
    fn new(data: &str) -> Self {
        let mut program = data.lines().map(|l| l.parse::<Instruction>().unwrap());
        let instructions = std::array::from_fn(|_| program.next().unwrap());
        Self {
            instructions,
            registers: [0; 8],
            instruction_pointer: 0,
            muls: 0,
        }
    }

    #[allow(dead_code)]
    fn run(&mut self) {
        while self.instruction_pointer < INSTRUCTIONS_SIZE {
            self.step();
        }
    }

    fn step(&mut self) {
        match self.instructions[self.instruction_pointer] {
            Instruction::Set((x, y)) => self.set_value(x, self.get_operand(y)),
            Instruction::Sub((x, y)) => {
                self.set_value(x, self.get_register(x) - self.get_operand(y));
            }
            Instruction::Mul((x, y)) => {
                self.set_value(x, self.get_register(x) * self.get_operand(y));
                self.muls += 1;
            }
            Instruction::Jump((x, y)) => {
                if self.get_operand(x) != 0 {
                    let jump = self.get_operand(y) - 1;
                    if jump < 0 {
                        self.instruction_pointer -= usize::try_from(jump.abs()).unwrap();
                    } else {
                        self.instruction_pointer += usize::try_from(jump).unwrap();
                    }
                }
            }
        }
        self.instruction_pointer += 1;
    }

    const fn get_register(&self, x: Register) -> i64 {
        match x {
            Register::A => self.registers[0],
            Register::B => self.registers[1],
            Register::C => self.registers[2],
            Register::D => self.registers[3],
            Register::E => self.registers[4],
            Register::F => self.registers[5],
            Register::G => self.registers[6],
            Register::H => self.registers[7],
        }
    }

    const fn get_operand(&self, x: Operand) -> i64 {
        match x {
            Operand::Register(y) => self.get_register(y),
            Operand::Val(v) => v,
        }
    }

    fn set_value(&mut self, x: Register, v: i64) {
        match x {
            Register::A => self.registers[0] = v,
            Register::B => self.registers[1] = v,
            Register::C => self.registers[2] = v,
            Register::D => self.registers[3] = v,
            Register::E => self.registers[4] = v,
            Register::F => self.registers[5] = v,
            Register::G => self.registers[6] = v,
            Register::H => self.registers[7] = v,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Set((Register, Operand)),
    Sub((Register, Operand)),
    Mul((Register, Operand)),
    Jump((Operand, Operand)),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instr, data) = s.split_once(' ').unwrap();
        match instr {
            "set" => {
                let (x, y) = data
                    .split_once(' ')
                    .ok_or_else(|| format!("Could not split {data} into two parts for 'set'"))?;
                let x = x.parse()?;
                let y = y.parse()?;
                Ok(Self::Set((x, y)))
            }
            "sub" => {
                let (x, y) = data
                    .split_once(' ')
                    .ok_or_else(|| format!("Could not split {data} into two parts for 'sub'"))?;
                let x = x.parse()?;
                let y = y.parse()?;
                Ok(Self::Sub((x, y)))
            }
            "mul" => {
                let (x, y) = data
                    .split_once(' ')
                    .ok_or_else(|| format!("Could not split {data} into two parts for 'mul'"))?;
                let x = x.parse()?;
                let y = y.parse()?;
                Ok(Self::Mul((x, y)))
            }
            "jnz" => {
                let (x, y) = data
                    .split_once(' ')
                    .ok_or_else(|| format!("Could not split {data} into two parts for 'jnz'"))?;
                let x = x.parse()?;
                let y = y.parse()?;
                Ok(Self::Jump((x, y)))
            }
            _ => Err(format!("{instr} is not a valid instruction")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Register(Register),
    Val(i64),
}

impl FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().next().unwrap().is_ascii_alphabetic() {
            let inner = s.parse()?;
            return Ok(Self::Register(inner));
        }
        let inner = s.parse().map_err(|_| {
            format!("{s} cannot be parsed into either a register or a static value")
        })?;
        Ok(Self::Val(inner))
    }
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            return Err(format!(
                "{s} is expected to be only character long to represent a register"
            ));
        }
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            "e" => Ok(Self::E),
            "f" => Ok(Self::F),
            "g" => Ok(Self::G),
            "h" => Ok(Self::H),
            _ => Err(format!("{s} is not a valid register name")),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("input.txt");
        assert_eq!(0, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("input.txt");
        assert_eq!(0, part_two(data));
    }
}
