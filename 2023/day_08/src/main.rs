#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::Itertools;

#[allow(clippy::missing_panics_doc)]
pub fn main() {
    let data = include_str!("input.txt");
    let (instructions, map) = data.split_once("\n\n").unwrap();
    let mut net = Network::<666>::new(map);
    println!("Part 1: {}", part_one::<666>(&mut net, instructions));
    println!("Part 2: {}", part_two::<666>(&net, instructions));
}

fn part_one<const N: usize>(net: &mut Network<N>, instructions: &str) -> usize {
    net.run(instructions)
}

fn part_two<const N: usize>(net: &Network<N>, instructions: &str) -> usize {
    net.data
        .iter()
        .enumerate()
        .filter(|&(_, &(n, _, _))| n.ends_with('A'))
        .map(|(i, _)| Network::<N> {
            data: net.data,
            current_node: (net.data[i].0, i),
        })
        .map(|mut n| n.run(instructions))
        .fold(1, |acc, v| v * (acc / gcd(acc, v)))
}

#[derive(Debug)]
struct Network<'a, const N: usize> {
    data: [(&'a str, usize, usize); N],
    current_node: (&'a str, usize),
}

impl<const N: usize> Network<'_, N> {
    fn run(&mut self, instructions: &str) -> usize {
        for (i, &dir) in instructions.as_bytes().iter().cycle().enumerate() {
            if self.current_node.0.ends_with('Z') {
                return i - 1;
            }
            self.step(dir);
        }
        0
    }

    fn step(&mut self, direction: u8) {
        let node = self.data[self.current_node.1];
        match direction {
            b'L' => self.current_node = (node.0, node.1),
            b'R' => self.current_node = (node.0, node.2),
            _ => unreachable!(),
        }
    }

    // Test code to explore the shape of the problem.
    // I'm deliberately leaving it in to prove I did my DD
    #[allow(dead_code)]
    fn find_cycle(&mut self, instructions: &str) {
        for (total, (instruction_step, &dir)) in instructions
            .as_bytes()
            .iter()
            .enumerate()
            .cycle()
            .enumerate()
            .take(100_000)
        {
            if self.current_node.0.ends_with('A') || self.current_node.0.ends_with('Z') {
                println!("{total} {instruction_step} {:?}", self.current_node);
            }
            self.step(dir);
        }
    }
}

impl<'a, const N: usize> Network<'a, N> {
    fn new(s: &'a str) -> Self {
        let nodes = s
            .lines()
            .map(|l| {
                let (node, dest) = l.split_once(" = ").unwrap();
                let (a, b) = dest
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .split_once(", ")
                    .unwrap();
                (node, a, b)
            })
            .sorted_unstable_by_key(|&(node, _, _)| node)
            .collect_vec();
        let mut data = [("", 0, 0); N];
        for (&(node, left, right), d) in nodes.iter().zip(data.iter_mut()) {
            let left = nodes.binary_search_by_key(&left, |&(n, _, _)| n).unwrap();
            let right = nodes.binary_search_by_key(&right, |&(n, _, _)| n).unwrap();
            *d = (node, left, right);
        }
        Self {
            data,
            current_node: (nodes[0].0, 0),
        }
    }
}

fn gcd(mut u: usize, mut v: usize) -> usize {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

    let gcd_exponent_on_two = (u | v).trailing_zeros();

    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    while u != v {
        if u < v {
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    u << gcd_exponent_on_two
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let (instructions, map) = data.split_once("\n\n").unwrap();
        let mut net = Network::<7>::new(map);
        assert_eq!(2, part_one::<7>(&mut net, instructions));
    }

    #[test]
    fn one_two() {
        let data = include_str!("test2.txt");
        let (instructions, map) = data.split_once("\n\n").unwrap();
        let mut net = Network::<3>::new(map);
        assert_eq!(6, part_one::<3>(&mut net, instructions));
    }

    #[test]
    fn two() {
        let data = include_str!("test3.txt");
        let (instructions, map) = data.split_once("\n\n").unwrap();
        let net = Network::<8>::new(map);
        assert_eq!(6, part_two::<8>(&net, instructions));
    }
}
