#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fmt::Display;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<28>(data));
    println!("Part 2: {}", part_two::<28>(data));
}

fn part_one<const N: usize>(data: &str) -> u64 {
    let inputs = read_in::<N>(data);
    let target: u16 = inputs.iter().sum::<u16>() / 3;
    let min = min_selection(target, &inputs[..], Selection::new(), Selection::max());
    min.quantum_entanglement
}

fn part_two<const N: usize>(data: &str) -> u64 {
    let inputs = read_in::<N>(data);
    let target: u16 = inputs.iter().sum::<u16>() / 4;
    let min = min_selection(target, &inputs[..], Selection::new(), Selection::max());
    min.quantum_entanglement
}

#[derive(Clone, Copy, Debug)]
struct Selection {
    count: usize,
    quantum_entanglement: u64,
    items: [u16; 10],
}

impl Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Selected Items: ")?;
        for i in (1..self.count).rev() {
            write!(f, "{}, ", self.items[i])?;
        }
        writeln!(f, "{} - ({} items)", self.items[0], self.count)?;
        writeln!(f, "Quantum Entaglement: {}", self.quantum_entanglement)?;
        writeln!(f)?;
        Ok(())
    }
}

impl Selection {
    const fn new() -> Self {
        Self {
            count: 0,
            quantum_entanglement: 1,
            items: [0; 10],
        }
    }

    const fn max() -> Self {
        Self {
            count: 10,
            quantum_entanglement: u64::MAX,
            items: [0; 10],
        }
    }

    fn add(&mut self, item: u16) {
        self.items[self.count] = item;
        self.count += 1;
        self.quantum_entanglement *= u64::from(item);
    }

    const fn still_valid(&self, test: Self) -> bool {
        self.count <= test.count && self.quantum_entanglement < test.quantum_entanglement
    }
}

fn min_selection(
    target: u16,
    mut list: &[u16],
    current: Selection,
    mut min: Selection,
) -> Selection {
    while let Some((elem, rest)) = list.split_last() {
        list = rest;
        if elem > &target {
            continue;
        }
        let mut next = current;
        next.add(*elem);
        if !next.still_valid(min) {
            continue;
        }
        if elem == &target {
            return next;
        }
        min = min_selection(target - *elem, rest, next, min);
    }
    min
}

fn read_in<const N: usize>(data: &str) -> [u16; N] {
    let mut out = [0; N];
    for (elem, line) in out.iter_mut().zip(data.lines()) {
        *elem = line.parse().unwrap();
    }
    out
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(99, part_one::<10>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(44, part_two::<10>(data));
    }
}
