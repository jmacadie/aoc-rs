#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fmt::Display;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<1074>(data));
    println!("Part 2: {}", part_two::<1074>(data));
}

fn part_one<const N: usize>(data: &str) -> u32 {
    let mut p = Program::<N>::new(data);
    p.run(false)
}

fn part_two<const N: usize>(data: &str) -> u32 {
    let mut p = Program::<N>::new(data);
    p.run(true)
}

struct Program<const N: usize> {
    instructions: [i16; N],
    position: usize,
}

impl<const N: usize> Program<N> {
    fn new(data: &str) -> Self {
        let mut instructions = [0; N];
        for (l, i) in data.lines().zip(instructions.iter_mut()) {
            *i = l.parse().unwrap();
        }
        Self {
            instructions,
            position: 0,
        }
    }

    fn step(&mut self, inc_and_dec: bool) -> bool {
        let jump = self.instructions[self.position];
        let jump_abs = usize::try_from(jump.abs()).unwrap();
        let new_position = if jump >= 0 {
            self.position + jump_abs
        } else {
            self.position - jump_abs
        };
        if new_position >= N {
            return false;
        }
        if inc_and_dec && jump >= 3 {
            self.instructions[self.position] -= 1;
        } else {
            self.instructions[self.position] += 1;
        }
        self.position = new_position;
        true
    }

    fn run(&mut self, inc_and_dec: bool) -> u32 {
        let mut counter = 1;
        while self.step(inc_and_dec) {
            counter += 1;
        }
        counter
    }
}

impl<const N: usize> Display for Program<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.instructions.iter().enumerate() {
            if i == self.position {
                write!(f, "({v}) ")?;
            } else {
                write!(f, " {v}  ")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(5, part_one::<5>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(10, part_two::<5>(data));
    }
}
