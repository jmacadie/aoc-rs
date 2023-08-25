#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{str::FromStr, sync::mpsc, thread};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<41>(data));
    println!("Part 2: {}", part_two::<41>(data));
}

fn part_one<const N: usize>(data: &str) -> i64 {
    let (tx0, rx1) = mpsc::channel();
    let (tx1, rx0) = mpsc::channel();
    let (tx, _rx) = mpsc::channel();
    drop(tx1);

    let mut p = Program::<N>::new(data, 0, tx0, rx0, tx);
    let _ = thread::spawn(move || {
        p.run();
    });

    let mut frequency = 0;
    while let Ok(f) = rx1.recv() {
        frequency = f;
    }
    frequency
}

fn part_two<const N: usize>(data: &str) -> usize {
    let (tx0, rx1) = mpsc::channel();
    let (tx1, rx0) = mpsc::channel();
    let (tx, rx) = mpsc::channel();

    let mut p0 = Program::<N>::new(data, 0, tx0, rx0, tx.clone());
    let mut p1 = Program::<N>::new(data, 1, tx1, rx1, tx);

    let _ = thread::spawn(move || {
        p0.run();
    });

    let _ = thread::spawn(move || {
        p1.run();
    });

    let mut count = 0;
    let mut queue_size = [0, 0];
    let mut receiving = [false, false];

    loop {
        match rx.recv() {
            Err(_) => {
                // If we're getting an error then both the programs must have finished
                // naturally and closed their status channel
                return count;
            }
            Ok(MessageType::Send(x)) => {
                // Increment the other program's queue
                queue_size[1 - x] += 1;
                // Record the send count for the program we're tracking
                count += x;
            }
            Ok(MessageType::Receiving(x)) => {
                receiving[x] = true;
                // Deadlocked - this is a condition to terminate both programs & quit out
                if receiving[0] && receiving[1] && queue_size[0] == 0 && queue_size[1] == 0 {
                    return count;
                }
            }
            Ok(MessageType::Received(x)) => {
                receiving[x] = false;
                queue_size[x] -= 1;
            }
        }
    }
}

enum MessageType {
    Send(usize),
    Receiving(usize),
    Received(usize),
}

struct Program<const N: usize> {
    name: usize,
    instructions: [Instruction; N],
    registers: [i64; 5],
    instruction_pointer: usize,
    send_channel: mpsc::Sender<i64>,
    recv_channel: mpsc::Receiver<i64>,
    staus_channel: mpsc::Sender<MessageType>,
    other_side_closed: bool,
}

impl<const N: usize> Program<N> {
    fn new(
        data: &str,
        program_number: i64,
        send: mpsc::Sender<i64>,
        recv: mpsc::Receiver<i64>,
        status: mpsc::Sender<MessageType>,
    ) -> Self {
        let mut program = data.lines().map(|l| l.parse::<Instruction>().unwrap());
        let instructions = std::array::from_fn(|_| program.next().unwrap());
        Self {
            name: usize::try_from(program_number).unwrap(),
            instructions,
            registers: [0, 0, 0, 0, program_number],
            instruction_pointer: 0,
            send_channel: send,
            recv_channel: recv,
            staus_channel: status,
            other_side_closed: false,
        }
    }

    fn run(&mut self) {
        while !self.other_side_closed && self.instruction_pointer < N {
            self.step();
        }
    }

    fn step(&mut self) {
        match self.instructions[self.instruction_pointer] {
            Instruction::Set((x, y)) => self.set_value(x, self.get_operand(y)),
            Instruction::Add((x, y)) => {
                self.set_value(x, self.get_register(x) + self.get_operand(y));
            }
            Instruction::Mul((x, y)) => {
                self.set_value(x, self.get_register(x) * self.get_operand(y));
            }
            Instruction::Mod((x, y)) => {
                self.set_value(x, self.get_register(x) % self.get_operand(y));
            }
            Instruction::Send(x) => {
                let _ = self.staus_channel.send(MessageType::Send(self.name));
                let _ = self.send_channel.send(self.get_register(x));
            }
            Instruction::Recieve(x) => {
                let _ = self.staus_channel.send(MessageType::Receiving(self.name));
                if let Ok(msg) = self.recv_channel.recv() {
                    let _ = self.staus_channel.send(MessageType::Received(self.name));
                    self.set_value(x, msg);
                } else {
                    self.other_side_closed = true;
                }
            }
            Instruction::Jump((x, y)) => {
                if self.get_operand(x) > 0 {
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
            Register::F => self.registers[2],
            Register::I => self.registers[3],
            Register::P => self.registers[4],
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
            Register::F => self.registers[2] = v,
            Register::I => self.registers[3] = v,
            Register::P => self.registers[4] = v,
        }
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Set((Register, Operand)),
    Add((Register, Operand)),
    Mul((Register, Operand)),
    Mod((Register, Operand)),
    Send(Register),
    Recieve(Register),
    Jump((Operand, Operand)),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instr, data) = s.split_once(' ').unwrap();
        match instr {
            "snd" => {
                let inner = data.parse()?;
                Ok(Self::Send(inner))
            }
            "set" => {
                let (x, y) = data
                    .split_once(' ')
                    .ok_or_else(|| format!("Could not split {data} into two parts for 'set'"))?;
                let x = x.parse()?;
                let y = y.parse()?;
                Ok(Self::Set((x, y)))
            }
            "add" => {
                let (x, y) = data
                    .split_once(' ')
                    .ok_or_else(|| format!("Could not split {data} into two parts for 'add'"))?;
                let x = x.parse()?;
                let y = y.parse()?;
                Ok(Self::Add((x, y)))
            }
            "mul" => {
                let (x, y) = data
                    .split_once(' ')
                    .ok_or_else(|| format!("Could not split {data} into two parts for 'mul'"))?;
                let x = x.parse()?;
                let y = y.parse()?;
                Ok(Self::Mul((x, y)))
            }
            "mod" => {
                let (x, y) = data
                    .split_once(' ')
                    .ok_or_else(|| format!("Could not split {data} into two parts for 'mod'"))?;
                let x = x.parse()?;
                let y = y.parse()?;
                Ok(Self::Mod((x, y)))
            }
            "rcv" => {
                let inner = data.parse()?;
                Ok(Self::Recieve(inner))
            }
            "jgz" => {
                let (x, y) = data
                    .split_once(' ')
                    .ok_or_else(|| format!("Could not split {data} into two parts for 'mod'"))?;
                let x = x.parse()?;
                let y = y.parse()?;
                Ok(Self::Jump((x, y)))
            }
            _ => Err(format!("{instr} is not a valid instruction")),
        }
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
enum Register {
    A,
    B,
    F,
    I,
    P,
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
            "f" => Ok(Self::F),
            "i" => Ok(Self::I),
            "p" => Ok(Self::P),
            _ => Err(format!("{s} is not a valid register name")),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(4, part_one::<10>(data));
    }
}
