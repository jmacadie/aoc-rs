#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{collections::HashMap, fmt::Display};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two::<100>(data));
}

fn part_one(data: &str) -> usize {
    let size = data.lines().next().unwrap().chars().count();
    let platform = data.as_bytes();
    (0..size)
        .map(|col| {
            let mut avail = size;
            (0..size).fold(0, |acc, row| match platform[row * (size + 1) + col] {
                b'O' => {
                    avail -= 1;
                    acc + avail + 1
                }
                b'#' => {
                    avail = size - row - 1;
                    acc
                }
                b'.' => acc,
                _ => unreachable!(),
            })
        })
        .sum()
}

fn part_two<const S: usize>(data: &str) -> usize {
    let mut p = Platform::<S>::new(data);
    p.run(1_000_000_000);
    p.load()
}

#[derive(Debug)]
struct Platform<const S: usize> {
    rocks: [[Rock; S]; S],
    seen: HashMap<[[Rock; S]; S], usize>,
}

impl<const S: usize> Platform<S> {
    fn new(data: &str) -> Self {
        let mut rocks = [[Rock::None; S]; S];
        data.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, &val)| match val {
                b'O' => rocks[i / (S + 1)][i % (S + 1)] = Rock::Rolling,
                b'#' => rocks[i / (S + 1)][i % (S + 1)] = Rock::Fixed,
                b'.' | b'\n' => {}
                _ => unreachable!(),
            });
        Self {
            rocks,
            seen: HashMap::new(),
        }
    }

    fn load(&self) -> usize {
        self.rocks
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| (i + 1) * row.iter().filter(|&&x| x == Rock::Rolling).count())
            .sum()
    }

    fn run(&mut self, cycles: usize) {
        let mut cycle_size = 1;
        let current = (0..)
            .take(cycles)
            .find(|&c| {
                if let Some(from) = self.seen.get(&self.rocks) {
                    cycle_size = c - from;
                    return true;
                }
                self.seen.insert(self.rocks, c);
                self.cycle();
                false
            })
            .unwrap();
        let remaining = (cycles - current) % cycle_size;
        for _ in 0..remaining {
            self.cycle();
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn tilt_north(&mut self) {
        (0..S).for_each(|col| {
            let mut avail = 0;
            (0..S).for_each(|row| match self.rocks[row][col] {
                Rock::Rolling => {
                    self.rocks[row][col] = Rock::None;
                    self.rocks[avail][col] = Rock::Rolling;
                    avail += 1;
                }
                Rock::Fixed => {
                    avail = row + 1;
                }
                Rock::None => {}
            });
        });
    }

    fn tilt_south(&mut self) {
        (0..S).for_each(|col| {
            let mut avail = S - 1;
            (0..S).rev().for_each(|row| match self.rocks[row][col] {
                Rock::Rolling => {
                    self.rocks[row][col] = Rock::None;
                    self.rocks[avail][col] = Rock::Rolling;
                    avail = avail.saturating_sub(1);
                }
                Rock::Fixed => {
                    avail = row.saturating_sub(1);
                }
                Rock::None => {}
            });
        });
    }

    fn tilt_west(&mut self) {
        (0..S).for_each(|row| {
            let mut avail = 0;
            (0..S).for_each(|col| match self.rocks[row][col] {
                Rock::Rolling => {
                    self.rocks[row][col] = Rock::None;
                    self.rocks[row][avail] = Rock::Rolling;
                    avail += 1;
                }
                Rock::Fixed => {
                    avail = col + 1;
                }
                Rock::None => {}
            });
        });
    }

    fn tilt_east(&mut self) {
        (0..S).for_each(|row| {
            let mut avail = S - 1;
            (0..S).rev().for_each(|col| match self.rocks[row][col] {
                Rock::Rolling => {
                    self.rocks[row][col] = Rock::None;
                    self.rocks[row][avail] = Rock::Rolling;
                    avail = avail.saturating_sub(1);
                }
                Rock::Fixed => {
                    avail = col.saturating_sub(1);
                }
                Rock::None => {}
            });
        });
    }
}

impl<const S: usize> Display for Platform<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rocks {
            for cell in row {
                match cell {
                    Rock::Rolling => write!(f, "O")?,
                    Rock::Fixed => write!(f, "#")?,
                    Rock::None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Rock {
    Rolling,
    Fixed,
    None,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(136, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(64, part_two::<10>(data));
    }
}
