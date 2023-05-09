#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{fmt::Display, str::FromStr};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

fn part_one(data: &str) -> u32 {
    let limit = get_magic_number(data);

    // The series 2^n plus the value from two steps previous will always produce alternating odd
    // then even numbers from successive integer division by two.
    // By anaylsing the assembly, we can see that this is what we need to get the value of b to
    // output alternating 0 and 1
    // The magic number extracted above, is variable by input and is added to the required answer,
    // so we need to loop up the series until we get to the first value above the limit.
    // We can then subtract the two to work out the smallest number we need add to get onto
    // this desired series
    let out = (0..)
        .take(100)
        .scan((0, 0), |(a, b), i| {
            let c = 2_u32.pow(i) + *a;
            *a = *b;
            *b = c;
            Some(*b)
        })
        .find(|v| v > &limit)
        .unwrap()
        - limit;

    // Run the programme just to check it works :)
    let mut p = Programme::new(data, i32::try_from(out).unwrap());
    p.run();
    println!();

    out
}

fn get_magic_number(data: &str) -> u32 {
    let mut all_lines = data.lines();
    let mut line1 = all_lines.nth(1).unwrap().split_whitespace();
    let mut line2 = all_lines.next().unwrap().split_whitespace();
    let (Some("cpy"), Some(x), Some("c"), None) = (line1.next(), line1.next(), line1.next(), line1.next()) else { unreachable!() };
    let (Some("cpy"), Some(y), Some("b"), None) = (line2.next(), line2.next(), line2.next(), line2.next()) else { unreachable!() };
    let x: u32 = x.parse().unwrap();
    let y: u32 = y.parse().unwrap();
    x * y
}

#[derive(Debug)]
struct Programme {
    instructions: [Instruction; 30],
    step: usize,
    registers: [i32; 4],
}

impl Display for Programme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Step: {}, a: {}, b: {}, c: {}, d: {}",
            self.step, self.registers[0], self.registers[1], self.registers[2], self.registers[3]
        )?;
        Ok(())
    }
}

impl Programme {
    fn new(data: &str, init: i32) -> Self {
        let mut instructions = [Instruction::Inc(Operand::Reg(Register::A)); 30];
        for (line, instruction) in data.lines().zip(instructions.iter_mut()) {
            *instruction = line.parse().unwrap();
        }
        Self {
            instructions,
            step: 0,
            registers: [init, 0, 0, 0],
        }
    }

    fn run(&mut self) {
        let mut counter = 0;
        while self.step < 30 && counter < 100_000 {
            self.run_step();
            counter += 1;
        }
    }

    fn run_step(&mut self) {
        match self.instructions.get(self.step).unwrap() {
            Instruction::Inc(op) => self.inc(*op),
            Instruction::Dec(op) => self.dec(*op),
            Instruction::Cpy((op1, op2)) => self.cpy(*op1, *op2),
            Instruction::Jnz((op1, op2)) => self.jnz(*op1, *op2),
            Instruction::Out(op) => self.out(*op),
        }
        self.step += 1;
    }

    fn inc(&mut self, op: Operand) {
        match op {
            Operand::Reg(r) => *self.get_mut(r) += 1,
            Operand::Val(_) => (), // Ignore, can't increment a static number
        }
    }

    fn dec(&mut self, op: Operand) {
        match op {
            Operand::Reg(r) => *self.get_mut(r) -= 1,
            Operand::Val(_) => (), // Ignore, can't decrement a static number
        }
    }

    fn cpy(&mut self, op1: Operand, op2: Operand) {
        match (op1, op2) {
            (Operand::Reg(from), Operand::Reg(to)) => {
                *self.get_mut(to) = self.get(from);
            }
            (Operand::Val(x), Operand::Reg(to)) => {
                *self.get_mut(to) = x;
            }
            (_, Operand::Val(_)) => (), // can't copy to a static value, do nothing
        }
    }

    fn jnz(&mut self, op1: Operand, op2: Operand) {
        match (op1, op2) {
            (Operand::Reg(test), Operand::Reg(jump)) => {
                if self.get(test) != 0 {
                    self.jump(self.get(jump));
                }
            }
            (Operand::Reg(test), Operand::Val(jump)) => {
                if self.get(test) != 0 {
                    self.jump(jump);
                }
            }
            (Operand::Val(test), Operand::Reg(jump)) => {
                if test != 0 {
                    self.jump(self.get(jump));
                }
            }
            (Operand::Val(test), Operand::Val(jump)) => {
                if test != 0 {
                    self.jump(jump);
                }
            }
        };
    }

    fn jump(&mut self, jump: i32) {
        if jump < 0 {
            self.step -= usize::try_from(jump.abs()).unwrap();
        } else {
            self.step += usize::try_from(jump).unwrap();
        }
        self.step -= 1;
    }

    fn out(&self, op: Operand) {
        match op {
            Operand::Reg(r) => print!("{}_", self.registers[r.to_index()]),
            Operand::Val(_) => (), // No Op
        }
    }

    fn get(&self, r: Register) -> i32 {
        *self.registers.get(r.to_index()).unwrap()
    }

    fn get_mut(&mut self, r: Register) -> &mut i32 {
        self.registers.get_mut(r.to_index()).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Inc(Operand),
    Dec(Operand),
    Jnz((Operand, Operand)),
    Cpy((Operand, Operand)),
    Out(Operand),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, rest) = s.split_once(' ').ok_or("No space in instruction")?;
        match op {
            "inc" => {
                let op = rest.parse()?;
                Ok(Self::Inc(op))
            }
            "dec" => {
                let op = rest.parse()?;
                Ok(Self::Dec(op))
            }
            "jnz" => {
                let (op1, op2) = rest.split_once(' ').ok_or("jnz needs two operands")?;
                let op1 = op1.parse()?;
                let op2 = op2.parse()?;
                Ok(Self::Jnz((op1, op2)))
            }
            "cpy" => {
                let (op1, op2) = rest.split_once(' ').ok_or("cpy needs two operands")?;
                let op1 = op1.parse()?;
                let op2 = op2.parse()?;
                Ok(Self::Cpy((op1, op2)))
            }
            "out" => {
                let op = rest.parse()?;
                Ok(Self::Out(op))
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Reg(Register),
    Val(i32),
}

impl FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::Reg(Register::A)),
            "b" => Ok(Self::Reg(Register::B)),
            "c" => Ok(Self::Reg(Register::C)),
            "d" => Ok(Self::Reg(Register::D)),
            _ => {
                let num: i32 = s
                    .parse()
                    .map_err(|_| "Operand not a number, and not a register 'a' to 'd'")?;
                Ok(Self::Val(num))
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    const fn to_index(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_one(data));
    }
}
