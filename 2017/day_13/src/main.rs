#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<93>(data));
    println!("Part 2: {}", part_two::<93>(data));
}

fn part_one<const N: usize>(data: &str) -> usize {
    let firewall = Firewall::<N>::new(data);
    firewall.severity()
}

fn part_two<const N: usize>(data: &str) -> usize {
    let mut cycles = find_main_cycles(data);
    let mut count = 0;
    let mut step = 2;

    remove_solved_cycles(&mut cycles, &mut step);
    while !cycles.is_empty() {
        for (cycle, position) in &mut cycles {
            *position = (*position + step) % *cycle;
        }
        count += step;
        remove_solved_cycles(&mut cycles, &mut step);
    }

    let mut firewall = Firewall::<N>::new(data);
    firewall.step_by(count);
    while firewall.blocked() {
        firewall.step_by(step);
        count += step;
    }
    count
}

fn find_main_cycles(data: &str) -> Vec<(usize, usize)> {
    let mut main_cycles = Vec::new();

    for (cycle, positions) in &data
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(d, r)| (d.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
        .map(|(d, r)| (2 * (r - 1), d))
        .map(|(c, d)| (c, d % c))
        .sorted_by_key(|&(c, _)| c)
        .group_by(|&(c, _)| c)
    {
        let group: Vec<_> = positions.map(|(_, d)| d).sorted().collect();
        if group.len() == cycle / 2 - 1 {
            let cycle_gap = match (0..).step_by(2).zip(group.iter()).find(|&(c, &g)| c != g) {
                Some((c, _)) => c,
                None => cycle - 2,
            };
            main_cycles.push((cycle, cycle_gap));
        }
    }

    main_cycles
}

fn remove_solved_cycles(cycles: &mut Vec<(usize, usize)>, step: &mut usize) {
    if let Some(steps) = cycles
        .iter()
        .filter(|(_, p)| p == &0)
        .map(|(c, _)| c / 2)
        .optional_product()
    {
        *step *= steps;
        cycles.retain(|(_, p)| p != &0);
    }
}

trait OptionalProduct {
    fn optional_product(self) -> Option<usize>
    where
        Self: Sized + std::iter::Iterator<Item = usize>,
    {
        let mut product = 1;
        let mut any = false;

        for item in self {
            any = true;
            product *= item;
        }

        if any {
            Some(product)
        } else {
            None
        }
    }
}

impl<I> OptionalProduct for I where I: Iterator {}

#[derive(Debug)]
struct Firewall<const N: usize> {
    ranges: [usize; N],
    positions: [Option<usize>; N],
}

impl<const N: usize> Firewall<N> {
    fn new(input: &str) -> Self {
        let mut ranges = [0; N];
        let mut positions = [None; N];
        for (depth, range) in input
            .lines()
            .map(|l| l.split_once(": ").unwrap())
            .map(|(d, r)| (d.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
        {
            ranges[depth] = range;
            positions[depth] = Some(depth % (2 * (range - 1)));
        }
        Self { ranges, positions }
    }

    fn severity(&self) -> usize {
        self.positions
            .iter()
            .zip(self.ranges.iter())
            .enumerate()
            .filter(|&(_, (&p, _))| p == Some(0))
            .map(|(i, (_, &r))| i * r)
            .sum()
    }

    fn step_by(&mut self, val: usize) {
        for (p, &r) in self.positions.iter_mut().zip(self.ranges.iter()) {
            *p = p.map(|v| (v + val) % (2 * (r - 1)));
        }
    }

    fn blocked(&self) -> bool {
        self.positions.iter().any(|&p| p == Some(0))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(24, part_one::<7>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(10, part_two::<7>(data));
    }
}
