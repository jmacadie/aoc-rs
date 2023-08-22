#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{char, collections::HashMap, fmt::Display, str::FromStr};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<16>(data));
    println!("Part 2: {}", part_two::<16>(data));
}

fn part_one<const N: usize>(data: &str) -> String {
    let mut p = Promenade::<N>::new();
    p.dance(data);
    format!("{p}")
}

fn part_two<const N: usize>(data: &str) -> String {
    let mut p = Promenade::<N>::new();
    let mut seen = HashMap::new();
    seen.insert(format!("{p}"), 0);

    for i in 1_u32.. {
        p.dance(data);

        // We've seen this string representation before
        // Which means we've found a cycle and can use modulo arthimetic
        // to ignore all the cycles up to 1 billion
        // Then we only need find which key (i.e. string representation of the programs)
        // is the right number of steps into the cycle, and return that
        if let Some(start) = seen.get(&format!("{p}")) {
            let cycle = i - *start;
            let rem = (1_000_000_000 - *start) % cycle;
            for (k, v) in &seen {
                if *v == *start + rem {
                    return k.clone();
                }
            }
        }

        // Not seen this string representation before, so add it and the step count we're on
        seen.insert(format!("{p}"), i);
    }
    unreachable!()
}

struct Promenade<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Promenade<N> {
    fn new() -> Self {
        let data = std::array::from_fn(|i| u8::try_from(i).unwrap());
        Self { data }
    }

    fn step(&mut self, instr: Instruction) {
        match instr {
            Instruction::Spin(x) => {
                let next = std::array::from_fn(|i| {
                    let posn = (i + N - x) % N;
                    self.data[posn]
                });
                self.data = next;
            }
            Instruction::Exchange((a, b)) => self.data.swap(a, b),
            Instruction::Partner((a, b)) => {
                let posn_a = self.data.iter().position(|&v| v == a).unwrap();
                let posn_b = self.data.iter().position(|&v| v == b).unwrap();
                self.data.swap(posn_a, posn_b);
            }
        }
    }

    fn dance(&mut self, data: &str) {
        for instr in data
            .trim()
            .split(',')
            .map(|i| i.parse::<Instruction>().unwrap())
        {
            self.step(instr);
        }
    }
}

impl<const N: usize> Display for Promenade<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.data {
            let ch = char::from_u32((i + b'a').into()).unwrap();
            write!(f, "{ch}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Spin(usize),
    Exchange((usize, usize)),
    Partner((u8, u8)),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, data) = s.split_at(1);
        match prefix {
            "s" => {
                let val = data
                    .parse()
                    .map_err(|_| format!("{data} is not a number in the spin instruction {s}"))?;
                Ok(Self::Spin(val))
            }
            "x" => {
                let (a, b) = data.split_once('/').ok_or_else(|| {
                    format!("Was expecting a '/' delimiter in the exchange move {s}")
                })?;
                let a = a
                    .parse()
                    .map_err(|_| format!("{a} is not a number in the exchange instruction {s}"))?;
                let b = b
                    .parse()
                    .map_err(|_| format!("{a} is not a number in the exchange instruction {s}"))?;
                Ok(Self::Exchange((a, b)))
            }
            "p" => {
                let (a, b) = data.split_once('/').ok_or_else(|| {
                    format!("Was expecting a '/' delimiter in the partner move {s}")
                })?;
                // TODO: could do a whole lot more checking here but it's AoC and I know the inputs
                // will be good
                let a = a.as_bytes()[0] - b'a';
                let b = b.as_bytes()[0] - b'a';
                Ok(Self::Partner((a, b)))
            }
            _ => Err(format!("{s} doesn not begin with a valid move character")),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!("baedc", part_one::<5>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!("abcde", part_two::<5>(data));
    }
}
